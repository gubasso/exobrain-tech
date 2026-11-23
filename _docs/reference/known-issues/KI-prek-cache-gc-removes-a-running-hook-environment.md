---
upstream: https://github.com/j178/prek/issues/3066
affects: prek cache gc while a prek hook run is in progress
state: monitoring
filing: filed
workaround: none, a rerun of the failed hook run passes
retire_when: a prek release keeps the store lock through the hook run, or teaches cache gc about live runs
checked: 2026-11-23
---

# prek cache gc removes the clone and environment of a hook that still runs

prek 0.5.7 to 0.5.11. The last check on 2026-11-23 read prek 0.5.11 and the prek trunk at `a4f7dcf`. No release fixes the defect.

## Symptom

A hook fails with a missing file under the prek store, such as `cat: <store>/repos/<hash>/payload: No such file or directory`. A second `prek run` in the same project passes.

## How it works

Three parts act: the store lock, the hook run of `prek run`, and the sweep of `prek cache gc`.

1. `prek run` takes the store lock in `ensure_hooks_installed` of `cli/run/run.rs`. It clones each hook repository and installs each environment under that lock.
2. `prek run` releases the lock before it starts the hooks. The hooks run with no lock.
3. A configuration on disk moves a `rev` while a hook runs, through a checkout, an edit, or `prek update`.
4. `prek cache gc` in `cli/cache_gc.rs` takes the free lock. It keeps only the clones and environments that the configurations on disk pin at that moment.
5. The sweep removes the clone and the environment of the old `rev`. It also removes the `scratch/` folder whole, where a hook with a shell entry writes its script as it starts.
6. The running hook reads a file of its clone, and the file is gone. That hook run fails.

The cause is that the lock covers the install and not the run. A triager first reads the failure as a broken hook repository. That reading is wrong, because a rerun installs the new `rev` and passes. The smallest fix holds a shared store lock through the last hook and lets `gc` take it exclusively.

The upstream report reproduced the failure on prek 0.5.10 in 3 of 3 runs. The code path is the same in 0.5.7, 0.5.11, and the trunk at `a4f7dcf`. The local evidence is the test `weekly_prek_gc_removes_the_clone_of_a_running_hook_whose_config_moved_its_rev` in `crates/exobrain/tests/cmd_store_guard.rs`, which asserts the race.

## Signal

A hook fails with `No such file or directory` on a path under `<store>/repos/` or `<store>/scratch/`, and a `prek cache gc` ran at the same time. `exobrain store-guard --daily` runs that `gc` once a week.

## Workaround

None. The effect is one failed hook run, and a rerun passes. `cli:store-guard-daily-runs-prek-gc-once-a-week` in `_docs/specs/SPEC-cli.md` runs that `gc` at most once a week per store, so the window opens rarely.

## Report

````markdown
### Before submitting

- [x] I have searched the issue list for similar bug reports.

### Summary

`prek cache gc` can delete the repository clone of a hook while that hook still runs. The hook fails if it reads a file of the clone after the sweep.

`prek run` holds the store lock while it clones and installs, and releases it before `run_hooks`. `cache gc` takes the same lock, but it keeps only the repositories and environments that the tracked configurations require at that moment. If a configuration changes `rev` during a run, for example through a checkout, an edit, or `prek update`, the sweep can remove the clone of the old `rev` that the running hook uses. Hooks with a managed environment can lose an old-revision environment the same way.

Code references at v0.5.10, in `crates/prek/src`:

- `cli/run/run.rs`: the store lock covers initialization (lines 157 to 169) and environment installation (lines 445 to 465). Both guards drop before `run_hooks` (line 302).
- `cli/cache_gc.rs`: the keep sets come from the tracked configurations on disk (lines 172 to 235), and the sweep follows (lines 245 to 260). Nothing registers the resources of a live run.
- `cli/cache_gc.rs` lines 290 to 293 also remove the whole `scratch/` folder. `hook_entry.rs` lines 219 to 230 write the script of a shell entry there before the hook runs. I read this second path in the code, but the reproduction below does not exercise it.

The same scopes exist in v0.5.11 and on main at 516c5e0. v0.5.11 replaced `lock_async` with `StoreGuard`, but the guards still drop before the hooks run.

Minimal reproduction. It needs `git` and `prek` on `PATH`, and uses a temporary `PREK_HOME`:

```sh
#!/bin/sh
# Reproduce: prek cache gc removes the clone of a hook that is still running.
set -eu
work=$(mktemp -d)
export PREK_HOME="$work/store"
export GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1
g() { git -c user.name=t -c user.email=t@t -c commit.gpgsign=false "$@"; }

# A hook repository with a script hook that waits, then reads a file of its clone.
mkdir "$work/hooks" && cd "$work/hooks" && git init -q
cat > .pre-commit-hooks.yaml <<'Y'
- id: slow
  name: slow
  entry: slow.sh
  language: script
  pass_filenames: false
  always_run: true
Y
cat > slow.sh <<S
#!/bin/sh
d=\$(dirname "\$0")
: > "$work/started"
while [ ! -e "$work/go" ]; do sleep 0.1; done
cat "\$d/payload"
S
chmod +x slow.sh
echo v1 > payload
g add -A && g commit -qm v1 && g tag v1
echo v2 > payload
g commit -qam v2 && g tag v2

# A project that pins v1.
mkdir "$work/project" && cd "$work/project" && git init -q
pin() { printf 'repos:\n  - repo: %s\n    rev: %s\n    hooks:\n      - id: slow\n' "$work/hooks" "$1" > .pre-commit-config.yaml; }
pin v1
g add -A && g commit -qm init

# Start the run, and wait until the hook runs.
prek run --all-files > "$work/run.log" 2>&1 &
run=$!
while [ ! -e "$work/started" ]; do sleep 0.1; done

# Move the rev while the hook runs, as a checkout or an update does, then gc.
pin v2
prek cache gc
touch "$work/go"

if wait "$run"; then echo "RESULT: hook passed (no race)"; else echo "RESULT: hook failed"; fi
cat "$work/run.log"
```

Output on prek 0.5.10. It failed the same way in 3 of 3 runs. Temporary paths are shortened, and the warning about the mutable `rev` is left out:

```text
Removed 1 repo (2.0KiB)
RESULT: hook failed
slow.....................................................................Failed
- hook id: slow
- exit code: 1
- files were modified by this hook

  cat: /tmp/tmp.XXXXXXXXXX/store/repos/d9537de0caff91ad/payload: No such file or directory
```

The line "files were modified by this hook" comes from the configuration edit during the run, not from the hook. A second `prek run --all-files` in the same project passed in all three runs.

In practice, a scheduled `prek cache gc` can fail a git hook that runs at the same moment.

Possible fixes:

- Hold a shared store lock for the whole run, from install through the last hook, and let `cache gc` take it exclusively.
- Or register the repositories, environments, and `scratch/` entries of each live run, and let `cache gc` keep them.

### Platform

Linux 7.3.0-1-default x86_64 GNU/Linux

### Version

prek 0.5.10

### .pre-commit-config.yaml

```yaml
repos:
  - repo: /tmp/tmp.XXXXXXXXXX/hooks
    rev: v1
    hooks:
      - id: slow
```

The reproduction writes this file, then changes `rev` to `v2` while the hook runs.

### Log file

The reproduction run with `prek -vvv run --all-files`. These lines show the store lock released before the hook starts, and the failure after the sweep:

```text
DEBUG Acquired lock resource="store"
TRACE Released lock path=/tmp/tmp.XXXXXXXXXX/store/.lock
DEBUG Hooks going to run: ["slow"]
TRACE run{hook_id=slow language=script}: Executing `cd /tmp/tmp.XXXXXXXXXX/project && /bin/sh /tmp/tmp.XXXXXXXXXX/store/repos/08f0a93199734632/slow.sh`
slow.....................................................................Failed
  cat: /tmp/tmp.XXXXXXXXXX/store/repos/08f0a93199734632/payload: No such file or directory
```
````

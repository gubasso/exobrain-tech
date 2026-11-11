# CLI Specification

## Purpose

Rules for the `exobrain` binary that `crates/exobrain` builds: its verbs, their exit codes, and their output. The binary enforces rules that other specs own. `store-guard` finds the store copies that `project-platform:a-flake-is-read-from-its-git-tree` forbids, and `SPEC-project-platform.md` owns that rule. This spec owns only how the guard finds, reports, and schedules the copies.

## Requirements

### `cli:store-guard-finds-copies-of-this-checkout` — store-guard finds the store copies of this checkout

When `exobrain store-guard` runs inside a git work tree, the guard MUST report each `-source` entry of the Nix store whose `.git` ties it to this checkout and whose registered NAR size reaches `--min-size`, with its store path, NAR size, registration date, and origin. A `.git` file ties a copy when its absolute `gitdir:` equals the git common dir or lies under `<common dir>/worktrees/`. A `.git` folder ties a copy when its normalized `remote.origin.url` equals the checkout's. The guard lists the store once, stats one `.git` per entry, and reads size and age from one `nix path-info --json` call. A relative or malformed `gitdir:`, an unreadable copy, or a `.git` folder with no remote goes under "origin unknown". The guard drops a path that `nix` no longer reports.

#### Scenario: A worktree is removed after its copy reached the store

- GIVEN a store copy whose `.git` file points under `<common dir>/worktrees/feat`
- WHEN `git worktree remove` deletes that worktree and the guard runs from the main checkout
- THEN the guard still reports the copy, with the origin `worktree feat (removed)`

Verify: `cargo nextest run -E 'test(finds_copies_of_this_checkout)'`

### `cli:store-guard-exit-codes` — store-guard exits by what it found

The guard MUST exit 0 when the checkout has no store copy, 1 when it has at least one, and 2 on a usage or environment error. With `--daily`, the guard MUST exit 0 in every case.

#### Scenario: The guard runs outside git

- GIVEN a folder that no git work tree holds
- WHEN the operator runs `exobrain store-guard` there, and `.envrc` runs `exobrain store-guard --daily` there
- THEN the operator run exits 2 with the reason on stderr, and the daily run exits 0 and prints nothing

Verify: `cargo nextest run -E 'test(exit_codes)'`

### `cli:store-guard-daily-runs-once-a-day` — The daily guard scans at most once a day

When `exobrain store-guard --daily` runs, the guard MUST take the checkout's lock, skip the scan if the stamp holds today's local date, and otherwise write today's date to the stamp atomically before it scans. The lock is an `O_EXCL` file beside the stamp that holds the owner's pid. A run takes the lock over when procfs shows its owner gone, or after 15 minutes where procfs cannot judge. A second concurrent run exits without a scan. The stamp is `${XDG_STATE_HOME:-$HOME/.local/state}/exobrain/store-guard/<basename>-<digest16>.stamp`, and the digest is the SHA-256 of the canonical checkout path. A state folder the guard cannot write skips the scan with one line on stderr. A clean store prints nothing, and copies print a three-line warning on stderr.

#### Scenario: The first scan of the day fails

- GIVEN a checkout whose stamp holds yesterday's date
- WHEN the daily run writes today's stamp and its `nix path-info` call then fails
- THEN no later run scans again that day, because the stamp landed before the scan

Verify: `cargo nextest run -E 'test(daily_runs_once_a_day)'`

### `cli:store-guard-stays-quiet-when-switched-off` — The daily guard stays off when switched off

If `EXOBRAIN_STORE_GUARD` is `0`, or one of `CI`, `GITHUB_ACTIONS`, `GITLAB_CI`, `BUILDKITE`, `CIRCLECI`, and `TF_BUILD` holds a value other than empty, `0`, or `false` in any case, then `store-guard --daily` MUST exit 0 without a scan and without a stamp. The switches bind only `--daily`, and an operator run always scans.

#### Scenario: A CI runner exports an empty CI variable

- GIVEN an environment with `CI` set to the empty string
- WHEN `.envrc` runs `exobrain store-guard --daily`
- THEN the guard runs and stamps, because an empty value counts as an explicit no

Verify: `cargo nextest run -E 'test(daily_stays_quiet_when_switched_off)'`

### `cli:output-is-human-or-json` — Output is human text or one JSON object

The binary MUST print its result on stdout, as human text by default and as one JSON object under `--json`, and MUST print every diagnostic on stderr. The `store-guard` object holds `checkout`, `copies` with `path`, `nar_size`, `registered`, and `origin` for each copy, `unattributed` with `path` and `reason` for each copy of unknown origin, and `total_nar_size`. `registered` is an RFC 3339 timestamp in UTC. The human report ends with the total, the remedy "run `nix store gc` once no process uses these paths", and the rule ID `project-platform:a-flake-is-read-from-its-git-tree`.

#### Scenario: A script reads the report

- GIVEN a checkout with one store copy and one copy of unknown origin
- WHEN a script runs `exobrain store-guard --json`
- THEN stdout parses as one object with exactly the documented keys, and the exit code is 1

Verify: `cargo nextest run -E 'test(output_is_human_or_json)'`

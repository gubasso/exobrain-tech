# Start a workspace off another line while the main directory is busy

Creates the second working directory parented on the release line `master` while the main directory
holds unfinished work on a different line, leaving that work untouched. Covers the start of the run
only; landing is [main-target-remote/recipe.md](./main-target-remote/recipe.md) or
[main-target-local/recipe.md](./main-target-local/recipe.md). Names carry a kind prefix while they
are only names in this repo, per [README.md](../README.md).

The steps run one worked example end to end: a repo at `~/code/repo` whose working copy holds an
edit on the change `feat: teach parser about arrays`, stacked on `develop@origin`, and a viewport
fix started from `master@origin` in `../wkspc-feat-y/`.

## The whole run

```sh
jj git fetch
jj workspace add -r master@origin ../wkspc-feat-y
cd ../wkspc-feat-y
$EDITOR src/render.rs
jj describe -m "fix: clamp viewport to the frame"
jj bookmark set bkmrk-feat-y
```

## Prerequisites

- `jj` 0.44 or later is on `PATH`.
  - check: `jj --version`
- The current directory is the workspace the repo was created with, named `default`.
  - check: `jj workspace list` names it `default`
- The repo has a Git remote named `origin` carrying the bookmark `master`.
  - check: `jj bookmark list --all-remotes` lists `master` with an `@origin:` line under it
- The parent directory of the repo is writable.
  - check: `test -w ..`
- The main directory carries work in progress, described or not, on a line that is not `master`.

## Steps

1. Fetch the remote so `master@origin` names the release tip.

   ```sh
   jj git fetch
   ```

   ```text
   bookmark: master@origin [updated] tracked
   ```

2. Record what the main directory holds, so step 4 has something to compare against.

   ```sh
   jj st
   ```

   ```text
   Working copy changes:
   M src/lexer.rs
   Working copy  (@) : zorplrpy f4a55a8c feat: teach parser about arrays
   Parent commit (@-): mryvwqyn 29233cfe develop@origin | docs: fix typo
   ```

3. Create the workspace parented on `master@origin`.

   - `-r` sets the parents of the new working-copy commit. Without it they are the parents of this
     directory's `@` — the line you are not building on.
   - `-r 'trunk()'` reaches the same commit wherever jj set that alias at clone time.
   - Keep the destination a sibling: a nested one makes a repo inside one.
   - Do not add `--ignore-working-copy`. It creates the directory and registers the workspace, then
     fails, leaving a commit on `root()` and a directory holding no files:

     ```text
     Created workspace in "../wkspc-feat-y"
     Error: This command must be able to update the working copy.
     Hint: Don't use --ignore-working-copy.
     ```

     Recover with `jj workspace forget wkspc-feat-y && rm -rf ../wkspc-feat-y`, then rerun without
     the flag.

   ```sh
   jj workspace add -r master@origin ../wkspc-feat-y
   ```

   ```text
   Created workspace in "../wkspc-feat-y"
   Working copy  (@) now at: ltmstotm 5d79caa9 (empty) (no description set)
   Parent commit (@-)      : tnoqnksp 16c35ce8 master | chore: release 1.4.1
   Added 3 files, modified 0 files, removed 0 files
   ```

4. Confirm the main directory stands where step 2 left it.

   The command of step 3 snapshotted `src/lexer.rs` into this directory's own `@`, which is what
   every jj command does and moves no file on disk.

   ```sh
   jj st
   ```

   ```text
   Working copy changes:
   M src/lexer.rs
   Working copy  (@) : zorplrpy f4a55a8c feat: teach parser about arrays
   Parent commit (@-): mryvwqyn 29233cfe develop@origin | docs: fix typo
   ```

5. Enter the workspace directory and confirm its parent is the release tip.

   ```sh
   cd ../wkspc-feat-y && jj st
   ```

   ```text
   The working copy has no changes.
   Working copy  (@) : ltmstotm 5d79caa9 (empty) (no description set)
   Parent commit (@-): tnoqnksp 16c35ce8 master | chore: release 1.4.1
   ```

6. Read across any ref plain `git` wrote in the main directory, which a jj-only workspace never
   syncs on its own, per [workspaces-share-one-repo.md](../workspaces-share-one-repo.md).

   Run it here and not in the main directory, where a colocated workspace declines the work.

   ```sh
   jj git import
   ```

   ```text
   bookmark: feature/from-git@git [new] tracked
   ```

7. Edit the files the change touches.

   ```sh
   $EDITOR src/render.rs
   ```

8. Describe the change once you know what it did.

   ```sh
   jj describe -m "fix: clamp viewport to the frame"
   ```

   ```text
   Working copy  (@) now at: ltmstotm db4d11aa fix: clamp viewport to the frame
   Parent commit (@-)      : tnoqnksp 16c35ce8 master | chore: release 1.4.1
   ```

9. Point the bookmark `bkmrk-feat-y` at the work so it has a name of its own.

   ```sh
   jj bookmark set bkmrk-feat-y
   ```

   ```text
   Created 1 bookmarks pointing to ltmstotm db4d11aa bkmrk-feat-y | fix: clamp viewport to the frame
   ```

10. Verify both workspaces and confirm the two lines meet no lower than `master`.

    ```sh
    jj workspace list && jj log -r '@ | master@origin | default@'
    ```

    ```text
    default: ../repo zorplrpy f4a55a8c feat: teach parser about arrays
    wkspc-feat-y: . ltmstotm db4d11aa bkmrk-feat-y | fix: clamp viewport to the frame
    @  ltmstotm you@example.com 2026-09-01 18:34:11 bkmrk-feat-y wkspc-feat-y@ db4d11aa
    │  fix: clamp viewport to the frame
    ◆  tnoqnksp you@example.com 2026-09-01 18:34:10 master 16c35ce8
    │  chore: release 1.4.1
    ~

    ○  zorplrpy you@example.com 2026-09-01 18:33:47 default@ f4a55a8c
    │  feat: teach parser about arrays
    ~
    ```

## Reference

- [main-target-remote/recipe.md](./main-target-remote/recipe.md) — landing the change at the remote
- [main-target-local/recipe.md](./main-target-local/recipe.md) — landing it in the main directory
- [workspaces-share-one-repo.md](../workspaces-share-one-repo.md) — addressing, staleness, jj-only workspaces
- [Revsets](https://docs.jj-vcs.dev/latest/revsets/) — `@`, `<workspace>@`, `<name>@origin`, `trunk()`
- [CLI reference](https://docs.jj-vcs.dev/latest/cli-reference/) — `jj workspace add` and its `-r`

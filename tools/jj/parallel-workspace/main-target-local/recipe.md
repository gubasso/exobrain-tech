# Work on a change in a parallel directory and land it in the main one

Creates a second working directory backed by the same jj repo, develops a change there, and
lands it in the main directory. No remote is involved at any point, and no bookmark: the two
directories address each other by workspace name. The `wkspc-` prefix is a teaching label, per
[README.md](../../README.md), and it stays for the life of the workspace. One change at a time; sparse
checkouts and conflict resolution are out of scope.

The steps run one worked example end to end: a repo at `~/code/repo`, one edit to
`src/parser.rs`, developed in `../wkspc-feat-x/`.

## Prerequisites

- `jj` 0.44 or later is on `PATH`.
  - check: `jj --version`
- The current directory is a jj workspace.
  - check: `jj workspace root`
  - if not:

    ```sh
    jj git clone <url>   # a fresh checkout
    jj git init          # or adopt the Git repo already here
    # both colocate on their own: --colocate is inert unless git.colocate = false
    ```
- The current directory is the workspace the repo was created with, named `default`.
  - check: `jj workspace list` names it `default`
  - if it carries another name, use that name wherever these steps write `default`
- The parent directory of the repo is writable.
  - check: `test -w ..`
  - if not: pick a writable parent, or `chmod u+w ..`

## Steps

1. Create the workspace as a sibling directory.

   The workspace name defaults to the basename of the destination, so this one is addressed as
   `wkspc-feat-x@`. Keep the destination a sibling: a nested one makes a repo inside one.

   ```sh
   jj workspace add ../wkspc-feat-x
   ```

   ```text
   Created workspace in "../wkspc-feat-x"
   Working copy  (@) now at: pyzykowm e70b383f (empty) (no description set)
   Parent commit (@-)      : xkwzzkqm a455aa68 docs: fix typo
   ```

2. Enter the workspace directory and confirm it sits on its own empty commit.

   ```sh
   cd ../wkspc-feat-x && jj st
   ```

   ```text
   The working copy has no changes.
   Working copy  (@) : pyzykowm e70b383f (empty) (no description set)
   Parent commit (@-): xkwzzkqm a455aa68 docs: fix typo
   ```

3. Edit the files the change touches.

   ```sh
   $EDITOR src/parser.rs
   ```

4. Describe the change once you know what it did.

   Only an empty, undescribed commit is discarded when it stops being the working copy, so
   describing this one is what makes it findable rather than what saves it.

   ```sh
   jj describe -m "feat: teach parser about arrays"
   ```

5. Rebase onto the main directory's line, whenever that line moves.

   `default@` is that directory's working copy: an empty commit jj discards the moment the
   working copy leaves it. Parent the change there and it has a descendant, so it can never be
   discarded and stays in the history. `default@-` is the commit under it, the work that
   directory builds on.

   ```sh
   jj rebase -o 'default@-'
   ```

   ```text
   Rebased 1 commits to destination.
   Working copy  (@) now at: pyzykowm 20f75034 feat: teach parser about arrays
   Parent commit (@-)      : zluyltwo af932ccb docs: add readme
   ```

6. Return to the main directory.

   ```sh
   cd -
   ```

7. Land the change by the route the state of this directory calls for. Both routes run before
   step 8, which deletes the workspace row that `wkspc-feat-x@` resolves through.

   - [land-into-a-clean-main-directory.md](./land-into-a-clean-main-directory.md)
     — this directory has nothing in progress
   - [land-over-work-in-progress.md](./land-over-work-in-progress.md)
     — this directory carries a change of its own

8. Retire the workspace `wkspc-feat-x@` and delete its directory.

   ```sh
   jj workspace forget wkspc-feat-x && rm -rf ../wkspc-feat-x
   ```

9. Verify the workspace is gone and the main directory stands on the change.

   ```sh
   jj workspace list && jj log -r '@-'
   ```

   ```text
   default: . zqtzskyu fc856264 (empty) (no description set)
   ○  pyzykowm you@example.com 2026-08-27 14:02:11 20f75034
   │  feat: teach parser about arrays
   ~
   ```

## Reference

- [main-target-remote/recipe.md](../main-target-remote/recipe.md) — the same recipe against a remote
- [land-into-a-clean-main-directory.md](./land-into-a-clean-main-directory.md)
  — landing where nothing is in progress
- [land-over-work-in-progress.md](./land-over-work-in-progress.md)
  — landing where the main directory carries a change
- [workspaces-share-one-repo.md](../../workspaces-share-one-repo.md) — landing, addressing, staleness
- [what-names-a-change.md](../../what-names-a-change.md) — the four names walked through one scenario
- [Revsets](https://docs.jj-vcs.dev/latest/revsets/) — `@`, `<workspace>@`, and `working_copies()`
- [CLI reference](https://docs.jj-vcs.dev/latest/cli-reference/) — every command used above

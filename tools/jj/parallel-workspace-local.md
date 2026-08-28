# Work on a change in a parallel directory and land it in the main one

Creates a second working directory backed by the same jj repo, develops a change there, and
lands it in the main directory. No remote is involved at any point, and no bookmark: the two
directories address each other by workspace name. The `wkspc-` prefix is a teaching label, per
[README.md](./README.md), and it stays for the life of the workspace. One change at a time; sparse
checkouts and conflict resolution are out of scope.

The steps run one worked example end to end: a repo at `~/code/repo`, one edit to
`src/parser.rs`, developed in `../wkspc-feat-x/`.

## Prerequisites

- `jj` 0.44 or later is on `PATH`.
  - check: `jj --version`
- The current directory is the workspace the repo was created with, named `default`.
  - check: `jj workspace list` names it `default`
  - if it carries another name, use that name wherever these steps write `default`
- The parent directory of the repo is writable.
  - check: `test -w ..`
  - if not: pick a writable parent, or `chmod u+w ..`

## Steps

1. Create the workspace as a sibling directory.

   The new working-copy commit takes the parents of the current workspace's `@` — the last commit
   you made, without the change in progress. The workspace name defaults to the basename of the
   destination, so this one is addressed as `wkspc-feat-x@`. Keep the destination a sibling: a
   nested one makes a repo inside one.

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

   An undescribed commit is discarded as soon as it stops being the working copy, so
   describing it is what makes the change durable.

   ```sh
   jj describe -m "feat: teach parser about arrays"
   ```

5. Rebase onto the main directory's line, whenever that line moves.

   `default@-` is the commit that directory is building on. `default@` is its working copy,
   normally an empty commit, and parenting the change there wedges that empty commit into the
   history for good.

   ```sh
   jj rebase -o 'default@-'
   ```

   ```text
   Rebased 1 commits to destination.
   Working copy  (@) now at: pyzykowm 20f75034 feat: teach parser about arrays
   Parent commit (@-)      : zluyltwo af932ccb docs: add readme
   ```

6. Return to the main directory and take the change onto its line.

   - `wkspc-feat-x@` resolves only while the workspace exists, so this runs before step 7.
   - Where that directory carries a change of its own in progress, `jj rebase -o 'wkspc-feat-x@'`
     puts that change on top of this one instead of leaving it a sibling.

   ```sh
   cd - && jj new 'wkspc-feat-x@'
   ```

   ```text
   Working copy  (@) now at: zqtzskyu fc856264 (empty) (no description set)
   Parent commit (@-)      : pyzykowm 20f75034 feat: teach parser about arrays
   Added 0 files, modified 1 files, removed 0 files
   ```

7. Retire the workspace `wkspc-feat-x@` and delete its directory.

   ```sh
   jj workspace forget wkspc-feat-x && rm -rf ../wkspc-feat-x
   ```

8. Verify the workspace is gone and the main directory stands on the change.

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

- [parallel-workspace.md](./parallel-workspace.md) — the same recipe against a remote
- [workspaces-share-one-repo.md](./workspaces-share-one-repo.md) — landing, addressing, staleness
- [what-names-a-change.md](./what-names-a-change.md) — the four names walked through one scenario
- [Revsets](https://docs.jj-vcs.dev/latest/revsets/) — `@`, `<workspace>@`, and `working_copies()`
- [CLI reference](https://docs.jj-vcs.dev/latest/cli-reference/) — every command used above

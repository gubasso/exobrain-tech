# Work on a change in a parallel directory with jj

Creates a second working directory backed by the same jj repo, develops a change there
independently, and lands it on the integration bookmark `develop`. One change at a time; sparse
checkouts and conflict resolution are out of scope. Names carry a kind prefix while they are only
names in this repo, per [README.md](./README.md); a bookmark drops it before becoming a branch.

The steps run one worked example end to end: a repo at `~/code/repo`, one edit to
`src/parser.rs`, developed in `../wkspc-feat-x/` and landed as the branch `feat-x`.

## Prerequisites

- `jj` 0.44 or later is on `PATH`.
  - check: `jj --version`
- The current directory is a jj workspace.
  - check: `jj workspace root`
  - if not:
    - `jj git clone <url>`
    - or `jj git init --colocate` where a Git repo already exists
- The repo has a Git remote named `origin`.
  - check: `jj git remote list`
  - if not: `jj git remote add origin <url>`
- The parent directory of the repo is writable.
  - check: `test -w ..`
  - if not: pick a writable parent, or `chmod u+w ..`

## Steps

1. List the workspaces the repo already has.

   ```sh
   jj workspace list
   ```

2. Fetch the remote so the remote bookmark `develop@origin` names the current tip.

   ```sh
   jj git fetch
   ```

3. Create the workspace as a sibling directory.

   The new working-copy commit takes the parents of the current workspace's `@` — the last commit
   you made, without the change in progress. The workspace name defaults to the basename of the
   destination, so this one is addressed as `wkspc-feat-x@`. Pass `-r develop@origin` to start from
   the remote tip instead, and keep the destination a sibling: a nested one makes a repo inside one.

   ```sh
   jj workspace add ../wkspc-feat-x
   ```

   ```text
   Created workspace in "../wkspc-feat-x"
   Working copy  (@) now at: rzvqmyuk bcc858e1 (empty) (no description set)
   Parent commit (@-)      : qpvuntsm 7b22a8cb docs: fix typo
   ```

4. Enter the workspace directory and confirm it sits on its own empty commit.

   ```sh
   cd ../wkspc-feat-x && jj st
   ```

   ```text
   The working copy has no changes.
   Working copy  (@) : rzvqmyuk bcc858e1 (empty) (no description set)
   Parent commit (@-): qpvuntsm 7b22a8cb docs: fix typo
   ```

5. Edit the files the change touches.

   ```sh
   $EDITOR src/parser.rs
   ```

6. Confirm jj picked the edit up. It snapshots on every command; nothing is staged.

   ```sh
   jj st
   ```

   ```text
   Working copy changes:
   M src/parser.rs
   Working copy  (@) : rzvqmyuk 9f3d1c07 (no description set)
   Parent commit (@-): qpvuntsm 7b22a8cb docs: fix typo
   ```

7. Describe the change once you know what it did.

   An undescribed commit is discarded as soon as it stops being the working copy, so
   describing it is what makes the change durable.

   ```sh
   jj describe -m "feat: teach parser about arrays"
   ```

8. Point the bookmark `bkmrk-feat-x` at the work so it has a name to push.

   - A bookmark is a reference: a name pointing at one commit id, never checked out, so it
     never advances on its own.
   - The branch takes its name from the bookmark and exists only once pushed (step 11); a
     colocated repo exports it, and this workspace has no `.git/` to export to.
   - Until then it is only a bookmark: a row in this repo, free to carry the prefix.

   ```sh
   jj bookmark set bkmrk-feat-x
   ```

   ```text
   Created 1 bookmarks pointing to rzvqmyuk 4b8e2a15 bkmrk-feat-x | feat: teach parser about arrays
   ```

9. Rebase onto the line this work belongs on, whenever that line moves, by one of:

   - the remote tip `develop@origin`, when the branch is shared and review runs against origin

     ```sh
     jj git fetch && jj rebase -b @ -o develop@origin
     ```

   - the main directory's own line, when you land there rather than on the remote

     `develop` names its branch, because a colocated repo's Git branch and the bookmark of
     that name are one line of work; `default@-` names its last commit when no bookmark does.
     Both are repo-wide, so they resolve here with no fetch and nothing to set up.

     ```sh
     jj rebase -b @ -o develop
     ```

10. Rename the bookmark to the name the branch will carry. This is the last step before the
    name leaves the repo, and the only moment it changes.

    - The branch takes the bookmark's name character for character, so the prefix goes here.
    - The rename is local; the new name points at the same commit.
    - A clean rename prints nothing.
    - Rename before the first push. Renaming an already-pushed bookmark leaves the old branch
      standing on the remote until you push the old name to delete it.

    ```sh
    jj bookmark rename bkmrk-feat-x feat-x
    ```

11. Push the bookmark `feat-x`, creating the branch of that name on the remote. A new remote
    bookmark is tracked automatically.

    ```sh
    jj git push --bookmark feat-x
    ```

    ```text
    Changes to push to origin:
      bookmark: feat-x [add to 7c21ef60d3a4]
    ```

12. Land the work on `develop`, by one of:

    - merge the pull request, then `jj git fetch`

      The remote moves its own branch; the fetch brings that position back as `develop@origin`.

    - move the bookmark `develop` locally, when no review is involved

      The `set` is local and changes nothing on the remote. The push is what writes the
      branch named `develop` there — updating it if it exists, creating it if not. A push is
      refused when the remote already carries that name untracked; `jj bookmark track
      develop@origin` clears that, once.

      ```sh
      jj bookmark set develop -r feat-x && jj git push --bookmark develop
      ```

13. Retire the workspace `wkspc-feat-x@` and delete its directory.

    ```sh
    cd - && jj workspace forget wkspc-feat-x && rm -rf ../wkspc-feat-x
    ```

14. Verify the workspace is gone and the work is on `develop`.

    ```sh
    jj workspace list && jj log -r develop
    ```

    ```text
    default: . rlvkpnrz 504e3d8c (empty) (no description set)
    ○  rzvqmyuk you@example.com 2026-08-27 14:02:11 develop 7c21ef60
    │  feat: teach parser about arrays
    ~
    ```

## Reference

- [workspaces-share-one-repo.md](./workspaces-share-one-repo.md) — landing, addressing, staleness
- [what-names-a-change.md](./what-names-a-change.md) — the four names walked through one scenario
- [Working copy](https://docs.jj-vcs.dev/latest/working-copy/) — workspaces and stale working copies
- [Bookmarks](https://docs.jj-vcs.dev/latest/bookmarks/) — tracking and push semantics
- [CLI reference](https://docs.jj-vcs.dev/latest/cli-reference/) — every command used above

# Work on a change in a parallel directory with jj

Creates a second working directory backed by the same jj repo, develops a change there
independently, and lands it on the trunk `master` at the remote. Landing in the main
directory instead, with no remote, is [main-target-local/recipe.md](../main-target-local/recipe.md).
One change at a time; sparse checkouts and conflict resolution are out of scope. Names carry a kind
prefix while they are only names in this repo, per [README.md](../../README.md); a bookmark drops it
before becoming a branch.

The steps run one worked example end to end: a repo at `~/code/repo`, one edit to
`src/parser.rs`, developed in `../wkspc-feat-x/` and landed as the branch `feat-x`.

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
- The repo has a Git remote named `origin`.
  - check: `jj git remote list`
  - if not: `jj git remote add origin <url>`
- The parent directory of the repo is writable.
  - check: `test -w ..`
  - if not: pick a writable parent, or `chmod u+w ..`

## Steps

1. Fetch the remote so the remote bookmark `master@origin` names the current tip.

   ```sh
   jj git fetch
   ```

2. Create the workspace as a sibling directory.

   The workspace name defaults to the basename of the destination, so this one is addressed as
   `wkspc-feat-x@`. Pass `-r master@origin` to start from the remote tip instead of this
   directory's line, and keep the destination a sibling: a nested one makes a repo inside one.

   ```sh
   jj workspace add ../wkspc-feat-x
   ```

   ```text
   Created workspace in "../wkspc-feat-x"
   Working copy  (@) now at: rzvqmyuk bcc858e1 (empty) (no description set)
   Parent commit (@-)      : qpvuntsm 7b22a8cb docs: fix typo
   ```

3. Enter the workspace directory and confirm it sits on its own empty commit.

   ```sh
   cd ../wkspc-feat-x && jj st
   ```

   ```text
   The working copy has no changes.
   Working copy  (@) : rzvqmyuk bcc858e1 (empty) (no description set)
   Parent commit (@-): qpvuntsm 7b22a8cb docs: fix typo
   ```

4. Edit the files the change touches.

   ```sh
   $EDITOR src/parser.rs
   ```

5. Describe the change once you know what it did.

   Only an empty, undescribed commit is discarded when it stops being the working copy, so
   describing this one is what makes it findable rather than what saves it.

   ```sh
   jj describe -m "feat: teach parser about arrays"
   ```

6. Point the bookmark `bkmrk-feat-x` at the work so it has a name to push.

   ```sh
   jj bookmark set bkmrk-feat-x
   ```

   ```text
   Created 1 bookmarks pointing to rzvqmyuk 4b8e2a15 bkmrk-feat-x | feat: teach parser about arrays
   ```

7. Rebase onto the remote tip, whenever it moves.

   ```sh
   jj git fetch && jj rebase -o master@origin
   ```

8. Rename the bookmark to the name the branch will carry.

   - The branch takes the bookmark's name character for character, so the prefix goes here.
   - Rename before the first push. Renaming an already-pushed bookmark leaves the old branch
     standing on the remote until you push the old name to delete it.

   ```sh
   jj bookmark rename bkmrk-feat-x feat-x
   ```

9. Push the bookmark `feat-x`, creating the branch of that name on the remote. A new remote
   bookmark is tracked automatically.

   ```sh
   jj git push --bookmark feat-x
   ```

   ```text
   Changes to push to origin:
     bookmark: feat-x [add to 7c21ef60d3a4]
   ```

10. Land the work on `master` by the route review calls for.

    - [land-through-a-pull-request.md](./land-through-a-pull-request.md)
      — the remote merges the branch, and a fetch brings the position back
    - [land-by-moving-the-bookmark.md](./land-by-moving-the-bookmark.md)
      — no review, so you move `master` yourself and push it

11. Return to the main directory and start a commit on top of the landed work.

    Either route moves a name alone, and a working copy never follows a name, so this directory
    lacks the change until you move it. `jj new`, `jj rebase`, and `jj edit` all get you there and
    differ in what they leave behind. The local recipe's companions compare the three against a
    workspace rather than `master@origin`, idle case first:

    - [land-into-a-clean-main-directory.md](../main-target-local/land-into-a-clean-main-directory.md)
    - [land-over-work-in-progress.md](../main-target-local/land-over-work-in-progress.md)

    ```sh
    cd - && jj new master@origin
    ```

    ```text
    Working copy  (@) now at: rlvkpnrz 504e3d8c (empty) (no description set)
    Parent commit (@-)      : rzvqmyuk 7c21ef60 master feat-x | feat: teach parser about arrays
    Added 0 files, modified 1 files, removed 0 files
    ```

12. Retire the workspace `wkspc-feat-x@` and delete its directory.

    ```sh
    jj workspace forget wkspc-feat-x && rm -rf ../wkspc-feat-x
    ```

13. Verify the workspace is gone and the main directory sits on the landed work.

    ```sh
    jj workspace list && jj log -r 'master@origin::'
    ```

    ```text
    default: . rlvkpnrz 504e3d8c (empty) (no description set)
    @  rlvkpnrz you@example.com 2026-08-27 14:02:11 504e3d8c
    │  (empty) (no description set)
    ◆  rzvqmyuk you@example.com 2026-08-27 14:02:11 master feat-x 7c21ef60
    │  feat: teach parser about arrays
    ~
    ```

## Reference

- [land-through-a-pull-request.md](./land-through-a-pull-request.md)
  — landing through review
- [land-by-moving-the-bookmark.md](./land-by-moving-the-bookmark.md)
  — landing with no review
- [main-target-local/recipe.md](../main-target-local/recipe.md) — the same recipe with no remote
- [workspaces-share-one-repo.md](../../workspaces-share-one-repo.md) — landing, addressing, staleness
- [what-names-a-change.md](../../what-names-a-change.md) — the four names walked through one scenario
- [Working copy](https://docs.jj-vcs.dev/latest/working-copy/) — workspaces and stale working copies
- [Bookmarks](https://docs.jj-vcs.dev/latest/bookmarks/) — tracking and push semantics
- [CLI reference](https://docs.jj-vcs.dev/latest/cli-reference/) — every command used above

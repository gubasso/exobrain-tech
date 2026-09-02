# Contribute commits back to the branch

Puts your own commits onto the branch a pull request proposes, pushes them so they appear in the
review, and takes what the other contributor pushes meanwhile — the loop repeated until the pull
request lands. Reading the branch without touching it is
[./read-the-branch.md](./read-the-branch.md); merging the pull request is out of scope.

The steps run one worked example: pull request 42 on the branch `feat-x`, where you add the change
`qpvuntsm` and the other contributor pushes `nlksuwmv` while you work.

## The whole run

```sh
jj git fetch
jj bookmark track feat-x@origin
jj new feat-x
$EDITOR src/parser.rs
jj describe -m "fix: reject empty arrays"
jj new
jj bookmark set feat-x -r qpvuntsm
jj git push --bookmark feat-x
jj git fetch && jj rebase -b qpvuntsm -d feat-x@origin
jj bookmark forget feat-x && jj new master@origin
```

## Prerequisites

- `jj` 0.44 or later is on `PATH`.
  - check: `jj --version`
- The current directory is a jj workspace with a Git remote named `origin`.
  - check: `jj workspace root && jj git remote list`
  - if not:

    ```sh
    jj git clone <url>   # a fresh checkout
    jj git init          # or adopt the Git repo already here
    # both colocate on their own: --colocate is inert unless git.colocate = false
    ```
- The remote bookmark `feat-x@origin` is present.
  - check: `jj bookmark list --all-remotes`
  - if not: step 1 of [./read-the-branch.md](./read-the-branch.md)
- You may write to the branch: it is on `origin` and you have push access, or it is on a fork whose
  pull request allows edits from maintainers.
  - check: `gh pr view 42 --json maintainerCanModify,isCrossRepository`

## Steps

1. Fetch, so the recorded position of the branch is current.

   ```sh
   jj git fetch
   ```

   ```text
   bookmark: feat-x@origin [updated] tracked
   ```

2. Track the branch, giving it a local bookmark that follows every later fetch.

   Pushing to a remote bookmark that already exists requires it to be tracked, so this is what makes
   the branch writable rather than a thing you can only read.

   ```sh
   jj bookmark track feat-x@origin
   ```

   ```text
   Started tracking 1 remote bookmarks.
   ```

3. Start your change on the branch head.

   ```sh
   jj new feat-x
   ```

   ```text
   Working copy  (@) now at: qpvuntsm 4c9018ab (empty) (no description set)
   Parent commit (@-)      : nlksuwmv 865cc949 feat-x | feat: teach parser about arrays
   ```

4. Edit the files your contribution touches.

   ```sh
   $EDITOR src/parser.rs
   ```

5. Describe the change.

   ```sh
   jj describe -m "fix: reject empty arrays"
   ```

   ```text
   Working copy  (@) : qpvuntsm 7c21ef60 fix: reject empty arrays
   ```

6. Start the next commit, so the described one stops being the working copy.

   Every jj command snapshots the working copy into `@`. Left as `@`, `qpvuntsm` would keep
   absorbing edits after the push at step 8, and the bookmark would follow each rewrite — reviewers
   would see a commit id that no longer matches what they read.

   ```sh
   jj new
   ```

   ```text
   Working copy  (@) now at: rlvkpnrz 504e3d8c (empty) (no description set)
   Parent commit (@-)      : qpvuntsm 7c21ef60 fix: reject empty arrays
   ```

7. Move the bookmark onto your change, so the branch names it.

   ```sh
   jj bookmark set feat-x -r qpvuntsm
   ```

   ```text
   Moved 1 bookmarks to qpvuntsm 7c21ef60 feat-x | fix: reject empty arrays
   ```

8. Push the bookmark, putting your commit in the pull request.

   ```sh
   jj git push --bookmark feat-x
   ```

   ```text
   Changes to push to origin:
     Move forward bookmark feat-x from 865cc949cb01 to 7c21ef60d3a4
   ```

   - The branch is on a fork added as the remote `fork`: `jj git push --remote fork --bookmark feat-x`
   - The remote moved since your last fetch, so jj refuses rather than overwriting, the way
     `git push --force-with-lease` does. Continue at step 9 and return here.

     ```text
     Error: Refusing to push a bookmark that unexpectedly moved on the remote. Affected refs: refs/heads/feat-x
     Hint: Try fetching from the remote, then make the bookmark point to where you want it to be, and push again.
     ```

9. Take what the other contributor pushed.

   ```sh
   jj git fetch
   ```

   ```text
   bookmark: feat-x@origin [updated] tracked
   ```

   ```sh
   jj bookmark list
   ```

   ```text
   feat-x (conflicted):
     + qpvuntsm 7c21ef60 fix: reject empty arrays
     + nlksuwmv 4a0b91d2 test: cover the empty case
     @origin: nlksuwmv 4a0b91d2 test: cover the empty case
   ```

   The bookmark is conflicted because both sides moved and neither contains the other. A fetch that
   arrives while you have nothing unpushed prints no conflict and needs no step 10.

10. Put your change on top of theirs.

    ```sh
    jj rebase -b qpvuntsm -d feat-x@origin
    ```

    ```text
    Rebased 2 commits to destination.
    Working copy  (@) now at: rlvkpnrz 91ac03f7 (empty) (no description set)
    Parent commit (@-)      : qpvuntsm 3f7d0c22 fix: reject empty arrays
    ```

11. Point the bookmark at the rebased change, which resolves the conflict, then push at step 8.

    ```sh
    jj bookmark set feat-x -r qpvuntsm
    ```

    ```text
    Moved 1 bookmarks to qpvuntsm 3f7d0c22 feat-x | fix: reject empty arrays
    ```

12. Leave the branch once the pull request lands.

    - Drop the local bookmark: `jj bookmark forget feat-x`, which does not mark the branch for
      deletion. `jj bookmark delete feat-x` would, removing it from the remote at the next push.
    - Return to the trunk: `jj new master@origin`

13. Verify the branch is no longer yours to move and the directory sits on the trunk.

    ```sh
    jj bookmark list && jj log -r '@'
    ```

    ```text
    master: zzkytpwl 7b22a8cb docs: fix typo
      @origin: zzkytpwl 7b22a8cb docs: fix typo
    @  kntqzsrs  (empty)
    ```

    Expected: no `feat-x` among the bookmarks, and the working copy parents on `master@origin`.

## Reference

- [./read-the-branch.md](./read-the-branch.md) — the same branch when nothing is pushed back, and
  why an untracked remote bookmark is immutable
- [../one-directory/recipe.md](../one-directory/recipe.md) — the same describe, rebase, and push
  spine on a branch of your own
- [../README.md](../README.md) — the naming convention these examples use
- [Bookmarks](https://docs.jj-vcs.dev/latest/bookmarks/) — tracking, the push safety checks, and how
  a conflicted bookmark is resolved
- [CLI reference](https://docs.jj-vcs.dev/latest/cli-reference/) — `jj bookmark track`, `set`,
  `forget`, `delete`, and `jj git push`

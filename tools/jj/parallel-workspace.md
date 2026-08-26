# Work on a change in a parallel directory with jj

Creates a second working directory backed by the same jj repo, develops a change there
independently, and lands it on the integration bookmark `bkmrk-develop`. Covers one change at
a time; sparse checkouts and conflict resolution are out of scope. Names carry a kind prefix,
per the convention in [README.md](./README.md).

## Prerequisites

- `jj` 0.44 or later is on `PATH`.
- The current directory is a jj workspace with a Git remote named `origin`.
- The parent directory of the repo is writable.

## Steps

1. List the workspaces the repo already has.

   ```sh
   jj workspace list
   ```

2. Fetch the remote so the remote bookmark `bkmrk-develop@origin` names the current tip.

   ```sh
   jj git fetch
   ```

3. Create the workspace as a sibling directory.

   The new working-copy commit takes the parents of the current workspace's `@` — the last
   commit you made, without the change you have in progress. The workspace name defaults to
   the basename of the destination, so this one is addressed as `wkspc-feat-x@`. Pass
   `-r bkmrk-develop@origin` instead to start from the remote tip, and keep the destination a
   sibling: a nested one makes a repo inside a repo.

   ```sh
   jj workspace add ../wkspc-feat-x
   ```

4. Enter the workspace directory and confirm it sits on its own empty commit.

   ```sh
   cd ../wkspc-feat-x && jj st
   ```

5. Edit the files. jj snapshots the working copy on every command; nothing is staged.

   ```sh
   jj st
   ```

6. Describe the change once you know what it did.

   An undescribed commit is discarded as soon as it stops being the working copy, so
   describing it is what makes the change durable.

   ```sh
   jj describe -m "feat: <what it did>"
   ```

7. Point the bookmark `bkmrk-feat-x` at the work so it has a name to push.

   - A bookmark is a reference: a name pointing at one commit id.
   - It is jj's stand-in for a Git branch, and this is the name the remote will see.
   - The branch takes its name from the bookmark, but exists only once pushed (step 9), or
     exported by a colocated repo — this workspace has no `.git/` to export to.
   - It is never checked out, so it never advances on its own.
   - Until then it is only a bookmark: a row in this repo.

   ```sh
   jj bookmark set bkmrk-feat-x
   ```

8. Rebase onto the latest `bkmrk-develop@origin` whenever the remote moves.

   ```sh
   jj git fetch && jj rebase -b @ -o bkmrk-develop@origin
   ```

9. Push the bookmark `bkmrk-feat-x`, creating the branch of that name on the remote. A new
   remote bookmark is tracked automatically.

   ```sh
   jj git push --bookmark bkmrk-feat-x
   ```

10. Land the work on `bkmrk-develop`, by one of:

    - merge the pull request, then `jj git fetch`

      The remote moves its own branch; the fetch brings that position back as the remote
      bookmark `bkmrk-develop@origin`.

    - move the bookmark `bkmrk-develop` locally, when no review is involved

      The `set` is local and changes nothing on the remote. The push is what writes the
      branch named `bkmrk-develop` there — updating it if it exists, creating it if not.

      ```sh
      jj bookmark set bkmrk-develop -r bkmrk-feat-x && jj git push --bookmark bkmrk-develop
      ```

11. Retire the workspace `wkspc-feat-x@` and delete its directory.

    ```sh
    cd - && jj workspace forget wkspc-feat-x && rm -rf ../wkspc-feat-x
    ```

12. Verify the workspace is gone and the work is on `bkmrk-develop`.

    ```sh
    jj workspace list && jj log -r bkmrk-develop
    ```

    Expected: `wkspc-feat-x` absent from the list, and its change reachable from the bookmark
    `bkmrk-develop`.

## Recovering a stale workspace

Rewriting a workspace's working-copy commit from a different workspace leaves its files
behind the repo. Run this in the workspace that went stale.

```sh
jj workspace update-stale
```

## Reference

- [workspaces-share-one-repo.md](./workspaces-share-one-repo.md) — what the shared repo means
  for landing work, addressing another workspace, and staleness
- [what-names-a-change.md](./what-names-a-change.md) — the directory, workspace, description,
  and bookmark walked through one scenario with commit graphs
- [Working copy](https://docs.jj-vcs.dev/latest/working-copy/) — workspaces and stale working copies
- [Bookmarks](https://docs.jj-vcs.dev/latest/bookmarks/) — tracking and push semantics
- [CLI reference](https://docs.jj-vcs.dev/latest/cli-reference/) — `jj workspace`, `jj rebase`, `jj git push`

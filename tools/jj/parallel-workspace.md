# Work on a change in a parallel directory with jj

Creates a second working directory backed by the same jj repo, develops a change there
independently, and lands it on `main`. Covers one change at a time; sparse checkouts and
conflict resolution are out of scope.

## Prerequisites

- `jj` 0.44 or later is on `PATH`.
- The current directory is a jj workspace with a Git remote named `origin`.
- The parent directory of the repo is writable.

## Steps

1. List the workspaces the repo already has.

   ```sh
   jj workspace list
   ```

2. Fetch the remote so `main@origin` names the current tip.

   ```sh
   jj git fetch
   ```

3. Create the workspace as a sibling directory, rooted on `main@origin`.

   Without `-r`, the new working-copy commit shares the parents of the current workspace's
   `@`, which is rarely what a from-main workspace wants. A nested destination makes a
   repo inside a repo.

   ```sh
   jj workspace add --name feat-x -r main@origin -m "feat-x: <what it does>" ../repo-feat-x
   ```

4. Enter the workspace and confirm it sits on its own empty commit.

   ```sh
   cd ../repo-feat-x && jj st
   ```

5. Edit the files. jj snapshots the working copy on every command; nothing is staged.

   ```sh
   jj st
   ```

6. Point a bookmark at the work so it has a name to push.

   ```sh
   jj bookmark set feat-x -r @
   ```

7. Rebase onto the latest `main` whenever the remote moves.

   `-d` is the deprecated spelling of `-o`.

   ```sh
   jj git fetch && jj rebase -b @ -o main@origin
   ```

8. Push the bookmark. A new remote bookmark is tracked automatically.

   ```sh
   jj git push --bookmark feat-x
   ```

9. Land the work on `main`, by one of:

   - merge the pull request, then `jj git fetch`
   - move the bookmark locally, when no review is involved

     ```sh
     jj bookmark set main -r feat-x && jj git push --bookmark main
     ```

10. Retire the workspace and delete its directory.

    ```sh
    cd - && jj workspace forget feat-x && rm -rf ../repo-feat-x
    ```

11. Verify the workspace is gone and the work is on `main`.

    ```sh
    jj workspace list && jj log -r main
    ```

    Expected: `feat-x` absent from the list, and its change reachable from `main`.

## Recovering a stale workspace

Rewriting a workspace's working-copy commit from a different workspace leaves its files
behind the repo. Run this in the workspace that went stale.

```sh
jj workspace update-stale
```

## Reference

- [workspaces-share-one-repo.md](./workspaces-share-one-repo.md) — what the shared repo means
  for landing work, addressing another workspace, and staleness
- [Working copy](https://docs.jj-vcs.dev/latest/working-copy/) — workspaces and stale working copies
- [Bookmarks](https://docs.jj-vcs.dev/latest/bookmarks/) — tracking and push semantics
- [CLI reference](https://docs.jj-vcs.dev/latest/cli-reference/) — `jj workspace`, `jj rebase`, `jj git push`

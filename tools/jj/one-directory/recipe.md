# Work on a change in the directory you are already in

Develops a change in the working directory you already have, with no second working copy and no
branch checked out, and lands it on the integration line `develop` at the remote. jj keeps every
unnamed line of work as its own head, so a change needs a bookmark only in the step before it
leaves the repo. Giving a change its own directory instead is
[../parallel-workspace/](../parallel-workspace/). Sparse checkouts and conflict resolution are out
of scope.

The steps run one worked example end to end: a repo at `~/code/repo`, one edit to
`src/parser.rs`, landed as the branch `feat-x`.

## The whole run

```sh
jj git fetch
jj new develop@origin
$EDITOR src/parser.rs
jj describe -m "feat: teach parser about arrays"
jj new
jj git fetch && jj rebase -b qpvuntsm -o develop@origin
jj bookmark set feat-x -r qpvuntsm
jj git push --bookmark feat-x
# land on develop, then:
jj git fetch && jj new develop@origin
```

`qpvuntsm` is the change id `jj describe` printed. The steps below run these one at a time, with
what each prints and where the run forks.

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

## Steps

1. Fetch the remote so the remote bookmark `develop@origin` names the current tip.

   ```sh
   jj git fetch
   ```

2. Start the change on that tip.

   `jj new <rev>` puts you on a new empty commit whose parent is `<rev>`; it never stands you on
   `<rev>` itself.

   ```sh
   jj new develop@origin
   ```

   ```text
   Working copy  (@) now at: qpvuntsm 5e12a7c0 (empty) (no description set)
   Parent commit (@-)      : zzkytpwl 7b22a8cb develop develop@origin | docs: fix typo
   ```

3. Edit the files the change touches.

   ```sh
   $EDITOR src/parser.rs
   ```

4. Describe the change once you know what it did.

   Note the change id `qpvuntsm`: it survives every rewrite below and is how you address this
   change for the rest of the run.

   ```sh
   jj describe -m "feat: teach parser about arrays"
   ```

   ```text
   Working copy  (@) : qpvuntsm 4b8e2a15 feat: teach parser about arrays
   ```

5. Start the next commit, so this one stops being the working copy.

   - Stacking on top of the change: `jj new` with no argument.
   - Starting something unrelated: `jj new develop@origin`, which parks this change as a head of
     its own and opens a second line in the same directory. Coming back to either is
     [./switching-changes-in-place.md](./switching-changes-in-place.md).

   ```sh
   jj new
   ```

   ```text
   Working copy  (@) now at: rlvkpnrz 504e3d8c (empty) (no description set)
   Parent commit (@-)      : qpvuntsm 4b8e2a15 feat: teach parser about arrays
   ```

6. Restack the line onto the remote tip, whenever it moves.

   `-b` names any change on the line to move, so this works from wherever you are standing.

   ```sh
   jj git fetch && jj rebase -b qpvuntsm -o develop@origin
   ```

   ```text
   Rebased 2 commits to destination.
   Working copy  (@) now at: rlvkpnrz 91ac03f7 (empty) (no description set)
   Parent commit (@-)      : qpvuntsm 7c21ef60 feat: teach parser about arrays
   ```

7. Name the change with a bookmark, in the step before it leaves the repo.

   The branch takes the bookmark's name character for character, so the name is written once, in
   its final form, and carries no teaching prefix.

   ```sh
   jj bookmark set feat-x -r qpvuntsm
   ```

   ```text
   Created 1 bookmarks pointing to qpvuntsm 7c21ef60 feat-x | feat: teach parser about arrays
   ```

8. Push the bookmark `feat-x`, creating the branch of that name on the remote. A newly created
   remote bookmark is marked as tracked.

   ```sh
   jj git push --bookmark feat-x
   ```

   ```text
   Changes to push to origin:
     bookmark: feat-x [add to 7c21ef60d3a4]
   ```

9. Land the work on `develop` by the route review calls for.

   - [../parallel-workspace/main-target-remote/land-through-a-pull-request.md](../parallel-workspace/main-target-remote/land-through-a-pull-request.md)
     — the remote merges the branch, and a fetch brings the position back
   - [../parallel-workspace/main-target-remote/land-by-moving-the-bookmark.md](../parallel-workspace/main-target-remote/land-by-moving-the-bookmark.md)
     — no review, so you move `develop` yourself and push it

10. Start the next change on top of the landed work.

    Either route moves a name alone, and a working copy never follows a name, so this directory
    lacks the landed position until you move it.

    ```sh
    jj git fetch && jj new develop@origin
    ```

    ```text
    Working copy  (@) now at: kntqzsrs 0f3b1c48 (empty) (no description set)
    Parent commit (@-)      : qpvuntsm 7c21ef60 develop feat-x | feat: teach parser about arrays
    ```

11. Verify the directory sits on the landed work and nothing is left standing beside it.

    ```sh
    jj log -r 'develop@origin::' && jj log -r 'mutable() ~ ::@'
    ```

    ```text
    @  kntqzsrs you@example.com 2026-08-27 14:02:11 0f3b1c48
    │  (empty) (no description set)
    ◆  qpvuntsm you@example.com 2026-08-27 14:02:11 develop feat-x 7c21ef60
    │  feat: teach parser about arrays
    ~
    ```

## Reference

- [./switching-changes-in-place.md](./switching-changes-in-place.md) — parking a change and coming
  back to it, and what `jj new` and `jj edit` cost here
- [../parallel-workspace/main-target-remote/recipe.md](../parallel-workspace/main-target-remote/recipe.md)
  — the same landing from a second directory
- [../what-names-a-change.md](../what-names-a-change.md) — the names a change carries
- [Working with GitHub](https://docs.jj-vcs.dev/latest/github/) — the upstream single-directory routes
- [Bookmarks](https://docs.jj-vcs.dev/latest/bookmarks/) — tracking and push safety checks
- [CLI reference](https://docs.jj-vcs.dev/latest/cli-reference/) — every command used above

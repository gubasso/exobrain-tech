# Read the branch without changing it

Puts the branch a pull request proposes into the directory you are already in, so its code can be
read and run, and takes you back off it afterwards. No local bookmark is created: reading remote
work needs no name. Pushing commits onto the branch is
[./contribute-to-the-branch.md](./contribute-to-the-branch.md).

The steps run one worked example: pull request 42, proposing the branch `feat-x` against the
integration line `develop`.

## The whole run

```sh
jj git fetch
jj log -r 'develop@origin..feat-x@origin'
jj new feat-x@origin
jj diff --from 'fork_point(develop@origin | feat-x@origin)' --to feat-x@origin
jj new develop@origin
```

## Prerequisites

- `jj` 0.44 or later is on `PATH`.
  - check: `jj --version`
- The current directory is a jj workspace with a Git remote named `origin`.
  - check: `jj workspace root && jj git remote list`
- The branch name behind the pull request, and whether it lives in a fork, are known.
  - check: `gh pr view 42 --json headRefName,isCrossRepository,headRepositoryOwner`

## Steps

1. Fetch the remote that carries the branch.

   - `"isCrossRepository": false` — the branch is on `origin`, so the ordinary fetch has it:

     ```sh
     jj git fetch
     ```

   - `"isCrossRepository": true` — the branch lives in a fork, which is a remote of its own, and
     every `feat-x@origin` below reads `feat-x@fork`:

     ```sh
     jj git remote add fork https://github.com/<owner>/<repo>.git
     jj git fetch --remote fork
     ```

2. Confirm the branch arrived as a remote bookmark.

   A fetch leaves it untracked, so it has no local twin and `jj bookmark list` alone omits it.

   ```sh
   jj bookmark list --all-remotes
   ```

   ```text
   develop: zzkytpwl 7b22a8cb docs: fix typo
     @origin: zzkytpwl 7b22a8cb docs: fix typo
   feat-x@origin: nlksuwmv 865cc949 feat: teach parser about arrays
   ```

3. Read which commits the pull request proposes.

   ```sh
   jj log -r 'develop@origin..feat-x@origin'
   ```

   ```text
   ○  nlksuwmv  feat-x@origin  feat: teach parser about arrays
   ○  qkzsnpuw  test: cover the array case
   ~
   ```

4. Stand on the branch head.

   `jj new` parents a new empty commit on it, so the branch's files land in the directory and any
   edit becomes a change of your own. `jj edit feat-x@origin` is refused instead: an untracked
   remote bookmark is immutable by default.

   ```sh
   jj new feat-x@origin
   ```

   ```text
   Working copy  (@) now at: xtvrlsmq 4c9018ab (empty) (no description set)
   Parent commit (@-)      : nlksuwmv 865cc949 feat-x@origin | feat: teach parser about arrays
   Added 2 files, modified 1 files, removed 0 files
   ```

   - Whatever the working copy held is parked as a head of its own, reachable by its change id:
     [../one-directory/switching-changes-in-place.md](../one-directory/switching-changes-in-place.md)
   - Carrying review notes onto the branch instead of parking them, where `wqnrkuzp` is the change
     holding them: `jj rebase -r wqnrkuzp -d feat-x@origin && jj edit wqnrkuzp`

5. Read the diff the pull request contributes.

   `fork_point()` is where the branch left `develop`, so whatever `develop` gained afterwards stays
   out of the diff.

   ```sh
   jj diff --from 'fork_point(develop@origin | feat-x@origin)' --to feat-x@origin
   ```

   ```text
   Modified regular file src/parser.rs:
      12    12: fn parse_value(input: &str) -> Value {
         13:     if input.starts_with('[') { return parse_array(input); }
   ```

6. Leave the branch once the review is done.

   ```sh
   jj new develop@origin
   ```

   ```text
   Working copy  (@) now at: kntqzsrs 0f3b1c48 (empty) (no description set)
   Parent commit (@-)      : zzkytpwl 7b22a8cb develop develop@origin | docs: fix typo
   ```

   - Added a fork remote in step 1: `jj git remote remove fork`

7. Verify the directory sits on the integration line and the review left no name behind.

   ```sh
   jj log -r '@' && jj bookmark list
   ```

   ```text
   @  kntqzsrs  (empty)
   develop: zzkytpwl 7b22a8cb docs: fix typo
     @origin: zzkytpwl 7b22a8cb docs: fix typo
   ```

   Expected: the working copy parents on `develop@origin`, and no bookmark names the reviewed
   branch.

## Reference

- [./contribute-to-the-branch.md](./contribute-to-the-branch.md) — the same branch, once the review
  turns into commits you push onto it
- [../one-directory/switching-changes-in-place.md](../one-directory/switching-changes-in-place.md) —
  parking a change and coming back to it
- [../README.md](../README.md) — the naming convention these examples use
- [Working with GitHub](https://docs.jj-vcs.dev/latest/github/) — `jj new <bookmark>@<remote>` as
  the upstream route onto a contributor's branch, and `remotes.<name>.auto-track-bookmarks` for
  dropping the `@origin` suffix
- [Bookmarks](https://docs.jj-vcs.dev/latest/bookmarks/) — tracked and untracked remote bookmarks
- [Revsets](https://docs.jj-vcs.dev/latest/revsets/) — `fork_point()` and the `x..y` range

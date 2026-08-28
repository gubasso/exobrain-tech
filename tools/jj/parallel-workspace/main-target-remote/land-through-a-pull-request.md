# Landing through a pull request

Review happens on the remote, so the remote is what moves `develop`. No command here lands the
work; the merge button does, and a fetch brings the new position back. Where no review is involved
and you move the name yourself, that is
[land-by-moving-the-bookmark.md](./land-by-moving-the-bookmark.md).

The recipe is [recipe.md](./recipe.md); this walks its step 10.

## The scenario

The recipe has run through its push, so the branch `feat-x` exists on the remote and
`develop@origin` still names the commit it did before:

```text
@  tpvxtqly  feat-x wkspc-feat-x@  feat: teach parser about arrays
│ ○  wmxomuyw  default@  (empty)
├─╯
◆  slqynpzq  develop  docs: add readme
```

## Reading the merge back

The pull request is merged on the remote. That writes a commit this repo has never seen, and one
command brings it back:

```sh
jj git fetch
```

```text
bookmark: develop@origin [updated] tracked
Warning: The working-copy commit became immutable; a new commit has been created on top of it.
Working copy  (@) now at: zrzuxmvp d8155548 (empty) (no description set)
Parent commit (@-)      : tpvxtqly 59023837 feat-x | feat: teach parser about arrays
```

Two things moved. `develop@origin` now names the merge commit, and your change became immutable —
it is an ancestor of a remote bookmark, so jj will not rewrite it and steps the working copy off it
onto a fresh empty commit. Nothing was lost; the change is exactly where the remote has it.

```sh
jj log -r 'develop@origin'
```

```text
◆  osxkxutm  develop  (empty) Merge branch 'feat-x' into develop
```

The merge commit is `(empty)` because it introduces no content over `feat-x`. What it introduces is
a second parent: this route puts a two-parent commit at the tip of `develop`, which the other route
does not.

## Bringing the main directory onto it

```sh
cd - && jj new develop@origin
```

```text
Working copy  (@) now at: uoyuumrl 2c53c9d5 (empty) (no description set)
Parent commit (@-)      : osxkxutm e75db4ca develop | (empty) Merge branch 'feat-x' into develop
Added 1 files, modified 0 files, removed 0 files
```

```text
@  uoyuumrl  default@  (empty)
◆    osxkxutm  develop  (empty) Merge branch 'feat-x' into develop
├─╮
│ ◆  tpvxtqly  feat-x  feat: teach parser about arrays
├─╯
◆  slqynpzq  docs: add readme
```

`Added 1 files` is `src/parser.rs` arriving in this directory. It parents on the merge commit rather
than on `feat-x`, so this directory sits on the integration line, not on the branch that fed it.

## Reference

- [land-by-moving-the-bookmark.md](./land-by-moving-the-bookmark.md) — landing with no review
- [../main-target-local/land-over-work-in-progress.md](../main-target-local/land-over-work-in-progress.md)
  — what `jj new` costs when this directory carries a change
- [../../workspaces-share-one-repo.md](../../workspaces-share-one-repo.md) — why no merge back is needed
- [CLI reference](https://docs.jj-vcs.dev/latest/cli-reference/) — `jj git fetch`, `jj new`

# Landing by moving the bookmark

No review gate stands between the change and the trunk, so you move `master` yourself. The `set` is
local and changes nothing on the remote; the push is what moves the branch of that name there.

This route needs a trunk that accepts a direct push — a repository of your own, or a line nobody
else builds on. A trunk protected the way
[trunk-based development](../../../../workflows/trunk-based-development.md) describes refuses the
push, and the work lands through
[land-through-a-pull-request.md](./land-through-a-pull-request.md) instead.

The recipe is [recipe.md](./recipe.md); this walks its step 10.

## The scenario

The recipe has run through its push, so the branch `feat-x` exists on the remote and `master`
still names the commit it did before:

```text
@  nyqkoonw  feat-x wkspc-feat-x@  feat: teach parser about arrays
│ ○  yyxyvumu  default@  (empty)
├─╯
◆  lkwytump  master  docs: add readme
```

## Moving the name, then the branch

```sh
jj bookmark set master -r feat-x && jj git push --bookmark master
```

```text
Moved 1 bookmarks to nyqkoonw 6ef4f4c7 master* feat-x | feat: teach parser about arrays
Changes to push to origin:
  bookmark: master [move forward from d0ec85991387 to 6ef4f4c7fe47]
Warning: The working-copy commit became immutable; a new commit has been created on top of it.
Working copy  (@) now at: uwuluzoq 4f0919ec (empty) (no description set)
Parent commit (@-)      : nyqkoonw 6ef4f4c7 master* feat-x | feat: teach parser about arrays
```

The `*` on `master` is the local bookmark sitting ahead of the remote one, and it clears with the
push. `move forward` is the whole event: one commit now carries both names, and nothing was
merged, duplicated, or rewritten. The working copy became immutable for the same reason as the
other route — the change is an ancestor of a remote bookmark now — so jj stepped off it.

A remote bookmark jj is not tracking refuses the push instead:

```text
Error: Non-tracking remote bookmark master@origin exists
Hint: Run `jj bookmark track master@origin` to import the remote bookmark.
```

`jj git clone` tracks the branch it clones, so this is the state of a repo jj adopted with
`jj git init` or a line that arrived by a later fetch. Run the hint once and the push goes through.

## Bringing the main directory onto it

```sh
cd - && jj new master@origin
```

```text
Working copy  (@) now at: qnmmuxvl f1e91278 (empty) (no description set)
Parent commit (@-)      : nyqkoonw 6ef4f4c7 master feat-x | feat: teach parser about arrays
Added 1 files, modified 0 files, removed 0 files
```

```text
@  qnmmuxvl  default@  (empty)
◆  nyqkoonw  master feat-x  feat: teach parser about arrays
◆  lkwytump  docs: add readme
```

The line stays linear and one commit wears both names. `Added 1 files` is `src/parser.rs` arriving
in this directory, which had it in the repo and not on disk until now.

## Reference

- [land-through-a-pull-request.md](./land-through-a-pull-request.md) — landing through review
- [../main-target-local/land-over-work-in-progress.md](../main-target-local/land-over-work-in-progress.md)
  — what `jj new` costs when this directory carries a change
- [../../workspaces-share-one-repo.md](../../workspaces-share-one-repo.md) — why landing is a bookmark move
- [Bookmarks](https://docs.jj-vcs.dev/latest/bookmarks/) — tracking and push semantics

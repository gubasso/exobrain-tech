# Landing by moving the bookmark

No review stands between the change and the integration line, so you move `develop` yourself. The
`set` is local and changes nothing on the remote; the push is what moves the branch of that name
there. Where the remote merges the work instead, that is
[land-through-a-pull-request.md](./land-through-a-pull-request.md).

The recipe is [recipe.md](./recipe.md); this walks its step 10.

## The scenario

The recipe has run through its push, so the branch `feat-x` exists on the remote and `develop`
still names the commit it did before:

```text
@  nyqkoonw  feat-x wkspc-feat-x@  feat: teach parser about arrays
│ ○  yyxyvumu  default@  (empty)
├─╯
◆  lkwytump  develop  docs: add readme
```

## Moving the name, then the branch

```sh
jj bookmark set develop -r feat-x && jj git push --bookmark develop
```

```text
Moved 1 bookmarks to nyqkoonw 6ef4f4c7 develop* feat-x | feat: teach parser about arrays
Changes to push to origin:
  bookmark: develop [move forward from d0ec85991387 to 6ef4f4c7fe47]
Warning: The working-copy commit became immutable; a new commit has been created on top of it.
Working copy  (@) now at: uwuluzoq 4f0919ec (empty) (no description set)
Parent commit (@-)      : nyqkoonw 6ef4f4c7 develop* feat-x | feat: teach parser about arrays
```

The `*` on `develop` is the local bookmark sitting ahead of the remote one, and it clears with the
push. `move forward` is the whole event: one commit now carries both names, and nothing was
merged, duplicated, or rewritten. The working copy became immutable for the same reason as the
other route — the change is an ancestor of a remote bookmark now — so jj stepped off it.

A remote bookmark jj is not tracking refuses the push instead:

```text
Error: Non-tracking remote bookmark develop@origin exists
Hint: Run `jj bookmark track develop@origin` to import the remote bookmark.
```

`jj git clone` tracks the branch it clones, so this is the state of a repo jj adopted with
`jj git init` or a line that arrived by a later fetch. Run the hint once and the push goes through.

## Bringing the main directory onto it

```sh
cd - && jj new develop@origin
```

```text
Working copy  (@) now at: qnmmuxvl f1e91278 (empty) (no description set)
Parent commit (@-)      : nyqkoonw 6ef4f4c7 develop feat-x | feat: teach parser about arrays
Added 1 files, modified 0 files, removed 0 files
```

```text
@  qnmmuxvl  default@  (empty)
◆  nyqkoonw  develop feat-x  feat: teach parser about arrays
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

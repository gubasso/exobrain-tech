# Parking a change and coming back to it, in one directory

Two changes, one directory, no bookmark on either. jj keeps a described change as a head of its
own the moment you leave it, so switching is picking a change id out of `jj log` and standing
somewhere relative to it. Two commands do that and they differ in what they rewrite. The recipe is
[./recipe.md](./recipe.md); this walks its step 5.

## The scenario

```sh
cd ~/code/repo
jj git fetch
jj new master@origin
echo 'fn parse_array() {}' > src/parser.rs
jj describe -m "feat: teach parser about arrays"
jj new master@origin -m "fix: reject empty input"
echo 'fn reject_empty() {}' > src/validate.rs
```

Two lines now hang off the same parent, and the working copy is on the second:

```sh
jj log -r 'mutable()'
```

```text
@  vryxmzkw  fix: reject empty input
│ ○  qpvuntsm  feat: teach parser about arrays
├─╯
◆  zzkytpwl  docs: fix typo
```

```sh
ls src
```

```text
validate.rs
```

`src/parser.rs` is in the repo and not in this directory: the files on disk are whatever `@`
holds. Each route below is a separate run of that setup, so the commit hashes differ between them.

## Route 1: `jj new`, then `jj squash`

```sh
jj new qpvuntsm
```

```text
Working copy  (@) now at: xnkposlq 3ad7fb24 (empty) (no description set)
Parent commit (@-)      : qpvuntsm 4b8e2a15 feat: teach parser about arrays
Added 1 files, modified 0 files, removed 1 files
```

`src/parser.rs` arrived and `src/validate.rs` left: the other change is untouched in the repo, and
this directory now shows the line you asked for. Edit, read what you did, then fold it in:

```sh
echo 'fn parse_array() { todo!() }' > src/parser.rs
jj diff
jj squash
```

```text
Working copy  (@) now at: mzvwutvl 6d21b9ee (empty) (no description set)
Parent commit (@-)      : qpvuntsm ff0c4a13 feat: teach parser about arrays
```

The diff moved into `qpvuntsm`, which keeps its change id and takes a new hash, and the emptied
commit above it was abandoned. Nothing was rewritten until you ran `squash`, so `jj diff` was a
review step with something still to reject.

## Route 2: `jj edit`

```sh
jj edit qpvuntsm
```

```text
Working copy  (@) now at: qpvuntsm 4b8e2a15 feat: teach parser about arrays
Parent commit (@-)      : zzkytpwl 7b22a8cb docs: fix typo
Added 1 files, modified 0 files, removed 1 files
```

This stands on the change itself. Every edit is snapshotted into it by the next command, with no
step in between where the change is still as you found it:

```sh
echo 'fn parse_array() { todo!() }' > src/parser.rs
jj st
```

```text
Working copy changes:
M src/parser.rs
Working copy  (@) : qpvuntsm ff0c4a13 feat: teach parser about arrays
```

The hash moved from `4b8e2a15` to `ff0c4a13` without a command that asked for it. Upstream
recommends the first route for this reason, and warns off `jj edit` where the change carries a
conflict.

## Which one

| Route                          | Where you stand  | What it rewrites                | Reach for it when                           |
| ------------------------------ | ---------------- | ------------------------------- | ------------------------------------------- |
| `jj new <id>` then `jj squash` | above the change | nothing until `squash` runs     | adding to a change you want to review first |
| `jj edit <id>`                 | on the change    | the change, on the next command | a small amendment you are sure of           |

Either way the change you left is still there: `jj log -r 'mutable()'` lists every head, and a
head that is not `@` is what a branch name would otherwise be doing for you.

## Getting back out

A rewrite you did not mean is one operation, and operations are what jj undoes:

```sh
jj undo
```

```text
Undid operation: 8f2c1a04b7de (2026-08-27 14:05:31) squash commits into qpvuntsm
```

For anything further back, read the log first and restore by id:

```sh
jj op log
jj op restore 5c9e33a1f2b8
```

`jj op restore` puts the repo back to that operation's state. It is not `jj restore`, which
replaces file contents in a commit.

## Hazards

- `jj new <id>` does not stand you on `<id>`; it stands you above it.
- `jj squash` with no argument moves the whole working-copy diff into the parent. Use `-i` or a
  path to move part of it.
- `jj abandon <id>` deletes bookmarks pointing at that change; `--retain-bookmarks` keeps them on
  the parent.
- An empty, undescribed commit is collected once you leave it. Describing a change is what keeps
  it findable, and every route above assumes you did.

## Reference

- [./recipe.md](./recipe.md) — the recipe this walks a step of
- [../parallel-workspace/main-target-local/land-into-a-clean-main-directory.md](../parallel-workspace/main-target-local/land-into-a-clean-main-directory.md)
  — the same three commands compared across two directories
- [FAQ](https://docs.jj-vcs.dev/latest/FAQ/) — resuming a change, and when to avoid `jj edit`
- [Operation log](https://docs.jj-vcs.dev/latest/operation-log/) — `jj undo`, `jj op log`, `jj op restore`
- [CLI reference](https://docs.jj-vcs.dev/latest/cli-reference/) — `jj new`, `jj edit`, `jj squash`

# Three ways to land into a clean main directory

The main directory has nothing in progress: its `@` is the empty, undescribed commit jj leaves
you on after a change. A workspace holds a described change, and the main directory should end up
standing on it. Three commands get there, and they differ in what they leave behind. Where the
main directory carries work of its own, none of this applies — that is
[land-over-work-in-progress.md](./land-over-work-in-progress.md).

The recipe is [parallel-workspace-local.md](../parallel-workspace-local.md); this walks its
step 7.

## The scenario

```sh
cd ~/code/repo
jj workspace add ../wkspc-feat-x
cd ../wkspc-feat-x
echo 'fn parse_array() {}' > src/parser.rs
jj describe -m "feat: teach parser about arrays"
cd -
```

The main directory now sits beside the change, not on it:

```text
@  pkloyysx   (empty)
│ ○  wskwsvrp  feat: teach parser about arrays
├─╯
○  lttnvsoy  docs: add readme
◆  zzzzzzzz   (empty)
```

```sh
ls
```

```text
README.md
```

`src/parser.rs` is in the repo and not in this directory. Each route below is a separate run of
that setup, so the change ids differ between them.

## Route 1: `jj new`

```sh
jj new 'wkspc-feat-x@'
```

```text
Working copy  (@) now at: mmolxzrx ad0338ae (empty) (no description set)
Parent commit (@-)      : wskwsvrp a3f94197 feat: teach parser about arrays
Added 1 files, modified 0 files, removed 0 files
```

```text
@  mmolxzrx   (empty)
○  wskwsvrp  feat: teach parser about arrays
○  lttnvsoy  docs: add readme
◆  zzzzzzzz   (empty)
```

A fresh empty commit, parented on the change, and `Added 1 files` is `src/parser.rs` arriving on
disk. The empty commit this directory used to stand on is gone: nothing was ever put on top of it,
so jj collected it.

## Route 2: `jj rebase -o`

```sh
jj rebase -o 'wkspc-feat-x@'
```

```text
Rebased 1 commits to destination.
Working copy  (@) now at: usqkzpsk 3ad7fb24 (empty) (no description set)
Parent commit (@-)      : xwxwpzzx 9e869c79 feat: teach parser about arrays
Added 1 files, modified 0 files, removed 0 files
```

```text
@  usqkzpsk   (empty)
○  xwxwpzzx  feat: teach parser about arrays
○  stuzuxpw  docs: add readme
◆  zzzzzzzz   (empty)
```

The same shape. The difference is in what moved: this rewrites the empty commit you are standing on
so its parent becomes the change, rather than making a new one. With nothing in that commit, the two
routes are indistinguishable afterwards. `jj rebase` takes `-b @` as its source when none is given.

## Route 3: `jj edit`

```sh
jj edit 'wkspc-feat-x@'
```

```text
Working copy  (@) now at: vmpvqtzr 23bf56bd feat: teach parser about arrays
Parent commit (@-)      : pomlrpms d7e0c417 docs: add readme
Added 1 files, modified 0 files, removed 0 files
```

```text
@  vmpvqtzr  feat: teach parser about arrays
○  pomlrpms  docs: add readme
◆  zzzzzzzz   (empty)
```

This stands on the change itself rather than above it, and both directories are now checked out on
one commit. Any edit here rewrites the commit the workspace has checked out:

```sh
echo 'fn parse_array() { todo!() }' > src/parser.rs
jj st
```

```text
Working copy changes:
A src/parser.rs
Working copy  (@) : vmpvqtzr fa86f7e3 feat: teach parser about arrays
```

The commit id moved from `23bf56bd` to `fa86f7e3`, and the workspace is left behind:

```sh
cd ../wkspc-feat-x && jj st
```

```text
Error: The working copy is stale (not updated since operation 035cd691450a).
Hint: Run `jj workspace update-stale` to update it.
```

Nothing is lost — `jj workspace update-stale` catches those files up — but the route costs a repair
the other two do not.

## Which one

| Route                          | Where you land                                | Cost                                                    |
| ------------------------------ | --------------------------------------------- | ------------------------------------------------------- |
| `jj new 'wkspc-feat-x@'`       | a fresh empty commit above the change         | none                                                    |
| `jj rebase -o 'wkspc-feat-x@'` | the same, by rewriting the commit you were on | none here, and it also covers the work-in-progress case |
| `jj edit 'wkspc-feat-x@'`      | on the change itself, alongside the workspace | the workspace goes stale on your next edit              |

`jj new` is the one the recipe uses: it says what you mean, which is to start the next change on top
of this one. Reach for `jj edit` only to amend the change itself, and expect the staleness.

## Reference

- [land-over-work-in-progress.md](./land-over-work-in-progress.md) — the same landing, over a change
- [../workspaces-share-one-repo.md](../workspaces-share-one-repo.md) — what staleness is
- [CLI reference](https://docs.jj-vcs.dev/latest/cli-reference/) — `jj new`, `jj rebase`, `jj edit`

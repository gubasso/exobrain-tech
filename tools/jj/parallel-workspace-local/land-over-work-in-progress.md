# Three ways to land over work in progress

The main directory carries a change of its own. jj snapshots the working copy on every command, so
that change is already a commit — there is no uncommitted state to protect, only a commit sitting
on the old base. Landing the workspace's change means deciding what happens to it. Where the main
directory is idle, the simpler set applies:
[land-into-a-clean-main-directory.md](./land-into-a-clean-main-directory.md).

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
echo 'half done' > notes.md
jj describe -m "wip: my own change"
```

Two lines on one base, and `notes.md` on disk here:

```text
@  xzrysuop  wip: my own change
│ ○  vzurrxsk  feat: teach parser about arrays
├─╯
○  ylkmwvrp  docs: add readme
◆  zzzzzzzz   (empty)
```

```sh
ls
```

```text
README.md  notes.md
```

Each route below is a separate run of that setup, so the change ids differ between them.

## Route 1: `jj rebase -o`

```sh
jj rebase -o 'wkspc-feat-x@'
```

```text
Rebased 1 commits to destination.
Working copy  (@) now at: xzrysuop 88090f1a wip: my own change
Parent commit (@-)      : vzurrxsk 81b3283c feat: teach parser about arrays
Added 1 files, modified 0 files, removed 0 files
```

```text
@  xzrysuop  wip: my own change
○  vzurrxsk  feat: teach parser about arrays
○  ylkmwvrp  docs: add readme
```

```sh
ls
```

```text
README.md  notes.md  src
```

Your change moved onto the workspace's; you are still standing on it, and both sets of files are on
disk. `removed 0 files` is the point — nothing left. What gets combined is file content in the
working copy, which is where a conflict would appear if both changes touched the same lines. The
graph stays linear: this is not a merge, and no commit here has two parents.

## Route 2: `jj rebase -r`

`jj rebase -o` takes `-b @` as its source, which moves every commit on the line `@` belongs to. Give
it a line of two:

```text
@  omyxltzy  wip: part two
○  wylmpyum  wip: part one
│ ○  outtxnnz  feat: teach parser about arrays
├─╯
○  kprnpomo  docs: add readme
```

```sh
jj rebase -o 'wkspc-feat-x@'
```

```text
Rebased 2 commits to destination.
```

```text
@  omyxltzy  wip: part two
○  wylmpyum  wip: part one
○  outtxnnz  feat: teach parser about arrays
○  kprnpomo  docs: add readme
```

`-r` moves the one commit you name and leaves the rest of the line where it is:

```sh
jj rebase -r @ -o 'wkspc-feat-x@'
```

```text
Rebased 1 commits to destination.
Working copy  (@) now at: nszulwkl e9baaf04 wip: part two
Parent commit (@-)      : ypuukpol 9af7809f feat: teach parser about arrays
Added 1 files, modified 0 files, removed 1 files
```

```text
@  nszulwkl  wip: part two
○  ypuukpol  feat: teach parser about arrays
│ ○  vnrpopwo  wip: part one
├─╯
○  nkvnkvyo  docs: add readme
```

`part one` stayed behind, and `removed 1 files` is its file leaving the disk — the commit you moved
no longer has it as an ancestor. That is the tool for lifting one commit out of a line, and the
wrong default for taking a line onto the change.

## Route 3: `jj new`, and the repair

```sh
jj new 'wkspc-feat-x@'
```

```text
Working copy  (@) now at: xwupynwy a3b2e768 (empty) (no description set)
Parent commit (@-)      : zvromwzo bb332978 feat: teach parser about arrays
Added 1 files, modified 0 files, removed 1 files
```

```text
@  xwupynwy   (empty)
○  zvromwzo  feat: teach parser about arrays
│ ○  srlklswz  wip: my own change
├─╯
○  zvxommpr  docs: add readme
```

```sh
ls
```

```text
README.md  src
```

`jj new` moves you, never your commit, so your change stays on the old base and `notes.md` leaves
the directory. Nothing is lost and nothing is stale; you are simply no longer standing on your own
work. Take its change id from the log and move it:

```sh
jj rebase -r srlklswz -o 'wkspc-feat-x@'
jj edit srlklswz
```

```text
Rebased 1 commits to destination.
Working copy  (@) now at: srlklswz 5e2fa6b9 wip: my own change
Parent commit (@-)      : zvromwzo bb332978 feat: teach parser about arrays
Added 1 files, modified 0 files, removed 0 files
```

That is route 1's result, reached in three commands instead of one.

## Which one

| Route                                  | Your change                                       | Your files         |
| -------------------------------------- | ------------------------------------------------- | ------------------ |
| `jj rebase -o 'wkspc-feat-x@'`         | its whole line moves onto the workspace's change  | kept               |
| `jj rebase -r <id> -o 'wkspc-feat-x@'` | that one commit moves; the rest of the line stays | only that commit's |
| `jj new 'wkspc-feat-x@'`               | stays on the old base, and you leave it           | dropped from disk  |

`jj rebase -o` is the one to reach for. `-r` is for lifting a single commit out of a stack, and
`jj new` belongs to the case where there is nothing to carry.

## Reference

- [land-into-a-clean-main-directory.md](./land-into-a-clean-main-directory.md) — the idle case
- [../workspaces-share-one-repo.md](../workspaces-share-one-repo.md) — addressing and staleness
- [CLI reference](https://docs.jj-vcs.dev/latest/cli-reference/) — `jj rebase`, `jj new`, `jj edit`

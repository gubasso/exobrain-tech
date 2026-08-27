# The four names a change carries in jj

A change worked on in a second directory ends up with four names that all tend to say the same
word. This walks one through the recipe in [./parallel-workspace.md](./parallel-workspace.md).

| Name               | Written as         | Lives in                         | Answers                                        | Who sees it                |
| ------------------ | ------------------ | -------------------------------- | ---------------------------------------------- | -------------------------- |
| directory path     | `../wkspc-feat-x/` | the filesystem                   | where do I edit files                          | you                        |
| workspace name     | `wkspc-feat-x@`    | the repo's view                  | which commit is checked out in that directory  | this repo only             |
| commit description | quoted prose       | the commit object                | what did this change do                        | everyone, permanently      |
| bookmark           | `bkmrk-feat-x`     | the repo's view, exported to git | which commit does the world fetch as this name | the remote and its readers |

Prefixes are teaching labels lasting while a name is jj's alone; the bookmark is renamed to
`feat-x` before the push. In the graphs `@` is the working copy you stand in, letters are change ids.

## Starting point

```text
DISK                          COMMIT GRAPH
~/code/repo/   <- you are     @  C   (empty) (no description)
                              ◆  B   docs: fix typo
                              ○  A   feat: add parser

REPO TABLES
  workspaces:  default@ -> C
  bookmarks:   develop -> B    develop@origin -> B
```

Three of the four names already exist: the directory, the workspace `default@`, the bookmark
`develop`. `C` is empty and undescribed, so jj discards it once it is no longer checked out.

## Adding the workspace

```sh
jj workspace add ../wkspc-feat-x
```

The new working-copy commit takes `C`'s parents, so `D` is born a sibling of `C`, not a child.

```text
DISK                               COMMIT GRAPH
~/code/repo/          <- you       @  C   (empty)          default@
~/code/wkspc-feat-x/  <- new       │ ○  D   (empty)        wkspc-feat-x@
                                   ├─╯
                                   ◆  B   docs: fix typo
                                   ○  A   feat: add parser

REPO TABLES
  workspaces:  default@ -> C          wkspc-feat-x@ -> D
  bookmarks:   develop -> B           develop@origin -> B
```

One directory, one new row in the workspace table. The workspace name came from the directory
basename; that is the only link between the two.

## Entering it

`cd ../wkspc-feat-x && jj st` changes nothing in the repo. You moved, so `@` now resolves to `D`
instead of `C`, and the revset `default@` names `C`.

## Editing

jj snapshots on every command, rewriting `D` in place: same change id, new commit hash.

```text
COMMIT GRAPH                            REPO TABLES
@  D   parser.rs +12 -1                   workspaces:  default@ -> C  wkspc-feat-x@ -> D
◆  B   docs: fix typo                     bookmarks:   develop -> B
```

## Describing

```sh
jj describe -m "feat: teach parser about arrays"
```

Rewrites `D` again, writing text into the commit object. The commit is now durable rather than
discardable, and still nothing is shareable.

## Setting the bookmark

```sh
jj bookmark set bkmrk-feat-x
```

A bookmark is a reference: a name pointing at one commit id. A row appears in the bookmark table
and the graph does not change. So far it is only a bookmark — a row in this repo, which is what
licenses the prefix. A colocated repo would export it to its own `.git` on the next command; this
workspace has no `.git/`, and the remote knows nothing until a push.

```text
COMMIT GRAPH                                       REPO TABLES
○  C   (empty)                    default@         workspaces:  default@ -> C
│ @  D   feat: teach parser...    bkmrk-feat-x                  wkspc-feat-x@ -> D
├─╯                                                bookmarks:   develop -> B
◆  B   docs: fix typo             develop                       bkmrk-feat-x -> D
                                                                develop@origin -> B
```

Two rows point at `D`: jj moves `wkspc-feat-x@` on every command; `bkmrk-feat-x` is yours to move.

## Rebasing

A colleague lands `E`. The rebase replays your change on top of it.

```text
BEFORE                                   AFTER
○  E   fix: null deref                   @  D   feat: parser...  bkmrk-feat-x
│      develop@origin                    ○  E   fix: null deref
│ @  D   feat: parser...  bkmrk-feat-x   │      develop@origin
├─╯                                      ◆  B   docs: fix typo
◆  B   docs: fix typo  develop           ○  A   feat: add parser
```

`bkmrk-feat-x` moved with `D`, untouched by hand.

## Renaming

```sh
jj bookmark rename bkmrk-feat-x feat-x
```

A branch takes the bookmark's name character for character, so the label comes off in the last
step before the name leaves the repo. No commit moves, the row now reads `feat-x -> D`, and a
clean rename prints nothing.

## Pushing

```sh
jj git push --bookmark feat-x
```

The first time any of these names leaves the machine.

```text
REPO TABLES                              ORIGIN
  bookmarks:  develop -> B               refs/heads/develop -> E
              feat-x -> D                refs/heads/feat-x  -> D
              develop@origin -> E
              feat-x@origin -> D
```

This is where the bookmark becomes a branch someone else can fetch, of that exact name, and the
mapping runs both ways — a branch made in the backing git repo comes back as a bookmark. The
directory and workspace names did not travel.

## Landing and retiring

```text
REPO TABLES
  workspaces:  default@ -> C             (the wkspc-feat-x@ row is gone)
  bookmarks:   develop -> D      feat-x -> D
```

`jj bookmark set develop -r feat-x` is the landing: a local pointer move, no merge, because both
directories always wrote into one repo; the remote branch moves only when that bookmark is pushed.
`jj workspace forget wkspc-feat-x` drops the workspace row and touches no commit.

## The rule that catches people

A bookmark follows the change through any operation that rewrites a commit — `rebase`, `describe`,
`squash`, an `abandon` upstream of it. It does not follow you onto a new commit: `jj new` starts a
different change, and the bookmark stays where it was.

```text
rewrite (same change)             new commit (different change)

  D  ->  D'   feat-x moves          @  F   (empty)
              on its own            ○  D   feat: parser...  feat-x
```

`jj bookmark set feat-x` brings it forward: rewriting is this change revised, so the name comes
along; `jj new` is a different change. Where a git branch advances on every commit, in jj
publishing is a deliberate act.

## What a bookmark is not

A bookmark has no workspace component. It is one repo-wide reference — a name pointing at one
commit id — readable and settable from any workspace, and it may point at a commit that nothing
has checked out.

The workspace table is its mirror image, which is why the two blur together:

```text
  workspaces:  default@      -> C     one row per directory, jj owns it, never pushed
               wkspc-feat-x@ -> D

  bookmarks:   develop       -> D     repo-wide, you own it, pushed to origin
               feat-x        -> D
```

Both map a name to a commit. One is jj's bookkeeping about desks; the other is your published
label. They looked coupled above only because the bookmark was set while standing on `D`.

## Reference

- [./parallel-workspace.md](./parallel-workspace.md) — the recipe these steps come from
- [./workspaces-share-one-repo.md](./workspaces-share-one-repo.md) — landing and staleness
- [Bookmarks](https://docs.jj-vcs.dev/latest/bookmarks/) — tracking, push, fast-forward rule
- [Revsets](https://docs.jj-vcs.dev/latest/revsets/) — `@`, `<workspace>@`, `working_copies()`

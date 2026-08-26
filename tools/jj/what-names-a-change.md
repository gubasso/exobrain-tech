# The four names a change carries in jj

A change worked on in a second directory ends up with four names, easy to confuse because
they all tend to say the same word. This walks one change through the recipe in
[./parallel-workspace.md](./parallel-workspace.md), showing what each name does at each step.

| Name               | Written as         | Lives in                         | Answers                                        | Who sees it                |
| ------------------ | ------------------ | -------------------------------- | ---------------------------------------------- | -------------------------- |
| directory path     | `../wkspc-feat-x/` | the filesystem                   | where do I edit files                          | you                        |
| workspace name     | `wkspc-feat-x@`    | the repo's view                  | which commit is checked out in that directory  | this repo only             |
| commit description | quoted prose       | the commit object                | what did this change do                        | everyone, permanently      |
| bookmark           | `bkmrk-feat-x`     | the repo's view, exported to git | which commit does the world fetch as this name | the remote and its readers |

The `wkspc-` and `bkmrk-` prefixes are this shelf's teaching labels; the `@` forms are jj's own
syntax — a workspace is the revset `<name>@`, a remote bookmark `<name>@origin`, a local bookmark
its bare name. In the graphs `@` alone is the working copy you stand in, letters are change ids.

## Starting point

```text
DISK                          COMMIT GRAPH
~/code/repo/   <- you are     @  C   (empty) (no description)
                              ◆  B   docs: fix typo
                              ○  A   feat: add parser

REPO TABLES
  workspaces:  default@ -> C
  bookmarks:   bkmrk-develop -> B    bkmrk-develop@origin -> B
```

Three of the four names already exist: the directory, the workspace `default@`, the bookmark
`bkmrk-develop`. `C` is empty and undescribed, so jj discards it once it is no longer checked out.

## Adding the workspace

```sh
jj workspace add ../wkspc-feat-x
```

The new working-copy commit takes the parents of `C`, which is `B`. So `D` is born a sibling
of `C`, not a child.

```text
DISK                               COMMIT GRAPH
~/code/repo/          <- you       @  C   (empty)          default@
~/code/wkspc-feat-x/  <- new       │ ○  D   (empty)        wkspc-feat-x@
                                   ├─╯
                                   ◆  B   docs: fix typo
                                   ○  A   feat: add parser

REPO TABLES
  workspaces:  default@ -> C          wkspc-feat-x@ -> D
  bookmarks:   bkmrk-develop -> B     bkmrk-develop@origin -> B
```

One directory, one new row in the workspace table. The workspace name came from the
directory basename; that is the only link between the two.

## Entering it

`cd ../wkspc-feat-x && jj st` changes nothing in the repo. You moved, so `@` now resolves to
`D` instead of `C`, and the revset `default@` names `C`.

## Editing

jj snapshots on every command, rewriting `D` in place: same change id, new content, new
commit hash. jj updates the workspace row for you.

```text
COMMIT GRAPH                            REPO TABLES
@  D   parser.rs +12 -1                   workspaces:  default@ -> C
       (no description)                                wkspc-feat-x@ -> D
◆  B   docs: fix typo                     bookmarks:   bkmrk-develop -> B
```

## Describing

```sh
jj describe -m "feat: teach parser about arrays"
```

Rewrites `D` again, this time writing text into the commit object itself. Same change id,
and now the commit is durable rather than discardable. Still nothing shareable.

## Setting the bookmark

```sh
jj bookmark set bkmrk-feat-x
```

A bookmark is a reference: a name pointing at one commit id. A row appears in the bookmark
table and the graph does not change. So far it is only a bookmark — a row in this repo. A
colocated repo exports it to its own `.git` as a local branch on the next command; a jj-only
workspace has no `.git/`, and the remote knows nothing either way until you push.

```text
COMMIT GRAPH                                     REPO TABLES
○  C   (empty)                    default@         workspaces:  default@ -> C
│ @  D   feat: teach parser...    bkmrk-feat-x                  wkspc-feat-x@ -> D
├─╯                                                bookmarks:   bkmrk-develop -> B
◆  B   docs: fix typo             bkmrk-develop                 bkmrk-feat-x -> D
                                                                bkmrk-develop@origin -> B
```

Two rows point at `D`: jj moves `wkspc-feat-x@` on every command; `bkmrk-feat-x` is yours to move.

## Rebasing

A colleague lands `E`. The rebase replays your change on top of it.

```text
BEFORE                                   AFTER
○  E   fix: null deref                   @  D   feat: parser...  bkmrk-feat-x
│      bkmrk-develop@origin              ○  E   fix: null deref
│ @  D   feat: parser...  bkmrk-feat-x   │      bkmrk-develop@origin
├─╯                                      ◆  B   docs: fix typo
◆  B   docs: fix typo  bkmrk-develop     ○  A   feat: add parser
```

`bkmrk-feat-x` moved with `D`, untouched by hand.

## Pushing

```sh
jj git push --bookmark bkmrk-feat-x
```

The first time any of these names leaves the machine.

```text
REPO TABLES                                ORIGIN
  bookmarks:  bkmrk-develop -> B             refs/heads/bkmrk-develop  -> E
              bkmrk-feat-x -> D              refs/heads/bkmrk-feat-x   -> D
              bkmrk-develop@origin -> E
              bkmrk-feat-x@origin -> D
```

This is where the bookmark becomes a branch someone else can fetch: on the remote it is an
ordinary git branch of that exact name, and the mapping runs both ways — a branch made in the
backing git repo comes back as a bookmark. The directory and workspace names did not travel.

## Landing and retiring

```text
REPO TABLES
  workspaces:  default@ -> C            (the wkspc-feat-x@ row is gone)
  bookmarks:   bkmrk-develop -> D       bkmrk-feat-x -> D
```

`jj bookmark set bkmrk-develop -r bkmrk-feat-x` is the landing: a local pointer move, no merge,
because both directories were always writing into one repo; the branch on the remote moves only
when that bookmark is pushed. `jj workspace forget wkspc-feat-x` drops the workspace row and
touches no commit; the bookmark outlives it until deleted.

## The rule that catches people

A bookmark follows the change, through any operation that rewrites a commit — `rebase`,
`describe`, `squash`, an `abandon` upstream of it. jj rewrites every ref that pointed at the
old commit to point at the new one.

A bookmark does not follow you onto a new commit. `jj new` starts a different change, and
the bookmark stays where it was.

```text
rewrite (same change)             new commit (different change)

  D  ->  D'   bkmrk-feat-x moves    @  F   (empty)
              on its own            ○  D   feat: parser...  bkmrk-feat-x
```

`jj bookmark set bkmrk-feat-x` brings it forward. Rewriting is this change, revised, so the
name comes along; `jj new` is a different change, so it does not. Where a git branch advances on
every commit, in jj publishing is a deliberate act.

## What a bookmark is not

A bookmark has no workspace component. It is one repo-wide reference — a name pointing at one
commit id — readable and settable from any workspace, saying nothing about which directory
produced the commit. It may point at a commit that nothing has checked out.

The workspace table is its mirror image, which is why the two blur together:

```text
  workspaces:  default@      -> C     one row per directory, jj owns it, never pushed
               wkspc-feat-x@ -> D

  bookmarks:   bkmrk-develop -> B     repo-wide, you own it, pushed to origin
               bkmrk-feat-x  -> D
```

Both map a name to a commit. One is jj's bookkeeping about desks; the other is your published
label. They looked coupled above only because `bkmrk-feat-x` was set while standing on `D`.

## Reference

- [./parallel-workspace.md](./parallel-workspace.md) — the recipe these steps come from
- [./workspaces-share-one-repo.md](./workspaces-share-one-repo.md) — why landing is a
  bookmark move and what staleness is
- [Bookmarks](https://docs.jj-vcs.dev/latest/bookmarks/) — tracking, push, fast-forward rule
- [Revsets](https://docs.jj-vcs.dev/latest/revsets/) — `@`, `<workspace>@`, `working_copies()`

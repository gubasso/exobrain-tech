# The four names a change carries in jj

A change worked on in a second directory ends up with four names, easy to confuse because
they all tend to say the same word. This walks one change through the recipe in
[./parallel-workspace.md](./parallel-workspace.md), showing what each name does at each step.

| Name               | Lives in                         | Answers                                        | Who sees it                |
| ------------------ | -------------------------------- | ---------------------------------------------- | -------------------------- |
| directory path     | the filesystem                   | where do I edit files                          | you                        |
| workspace name     | the repo's view                  | which commit is checked out in that directory  | this repo only             |
| commit description | the commit object                | what did this change do                        | everyone, permanently      |
| bookmark           | the repo's view, exported to git | which commit does the world fetch as this name | the remote and its readers |

Legend for the graphs below: `@` marks the working-copy commit of the workspace you are
standing in, and single letters are change ids.

## Starting point

```text
DISK                          COMMIT GRAPH
~/code/repo/   <- you are     @  C   (empty) (no description)
                              ◆  B   docs: fix typo
                              ○  A   feat: add parser

REPO TABLES
  workspaces:  default -> C
  bookmarks:   main -> B          main@origin -> B
```

Three of the four names already exist: the directory, the workspace `default`, the bookmark
`main`. Commit `C` is empty and undescribed, so jj discards it once it stops being a working copy.

## Adding the workspace

```sh
jj workspace add ../repo-feat-x
```

The new working-copy commit takes the parents of `C`, which is `B`. So `D` is born a sibling
of `C`, not a child.

```text
DISK                              COMMIT GRAPH
~/code/repo/         <- you       @  C   (empty)          [default]
~/code/repo-feat-x/  <- new       │ ○  D   (empty)        [repo-feat-x]
                                  ├─╯
                                  ◆  B   docs: fix typo
                                  ○  A   feat: add parser

REPO TABLES
  workspaces:  default -> C       repo-feat-x -> D
  bookmarks:   main -> B          main@origin -> B
```

One directory, one new row in the workspace table. The workspace name came from the
directory basename; that is the only link between the two.

## Entering it

`cd ../repo-feat-x && jj st` changes nothing in the repo. You moved, so `@` now resolves to
`D` instead of `C`, and the revset `default@` names `C`.

## Editing

jj snapshots on every command, rewriting `D` in place: same change id, new content, new
commit hash. jj updates the workspace row for you.

```text
COMMIT GRAPH                            REPO TABLES
@  D   parser.rs +12 -1                   workspaces:  default -> C
       (no description)                                repo-feat-x -> D
◆  B   docs: fix typo   main              bookmarks:   main -> B
```

This is the workspace name doing its one job: tracking a moving target so the repo knows
what sits on that desk.

## Describing

```sh
jj describe -m "feat-x: teach parser about arrays"
```

Rewrites `D` again, this time writing text into the commit object itself. Same change id,
and now the commit is durable rather than discardable. Still nothing shareable.

## Setting the bookmark

```sh
jj bookmark set feat-x
```

A row appears in the bookmark table. The graph does not change.

```text
COMMIT GRAPH                                     REPO TABLES
○  C   (empty)                    [default]        workspaces:  default -> C
│ @  D   feat-x: teach parser...  feat-x                        repo-feat-x -> D
├─╯                                                bookmarks:   main -> B
◆  B   docs: fix typo             main                          feat-x -> D
                                                                main@origin -> B
```

Two rows now point at `D`, and their behaviour diverges from here: jj rewrites
`repo-feat-x -> D` on every command, while `feat-x -> D` is yours to move.

## Rebasing

A colleague lands `E`. The rebase replays your change on top of it.

```text
BEFORE                              AFTER
○  E   fix: null deref              @  D   feat-x: teach parser...  feat-x
│      main@origin                  ○  E   fix: null deref          main@origin
│ @  D   feat-x: ...  feat-x        ◆  B   docs: fix typo
├─╯                                 ○  A   feat: add parser
◆  B   docs: fix typo  main
```

`feat-x` moved with `D`, untouched by hand.

## Pushing

```sh
jj git push --bookmark feat-x
```

The first time any of these names leaves the machine.

```text
REPO TABLES                          ORIGIN
  bookmarks:  main -> B                refs/heads/main   -> E
              feat-x -> D              refs/heads/feat-x -> D
              main@origin -> E
              feat-x@origin -> D
```

The directory and the workspace name did not travel — git has no vocabulary for either.

## Landing and retiring

```text
REPO TABLES
  workspaces:  default -> C            (the repo-feat-x row is gone)
  bookmarks:   main -> D    feat-x -> D
```

`jj bookmark set main -r feat-x` is the landing: a pointer move, no merge, because both
directories were always writing into one repo. `jj workspace forget repo-feat-x` then drops
the workspace row and touches no commit; the bookmark outlives it until
`jj bookmark delete feat-x`.

## The rule that catches people

A bookmark follows the change, through any operation that rewrites a commit — `rebase`,
`describe`, `squash`, an `abandon` upstream of it. jj rewrites every ref that pointed at the
old commit to point at the new one.

A bookmark does not follow you onto a new commit. `jj new` starts a different change, and
the bookmark stays where it was.

```text
rewrite (same change)         new commit (different change)

  D  ->  D'   feat-x moves      @  F   (empty)
              on its own        ○  D   feat-x: teach parser...  feat-x
```

`jj bookmark set feat-x` brings it forward. Rewriting is this change, revised, so the name
comes along; `jj new` is a different change, so it does not. Where a git branch advances on
every commit, in jj publishing is a deliberate act.

## What a bookmark is not

A bookmark has no workspace component. It is one repo-wide row mapping a name to a commit
id, readable and settable from any workspace, saying nothing about which directory produced
the commit or which working copy sits near it. You can point one at a commit no workspace
has ever checked out.

The workspace table is its mirror image, which is why the two blur together:

```text
  workspaces:  default     -> C     one row per directory, jj owns it, never pushed
               repo-feat-x -> D

  bookmarks:   main   -> B          repo-wide, you own it, pushed to origin
               feat-x -> D
```

Both map a name to a commit. One is jj's bookkeeping about desks; the other is your
published label. They looked coupled above only because `feat-x` happened to be set while
standing on `D`.

## Reference

- [./parallel-workspace.md](./parallel-workspace.md) — the recipe these steps come from
- [./workspaces-share-one-repo.md](./workspaces-share-one-repo.md) — why landing is a
  bookmark move and what staleness is
- [Bookmarks](https://docs.jj-vcs.dev/latest/bookmarks/) — tracking, push, fast-forward rule
- [Revsets](https://docs.jj-vcs.dev/latest/revsets/) — `@`, `<workspace>@`, `working_copies()`

# Working on a change in a parallel directory

One repo, two working directories. `jj workspace add` gives a change its own directory and its own
working-copy commit while the main directory keeps whatever it was holding, and both directories
write into the same commit graph — which is why nothing is ever merged back between them.

Both variants below run the same spine: add the workspace, edit, describe, rebase onto whatever
moved, land, retire the workspace. They split on where the change has to arrive.

| The change must reach                   | Use                   | What carries it there               |
| --------------------------------------- | --------------------- | ----------------------------------- |
| other people, through `origin`          | `main-target-remote/` | a bookmark, pushed as a branch      |
| only the main directory on this machine | `main-target-local/`  | the workspace name, and no bookmark |

## Starting

| Document                                                                                 | What it gets you                                                                                                                                |
| ---------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| [start-while-the-main-directory-is-busy.md](./start-while-the-main-directory-is-busy.md) | The workspace parented on `master@origin` with `-r` while the main directory holds unfinished work on another line, and nothing there disturbed |

## Landing at the remote

| Document                                                                              | What it gets you                                                                                                                   |
| ------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| [main-target-remote/recipe.md](./main-target-remote/recipe.md)                        | The whole run against a remote: workspace, bookmark, rebase onto `develop@origin`, rename, push, land, retire                      |
| [land-through-a-pull-request.md](./main-target-remote/land-through-a-pull-request.md) | The remote merges the branch and a fetch reads the position back, leaving a two-parent commit at the tip and your change immutable |
| [land-by-moving-the-bookmark.md](./main-target-remote/land-by-moving-the-bookmark.md) | No review: you point `develop` at the work and push, the line stays linear, and one commit wears both names                        |

## Landing in the main directory

| Document                                                                                       | What it gets you                                                                                                       |
| ---------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| [main-target-local/recipe.md](./main-target-local/recipe.md)                                   | The whole run with no remote and no bookmark: the two directories address each other as `default@` and `wkspc-feat-x@` |
| [land-into-a-clean-main-directory.md](./main-target-local/land-into-a-clean-main-directory.md) | `jj new`, `jj rebase`, and `jj edit` compared where nothing is in progress, and why `jj edit` costs a stale workspace  |
| [land-over-work-in-progress.md](./main-target-local/land-over-work-in-progress.md)             | The same three where the main directory carries its own change, and which one strands that change off to the side      |

## Reference

- [../one-directory/](../one-directory/) — the same change developed where you already stand, with
  no second working copy and no directory to retire
- [../workspaces-share-one-repo.md](../workspaces-share-one-repo.md) — why landing is a bookmark
  move rather than a merge, how `<workspace>@` addresses another working copy, and what staleness is
- [../what-names-a-change.md](../what-names-a-change.md) — the four names a change picks up along
  the way, walked through one scenario with commit graphs and repo tables
- [../README.md](../README.md) — the rest of the jj notes, and the naming convention these use

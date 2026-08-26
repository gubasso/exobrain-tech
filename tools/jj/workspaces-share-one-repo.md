# Why a jj workspace needs no merge back

A jj workspace is a second working copy — its own files plus a `.jj/` directory linked to
the same repo. Every workspace shares one commit graph and one operation log, so a commit
made in one is visible from all of them the moment it exists. This walks what that changes
about a parallel-directory workflow, alongside the recipe in
[parallel-workspace.md](./parallel-workspace.md).

## The scenario

A repo at `~/src/app` holds a half-finished refactor. A bug report arrives that needs a fix
against `main`, and the refactor is not in a state worth interrupting.

### Adding the workspace

```sh
cd ~/src/app
jj git fetch
jj workspace add --name hotfix -r main@origin ../app-hotfix
```

Nothing moved in `~/src/app`. Its `@` still holds the refactor, files untouched. What the
repo gained is a second working-copy commit, empty, parented on `main@origin`, and a
directory whose files match it.

`jj workspace list` now names two workspaces with their roots. From either directory,
`jj log` shows both lines of work in one graph — there is no fetch or push between them.

### Addressing the other workspace

In `../app-hotfix`, `@` is the hotfix commit. The refactor is still reachable:

```sh
jj log -r 'default@ | @'
```

`default@` is the working-copy commit of the workspace named `default`. Any workspace name
followed by `@` resolves the same way, and `working_copies()` resolves to all of them at
once. This is what replaces walking over to the other directory to look.

### Landing the hotfix

The fix is committed in `../app-hotfix`. Landing it is a bookmark move, not a merge:

```sh
jj bookmark set hotfix -r @
jj git push --bookmark hotfix
```

The commit was already in the repo `~/src/app` reads from. Nothing was transferred between
the two directories, because there was never a second copy of the history to transfer. A
git worktree would have needed a branch checked out here and a merge or rebase there; jj
has no checked-out bookmark at all, which is also why two workspaces may sit on the same
line of work.

### Where staleness comes in

Back in `~/src/app`, the refactor should now sit on top of the fix:

```sh
jj rebase -b @ -o hotfix
```

Run from `~/src/app`, this is ordinary. Run the same rebase from `../app-hotfix` and it
rewrites a commit that another workspace has checked out. The repo records the new commit
immediately; the files in `~/src/app` still match the old one. That workspace is stale —
its `.jj/working_copy/` records an operation older than the repo's current one.

```sh
cd ~/src/app
jj workspace update-stale
```

The files catch up. Nothing was lost: staleness is a statement about the working copy
lagging the repo, not about damage. The same state follows a `Ctrl-C` during the final
update step, or an operation abandoned out from under the working copy — in which case
`update-stale` writes a recovery commit holding the working-copy contents.

### Retiring it

```sh
jj workspace forget hotfix
rm -rf ../app-hotfix
```

`forget` stops the repo tracking that working-copy commit and touches no files. The
commits stay; only the second working copy goes away.

## What a secondary workspace does not have

A Git-backed jj repo is colocated by default: `.jj/` and `.git/` side by side, so git
tooling works in the primary directory. `jj workspace add` creates a jj-only workspace —
`.jj/` and no `.git/`. Anything in that directory that expects a git repo will not find
one. Per-workspace Git HEAD is tracked internally as of the development branch after
0.44.0, which prepares multiple Git worktrees for colocated repos; it is not in a release.

## Reference

- [Working copy](https://docs.jj-vcs.dev/latest/working-copy/) — the workspace and stale-working-copy sections
- [Revsets](https://docs.jj-vcs.dev/latest/revsets/) — `@`, `<workspace>@`, and `working_copies()`
- [Git compatibility](https://docs.jj-vcs.dev/latest/git-compatibility/) — colocation and its trade-offs

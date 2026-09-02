# Why a jj workspace needs no merge back

A jj workspace is a second working copy — its own files plus a `.jj/` directory linked to
the same repo. Every workspace shares one commit graph and one operation log, so a commit
made in one is visible from all of them the moment it exists. This walks what that changes
about a parallel-directory workflow, alongside the
[remote recipe](./parallel-workspace/main-target-remote/recipe.md). Names carry a kind prefix
while they are jj's alone, per the convention in [README.md](./README.md).

## The scenario

A repo at `~/src/app` holds a half-finished refactor. A bug report arrives that needs a fix
against the trunk `master`, and the refactor is not in a state worth interrupting.

### Adding the workspace

```sh
cd ~/src/app
jj git fetch
jj workspace add -r master@origin ../wkspc-hotfix
```

The workspace is named for its directory, so it is addressed as `wkspc-hotfix@`, while the
bookmark this work will land under is `bkmrk-hotfix`, and the branch the remote ends up with is
`hotfix` — three names, deliberately distinct.

Nothing moved in `~/src/app`. Its `@` still holds the refactor, files untouched. What the
repo gained is a second working-copy commit, empty, parented on `master@origin`, and a
directory whose files match it. Without `-r`, that commit takes the parents of the current
workspace's `@` instead — the last commit you made, without the change in progress.

`jj workspace list` now names two workspaces with their roots. From either directory,
`jj log` shows both lines of work in one graph — there is no fetch or push between them.

### Addressing the other workspace

In `../wkspc-hotfix`, `@` is the hotfix commit. The refactor is still reachable:

```sh
jj log -r 'default@ | @'
```

`default@` is the working-copy commit of the workspace jj named `default` when the repo was
created. Any workspace name followed by `@` resolves the same way, and `working_copies()`
resolves to all of them at once. This is what replaces walking over to the other directory
to look.

### Landing the hotfix

The fix is committed in `../wkspc-hotfix`. Landing it is a bookmark move, not a merge:

```sh
jj bookmark set bkmrk-hotfix
jj bookmark rename bkmrk-hotfix hotfix
jj git push --bookmark hotfix
```

The `set` is local, and so is the rename: it drops the teaching prefix in the step before the
name leaves the repo, because the branch takes the bookmark's name character for character. The
push is what creates the branch `hotfix` on the remote.

The commit was already in the repo `~/src/app` reads from. Nothing was transferred between
the two directories, because there was never a second copy of the history to transfer. A
git worktree would have needed a branch checked out here and a merge or rebase there. A
bookmark is a reference — a name pointing at one commit id — and what jj pushes as a branch,
but it is never checked out, which is why two workspaces may sit on the same line of work.

### Where staleness comes in

Back in `~/src/app`, the refactor should now sit on top of the fix:

```sh
jj rebase -o hotfix
```

`jj rebase` takes `-b @` as its source when none is given. Run from `~/src/app`, this is
ordinary. Run the same rebase from `../wkspc-hotfix` and it
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
jj workspace forget wkspc-hotfix
rm -rf ../wkspc-hotfix
```

`forget` stops the repo tracking that working-copy commit and touches no files. The
commits stay; only the second working copy goes away.

## What a secondary workspace does not have

A Git-backed jj repo is colocated by default: `.jj/` and `.git/` side by side, so git
tooling works in the primary directory. Exactly one directory can be that one, because the
Git store itself lives there — `jj git colocation enable` works by moving
`.jj/repo/store/git` to `.git`, and a secondary workspace has no store of its own to move.
So `jj workspace add` creates a jj-only workspace: `.jj/` and no `.git/`. Per-workspace Git
HEAD is tracked internally as of the development branch after 0.44.0, which prepares
multiple Git worktrees for colocated repos; it is not in a release.

What that costs is narrower than it sounds. One commit store and one bookmark table serve
every workspace, so a Git branch and the bookmark of that name are one line of work under
two names, and this directory reads every branch, commit, and bookmark exactly as the
primary one does. What it lacks is a place a `git` command can run.

### When the two sets of books disagree

jj keeps its own bookmark table and syncs it with the backing Git repo at both ends of every
command run in a colocated workspace. A jj-only workspace triggers no such sync, so a branch
that plain `git` wrote in the primary directory stays invisible here until something reads
it across:

```sh
jj git import
```

`jj git export` is the same in the other direction, and both run from a jj-only workspace.
The gap opens only when `git` writes a ref in the primary directory and no jj command runs
there afterwards; any jj command in that directory closes it, and driving that directory
with jj rather than git means it never opens. Import abandons commits Git can no longer
reach, so `jj undo` is the way back from one that read in a force-moved branch.

## Reference

- [Working copy](https://docs.jj-vcs.dev/latest/working-copy/) — the workspace and stale-working-copy sections
- [Revsets](https://docs.jj-vcs.dev/latest/revsets/) — `@`, `<workspace>@`, and `working_copies()`
- [Git compatibility](https://docs.jj-vcs.dev/latest/git-compatibility/) — colocation and its trade-offs

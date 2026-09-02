# Landing through a pull request

Review happens on the remote, so the remote is what moves `master`. No command here lands the
work; the merge button does, and a fetch brings the new position back. Where no review gate stands
and you move the name yourself, that is
[land-by-moving-the-bookmark.md](./land-by-moving-the-bookmark.md).

The recipe is [recipe.md](./recipe.md); this walks its step 10.

## The scenario

The recipe has run through its push, so the branch `feat-x` exists on the remote and
`master@origin` still names the commit it did before:

```text
@  tpvxtqly  feat-x wkspc-feat-x@  feat: teach parser about arrays
│ ○  wmxomuyw  default@  (empty)
├─╯
◆  slqynpzq  master  docs: add readme
```

## Reading the merge back

The pull request is squash-merged on the remote. That writes one new commit this repo has never
seen, carrying the request's title as its message, and one command brings it back:

```sh
jj git fetch
```

```text
bookmark: master@origin [updated] tracked
```

One line, and nothing else. The working copy did not move and your change did not become immutable,
because a squash merge writes a new commit rather than taking yours: the commit the remote now
carries is not your commit, so yours is not an ancestor of `master@origin`.

```sh
jj log
```

```text
@  tpvxtqly  feat-x wkspc-feat-x@  feat: teach parser about arrays
│ ◆  osxkxutm  master  feat: teach parser about arrays (#42)
├─╯
◆  slqynpzq  docs: add readme
```

The two lines are siblings. Your change and the landed commit hold the same content under different
ids, and the fork point is the commit `master` named before the merge. This is the whole difference
from a merge-commit route, which would have put your commit itself under the new tip.

## Retiring the name and the change

Nothing on the remote is yours to keep now. Forget the bookmark, on both sides:

```sh
jj bookmark forget --include-remotes feat-x
```

```text
Forgot 1 local bookmarks.
Forgot 2 remote bookmarks.
```

Then abandon the local change the squash superseded:

```sh
jj abandon tpvxtqly
```

```text
Abandoned 1 commits:
  tpvxtqly 59023837 feat: teach parser about arrays
```

Abandoning is safe because the content landed — it is on `master` under `osxkxutm`. Skipping this
step is what leaves a repo carrying a duplicate of every change it ever landed.

## Bringing the main directory onto it

```sh
cd - && jj new master@origin
```

```text
Working copy  (@) now at: uoyuumrl 2c53c9d5 (empty) (no description set)
Parent commit (@-)      : osxkxutm e75db4ca master | feat: teach parser about arrays (#42)
Added 1 files, modified 0 files, removed 0 files
```

```text
@  uoyuumrl  default@  (empty)
◆  osxkxutm  master  feat: teach parser about arrays (#42)
◆  slqynpzq  docs: add readme
```

`Added 1 files` is `src/parser.rs` arriving in this directory. The line is linear — one commit per
landed pull request, which is what the squash-only trunk buys.

## Reference

- [land-by-moving-the-bookmark.md](./land-by-moving-the-bookmark.md) — landing with no review
- [../main-target-local/land-over-work-in-progress.md](../main-target-local/land-over-work-in-progress.md)
  — what `jj new` costs when this directory carries a change
- [../../workspaces-share-one-repo.md](../../workspaces-share-one-repo.md) — why no merge back is needed
- [../../../../workflows/trunk-based-development.md](../../../../workflows/trunk-based-development.md)
  — why the trunk takes squash merges only
- [CLI reference](https://docs.jj-vcs.dev/latest/cli-reference/) — `jj git fetch`, `jj bookmark
  forget`, `jj abandon`, `jj new`

# Trunk-Based Development

One permanent branch, short-lived branches that die at the merge, and releases promoted from the
trunk. This chapter states the model; the git and jj buckets carry the commands that run it.

## The trunk

`master` is the only permanent branch and the repository default. Every other branch is short-lived
and expected to disappear.

- The trunk takes no direct push and no force-push.
- A merge requires a pull request carrying the named passing check.
- Squash is the only merge method, so one pull request is one commit and the history stays linear.
- The forge deletes a branch when its merge lands, so keeping the trunk sole costs no one a habit.

Resistance to a second long-lived branch is the whole discipline. What looks like a need for one is
usually one of three needs with better answers:

| The apparent need               | The answer                    |
| ------------------------------- | ----------------------------- |
| Stage unfinished work somewhere | A feature flag                |
| Stabilize before a ship         | A just-in-time release branch |
| Integrate before production     | An environment                |

Branches isolate code; environments isolate deployment.

## The short-lived branch

A branch lives a day or two and carries one author's work. It is rebased onto `origin/master`,
squash-merged after review and CI, and deleted.

The branch dies at the merge and its name never enters history, so the name has one job: routing
while the branch is alive. Two forms do that job, and they compose.

- `<type>/<slug>`, with the type mirroring the Conventional Commit type the squash title will
  carry: `feat/oauth-login`, `fix/empty-csv-upload`.
- `<issue-id>-<slug>`, the shape a forge mints when it generates a branch from an issue. Prefer
  letting the forge mint it — one command links the branch, its pull request, and the issue's
  closing.

A tracker outside the forge matches its issue keys anywhere in a branch name, so its key rides
inside either form: `fix/PROJ-412-empty-csv`.

The branch name binds nothing downstream. The squash title, not the branch name, is what the
history reads, which is why that title is the line that must follow the commit convention.

## The trunk is always releasable

Any trunk commit can be released at any moment, because unfinished work lands dark:

- A feature flag keeps incomplete code out of every execution path.
- A change too large to flag proceeds by branch by abstraction — the new implementation grows behind
  an interface the old one already satisfies.

A habit that needs a stabilization branch to make the trunk trustworthy is treating the symptom. The
check that gates every merge is what makes the trunk trustworthy.

## The two release styles

### Release from trunk

The default. Every release ships the trunk's tip, a fix reaches users by rolling forward, and
exactly one version is alive in the world.

A bug reported against the released version is reproduced at the trunk's tip, not at the tag. Once
the bug lives at the tip, no thinking about versions is needed: fix it on a short-lived branch,
squash-merge, release.

```text
master:  A──B──C──D──E──F──G──H──I──R
               │                    │
            v1.0.0               v1.1.0
```

Shipping the fix ships E through H with it. That is legal under three standing conditions: every
trunk commit was already releasable, no code freeze existed, and the version follows the trunk — the
release is v1.1.0 rather than v1.0.1, because it contains features and not only the fix.

A truly patch-only release is unreachable in this style. That is what the second one exists for.

### Branch for release

For older lines only, when users cannot simply be moved forward — they run pinned versions on their
own infrastructure, a support contract covers an old line, or a sign-off gate stands before a ship.

A `release/<major>.<minor>` branch is cut just in time from a chosen trunk commit. Chosen need not
mean latest: with unflagged half-finished work at the tip, the branch is cut from an earlier commit,
and it can be created retroactively from any commit or tag.

The line flows one direction. A fix lands on the trunk first, then that one commit crosses by
cherry-pick, and nothing merges back:

```text
master:       ──H──J──K──L──M
                 │           └── cherry-pick
                 ↓                  ↓
release/1.1:     ●──────────────────●
              v1.1.0             v1.1.1
```

J, K, and L did not travel — a cherry-pick is not a merge. CI now runs twice, once on the trunk
guarding the fix and once on the branch guarding the cherry-pick. A duplicated pipeline per active
line is the real cost of this style, and the reason not to use it without the precondition.

The branch is deleted once its tags pin its commits; a tag outlives its branch, which is what makes
the deletion safe and the line recoverable.

### Choosing

| Question                                       | If yes             |
| ---------------------------------------------- | ------------------ |
| Can every user be on the same version at once? | Release from trunk |
| Do you ship several times a week or more?      | Release from trunk |
| Do customers self-host or pin versions?        | Branch for release |
| Do you owe someone a patch-only release?       | Branch for release |
| Does a sign-off gate stand before a ship?      | Branch for release |

Default to the trunk. Cut the first release branch the day someone actually needs a backport —
retroactively, from the tag — never ahead of the need.

## The four ways the release branch breaks

- Fixing on the release branch, then merging down to the trunk. One merge is forgotten and the bug
  regresses at the next branch cut. The one exception is a bug that truly cannot reproduce on the
  trunk, fixed on the branch and merged down with the regression risk accepted knowingly.
- Merging the trunk into the release branch instead of cherry-picking. Pulling everything since the
  cut means the branch was cut on the wrong day.
- Keeping one eternal release branch across versions. Each line gets a fresh branch from the trunk.
- Merging one release branch into another. Never; each line takes its cherry-picks from the trunk
  independently.

## Related

- [tools/git/rebase-workflow.md](../tools/git/rebase-workflow.md) — the rebase mechanics the loop
  depends on
- [tools/git/feature-lifecycle.md](../tools/git/feature-lifecycle.md) — the branch lifecycle from
  issue to cleanup
- [tools/jj/](../tools/jj/) — the same model run with jj

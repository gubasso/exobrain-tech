# Reviewing a pull request branch

Someone else's branch, brought into the directory you are already in. Both routes below open with
the same fetch and split on one thing: whether the branch stays theirs.

Tracking is the hinge. A fetched bookmark arrives untracked, and an untracked remote bookmark is
immutable by default — you can stand on `feat-x@origin` and run its code, but nothing you do
rewrites it. `jj bookmark track` gives it a local twin that follows later fetches, and pushing to a
branch that already exists on the remote requires exactly that. So the same command that turns a
reader into a contributor is also the one that starts costing you upkeep.

| You intend to                                           | Use                                                          | What it costs                                                           |
| ------------------------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------------------------------- |
| read and run the code, then comment on the pull request | [read-the-branch.md](./read-the-branch.md)                   | nothing is created and nothing is left behind                           |
| push your own commits onto the branch                   | [contribute-to-the-branch.md](./contribute-to-the-branch.md) | a tracked bookmark to keep in sync, restack, and forget when it is over |

## Reference

- [../one-directory/recipe.md](../one-directory/recipe.md) — a change of your own developed in this
  same directory, once the review turns into work with a branch of its own
- [../one-directory/switching-changes-in-place.md](../one-directory/switching-changes-in-place.md) —
  parking whatever the directory held before the review and coming back to it
- [../README.md](../README.md) — the rest of the jj notes, and the naming convention these use

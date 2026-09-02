# Working on a change in the directory you are in

One repo, one working directory, no branch checked out. Every described change jj holds is a head
of its own, so concurrent work needs no second directory and no bookmark until a change is pushed.

| Document                                                           | What it gets you                                                                                                 |
| ------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| [./recipe.md](./recipe.md)                                         | The whole run: start on `master@origin`, describe, restack, name, push, land, move onto the landed work          |
| [./switching-changes-in-place.md](./switching-changes-in-place.md) | `jj new` plus `jj squash` against `jj edit` for resuming a parked change, and undoing a rewrite you did not mean |

## Reference

- [../parallel-workspace/](../parallel-workspace/) — the same work given its own directory
- [../what-names-a-change.md](../what-names-a-change.md) — the names a change carries along the way
- [../README.md](../README.md) — the rest of the jj notes, and the naming convention these use

# 04 — Gating the Plan

What proves the record is coherent, and what useful views derive from it? A schema owns local shape,
one linter owns cross-file facts, and views never become a second database.

## Validation ownership

[`plan-lane.schema.json`](./plan/plan-lane.schema.json) validates field names, types, ids, slugs,
the three-point enum, and closed-lane conditionals. [`check-plan`](./plan/check-plan) validates
facts that require more than one file: unique ids, story-file agreement, graph integrity, lane
gates, question edges, `Amends` paths, example fences, and legal ranking.

Both artifacts accept a canonical YAML subset: flat mappings, one entry nesting level, and flow
sequences for lists. This keeps the linter dependency-free and the record readable without a YAML
processor. The shipped [fixture and adoption guide](./plan/README.md) prove the split.

The default linter mode is read-only. `--rank-fix` moves original entry blocks to the smallest legal
order without re-serializing or moving work between lanes. It changes nothing on legal input or a
cyclic graph. `--rank-slots` reports the ids permitted at each position without writing. Hooks never
invoke the writer.

## Derived verbs

Projects may expose five commands over the record:

| Verb            | Answer                                              |
| --------------- | --------------------------------------------------- |
| `plan validate` | Does the schema and cross-file record pass?         |
| `plan next`     | Which story is first in `todo.yml`?                 |
| `plan board`    | What occupies each lane, with derived badges?       |
| `plan graph`    | What dependency and question edges constrain work?  |
| `plan flow`     | What does repository history say about review flow? |

Generated boards and graphs are never committed. A UI is a controller over the lane files, not a
second plan store, and every write validates before it lands.

## Flow measures

`plan flow` reads repository history and reports three diagnostics:

- story age, from the first entry into `doing.yml`;
- review dwell, from entry and exit pairs on `review.yml`;
- rework rounds, from entries into `doing.yml` beyond the first.

These measures are read-only, local, and never gated. A shallow clone may report them as unknown.
Velocity remains the portable committed measure because its points and close date live in the
record.

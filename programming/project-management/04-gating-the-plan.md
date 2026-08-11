# 04 — Gating the Plan

What proves the record is coherent, and what useful views derive from it? A schema owns local shape,
one linter owns cross-file facts, and views never become a second database.

## Validation ownership

[`plan-lane.schema.json`](./plan/plan-lane.schema.json) validates field names, types, ids, slugs,
the three-point enum, and closed-lane conditionals.
[`plan-config.schema.json`](./plan/plan-config.schema.json) does the same for the optional
`config.yml`, which carries what a script needs and cannot derive from the record.
[`check-plan`](./plan/check-plan) validates facts that require more than one file: unique ids,
story-file agreement, epic-document existence, graph integrity, lane gates, question edges, `Amends`
paths in stories and epics alike, example fences, and legal ranking.

Both artifacts accept a canonical YAML subset: flat mappings, one entry nesting level, and flow
sequences for lists. This keeps the linter dependency-free and the record readable without a YAML
processor. The shipped [fixture and adoption guide](./plan/README.md) prove the split.

The default linter mode is read-only. `--rank-fix` moves original entry blocks to the smallest legal
order in `backlog.yml` and `todo.yml`, and to close-date order in `closed.yml`, without
re-serializing or moving work between lanes. It changes nothing on legal input or a cyclic graph.
`--rank-slots` reports the ids permitted at each position without writing. Hooks never invoke the
writer.

## Three classes of diagnostic

A failure names something the record cannot express correctly and no tool can decide: a dangling id,
a missing story file, a story in `doing` whose dependency is open. It exits non-zero.

A ranking failure names an order that contradicts the dependency graph. It also exits non-zero,
because a record whose top contradicts its own edges misleads whoever reads it first.

A warning names something the writer can repair on request, and today that is one rule: a close date
out of sequence in `closed.yml`. It reaches the reader and never reaches an exit code. Gating on a
fact the tool can fix buys friction and no falsifiability, so the log stays a warning and
`--rank-fix` restores it.

## Derived verbs

Projects may expose five commands over the record:

| Verb            | Answer                                              |
| --------------- | --------------------------------------------------- |
| `plan validate` | Does the schema and cross-file record pass?         |
| `plan next`     | Which story is first in `todo.yml`?                 |
| `plan board`    | What occupies each lane, with derived badges?       |
| `plan graph`    | What dependency and question edges constrain work?  |
| `plan velocity` | How many points closed per iteration?               |
| `plan flow`     | What does repository history say about review flow? |

`plan velocity` is the one that needs a parameter the record does not hold, and it reads
`config.yml` for the iteration anchor and length. A board that groups by epic reads nothing extra:
it searches the lane files for an id, which is the only direction membership runs.

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

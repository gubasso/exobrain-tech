# An Epic Is a Field and a Document

## Context and Problem Statement

Points cap a story at three, so any substantial feature must split. Each piece then carries its own
goal and none carries the goal they share. The only workaround was `tags`, which nothing gates: a
misspelled label silently creates a second grouping, and a label carries no goal, worked example, or
guarantee.

## Considered Options

- One optional entry field naming a document — chosen.
- A sixth record type with its own lane entries, points, and ranking — rejected: it reintroduces the
  parent/child hierarchy the flat lane files exist to avoid, and needs ranking rules of its own.
- A `tags` convention enforced by review — rejected: gating it would turn every free label into a
  gated reference.

## Decision Outcome

Chosen option: an optional `epic: "<id>"` on the lane entry, naming one document at
`docs/plan/epics/<id>-<slug>.md`.

Epics draw from the story id sequence, so one id names one thing anywhere in the zone and renaming a
slug never touches the entries that reference it. `check-plan` rejects an epic id already used by a
story, and two documents claiming one id.

Four properties are load-bearing:

- One level. An epic has no lane entry, so no field exists in which a parent could be written.
- Non-sequencing. `needs` keeps sole ownership of order, so `--rank-fix` stays unaware of epics.
- Derived completion. No status field; an epic closes when every member is in `closed.yml`, and its
  own `Done when` states what that was meant to achieve.
- One-way membership. The document never lists its members. An epic with no members is legal:
  writing the end state before decomposing it is the normal order.

## Consequences

- Good: a multi-story goal gets a gated home, and closing a story writes nothing extra.
- Bad: epics need their own `MD043` array and hook entry, and an epic's members are found only by
  searching the lane files, the price of refusing a back-index.

## Status

Superseded by [ADR-the-planning-method-moves-to-plan-xp](./ADR-the-planning-method-moves-to-plan-xp.md) — the method and its artifacts now live at [plan-xp](https://github.com/gubasso/plan-xp).

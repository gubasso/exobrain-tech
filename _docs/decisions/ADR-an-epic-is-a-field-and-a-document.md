# An Epic Is a Field and a Document

## Context and Problem Statement

Points cap a story at three, so any substantial feature must split. Each piece then carries its own
goal and none carries the goal they share. The only workaround was `tags`, which nothing gates: a
misspelled label silently creates a second grouping, and a label cannot carry a goal, a worked
example, or a guarantee.

## Considered Options

- A sixth record type with its own lane entries, points, and ranking.
- A `tags` convention, enforced by review.
- One optional entry field naming a document.

## Decision Outcome

Chosen option: **an optional `epic: "<id>"` on the lane entry, naming one document at
`docs/plan/epics/<id>-<slug>.md`**. A sixth record type would reintroduce the parent/child hierarchy
the flat lane files exist to avoid, and would need ranking rules of its own. Gating `tags` would
turn every free label into a gated reference.

Epics are named like stories and draw from the same id sequence, so one id names one thing anywhere
in the zone and renaming a slug never touches the entries that reference it. `check-plan` rejects an
epic id that is already a story entry, and two documents claiming one id.

Four properties are load-bearing:

- One level. An epic has no lane entry, so no field exists in which a parent could be written.
- Non-sequencing. `needs` keeps sole ownership of order, so `--rank-fix` stays unaware of epics.
- Derived completion. No status field; an epic is closed out when every member is in `closed.yml`,
  and its own `Done when` states what that was supposed to achieve.
- One-way membership. The document never lists its members, per ADR-filesystem-owns-disk-state. An epic with no members
  is legal — writing the end state before decomposing it is the normal order.

`check-plan` verifies the document exists and holds its `Amends` paths to the check a story's get.

## Consequences

- Good: a multi-story goal gets a gated home with a worked example, and closing a story writes
  nothing extra.
- Bad: the record grows a second gated reference; epics need their own `MD043` array and hook entry;
  and an epic's members are found only by searching the lane files, the price of refusing a
  back-index.

## Status

Superseded by [ADR-the-planning-method-moves-to-plan-xp](./ADR-the-planning-method-moves-to-plan-xp.md) — the method and its artifacts now live at [plan-xp](https://github.com/gubasso/plan-xp).

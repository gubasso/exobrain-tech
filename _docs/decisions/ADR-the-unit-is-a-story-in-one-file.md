# The Unit Is a Story in One File

## Context and Problem Statement

The work unit used a project-specific name, a directory for every unit, and a calendar-oriented
budget. Agent implementation makes human review judgment the scarce resource, and the directory
shape adds ceremony before artifacts exist.

## Considered Options

- Keep the existing work unit, directory, and time budget.
- Adopt XP stories, one story file, and points on the lane entry.
- Store stories only in an external tracker.

## Decision Outcome

Chosen option: XP stories in one file — each work unit is
`docs/plan/stories/<id>-<slug>.md`, with an optional same-name sibling directory only for traces,
fixtures, diagram sources, or other non-narrative artifacts.

Each lane entry owns `type` and `points`. Types are `story`, `spike`, or `chore`; points are
`1 | 2 | 3`, count irreducible human judgments, and force work needing four or more judgments to
split. Velocity is derived from points on completed entries and is a cap on reviewer capacity, not
an estimate of agent time.

## Consequences

- Good: a story maps directly to common issue trackers and never moves when artifacts appear.
- Good: the record has one estimate owner and no cached value in the story.
- Bad: optional artifact directories require discipline to keep narrative material in the story or
  a durable documentation zone.

## Status

Superseded by [ADR-the-planning-method-moves-to-plan-xp](./ADR-the-planning-method-moves-to-plan-xp.md) — the method and its artifacts now live at [plan-xp](https://github.com/gubasso/plan-xp).

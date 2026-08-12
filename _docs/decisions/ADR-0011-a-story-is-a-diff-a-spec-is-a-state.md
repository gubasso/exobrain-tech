# ADR-0011: A Story Is a Diff, a Spec Is a State

## Context and Problem Statement

Combining thin XP stories with spec-driven development can create two live documents that describe
the same behavior and drift. The plan needs intent and acceptance before implementation while
readers need one durable statement of current behavior afterward.

## Considered Options

- Keep all behavioral detail in stories.
- Create one specification per story.
- Treat stories as changes and capability specs as current state.

## Decision Outcome

Chosen option: stories are diffs and specs are state — a story says what will change, while a
reference-zone capability spec says what is true.

Every story carries `## Amends`, naming the specs it must leave changed or `None`. When behavior
lands, acceptance assertions are rewritten into present tense in those specs in the same commit.
Closed stories remain historical records and are never updated to mirror later spec changes.

## Consequences

- Good: live behavior has one owner while the bargain made for each change remains inspectable.
- Good: spec-driven development adds no new documentation zone or per-story duplicate.
- Bad: a human must verify that acceptance reached the amended spec; only path existence is
  mechanically checked.

## Status

Superseded by [ADR-0017](./ADR-0017-the-planning-method-moves-to-plan-xp.md) — the method and its artifacts now live at [plan-xp](https://github.com/gubasso/plan-xp).

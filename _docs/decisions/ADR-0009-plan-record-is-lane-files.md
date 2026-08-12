# ADR-0009: Plan Record Is Lane Files

## Context and Problem Statement

A single Markdown status document mixes presentation, parsing, and coordination state. The record
needs one unambiguous owner for lane membership, ranking, dependencies, and terminal outcomes.

## Considered Options

- Keep one Markdown status document with a line grammar.
- Store one YAML file per lane and derive views from it.
- Distribute coordination fields into story documents.

## Decision Outcome

Chosen option: one YAML file per lane — the filename owns lane membership, sequence position owns
human ranking, and `needs` owns dependency edges without duplicating status or priority fields.

The five lanes are `backlog`, `todo`, `doing`, `review`, and `closed`. A blocked state is derived
from open dependencies or open questions rather than stored. Ranking in `backlog` and `todo` must
respect dependencies, and `todo` keeps eligible entries ahead of ineligible entries.

## Consequences

- Good: schema validation owns each file's shape while one linter owns cross-file coherence.
- Good: views and workflow state derive from a compact machine-readable record.
- Bad: moving an entry between lanes changes two files, and close operations may also re-rank
  `todo.yml`.

## Status

Superseded by [ADR-0017](./ADR-0017-the-planning-method-moves-to-plan-xp.md) — the method and its artifacts now live at [plan-xp](https://github.com/gubasso/plan-xp).

Amended by [ADR-0014](./ADR-0014-an-epic-is-a-field-and-a-document.md) — an entry may carry one
gated reference to an epic document.

Amended by [ADR-0015](./ADR-0015-closed-is-an-append-only-log.md) — `closed.yml` is ordered by
close date rather than unordered.

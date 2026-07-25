# ADR-0005: Add Deprecated status and the Amended-by convention to the ADR canon

## Context and Problem Statement

The ADR canon (`programming/docs-design/02-lean-adrs.md`) allowed only
`Proposed | Accepted | Implemented | Superseded | Rejected` and explicitly
forbade `Deprecated` as a synonym. Practice in downstream projects surfaced two
gaps: a decision can stop applying with **no successor** (the context
evaporated), and a later ADR can change **part** of a decision that otherwise
stands (a rename, a closed deferred item). Marking such records `Superseded` is
false; leaving them untouched misleads readers and agents into following stale
details.

## Considered Options

- Keep the closed five-value vocabulary; force `Superseded` for every retirement.
- Allow free-form status annotations per project.
- Add `Deprecated` as a sixth status and codify an `Amended by` annotation.

## Decision Outcome

Chosen option: **add `Deprecated` and codify `Amended by`** — the vocabulary
stays closed and grep-able while the two real lifecycle cases become
expressible.

- `Deprecated`: the decision no longer applies and no successor exists. Say why.
- `Amended by ADR-NNNN — <what changed>`: an annotation under `## Status`, not a
  status value. The record keeps its status; the old body is edited only where
  its wording would actively mislead, never to rewrite history.
- Never-delete is restated as "never delete — and never mislead": superseded,
  deprecated, rejected, or amended, an old ADR must not read as current when it
  is not.
- `Done`, `Canceled`, and `Obsolete` remain forbidden synonyms.

## Consequences

- Good: retirements without successors and partial changes are now honest and
  machine-classifiable; old ADRs stop being traps for readers and agents.
- Bad: one more status value to teach, and existing downstream templates carry
  the five-value list until they re-sync with the canon.

## Status

Accepted

Enacted across `programming/docs-design/02-lean-adrs.md`, `template-adr.md`,
`_docs/decisions/template.md`, `_docs/reference/docs-conventions.md`,
`AGENTS.md`, and `CLAUDE.md`.

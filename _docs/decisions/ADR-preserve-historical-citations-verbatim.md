# Preserve historical citations verbatim

## Context and Problem Statement

Decision records are immutable accounts of what was chosen. One accepted record contains a markdown
link whose target is absent:
`_docs/decisions/ADR-cite-the-claim-not-the-coordinates.md:32`,
`[00 — Model](../../programming/spec-driven-docs/00-model.md)`.

## Considered Options

- `exclude records from relative-link liveness` — chosen.
- `rewrite the citation` — rejected: it would revise an immutable record after its decision.
- `disable all record gates` — rejected: filename, heading, status, and size remain current contracts.

## Decision Outcome

Chosen option: `exclude records from relative-link liveness` — `md-relative-links` excludes
`_docs/decisions/`, while `md-adr`, `adr-filename-shape`, and `adr-word-cap` continue to bind records.

Enforced by `decision-records:a-merged-record-is-permanent`.

## Consequences

- Good: historical text remains byte-stable.
- Bad: a record's relative link may not resolve in the current tree.

## Status

Accepted

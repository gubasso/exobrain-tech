# Drop the Amended-by annotation from decision records

## Context and Problem Statement

`ADR-adr-status-vocabulary-and-amendments` codified an `Amended by ADR-NNNN` line so a record changed
only in part would not mislead a reader. The framework this repository later adopted states the
opposite in as many words: there is no amendment annotation, deliberately. Both rules were live, in
`AGENTS.md` and in two reference pages, and the annotation's own syntax cited a counter that no
longer exists.

## Considered Options

- Drop the annotation and let the spec carry the present — chosen.
- Amend the framework to allow the annotation — rejected: it would import a maintenance burden the
  framework removed on purpose, and every record would need re-reading whenever a later one landed.
- Leave both and let authors pick — rejected: two live rules for one artifact is the failure the
  single-owner rule exists to prevent.

## Decision Outcome

Chosen option: drop it. An amendment annotation is only needed where records are asked to describe
the present. They are not asked to here — a reader who wants what binds today reads the spec — so a
record left alone for two years is not stale, it is just old, which is what a record is.

Annotations already written stay: those records are frozen, and editing them to remove a line would
be the revision the same rule forbids.

## Consequences

- Good: a record is written once and never revisited; `Deprecated` and `Superseded` carry every
  retirement case that remains.
- Bad: a decision changed only in part shows nothing at the record, so the reader must reach the spec
  to learn which half still binds.

## Status

Implemented. Supersedes [ADR-adr-status-vocabulary-and-amendments](./ADR-adr-status-vocabulary-and-amendments.md).

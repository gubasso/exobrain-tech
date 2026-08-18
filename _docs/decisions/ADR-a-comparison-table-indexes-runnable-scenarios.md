# A comparison table indexes runnable scenarios

## Context and Problem Statement

Projects in this KB's orbit publish pages comparing themselves to alternatives, and the genre has no
owner here. Left unspecified it produces the two shapes that fail: a matrix assembled by reading
competitors' documentation, which cannot be re-verified and is wrong within a release, and a matrix of
bare emoji, which is unreadable to a screen reader and carries no room for the qualified answer that
most rows actually have.

## Considered Options

- A chapter inside `programming/spec-driven-docs/`
- A new shelf, `programming/comparison-docs/`, with the matrix as the primary artifact
- A new shelf where the scenario is the primary artifact and the matrix is a view over it

## Decision Outcome

Chosen option: a new shelf where the scenario is primary — a verdict states what happened when the
author ran a named method against a named version on a named date, and the table is an index into
those runs. The spec-driven-docs shelf states its scope as the documentation framework itself, so one
document genre does not belong inside it, and a shelf built around the table would specify formatting
for claims nobody can check.

Two rules follow and are not separately recorded. A cell references a heading anchor rather than a
footnote, because a relative-link linter resolves an anchor against real headings on disk and nothing
validates a footnote target. A capability the author has not run carries an explicit untested verdict,
because a blank cell and a stale positive are the two ways a reader is misled without noticing.

## Consequences

- Good: a reader can check any verdict, and a refresh a year later is a re-run rather than a rewrite.
- Good: five of the shelf's rules have working commands, and the rest are listed as unenforced.
- Bad: a comparison page costs materially more to write, because every cell implies a run.
- Bad: honest documents will carry visible untested cells, which reads as incompleteness.

## Status

Accepted

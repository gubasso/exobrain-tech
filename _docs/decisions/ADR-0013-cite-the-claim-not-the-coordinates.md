# ADR-0013: Cite the Claim, Not the Coordinates

## Context and Problem Statement

A document reference that names only a file, topic, or line range makes comprehension depend on
opening another file. Line coordinates also decay silently when earlier text moves.

## Considered Options

- Permit coordinate citations as concise pointers.
- Require every reference to state its claim and link to a stable anchor.
- Copy the complete source passage into each consumer.

## Decision Outcome

Chosen option: state the claim and link to an anchor — the carrying sentence tells the reader what
the source establishes, and the link exists for verification rather than comprehension.

Restating a conclusion is compatible with ARID when deleting the summary leaves the canonical fact
intact and discoverable. Evidence-oriented documents may cite coordinates only after their prose
already carries the claim; story files have no such exception.

## Consequences

- Good: references survive unrelated edits and documents remain comprehensible in focused context.
- Good: summaries stay disposable and do not become competing sources of truth.
- Bad: most of the rule requires review because a regex cannot determine whether prose carries the
  referenced claim.

## Status

Implemented by [00 — Foundations](../../programming/docs-design/00-foundations.md) and
[08 — Lean Markdown](../../programming/docs-design/08-lean-markdown.md).

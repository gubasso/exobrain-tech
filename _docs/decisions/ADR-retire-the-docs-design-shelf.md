# Delete programming/docs-design

## Context and Problem Statement

`ADR-spec-driven-docs-is-the-documentation-method` made `programming/spec-driven-docs/` the method and
made the deletion of the older shelf conditional on every subject having an owner in the new one. A
section-by-section comparison closed the last gaps, and the retired shelf had since become actively
wrong: it taught `ADR-<number>-<decision>.md` filenames and ADR references from code comments, both
of which the adopted method forbids.

## Considered Options

- Delete the shelf — chosen.
- Keep it and rewrite its citations to the new form — rejected: it would spend a rewrite on content
  whose rules contradict the method, leaving two shelves answering the same question differently.
- Keep it marked as historical — rejected: no document narrates its own history, and a shelf kept for
  its past is the archive that rule refuses.

## Decision Outcome

Chosen option: delete it. The condition the earlier record set is met, so the shelf is content with
no reader need left to serve.

Six merged records name paths inside it. They are frozen and stay as written: a record states what
was true when it was written, and a dangling path in one is not a defect.

## Consequences

- Good: one shelf answers documentation questions, and the contradictory naming and comment rules
  leave the tree with it.
- Bad: six merged records name paths that no longer exist, and anyone holding a bookmark or an open
  editor tab loses the chapter to the log.

## Status

Implemented

# Known Issues Specification

## Purpose

What a record of an external-system bug in `_docs/reference/known-issues/` is: how its case id is
formed, and what the record owes so the workaround it justifies can be removed. It binds the record
and the suppressions that cite it. A defect in this repository's own content is not a case; it is a
fix.

## Requirements

### `known-issues:case-id-is-a-slug` — A case id is a prefixed slug with no counter

A known-issue filename MUST be `KI-<slug>.md` and MUST NOT carry a counter.

#### Scenario: A case is filed against an upstream formatter

- GIVEN a reproducible bug in a pinned tool
- WHEN the record is added as `KI-0007.md`
- THEN the commit fails, because the id a suppression cites has to survive a merge

Verify: `pre-commit run ki-filename-shape --all-files`

### `known-issues:a-suppression-names-its-case` — A suppression names the case behind it

Where a document suppresses a tool, the suppression MUST name a case id that resolves to a record.

#### Scenario: A formatter corrupts one block and the block is fenced off

- GIVEN a range that stops a formatter from rewriting a passage
- WHEN it carries no case id
- THEN the next reader takes it for a design choice, and nothing says what would remove it

Verify: `pre-commit run suppression-names-its-case --all-files`

### `known-issues:a-record-carries-its-retirement-condition` — A record says what retires it

A known-issue record MUST carry the condition under which it is removed.

#### Scenario: A workaround outlives the bug it works around

- GIVEN a record whose workaround is applied in several documents
- WHEN the upstream tool is fixed
- THEN the stated condition is what tells a reader the guard can go, so the record cannot be
  permanent by default

Verify: `pre-commit run ki-retire-when --all-files`

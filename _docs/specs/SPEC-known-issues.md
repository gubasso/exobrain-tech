# Known Issues Specification

## Purpose

What a record of an external-system bug in `_docs/reference/known-issues/` is: how its case id is
formed, what it owes a reader who arrives from a suppression, what it owes a tracker it is filed
into, and what the record owes so the workaround it justifies can be removed. It binds the record
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

### `known-issues:a-record-walks-the-mechanism` — A record walks its mechanism step by step

A known-issue record MUST carry a `## How it works` walkthrough that runs one concrete case and
shows the state each step leaves behind.

#### Scenario: A reader arrives from a suppression that cites the case

- GIVEN a record stating only that the tool corrupts the block
- WHEN the reader who follows the case id has never seen the bug
- THEN the record answers nothing they can check, and the mechanism is re-derived from scratch

Verify: `pre-commit run ki-mechanism-walkthrough --all-files`

### `known-issues:a-filed-record-carries-its-report` — A filed record carries the body it was filed with

A record whose `upstream:` names one filed issue MUST carry a `## Report` section holding that
issue's body in the tracker's own markup.

#### Scenario: The bug is filed under the deadline that made it worth filing

- GIVEN a record written for this repository and a tracker expecting its own markup
- WHEN the reporter has nothing to paste
- THEN the report is written a second time, and the repository no longer holds what was said upstream

Verify: `pre-commit run ki-report-body --all-files`

### `known-issues:a-report-body-fits-in-79-columns` — A report body fits in 79 columns

A `## Report` section MUST hold its body in a fenced block whose lines are at most 79 columns.

#### Scenario: The report carries an aligned table and an annotated excerpt

- GIVEN a tracker that renders a comment in a fixed-width box and reflows nothing
- WHEN a line runs past that width
- THEN the tracker wraps it where it chooses, and the alignment that carried the argument is gone

Verify: `pre-commit run ki-report-width --all-files`

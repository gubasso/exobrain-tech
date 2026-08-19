# Decision Records Specification

## Purpose

What a decision record in `_docs/decisions/` is: how it is named, how long it may be, and what may
happen to it after it merges. It binds the record as an artifact. The shape a document holds in
general belongs to `SPEC-docs-format.md`, and the argument for any one decision belongs to the record
itself.

## Requirements

### `decision-records:a-merged-record-is-permanent` — A merged record is never deleted or retconned

A merged record MUST NOT be deleted, renamed, or edited so a later design appears to have been the
original choice.

#### Scenario: A record turns out to be wrong about the tree it described

- GIVEN a merged record asserting a consequence that was false when it was written
- WHEN the false statement is replaced with the true one
- THEN the correction is permitted, because it changes the fact and not the argument

Verify: `git log --diff-filter=DR --format='%h %s' -- '_docs/decisions/*'`, and a reviewer confirms
each entry is a sanctioned migration

### `decision-records:filename-is-a-slug` — A record filename is a slug with no counter

A record filename MUST be `ADR-<slug>.md` and MUST NOT carry a digit.

#### Scenario: Two branches each add the next record

- GIVEN two branches, each adding a record to the log
- WHEN both allocate the next number and merge
- THEN a counter would leave two records claiming one identity, so the slug is the identifier

Verify: `pre-commit run adr-filename-shape --all-files`

### `decision-records:record-fits-in-350-words` — A record fits in 350 words

A filled record MUST be at or below 350 words.

#### Scenario: A decision will not fit the cap

- GIVEN a design argument of 500 words
- WHEN it is written as one record
- THEN the commit fails, and the response is two records or a spec, never a raised cap

Verify: `pre-commit run adr-word-cap --all-files`

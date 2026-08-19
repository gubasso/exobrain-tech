# Specifications Specification

## Purpose

What a specification in `_docs/specs/` is as an artifact: the parts a requirement carries, how its
identifier resolves, and how long the document may be. It binds the shelf that states every other
rule, so a gap here is a gap in all of them. What a spec is for and where it belongs is the placement
question; the shape the prose holds belongs to `SPEC-docs-format.md`.

## Requirements

### `docs-specs:a-requirement-carries-its-parts` — A requirement carries an id and a verification

Every `###` heading in a spec MUST be a rule ID, and MUST be followed by a `Verify:` line.

#### Scenario: A requirement is added without saying how it is checked

- GIVEN a spec gaining a rule stated as a MUST
- WHEN the requirement carries no `Verify:` line
- THEN the commit fails, because a rule presented as binding and checked by nothing teaches a reader
  that this shelf describes intentions

Verify: `pre-commit run spec-requirement-parts --all-files`

### `docs-specs:a-rule-id-is-unique` — A rule id resolves to one requirement

A rule ID MUST appear as the heading of exactly one requirement across the shelf.

#### Scenario: Two specs claim the same identifier

- GIVEN a rule moved from one domain to another and restated rather than relocated
- WHEN both specs carry `docs-format:fence-declares-a-language`
- THEN a commit citing that id names two rules at once, so the citation stops being an address

Verify: `pre-commit run spec-rule-id-unique --all-files`

### `docs-specs:a-spec-fits-in-300-lines` — A spec fits in 300 authored lines

A spec MUST be at or below 300 lines excluding its table of contents, and MUST carry a generated
table of contents above 100.

#### Scenario: A domain outgrows the file that states it

- GIVEN a spec at 290 authored lines
- WHEN a requirement is added that takes it past the cap
- THEN the commit fails, and the domain splits into two specs rather than the cap rising

Verify: `pre-commit run spec-size-cap --all-files`

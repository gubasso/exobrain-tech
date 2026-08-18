# Documentation Format Specification

## Purpose

Rules governing the markdown every document in this project is written in, and the size budgets that
keep documents inside the range where retrieval stays reliable.

## Requirements

### `docs-format:chapter-stays-within-200-lines` — A chapter stays within 200 lines

The author MUST keep a chapter at or below 200 lines.

#### Scenario: A chapter acquires a second subject

- GIVEN a chapter approaching the cap
- WHEN more rules arrive
- THEN the excess becomes a requirement in a spec or a decision record, not a longer chapter

Verify: `find _docs -name '*.md' -exec sh -c 'test $(wc -l < "$1") -le 200 || echo "$1"' _ {} \; | grep . && exit 1 || exit 0`

### `docs-format:document-uses-structural-markdown-only` — A document uses structural markdown only

The author MUST NOT use bold or italic text.

#### Scenario: An agent drafts a rule list

- GIVEN a generated document with a bold lead-in on every item
- WHEN a reader scans it
- THEN nothing is emphasized because everything is, and the identifiers belong in inline code

Verify: `rg -n '\*\*[^*]+\*\*' _docs && exit 1 || exit 0`

### `docs-format:document-states-the-present` — A document states the present

The author MUST NOT record what a document used to say, what it replaces, or why something is absent.

#### Scenario: A rule is removed

- GIVEN a rule the project drops
- WHEN the author removes it
- THEN the deletion leaves no trace in prose, only in the log

Verify: `rg -ni 'formerly|used to be|this replaces|previously we' _docs | rg -v ':Verify: ' | grep . && exit 1 || exit 0`

### `docs-format:fence-declares-a-language` — Every fence declares a language

The author MUST give every fenced code block a language, using `text` when none applies.

#### Scenario: A diagram is added in a bare fence

- GIVEN an ASCII diagram in a fenced block
- WHEN no language is declared
- THEN the gate rejects it and `text` is the correct declaration

Verify: ``find _docs -name '*.md' -exec awk '/^```/{if(!inf){inf=1;if($0=="```"){print FILENAME":"FNR;bad=1}}else inf=0;next} END{exit bad}' {} +``

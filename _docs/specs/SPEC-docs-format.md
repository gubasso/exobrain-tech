# Documentation Format Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`docs-format:chapter-stays-within-200-lines` — A chapter stays within 200 lines](#docs-formatchapter-stays-within-200-lines--a-chapter-stays-within-200-lines)
    - [Scenario: A chapter acquires a second subject](#scenario-a-chapter-acquires-a-second-subject)
  - [`docs-format:author-instructions-stay-within-budget` — Author instructions stay within budget](#docs-formatauthor-instructions-stay-within-budget--author-instructions-stay-within-budget)
    - [Scenario: A root file accumulates a subtree's rules](#scenario-a-root-file-accumulates-a-subtrees-rules)
  - [`docs-format:every-budget-carries-a-gate` — Every count-shaped budget carries a gate](#docs-formatevery-budget-carries-a-gate--every-count-shaped-budget-carries-a-gate)
    - [Scenario: A document arrives over its budget](#scenario-a-document-arrives-over-its-budget)
  - [`docs-format:document-uses-structural-markdown-only` — A document uses structural markdown only](#docs-formatdocument-uses-structural-markdown-only--a-document-uses-structural-markdown-only)
    - [Scenario: An agent drafts a rule list](#scenario-an-agent-drafts-a-rule-list)
  - [`docs-format:document-states-the-present` — A document states the present](#docs-formatdocument-states-the-present--a-document-states-the-present)
    - [Scenario: A rule is removed](#scenario-a-rule-is-removed)
  - [`docs-format:fence-declares-a-language` — Every fence declares a language](#docs-formatfence-declares-a-language--every-fence-declares-a-language)
    - [Scenario: A diagram is added in a bare fence](#scenario-a-diagram-is-added-in-a-bare-fence)
  - [`docs-format:no-document-indexes-the-filesystem` — A document does not index the filesystem](#docs-formatno-document-indexes-the-filesystem--a-document-does-not-index-the-filesystem)
    - [Scenario: A digest lists the files of the directory it describes](#scenario-a-digest-lists-the-files-of-the-directory-it-describes)
  - [`docs-format:a-relative-link-is-explicit` — A relative link carries an explicit path](#docs-formata-relative-link-is-explicit--a-relative-link-carries-an-explicit-path)
    - [Scenario: A link names only a basename](#scenario-a-link-names-only-a-basename)

<!--TOC-->

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

Verify: `pre-commit run chapter-size-cap --all-files`

### `docs-format:author-instructions-stay-within-budget` — Author instructions stay within budget

The author MUST keep the root author-instructions file at or below 100 lines and a subtree one at or
below 150.

#### Scenario: A root file accumulates a subtree's rules

- GIVEN a rule that binds one subtree only
- WHEN it is written into the root file and pushes it past the cap
- THEN it belongs in that subtree's own author-instructions file, which the root points at

Verify: `pre-commit run agents-digest-size --all-files`

### `docs-format:every-budget-carries-a-gate` — Every count-shaped budget carries a gate

The project MUST enforce every budget stated as a count with a command that fails the change, and
MUST NOT raise a budget to admit a document that exceeds it.

#### Scenario: A document arrives over its budget

- GIVEN a chapter that will not fit in 200 lines
- WHEN an author reaches for the cap rather than the content
- THEN the chapter splits, because the gate that admits it would admit the next one too

Verify: `for h in adr-word-cap agents-digest-size spec-size-cap chapter-size-cap; do grep -q "id: $h$" .pre-commit-config.yaml || exit 1; done`

### `docs-format:document-uses-structural-markdown-only` — A document uses structural markdown only

The author MUST NOT use bold or italic text.

#### Scenario: An agent drafts a rule list

- GIVEN a generated document with a bold lead-in on every item
- WHEN a reader scans it
- THEN nothing is emphasized because everything is, and the identifiers belong in inline code

Verify: reviewer confirms the change introduces no bold or italic text

### `docs-format:document-states-the-present` — A document states the present

The author MUST NOT record what a document used to say, what it replaces, or why something is absent.

#### Scenario: A rule is removed

- GIVEN a rule the project drops
- WHEN the author removes it
- THEN the deletion leaves no trace in prose, only in the log

Verify: `pre-commit run no-self-narration --all-files`

### `docs-format:fence-declares-a-language` — Every fence declares a language

The author MUST give every fenced code block a language, using `text` when none applies.

#### Scenario: A diagram is added in a bare fence

- GIVEN an ASCII diagram in a fenced block
- WHEN no language is declared
- THEN the gate rejects it and `text` is the correct declaration

Verify: `pre-commit run markdownlint-cli2 --all-files`

### `docs-format:no-document-indexes-the-filesystem` — A document does not index the filesystem

A document MUST NOT carry an enumeration whose objective is to mirror what is on disk.

#### Scenario: A digest lists the files of the directory it describes

- GIVEN a bucket whose contents are already visible in its directory listing
- WHEN a digest adds a `source-files:` frontmatter key, or a section headed `Source map`
- THEN the change is rejected, while a list whose entries carry their own payload passes

Verify: `pre-commit run no-source-files-frontmatter -a && pre-commit run no-source-map-section -a`

### `docs-format:a-relative-link-is-explicit` — A relative link carries an explicit path

A relative link MUST open with `./`, `../`, or a multi-segment path.

#### Scenario: A link names only a basename

- GIVEN two files named `runbook.md` in different directories
- WHEN a document links to one of them by basename alone
- THEN the commit fails, because neither a reader nor an LSP can tell which file is meant

Verify: `pre-commit run no-bare-relative-links -a && pre-commit run no-bare-relative-link-defs -a`

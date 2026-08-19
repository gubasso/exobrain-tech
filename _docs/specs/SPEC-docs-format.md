# Documentation Format Specification

## Purpose

The shape every document in this repository holds, whatever bucket it lives in: how long it may be,
what it may enumerate, how it links, and how its fences are marked. It binds the library and `_docs/`
alike. What a document is for and where it belongs is the placement question, not this one; which
artifact records a choice belongs to `SPEC-decision-records.md`.

## Requirements

### `docs-format:document-states-the-present` — A document states what is true now

A document MUST NOT narrate how it came to say what it says.

#### Scenario: A rule replaces an earlier one and the document says so

- GIVEN a chapter rewritten under a rule that changed
- WHEN it keeps a clause opening `formerly`, `used to be`, `this replaces`, or `inherited from`
- THEN the clause goes, unless deleting it would lose something a reader can act on

Verify: `pre-commit run no-self-narration --all-files`

### `docs-format:no-document-indexes-the-filesystem` — A document does not index the filesystem

A document MUST NOT carry an enumeration whose objective is to mirror what is on disk.

#### Scenario: A digest lists the files of the directory it describes

- GIVEN a bucket whose contents are already visible in its directory listing
- WHEN a digest adds a `source-files:` frontmatter key, or a section headed `Source map`
- THEN the change is rejected, while a list whose entries carry their own payload passes

Verify: `pre-commit run no-source-files-frontmatter -a && pre-commit run no-source-map-section -a`

### `docs-format:chapter-fits-in-200-lines` — A chapter fits in 200 lines, a catalog in 300

A chapter MUST be at or below 200 lines, and a catalog at or below 300.

#### Scenario: A chapter grows past the cap while being extended

- GIVEN a chapter at 198 lines
- WHEN a section is added that takes it to 214
- THEN the commit fails, and the chapter splits rather than the cap rising

Verify: `pre-commit run chapter-size-cap --all-files`

### `docs-format:author-instructions-stay-in-budget` — Author instructions stay in budget

A root author-instructions file MUST be at or below 100 lines, and a subtree's at or below 150.

#### Scenario: A rule is added to a root file already at its cap

- GIVEN a root `AGENTS.md` at exactly 100 lines
- WHEN a rule is added
- THEN the commit fails until the prose that the owning spec already states is compressed

Verify: `pre-commit run agents-digest-size --all-files`

### `docs-format:every-verified-rule-names-a-live-hook` — Every verified rule names a live hook

Where a rule's verification runs a hook, a hook MUST exist under the name the rule gives.

#### Scenario: A gate is renamed while the rule that names it stays

- GIVEN `chapter-size-cap` named by the chapter budget's verification
- WHEN the hook is renamed or deleted and the rule is not
- THEN the commit fails, rather than the rule quietly reverting to a suggestion

Verify: `pre-commit run spec-verify-hooks-exist --all-files`

### `docs-format:a-relative-link-is-explicit` — A relative link carries an explicit path

A relative link MUST open with `./`, `../`, or a multi-segment path.

#### Scenario: A link names only a basename

- GIVEN two files named `runbook.md` in different directories
- WHEN a document links to one of them by basename alone
- THEN the commit fails, because neither a reader nor an LSP can tell which file is meant

Verify: `pre-commit run no-bare-relative-links -a && pre-commit run no-bare-relative-link-defs -a`

### `docs-format:fence-declares-a-language` — Every fenced block declares a language

A fenced code block MUST carry a language specifier, `text` where none applies.

#### Scenario: A bare fence holds output with no language

- GIVEN a block of terminal output that is not code in any language
- WHEN it is fenced with no specifier
- THEN the commit fails, and `text` is the specifier that clears it

Verify: `pre-commit run markdownlint-cli2 --all-files`

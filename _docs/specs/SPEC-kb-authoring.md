# Knowledge Base Authoring Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`kb-authoring:a-guide-is-an-ordered-recipe` — A guide is an ordered recipe](#kb-authoringa-guide-is-an-ordered-recipe--a-guide-is-an-ordered-recipe)
    - [Scenario: A procedure is written as an essay](#scenario-a-procedure-is-written-as-an-essay)
  - [`kb-authoring:a-step-is-one-action` — A step is one action](#kb-authoringa-step-is-one-action--a-step-is-one-action)
    - [Scenario: A step carries three actions](#scenario-a-step-carries-three-actions)
  - [`kb-authoring:a-step-leaves-no-gap` — A step leaves no gap](#kb-authoringa-step-leaves-no-gap--a-step-leaves-no-gap)
    - [Scenario: A step runs a command the guide never installs](#scenario-a-step-runs-a-command-the-guide-never-installs)
  - [`kb-authoring:a-subtask-nests-under-its-step` — A subtask nests under its step](#kb-authoringa-subtask-nests-under-its-step--a-subtask-nests-under-its-step)
    - [Scenario: A step's parts are promoted to steps](#scenario-a-steps-parts-are-promoted-to-steps)
  - [`kb-authoring:a-step-shows-its-output` — A step shows its output](#kb-authoringa-step-shows-its-output--a-step-shows-its-output)
    - [Scenario: A step describes its result in a sentence](#scenario-a-step-describes-its-result-in-a-sentence)
  - [`kb-authoring:a-guide-ends-in-a-verification` — A guide ends in a verification](#kb-authoringa-guide-ends-in-a-verification--a-guide-ends-in-a-verification)
    - [Scenario: A guide stops at the last edit](#scenario-a-guide-stops-at-the-last-edit)
  - [`kb-authoring:explanation-moves-to-a-companion` — Explanation moves to a companion document](#kb-authoringexplanation-moves-to-a-companion--explanation-moves-to-a-companion-document)
    - [Scenario: A step grows a rationale paragraph](#scenario-a-step-grows-a-rationale-paragraph)
  - [`kb-authoring:a-companion-walks-a-scenario` — A companion document walks a scenario](#kb-authoringa-companion-walks-a-scenario--a-companion-document-walks-a-scenario)
    - [Scenario: A companion becomes an essay](#scenario-a-companion-becomes-an-essay)
  - [`kb-authoring:no-document-indexes-the-filesystem` — A document does not index the filesystem](#kb-authoringno-document-indexes-the-filesystem--a-document-does-not-index-the-filesystem)
    - [Scenario: A digest lists the files of the directory it describes](#scenario-a-digest-lists-the-files-of-the-directory-it-describes)
  - [`kb-authoring:a-relative-link-is-explicit` — A relative link carries an explicit path](#kb-authoringa-relative-link-is-explicit--a-relative-link-carries-an-explicit-path)
    - [Scenario: A link names only a basename](#scenario-a-link-names-only-a-basename)
- [Unenforced](#unenforced)

<!--TOC-->

## Purpose

Rules this knowledge base adds to the adopted specs for how its own documents are written: the
recipe shape of a guide, what a step is, where explanation goes and the companion that receives it,
and two rules for any document, on filesystem indexes and on relative links. The adopted
`SPEC-guides.md` and `SPEC-docs-format.md` state what every adopter of the canon shares; this
spec states what this repository asks beyond them. `SPEC-kb-authoring/TEMPLATE-guide.md` carries
the skeleton a new guide is copied from.

## Requirements

### `kb-authoring:a-guide-is-an-ordered-recipe` — A guide is an ordered recipe

The author MUST write a guide as a `## Prerequisites` section followed by a `## Steps` section whose
body is one ordered list.

#### Scenario: A procedure is written as an essay

- GIVEN a document explaining how to bootstrap a project across six paragraphs
- WHEN a reader has to read the prose to find the command to run
- THEN the gate rejects it, and the paragraphs become numbered steps

Verify: `prek run guide-recipe-shape --all-files`

### `kb-authoring:a-step-is-one-action` — A step is one action

The author MUST make each step one imperative action, with the command it names in a fence.

#### Scenario: A step carries three actions

- GIVEN a step reading "install the toolchain, wire the hooks, and run the suite"
- WHEN a reader is interrupted after the second
- THEN nothing records where they stopped, and the step is three steps

Verify: reviewer confirms each step names one action and puts its command in a fence

### `kb-authoring:a-step-leaves-no-gap` — A step leaves no gap

Where a step assumes a state, that state MUST be produced by an earlier step or named in the
prerequisites.

#### Scenario: A step runs a command the guide never installs

- GIVEN step four running `just test`
- WHEN no earlier step installs the task runner and no prerequisite names it
- THEN the reader stops at a missing command, and the installing step is added before it

Verify: reviewer confirms every state a step assumes is produced earlier or listed as a prerequisite

### `kb-authoring:a-subtask-nests-under-its-step` — A subtask nests under its step

Where a step decomposes into parts, the author MUST write those parts as list items indented under
the step.

#### Scenario: A step's parts are promoted to steps

- GIVEN a step that configures three hooks
- WHEN each hook becomes a numbered step of its own
- THEN the list stops reading as the sequence of the job, and the gate rejects the flat bullets

Verify: `prek run guide-recipe-shape --all-files`

### `kb-authoring:a-step-shows-its-output` — A step shows its output

The author MUST follow a step's command with the output the reader should see, and MUST NOT spend
prose describing what that output already carries.

#### Scenario: A step describes its result in a sentence

- GIVEN a step whose command prints the position the working copy moved to
- WHEN the step describes that position in prose instead of showing it
- THEN the reader cannot tell whether their own run matched, and the transcript replaces the sentence

Verify: reviewer confirms each step's command is followed by its output rather than a description of it

### `kb-authoring:a-guide-ends-in-a-verification` — A guide ends in a verification

The author MUST make the last step a check the reader runs to confirm the result.

#### Scenario: A guide stops at the last edit

- GIVEN a guide whose final step writes a config file
- WHEN the reader has no command that says whether it worked
- THEN the guide promises completion it cannot show, and a verifying step closes it

Verify: reviewer confirms the final step is a command or observation confirming the result

### `kb-authoring:explanation-moves-to-a-companion` — Explanation moves to a companion document

Where a step needs justification beyond a decision, a hazard, or a non-obvious ordering constraint,
the author MUST move it into a linked companion document and MUST NOT expand the step into prose.

#### Scenario: A step grows a rationale paragraph

- GIVEN a step whose command needs the trade-off behind it explained
- WHEN the paragraph is written into the step
- THEN the recipe slows for every reader who already knows, and the paragraph belongs behind a link

Verify: reviewer confirms no step carries prose beyond a decision, a hazard, or an ordering constraint

### `kb-authoring:a-companion-walks-a-scenario` — A companion document walks a scenario

The author MUST give every document a guide offloads explanation to at least one worked scenario:
a concrete starting state, the commands run, and the outcome.

#### Scenario: A companion becomes an essay

- GIVEN a document explaining why a lock file is committed
- WHEN it argues the position for two pages and shows no run
- THEN it is as unusable as the guide it was extracted from, and a worked scenario replaces the argument

Verify: reviewer confirms each companion document carries a concrete scenario with commands and outcome

### `kb-authoring:no-document-indexes-the-filesystem` — A document does not index the filesystem

A document MUST NOT carry an enumeration whose objective is to mirror what is on disk.

#### Scenario: A digest lists the files of the directory it describes

- GIVEN a bucket whose contents are already visible in its directory listing
- WHEN a digest adds a `source-files:` frontmatter key, or a section headed `Source map`
- THEN the change is rejected, while a list whose entries carry their own payload passes

Verify: `prek run no-source-files-frontmatter -a && prek run no-source-map-section -a`

### `kb-authoring:a-relative-link-is-explicit` — A relative link carries an explicit path

A relative link MUST open with `./`, `../`, or a multi-segment path.

#### Scenario: A link names only a basename

- GIVEN two files named `runbook.md` in different directories
- WHEN a document links to one of them by basename alone
- THEN the commit fails, because neither a reader nor an LSP can tell which file is meant

Verify: `prek run no-bare-relative-links -a && prek run no-bare-relative-link-defs -a`

## Unenforced

No command decides these rules. A reviewer asks each one of every guide in the change.

| Rule                                            | Reviewer confirms                                                            |
| ----------------------------------------------- | ---------------------------------------------------------------------------- |
| `kb-authoring:a-step-is-one-action`             | No step names a second action                                                |
| `kb-authoring:a-step-leaves-no-gap`             | No step assumes a state that nothing earlier produced                        |
| `kb-authoring:a-step-shows-its-output`          | No step describes an output it could have shown                              |
| `kb-authoring:a-guide-ends-in-a-verification`   | The reader learns from the last step whether it worked                       |
| `kb-authoring:explanation-moves-to-a-companion` | No step carries prose beyond a decision, a hazard, or an ordering constraint |
| `kb-authoring:a-companion-walks-a-scenario`     | Each companion shows a run, not only an argument                             |

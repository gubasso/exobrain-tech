# Guides Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`guides:a-guide-is-an-ordered-recipe` — A guide is an ordered recipe](#guidesa-guide-is-an-ordered-recipe--a-guide-is-an-ordered-recipe)
    - [Scenario: A procedure is written as an essay](#scenario-a-procedure-is-written-as-an-essay)
  - [`guides:a-step-is-one-action` — A step is one action](#guidesa-step-is-one-action--a-step-is-one-action)
    - [Scenario: A step carries three actions](#scenario-a-step-carries-three-actions)
  - [`guides:a-step-leaves-no-gap` — A step leaves no gap](#guidesa-step-leaves-no-gap--a-step-leaves-no-gap)
    - [Scenario: A step runs a command the guide never installs](#scenario-a-step-runs-a-command-the-guide-never-installs)
  - [`guides:a-subtask-nests-under-its-step` — A subtask nests under its step](#guidesa-subtask-nests-under-its-step--a-subtask-nests-under-its-step)
    - [Scenario: A step's parts are promoted to steps](#scenario-a-steps-parts-are-promoted-to-steps)
  - [`guides:a-guide-ends-in-a-verification` — A guide ends in a verification](#guidesa-guide-ends-in-a-verification--a-guide-ends-in-a-verification)
    - [Scenario: A guide stops at the last edit](#scenario-a-guide-stops-at-the-last-edit)
  - [`guides:explanation-moves-to-a-companion` — Explanation moves to a companion document](#guidesexplanation-moves-to-a-companion--explanation-moves-to-a-companion-document)
    - [Scenario: A step grows a rationale paragraph](#scenario-a-step-grows-a-rationale-paragraph)
  - [`guides:a-companion-walks-a-scenario` — A companion document walks a scenario](#guidesa-companion-walks-a-scenario--a-companion-document-walks-a-scenario)
    - [Scenario: A companion becomes an essay](#scenario-a-companion-becomes-an-essay)
- [Unenforced rules](#unenforced-rules)
- [Seed](#seed)

<!--TOC-->

## Purpose

Rules governing a guide — the document that carries a reader from a starting state to a finished
result. Covers the recipe shape, what a step is, where explanation goes, and the shape of the
companion document that receives it. The markdown a guide is written in and the size it fits inside
belong to `SPEC-docs-format.md`; where a guide is placed belongs to `SPEC-knowledge-base-boundary.md`.

## Requirements

### `guides:a-guide-is-an-ordered-recipe` — A guide is an ordered recipe

The author MUST write a guide as a `## Prerequisites` section followed by a `## Steps` section whose
body is one ordered list.

#### Scenario: A procedure is written as an essay

- GIVEN a document explaining how to bootstrap a project across six paragraphs
- WHEN a reader has to read the prose to find the command to run
- THEN the gate rejects it, and the paragraphs become numbered steps

Verify: `pre-commit run guide-recipe-shape --all-files`

### `guides:a-step-is-one-action` — A step is one action

The author MUST make each step one imperative action, with the command it names in a fence.

#### Scenario: A step carries three actions

- GIVEN a step reading "install the toolchain, wire the hooks, and run the suite"
- WHEN a reader is interrupted after the second
- THEN nothing records where they stopped, and the step is three steps

Verify: reviewer confirms each step names one action and puts its command in a fence

### `guides:a-step-leaves-no-gap` — A step leaves no gap

Where a step assumes a state, that state MUST be produced by an earlier step or named in the
prerequisites.

#### Scenario: A step runs a command the guide never installs

- GIVEN step four running `just test`
- WHEN no earlier step installs the task runner and no prerequisite names it
- THEN the reader stops at a missing command, and the installing step is added before it

Verify: reviewer confirms every state a step assumes is produced earlier or listed as a prerequisite

### `guides:a-subtask-nests-under-its-step` — A subtask nests under its step

Where a step decomposes into parts, the author MUST write those parts as list items indented under
the step.

#### Scenario: A step's parts are promoted to steps

- GIVEN a step that configures three hooks
- WHEN each hook becomes a numbered step of its own
- THEN the list stops reading as the sequence of the job, and the gate rejects the flat bullets

Verify: `pre-commit run guide-recipe-shape --all-files`

### `guides:a-guide-ends-in-a-verification` — A guide ends in a verification

The author MUST make the last step a check the reader runs to confirm the result.

#### Scenario: A guide stops at the last edit

- GIVEN a guide whose final step writes a config file
- WHEN the reader has no command that says whether it worked
- THEN the guide promises completion it cannot show, and a verifying step closes it

Verify: reviewer confirms the final step is a command or observation confirming the result

### `guides:explanation-moves-to-a-companion` — Explanation moves to a companion document

Where a step needs justification beyond a decision, a hazard, or a non-obvious ordering constraint,
the author MUST move it into a linked companion document and MUST NOT expand the step into prose.

#### Scenario: A step grows a rationale paragraph

- GIVEN a step whose command needs the trade-off behind it explained
- WHEN the paragraph is written into the step
- THEN the recipe slows for every reader who already knows, and the paragraph belongs behind a link

Verify: reviewer confirms no step carries prose beyond a decision, a hazard, or an ordering constraint

### `guides:a-companion-walks-a-scenario` — A companion document walks a scenario

The author MUST give every document a guide offloads explanation to at least one worked scenario:
a concrete starting state, the commands run, and the outcome.

#### Scenario: A companion becomes an essay

- GIVEN a document explaining why a lock file is committed
- WHEN it argues the position for two pages and shows no run
- THEN it is as unusable as the guide it was extracted from, and a worked scenario replaces the argument

Verify: reviewer confirms each companion document carries a concrete scenario with commands and outcome

## Unenforced rules

No command decides these; review does. Each is asked of every guide in the change.

| Rule                                      | Asked at review                                                                             |
| ----------------------------------------- | ------------------------------------------------------------------------------------------- |
| `guides:a-step-is-one-action`             | Does any step name a second action?                                                         |
| `guides:a-step-leaves-no-gap`             | Does any step assume state nothing earlier produced?                                        |
| `guides:a-guide-ends-in-a-verification`   | Does the reader learn whether it worked?                                                    |
| `guides:explanation-moves-to-a-companion` | Is any prose there for a reason other than a decision, a hazard, or an ordering constraint? |
| `guides:a-companion-walks-a-scenario`     | Does the companion show a run, or only argue?                                               |

## Seed

`SPEC-guides/TEMPLATE-guide.md` carries the skeleton a new guide is copied from.

# ADR-0019: A guide is steps, not an essay

## Context and Problem Statement

The shelf governs where a guide goes, what markdown it may use, and how its phases name artifacts,
but nothing governs the body. Guides in this library drifted into essay shape: the command reachable
only by reading the paragraph around it, each step announced by one sentence and summarized by
another, and the concept behind a step explained at the step rather than linked to its owner. A
reader executing a procedure under pressure pays for every sentence that turns out not to be the
line to type.

## Considered Options

- Leave it to author judgment, as now.
- State it as a repository convention under `_docs/reference/`.
- Add a chapter to the documentation-design shelf.

## Decision Outcome

Chosen option: add a chapter. The rule is language-agnostic and transferable, which is what the
shelf holds; a convention under `_docs/reference/` would scope it to this repository and leave
every project adopting the shelf without it. `programming/docs-design/10-guide-shape.md` owns
step granularity, imperative voice, the prose budget, preconditions and verification, and length.
The root `AGENTS.md`, `99-checklist.md`, and `template-docs-rules.md` point at it and never
restate it.

The step-shape box previously carried under the Procedures section of the checklist moves to the
new Guide shape section, so one chapter owns it.

## Consequences

- Good: the body of a guide becomes reviewable against something, and every project adopting the
  shelf inherits the rule through `template-docs-rules.md`.
- Good: rationale is pushed to whatever owns it, so guides stop becoming a third copy of a concept.
- Bad: guides already in the library do not comply. They are migrated when touched, not in one
  sweep, so the tree holds both shapes for a while.
- Bad: the prose budget is a judgment call. No linter can hold it, and review carries it.

## Status

Accepted.

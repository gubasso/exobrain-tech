# A guide is a recipe, not an explanation with commands in it

## Context and Problem Statement

`ADR-guides-are-step-shaped` bound guide shape to a chapter that no longer exists, so the rule has
had no owner since the shelf holding it was deleted. Guides in this library drifted back toward
essays: prose the reader has to mine for the command, subtasks promoted to siblings of the step they
belong to, and steps that assume state nothing earlier produced. A reader pays for every sentence that
turns out not to be the line to type.

## Considered Options

- A spec of its own, seeded by a template and gated — chosen.
- Requirements folded into `SPEC-docs-format.md` — rejected: that spec owns markdown and size, and a
  guide's body is a different domain that would be read only by accident.
- Author judgment, restated in `AGENTS.md` — rejected: a rule the digest owns is a rule nothing
  checks, and the digest is a router.

## Decision Outcome

Chosen option: a spec of its own. `SPEC-guides.md` owns the recipe shape, step granularity, the
no-gap rule, where explanation goes, and the scenario shape of the document that receives it.
`SPEC-guides/TEMPLATE-guide.md` is the skeleton a new guide is copied from.

Three of its rules are decidable by a command and are enforced by `guide-recipe-shape`: the section
shape, the ordered opener, and subtasks nesting rather than flattening. The rest — one action per
step, no assumed state, a verifying last step, a prose budget — are held at review and listed as
unenforced in the spec.

Enforced by `guides:a-guide-is-an-ordered-recipe` and `guides:a-subtask-nests-under-its-step`.

## Consequences

- Good: an agent asked for a recipe has one shape to copy and a gate that refuses the essay.
- Good: rationale lands in a companion document that walks a scenario, so a guide stops being a
  second copy of a concept.
- Bad: the prose budget is a judgment call no command can hold, and review carries it.
- Bad: guides written before this are retrofitted when the gate reaches them.

## Status

Implemented by [SPEC-guides](../specs/SPEC-guides.md) and `.hooks/guide-recipe-shape.sh`.

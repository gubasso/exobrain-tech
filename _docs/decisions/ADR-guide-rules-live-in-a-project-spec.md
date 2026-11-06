# Guide rules this library adds live in a spec it owns

## Context and Problem Statement

This library wrote its guide rules into its adopted copy of `SPEC-guides.md`, and two authoring rules
into its adopted `SPEC-docs-format.md`. An upgrade keeps the bytes of an adopted file, so the copies
fell behind the shipped specs, and the shipped guide rules never reached this repository. An adopted
spec states what every spec-driven-docs instance shares, so a rule particular to this library has no
place in it.

## Considered Options

- `the adopted specs equal the shipped copies, and a spec this library owns holds its own rules` —
  chosen.
- `keep the local rewrite of the adopted specs` — rejected: every release then needs a hand merge,
  and the shipped rules stay out of reach.
- `drop the local rules` — rejected: the recipe shape is gated here and guides depend on it.

## Decision Outcome

Chosen option: `the adopted specs equal the shipped copies, and a spec this library owns holds its
own rules`. `SPEC-kb-authoring.md` holds the recipe shape of a guide, its steps, its companion
document, and the rules on filesystem indexes and relative links, under its own domain.
`SPEC-kb-authoring/TEMPLATE-guide.md` is the skeleton beside it. This record supersedes
`ADR-a-guide-is-a-recipe.md`, whose choice of a gated recipe shape stands.

Enforced by `kb-authoring:a-guide-is-an-ordered-recipe` and `kb-authoring:a-subtask-nests-under-its-step`.

## Consequences

- Good: an upgrade ports a shipped spec whole, because no adopted spec carries a local edit.
- Good: a reader finds this library's own rules in one spec of its own.
- Bad: a guide here answers to two specs, the shipped one and this library's.

## Status

Implemented by [SPEC-kb-authoring](../specs/SPEC-kb-authoring.md) and `.hooks/guide-recipe-shape.sh`.

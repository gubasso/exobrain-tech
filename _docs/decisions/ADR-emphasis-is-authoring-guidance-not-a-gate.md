# Emphasis is authoring guidance, held by review rather than by a gate

## Context and Problem Statement

`programming/spec-driven-docs/06-format.md` states that a document contains no bold or italic text,
and the shelf ships a hook for it. This repository holds hundreds of markdown files written before
the rule. Gating it means an exemption list of that size and a ratchet that runs for as long as the
list does, for breaches that change nothing a reader can act on.

## Considered Options

- State the rule, hold it by review, and record it unenforced — chosen.
- Gate it behind a shrinking exemption list — rejected: the list costs more than the rule is worth,
  and its entries are formatting rather than meaning.
- Soften the rule to admit what already exists — rejected: `06-format.md` forbids raising a budget to
  admit a document, and the same reasoning binds a rule.

## Decision Outcome

Chosen option: state it and hold it by review, because the rule earns its place at authoring time and
nothing is bought by enforcing it over material that predates it.

Lean markdown stays binding on what an author writes: headings, lists, tables, fences, inline code,
links, and no emphasis. There is no hook in this repository, no baseline file, and no debt list. The
shelf lists the rule in the unenforced table of `08-gates.md`, and
`07-lifecycle.md` owns the general choice between gating a rule and holding it by review.

## Consequences

- Good: the rule binds new authoring immediately, at no adoption cost.
- Good: the gate catalog stays honest, since every rule there is either checked or declared.
- Bad: existing emphasis survives, and a reviewer who does not look lets a new instance through.

## Status

Accepted

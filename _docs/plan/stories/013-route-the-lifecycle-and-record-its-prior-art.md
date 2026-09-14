# 013 — Route the Lifecycle and Record Its Prior Art

## Goal

A reader can see who drives the landing sequence, which checks a program may make, where the rules
came from, and which real tools implement them, without any of those answers being normative about
the others.

## Example

Four chapters state a lifecycle and nothing states who runs it.

```console
$ git grep -c "router\|gate" programming/agent-integration/09-disposable-staging-and-direct-landing.md
0
```

Nothing names the skill that drives the sequence, nothing separates a check a program can make from
a decision it cannot, and no chapter carries a source for the file-replacement guarantee the
lifecycle relies on.

## Core

One router skill drives the whole sequence and owns none of it. Gates split on one question: is the
answer a fact or an opinion. A check states what it holds, and a proxy that resembles the property
is not the property. The sources are dated and the implementations are mapped without becoming the
rule.

## In scope

- The router skill's responsibilities, and what it links rather than restates.
- The gate boundary, as two columns.
- The proxy rule, with the declared build target as its clearest case.
- The portable receipt pattern, extending skill distribution.
- A dated survey of primary sources, with every inference labeled.
- A concept-to-name map for two tools, with the unimplemented parts marked.

## Out of scope

- The worked example and the checklist, which the next story lands.
- A host-policy chapter, which the distribution chapter does not need at its current size.

## Governed by

- `_docs/specs/SPEC-knowledge-base-boundary.md` — what belongs in a bucket and what does not.
- `_docs/specs/SPEC-docs-format.md` — the register and the size budgets.
- `_docs/specs/SPEC-project-platform.md` — a portable chapter names a property, not a platform.

## Amends

- `programming/agent-integration/11-router-and-gates.md`, `12-prior-art.md`, and
  `13-reference-implementations.md` — created.
- `programming/agent-integration/04-skill-distribution.md` — the portable receipt pattern.
- `programming/agent-integration/README.md` and `AGENTS.md` — the routes.

## Acceptance

- Every prior-art claim carries a dated primary source, and every inference is labeled.
- The router chapter duplicates no candidate, staging, landing, or migration rule.
- Product names occur only in the prior-art and reference-implementation chapters.
- The distribution chapter stays within budget and agrees with the vendor adapters.
- No example gate represents target compilation by inspecting source text.
- `just check` passes.

## Tasks

- [ ] State the router's responsibilities and the gate boundary.
- [ ] Extend distribution with the portable receipt pattern.
- [ ] Record the sources, dated, with inferences labeled.
- [ ] Map both implementations, marking what is not implemented.

## Rabbit holes

- Letting the map become the rule — escape: the portable chapter owns the concept, and a tool
  changes it by argument rather than by example.
- Presenting a mailing-list report as a specification — escape: the entry says which it is, and the
  chapter takes only the narrow consequence.

## Done when

The sequence has a driver, the checks have a boundary, the rules have sources, and the tools are
examples rather than authority.

## Revisions

None.

# A spec marks its open questions inline

## Context and Problem Statement

A rule sometimes has to be agreed while one thing about it is undecided — two constraints held
without a resolution between them. The spec shape had nowhere to put that: requirement blocks admit
no prose between them, so the question was either dropped, which invites an agent to resolve it
silently and plausibly, or parked on a separate page that has to be maintained.

## Considered Options

- A living design page per subsystem, carrying an `Unresolved` list.
- An inline `[NEEDS CLARIFICATION: <question>]` marker at the requirement.
- Nothing: an undecided rule is not written until it is decided.

## Decision Outcome

Chosen option: the inline marker, taken from GitHub Spec Kit along with its cap of three per spec
and the three admissions that earn one — a question that changes scope, reads several ways with
different consequences, or has no sensible default. The inventory is then a grep rather than a page.

The marker is the single exception to `02-specs.md`'s no-prose-between-requirements rule, because it
is a property of the requirement above it rather than narrative. `03-rules.md` owns it.

Spec Kit resolves markers before planning; this framework blocks enactment instead, because its
specs are long-lived and an open question is legitimate state for an agreed spec. Shipping code
against one is not, and the check is a set intersection against the already-derived enacted list.

## Consequences

- Good: a whole page type collapses into a token plus `rg -F '[NEEDS CLARIFICATION'`.
- Good: the question travels with the rule it concerns, so a reader loading the spec cannot miss it.
- Bad: the shape gate now admits one line between requirement blocks, and a reader who mistakes that
  for a general licence will put prose there.
- Bad: a marked rule blocks work rather than the commit, so nothing forces a stale marker to be
  resolved.

## Status

Accepted.

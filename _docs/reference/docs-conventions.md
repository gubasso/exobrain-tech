# Documentation conventions

Where each documentation rule for `exobrain-tech` is stated, and the few conventions that are local to
this repository. The method is
[spec-driven-docs](../../programming/spec-driven-docs/README.md), and every rule below has one owner:
this page points at it rather than repeating it, because a restated rule drifts from the one that
binds.

## Who owns what

| Subject                                      | Owner                                                                        |
| -------------------------------------------- | ---------------------------------------------------------------------------- |
| Zones, and which one a document belongs in   | [01 — Placement](../../programming/spec-driven-docs/01-placement.md)         |
| The spec artifact: parts, ids, and size      | [SPEC-specs](../specs/SPEC-specs.md)                                         |
| Requirement grammar and typed clauses        | [02 — Specs](../../programming/spec-driven-docs/02-specs.md)                 |
| Decision records and their lifecycle         | [SPEC-decision-records](../specs/SPEC-decision-records.md)                   |
| Sizes, fences, links, and filesystem indexes | [SPEC-docs-format](../specs/SPEC-docs-format.md)                             |
| External-system bugs and their case ids      | [SPEC-known-issues](../specs/SPEC-known-issues.md)                           |
| One fact, one home                           | [00 — Model](../../programming/spec-driven-docs/00-model.md)                 |
| Drafts, and what promotion means             | [07 — Lifecycle](../../programming/spec-driven-docs/07-lifecycle.md)         |
| What an agent loads, and semantic names      | [05 — Agent Context](../../programming/spec-driven-docs/05-agent-context.md) |

The root `AGENTS.md` carries the rules that bind every file. Where it and a spec both speak, the spec
is the one that binds.

## Local to this repository

Zone comes first and topic second: `_docs/reference/<topic>/`, never `_docs/<topic>/reference.md`. A
top-level topic folder mixes reader needs and forces a reader to infer intent from prose. Where a
document has a claim on two zones, choose the owner and link from the other.

Each substantial content area carries an `AGENTS.md` digest, derived from that area's files and never
the source of truth for them. It declares a `last-synced` date so staleness is visible, and is
rewritten when the area's knowledge changes. See
[AGENTS.md digest template](./TEMPLATE-agents-digest.md) and
[Knowledge-base architecture](../explanation/knowledge-base-architecture.md).

A fact that expires — a price, a benchmark, a model roster, an external API shape — carries an owner,
a revalidation cadence, and a `last_checked` date in a small machine-readable registry. The tracked
artifact stays the source of truth; the registry is what keeps it from going stale unnoticed.

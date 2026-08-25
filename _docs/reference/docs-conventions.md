# Documentation conventions

Where each documentation rule for `exobrain-tech` is stated, and the few conventions that are local to
this repository. Every owner below is a file in this checkout, so no rule needs a network to read.
The further-reading column names the canon chapter that argues the same rule at length.

## Who owns what

| Subject                                      | Owner                                                      | Further reading                                                                                          |
| -------------------------------------------- | ---------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| Zones, and which one a document belongs in   | [AGENTS.md](../../AGENTS.md)                               | [01 — Placement](https://github.com/gubasso/spec-driven-docs/blob/v0.1.4/method/01-placement.md)         |
| The spec artifact: parts, ids, and size      | [SPEC-docs-specs](../specs/SPEC-docs-specs.md)             | [02 — Specs](https://github.com/gubasso/spec-driven-docs/blob/v0.1.4/method/02-specs.md)                 |
| Requirement grammar and typed clauses        | [SPEC-docs-specs](../specs/SPEC-docs-specs.md)             | [03 — Rules](https://github.com/gubasso/spec-driven-docs/blob/v0.1.4/method/03-rules.md)                 |
| Decision records and their lifecycle         | [SPEC-decision-records](../specs/SPEC-decision-records.md) | [04 — Decisions](https://github.com/gubasso/spec-driven-docs/blob/v0.1.4/method/04-decisions.md)         |
| Sizes, fences, links, and filesystem indexes | [SPEC-docs-format](../specs/SPEC-docs-format.md)           | [06 — Format](https://github.com/gubasso/spec-driven-docs/blob/v0.1.4/method/06-format.md)               |
| External-system bugs and their case ids      | [SPEC-known-issues](../specs/SPEC-known-issues.md)         | [07 — Lifecycle](https://github.com/gubasso/spec-driven-docs/blob/v0.1.4/method/07-lifecycle.md)         |
| One fact, one home                           | [AGENTS.md](../../AGENTS.md)                               | [00 — Model](https://github.com/gubasso/spec-driven-docs/blob/v0.1.4/method/00-model.md)                 |
| Drafts, and what promotion means             | [AGENTS.md](../../AGENTS.md)                               | [07 — Lifecycle](https://github.com/gubasso/spec-driven-docs/blob/v0.1.4/method/07-lifecycle.md)         |
| What an agent loads, and semantic names      | [AGENTS.md](../../AGENTS.md)                               | [05 — Agent Context](https://github.com/gubasso/spec-driven-docs/blob/v0.1.4/method/05-agent-context.md) |

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

# 02 — Governance & docs

The documents that tell humans and agents how to work in the repo, and how decisions are recorded.
Seed these early so conventions are set before the code grows around them.

## `CLAUDE.md` (agent instructions)

`CLAUDE.md` carries repository-specific instructions for LLM agents: scope, conventions, what is
canonical, and any hard rules. Author it for **self-containment** — an agent should be able to work
correctly from the repo alone, without external context. This repo's own `CLAUDE.md` is a worked
reference.

## The `AGENTS.md` convention

The root instruction file is hand-authored rules and routing, and a per-directory digest is a second
shape generated from the subtree it maps. What each one carries, and why conflating them costs a
project the place its conventions live, is
[agent-integration/01 — Instruction files](../agent-integration/01-instruction-files.md).

When you adopt the digest, fix its schema up front: required frontmatter (`digest-of`,
`last-synced`, `token-estimate`) and stable body headings, regenerated from sources rather than
hand-edited. The digest keeps no index of the directory; the filesystem owns what exists, and a
checked-in file list drifts on the next add or rename.

## ADR scaffold

Record consequential decisions as Architecture Decision Records so the _why_ survives. Use a
**MADR-minimal** template ([MADR](https://github.com/adr/madr)) under `docs/decisions/`, naming each
record `ADR-<slug>.md` so the slug is its identifier and no branch has to allocate a number. A good
seed is a self-containment ADR that states the docs-as-SoT principle itself.

## README-as-index discipline

Every directory's `README.md` is its index — it defines what the area is for and what belongs in it,
and routes by meaning. It may name the files under it, but not merely to list them. This keeps navigation deterministic and prevents the drift that duplication causes. This shelf's
[hub README](./README.md) is an example.

## When a tool writes these files for you

A tool that seeds governance documents into a project owns what it wrote and nothing else, and how
that ownership is recorded, previewed, and upgraded is
[agent-integration/09 — Disposable staging and direct landing](../agent-integration/09-disposable-staging-and-direct-landing.md).
This shelf routes the step. It states no rule about the record, the ownership classes, or what
happens on a later run.

## Automation

`bootstrap-governance` seeds `CLAUDE.md`, an `AGENTS.md`, and an ADR scaffold (MADR-minimal plus a
self-containment ADR). The conventions above are the SoT; see
[07 — Automation with cog](./07-automation-with-cog.md).

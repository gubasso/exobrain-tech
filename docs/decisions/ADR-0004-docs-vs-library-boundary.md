# ADR-0004: Reserve docs/ for product reference; knowledge lives in the library

## Context and Problem Statement

`exobrain-tech` is a knowledge base: the markdown content in the eight top-level buckets is the product, and `docs/` holds the specs _about_ that product. That boundary is described in prose (`README.md`, `AGENTS.md`, `docs/explanation/knowledge-base-architecture.md`) but was never recorded as a decision or enforced as a first-class rule. Without one, knowledge articles drift into `docs/`, `docs/` swells into a second content tree, and the library fractures.

## Considered Options

- No explicit rule; rely on the implicit understanding in the explanation doc.
- Let `docs/` hold everything, including knowledge content.
- Reserve `docs/` for product governance/reference only; keep all knowledge in the top-level subject buckets.

## Decision Outcome

Chosen option: **reserve `docs/` for the product's own reference — knowledge lives in the library.** `docs/` holds governance and specs about how the knowledge base works: decisions (ADRs), conventions, architecture, guides, and explanation. All knowledge articles and books live in the eight top-level buckets (`programming/`, `languages/`, `systems/`, `infra/`, `tools/`, `platforms/`, `workflows/`, `data/`). The analogy is exact: in a code project, code is the product and `docs/` holds specs about the code; here, the content is the product and `docs/` holds specs about the content. When recording _how the knowledge base works_, write in `docs/`; when recording _knowledge_, write in the content tree. Cross-link from `docs/` to a library article only when the product reference must reference it.

## Consequences

- Good: every piece has one clear home; `docs/` stays lean and governance-focused; agents and contributors know where to place new material.
- Bad: requires ongoing discipline to distinguish "specs _about_ the knowledge base" from "knowledge _in_ the knowledge base".

## Status

Accepted. The repository structure already reflects it; enacted by the boundary rules in `AGENTS.md` and `CLAUDE.md`.

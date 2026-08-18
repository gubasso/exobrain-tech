# Reserve `_docs/` for product metadata; knowledge lives in the library

## Context and Problem Statement

`exobrain-tech` is a knowledge base: the markdown content in the eight top-level buckets is the product — the library — and `_docs/` holds the metadata and specs _about_ that product. That boundary is described in prose (`README.md`, `AGENTS.md`, `_docs/explanation/knowledge-base-architecture.md`) but was never recorded as a decision or enforced as a first-class rule. Without one, knowledge articles drift into `_docs/`, `_docs/` swells into a second content tree, and the library fractures. The directory carries a leading underscore precisely to signal "metadata about the product," keeping it visually distinct from the content buckets.

## Considered Options

- No explicit rule; rely on the implicit understanding in the explanation doc.
- Let `_docs/` hold everything, including knowledge content.
- Reserve `_docs/` for product governance/metadata only; keep all knowledge in the top-level subject buckets.

## Decision Outcome

Chosen option: **reserve `_docs/` for the product's own metadata — knowledge lives in the library.** The product is the library: the knowledge itself, in the eight top-level buckets (`programming/`, `languages/`, `systems/`, `infra/`, `tools/`, `platforms/`, `workflows/`, `data/`). `_docs/` is not the product — it holds the metadata and specs about how the knowledge base works: decisions (ADRs), conventions, architecture, guides, and explanation. The analogy is exact: in a code project, the codebase is the product and `_docs/` holds specs about the code; here, the content is the product and `_docs/` holds specs about the content. When recording _how the knowledge base works_, write in `_docs/`; when recording _knowledge_, write in the content tree. Cross-link from `_docs/` to a library article only when the product reference must reference it.

## Consequences

- Good: every piece has one clear home; `_docs/` stays lean and governance-focused; agents and contributors know where to place new material.
- Bad: requires ongoing discipline to distinguish "specs _about_ the knowledge base" from "knowledge _in_ the knowledge base".

## Status

Accepted. The repository structure already reflects it; enacted by the boundary rules in `AGENTS.md`.

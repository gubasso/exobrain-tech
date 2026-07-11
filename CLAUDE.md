# CLAUDE.md

@AGENTS.md

Guidance for coding agents working in `exobrain-tech`. `AGENTS.md` is the single
source of truth for governance (self-containment, decisions); the
project-specific authoring rules below extend it.

## What this repo is

This is the public technical knowledge base. The knowledge is the product; it
lives in the eight top-level buckets (`programming/`, `languages/`, `systems/`,
`infra/`, `tools/`, `platforms/`, `workflows/`, `data/`) and may use whatever
file and directory structure best serves the knowledge within a bucket.

`docs/` is reserved for the product's own reference — governance, ADRs,
conventions, architecture, and explanation about how this knowledge base itself
works — using the Diátaxis scaffold. The boundary is non-negotiable: never place
a knowledge article under `docs/`. Placement test: "Is this about how the KB
works?" → `docs/`. "Is this knowledge the library serves?" → the content bucket.
See `docs/decisions/ADR-0004-docs-vs-library-boundary.md`.

## Documentation Maintenance

- Keep `docs/` organized by Diátaxis zones: `decisions/`, `guides/`,
  `reference/`, and `explanation/`.
- ADRs are lean MADR records at or below 350 words with exactly one `Status`.
- Valid statuses are `Proposed`, `Accepted`, `Implemented`, `Superseded`, and
  `Rejected`.
- Never delete accepted decisions; supersede or reject them with links.
- Keep each fact in one source of truth and cross-link instead of duplicating.
- Drafts live in `.draft/`; promotion means rewriting into the right zone.
- `README.md` and `AGENTS.md` are indexes or digests, not rule dumps.
- Use `docs/reference/known-issues/` for external-system bugs.
- Every fenced code block needs a language specifier; use `text` when needed.

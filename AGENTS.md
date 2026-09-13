# AGENTS

Single source of truth for how `exobrain-tech` works. `README.md` indexes the content; this file
holds the rules, as a digest of rules and pointers.

## Glossary

- `MONOREPO` — this public KB plus the private `../exobrain-tech-vault`, one logical knowledge base
  whose cross-references are in-repo, never external (ADR-self-containment).
- `$EXOBRAIN_TECH` and `$EXOBRAIN_TECH_VAULT` — checkouts of the public KB and the private vault,
  which docs and scripts point into as `$EXOBRAIN_TECH/<path>` and `$EXOBRAIN_TECH_VAULT/<path>`.

## What the product is

Non-negotiable: the product is the library — the knowledge itself. It lives in the eight top-level
buckets (`programming/`, `languages/`, `systems/`, `infra/`, `tools/`, `platforms/`, `workflows/`,
`data/`), and may use whatever structure serves the knowledge within a bucket.

`_docs/` is not the product. `specs/` states what binds; the Diátaxis zones (`decisions/`, `guides/`,
`reference/`, `explanation/`) hold the rest. Never place a knowledge article there. Placement test:
"Is this about how the KB works?" → `_docs/`; "Is this knowledge the library serves?" → the owning
bucket (ADR-docs-vs-library-boundary).

Two boundaries bind what lives here. Private equipment identity, security posture, recovery material,
credentials, and personal workflows belong in `exobrain-tech-vault`. And this repository names no
planning, project-management, or workflow method (ADR-the-repository-names-no-planning-method).

## Documentation canon

Non-negotiable: understanding, linting, formatting, testing, and building this checkout read only
local files and the tools `flake.nix` pins. `_docs/specs/` and `_docs/decisions/` are instance-owned;
`.spec-driven-docs/` is the managed projection, and `sdd verify` checks it offline. The upstream
[documentation canon](https://github.com/gubasso/spec-driven-docs) owns the method and serves the
delivered gates as a lock-pinned tool, never as a source this repository reads. This link is the only
place that names it; `.spec-driven-docs/manifest.json` records the version
(ADR-the-upstream-canon-is-named-once).

Load the specs of every domain you touch before acting. Each rule has a stable ID that commits,
reviews, and gate failures cite; change its owning spec with the behavior. Record a significant
hard-to-reverse choice as one slug-named ADR from the local template. Accepted records stay
immutable, and a record is loaded only when the rationale is needed.

## Working under these rules

Non-negotiable: a stated rule is applied, not re-raised. Where this file, the method, or a source it
cites answers something, that is the answer. Report only what is still open; a closed item hides the
live ones. Retrofitting a rule onto older files is ordinary pre-commit work
(ADR-a-stated-rule-is-applied-not-re-raised).

`master` is the trunk: the only permanent branch. Work reaches it through a short-lived branch and
one squash-merged pull request, never a direct push (ADR-master-is-the-trunk).

## Authoring

- Keep each fact in one source of truth and cross-link instead of duplicating.
- Drafts live in `.draft/`; promotion means rewriting into the right zone, not moving a file.
- `README.md` and every `AGENTS.md` are indexes or digests, not rule dumps. This root digest stays
  plain and frontmatter-free; per-directory digests carry frontmatter.
- Record an external-system bug in `_docs/reference/known-issues/` as `KI-<slug>.md`; that slug is
  the case id a suppression names, and the record carries the condition that retires it.
- Every fenced code block needs a language specifier; use `text` when none applies.
- Lean markdown: headings, lists, tables, fences, inline code, links, no bold or italic
  (ADR-emphasis-is-authoring-guidance-not-a-gate).
- Every size budget in `_docs/specs/SPEC-docs-format.md` is gated. Over budget, a document splits.
- A guide is a recipe, not an essay: prerequisites, then ordered steps, each one imperative action
  with its command and its check. `_docs/specs/SPEC-guides.md` owns the rules and
  `SPEC-guides/TEMPLATE-guide.md` beside it is the skeleton (ADR-a-guide-is-a-recipe).

## Executable artifacts and tools

A bucket may ship a working artifact a reader is expected to copy and run. It lives beside the
chapter that explains it and carries three obligations in the same change
(ADR-executable-artifacts-in-the-library): a gate, a `just test` case, and its tools in the devShell.

## No document narrates its own history

Non-negotiable: a file states what is true now, never how it got that way — no `formerly`, no
`used to`, no `this replaces`, no patch-series marker, no note explaining an absence. Delete the
clause; if nothing a reader can act on disappeared, it was archaeology. A decision record is the one
exemption, because holding history is its entire job.

## Filesystem state

Non-negotiable: the filesystem owns its own state, and no document indexes it. Forbidden is the
enumeration kept because the directory exists — a tree of the tree, a topic-to-filename table, a
`source-files` list. A list whose entries carry their own payload stays (ADR-filesystem-owns-disk-state).

<!-- BEGIN spec-driven-docs docs -->
## Documentation

- Load the affected specs before editing governed content: `_docs/specs/SPEC-<domain>.md`.
- Treat decision records as immutable rationale and load them only when asked why.
- Read the writing style before you author or edit prose: `sdd method writing-style`.
- Write and edit step-by-step guides to the adopted guides spec, `_docs/specs/SPEC-guides.md`.
- Name a document by a slug drawn from its subject, never by a number. Where its directory holds documents with no kind prefix, give that directory a `README.md` saying what it holds and what each document covers.
- Run `sdd verify` before handoff.
- Keep adopted specs, the tracking registry, and local integration instance-owned.
<!-- END spec-driven-docs docs -->

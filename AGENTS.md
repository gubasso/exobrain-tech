# AGENTS

Single source of truth for how `exobrain-tech` works. `README.md` indexes the content; this file
holds the rules. Keep it a digest — rules and pointers, never a rule dump.

## Glossary

- `MONOREPO` — this public KB plus the private `../exobrain-tech-vault`, one logical knowledge base.
  References between them are in-repo cross-references, not external ones (ADR-self-containment).
- `$EXOBRAIN_TECH` and `$EXOBRAIN_TECH_VAULT` — checkouts of the public KB and the private vault,
  which docs and scripts point into as `$EXOBRAIN_TECH/<path>` and `$EXOBRAIN_TECH_VAULT/<path>`.

## What the product is

Non-negotiable: the product is the library — the knowledge itself. It lives in the eight top-level
buckets (`programming/`, `languages/`, `systems/`, `infra/`, `tools/`, `platforms/`, `workflows/`,
`data/`), exactly as a code project's product is its codebase, and may use whatever structure serves
the knowledge within a bucket.

`_docs/` is not the product. `specs/` states what binds; the Diátaxis zones (`decisions/`, `guides/`,
`reference/`, `explanation/`) hold the rest, following the documentation canon below. Never place a
knowledge article there. Placement test: "Is this about how the KB works?" → `_docs/`; "Is this
knowledge the library serves?" → the owning bucket. Cross-link from `_docs/` only when the product
reference must mention it (ADR-docs-vs-library-boundary).

Two boundaries bind what lives here. Private equipment identity, security posture, recovery material,
credentials, and personal workflows belong in `exobrain-tech-vault`. And this repository names no
planning, project-management, or workflow method: the docs root may hold directories serving
complementary domains, and nothing names what fills them (ADR-the-repository-names-no-planning-method).

## Documentation canon

Non-negotiable: understanding, linting, formatting, testing, and building this checkout use only
local files. `_docs/specs/` and `_docs/decisions/` are instance-owned; `.spec-driven-docs/` is the
pinned managed projection, and `.spec-driven-docs/verify.sh --target . --offline` checks its hashes.
The upstream [documentation canon](https://github.com/gubasso/spec-driven-docs) supplies method
doctrine and upgrade material but is not an operational dependency. This link is the only place
that names it; `.spec-driven-docs/manifest.json` records the version this checkout implements
(ADR-the-upstream-canon-is-named-once).

Load the specs of every domain you touch before acting. Each rule has a stable ID that commits,
reviews, and gate failures cite; change its owning spec with the behavior. Load decisions only when
the rationale is needed. Record a significant hard-to-reverse choice as one slug-named ADR from the
local template, within the gated size and status contracts. Accepted records stay immutable.

## Working under these rules

Non-negotiable: a stated rule is applied, not re-raised. Where this file, the method, or a source it
cites answers something, that is the answer — including whether work earns a record, a unit of work,
or its own commit. Report only what is still open; a closed item hides the live ones. Retrofitting a
rule onto older files is ordinary pre-commit work (ADR-a-stated-rule-is-applied-not-re-raised).

`master` is the trunk: the only permanent branch and the repository default. Work reaches it through
a short-lived branch and one squash-merged pull request, never a direct push
(ADR-master-is-the-trunk).

## Authoring

- Keep each fact in one source of truth and cross-link instead of duplicating.
- Drafts live in `.draft/`; promotion means rewriting into the right zone, not moving a file.
- `README.md` and every `AGENTS.md` are indexes or digests, not rule dumps.
- Record an external-system bug in `_docs/reference/known-issues/` as `KI-<slug>.md`; that slug is
  the case id a suppression names, and the record carries the condition that retires it.
- Every fenced code block needs a language specifier; use `text` when none applies.
- Lean markdown: headings, lists, tables, fences, inline code, links, no bold or italic. It binds
  what you write and is held by review (ADR-emphasis-is-authoring-guidance-not-a-gate).
- Every size budget in `_docs/specs/SPEC-docs-format.md` is gated. Over budget, a document splits.
- A guide is a recipe, not an essay: prerequisites, then ordered steps, one imperative action each,
  the command in a fence, subtasks nested under their step, and a last step that verifies the result.
  Depth goes to a companion document that walks a scenario. Every command is followed by the output
  the reader should see, simulated from a worked example the whole guide runs, and no sentence
  describes what a transcript already shows. `_docs/specs/SPEC-guides.md` owns the rules and
  `_docs/specs/SPEC-guides/TEMPLATE-guide.md` is the skeleton to copy (ADR-a-guide-is-a-recipe).
- This root digest stays plain and frontmatter-free; per-directory digests carry frontmatter.

## Executable artifacts and tools

A bucket may ship a working artifact a reader is expected to copy and run. It is library content,
living beside the chapter that explains it, and shipping one carries three obligations in the same
change (ADR-executable-artifacts-in-the-library): a gate in `.pre-commit-config.yaml`, a case under
`just test`, and each tool it needs in the `flake.nix` devShell. Every tool a gate runs belongs there
too — one reachable only inside pre-commit cannot be tested at all. A fence stays right for
illustration; the line is whether the reader is meant to run the thing.

## No document narrates its own history

Non-negotiable: a file states what is true now, never how it got that way — no `formerly`, no
`used to`, no `this replaces`, no `inherited from`, no patch-series marker, no note explaining an
absence. The test: delete the clause; if nothing a reader can act on disappeared, it was archaeology.
A decision record is the one exemption, because holding history is its entire job.

## Filesystem state

Non-negotiable: the filesystem owns its own state, and no document indexes it. Forbidden is the
enumeration kept because the directory exists — a tree of the tree, a topic-to-filename table, a
`source-files` list. A list, table, or tree is welcome wherever its entries carry their own payload:
what a directory reserves, what a file specifies, its scope. Strip the paths out and read what is
left (ADR-filesystem-owns-disk-state).

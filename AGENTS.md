# AGENTS

Single source of truth for how `exobrain-tech` works. `README.md` indexes the content; this file
holds the rules. Keep it a digest — rules and pointers, never a rule dump.

## Glossary

- `MONOREPO` — this public KB plus the private `../exobrain-tech-vault`, together one logical
  knowledge base. References between them are in-repo cross-references, not external dependencies,
  which is what keeps them compatible with Self-containment below (ADR-self-containment).
- `$EXOBRAIN_TECH` and `$EXOBRAIN_TECH_VAULT` — checkouts of the public KB and the private vault.
  Docs and scripts point into them as `$EXOBRAIN_TECH/<path>` and `$EXOBRAIN_TECH_VAULT/<path>`.

## What the product is

Non-negotiable: the product is the library — the knowledge itself. It lives in the eight top-level
buckets (`programming/`, `languages/`, `systems/`, `infra/`, `tools/`, `platforms/`, `workflows/`,
`data/`), exactly as a code project's product is its codebase, and may use whatever structure best
serves the knowledge within a bucket.

`_docs/` is not the product. It holds the metadata and specs about the product, organized by Diátaxis
zone (`decisions/`, `guides/`, `reference/`, `explanation/`) and following the spec-driven-docs
method. Never place a knowledge article there. Placement test: "Is this about how the KB works?" →
`_docs/`. "Is this knowledge the library serves?" → the owning bucket. Cross-link from `_docs/` only
when the product reference must mention it. See ADR-docs-vs-library-boundary.

Public/private boundary: private equipment identity, security posture, recovery material,
credentials, and personal workflows belong in `exobrain-tech-vault`, never here.

## Self-containment

Non-negotiable: this project is self-contained. The knowledge it depends on is held in-repo. An
external reference is allowed only as a public link or citation for further reading — never as a
load-bearing dependency on a resource outside the repository, and never on an external, local,
personalized, or mutating repository, path, or tool. If one is required to understand, build, or
operate this project, copy its essential knowledge in so the repo stays complete (ADR-self-containment).

## Decisions

Non-negotiable: record every significant, hard-to-reverse decision as an ADR under
`_docs/decisions/`, one per file, named `ADR-<slug>.md` from `TEMPLATE-adr.md` — at or below 350
words, exactly one `Status`, so the rationale lives with the work. Valid statuses: `Proposed`,
`Accepted`, `Implemented`, `Deprecated`, `Superseded`, `Rejected`.

The slug is the identifier: a filename carries no counter and no digit, because two branches each
allocating the next number are both right and the merge leaves two records claiming one identity.
Cite a record by its slug, and never from code — code cites the rule ID it satisfies.

Accepted decisions are never deleted: a replaced one gets a new superseding ADR, and one whose
context evaporated is marked `Deprecated`. A record is not annotated to track how it aged; a reader
who wants what binds today reads the spec, the artifact that states the present.

## Authoring

- Keep each fact in one source of truth and cross-link instead of duplicating.
- Drafts live in `.draft/`; promotion means rewriting into the right zone, not moving a file.
- `README.md` and every `AGENTS.md` are indexes or digests, not rule dumps.
- Record an external-system bug in `_docs/reference/known-issues/` as `KI-<slug>.md`; that slug is
  the case id a suppression names, and the record carries the condition that retires it.
- Every fenced code block needs a language specifier; use `text` when none applies.
- A guide is steps, not an essay: imperative instruction, the command in a fence, and prose reserved
  for a decision, a hazard, or a non-obvious ordering constraint
  (`programming/spec-driven-docs/10-procedures.md`, ADR-guides-are-step-shaped).
- This root digest stays plain and frontmatter-free; per-directory digests carry frontmatter.

## Executable artifacts

A bucket may ship a working artifact — a schema, a script, a worked example — when a reader is
expected to copy it and run it. It is library content: it lives in the owning bucket beside the
chapter that explains it, never under `_docs/`. Shipping one carries three obligations, all in the
same change (ADR-executable-artifacts-in-the-library): a gate in `.pre-commit-config.yaml`, a case
under `just test`, and every tool it needs added to the `flake.nix` devShell.

A fenced code block is still right for illustration. The line is whether the reader is meant to copy
the thing and run it: if yes it is a file, because a fence cannot be validated and an unenforced
artifact reads as verified.

## No document narrates its own history

Non-negotiable: a file states what is true now. It never narrates how it got that way — no
`formerly`, no `used to`, no `this replaces`, no `inherited from`, no patch-series markers, and no
note explaining why something is absent. The test: delete the clause. If nothing a reader can act on
disappeared, it was archaeology.

Two homes are exempt, because holding history is their entire job: a decision record, frozen once
accepted, and a story's `Revisions` section. Git history holds the rest, and holds it better. This
binds every change, including the one that removes something: a deletion leaves no trace in prose,
only in the log.

## Filesystem state

Non-negotiable: the filesystem owns its own state, and no document indexes it. Forbidden is the
enumeration kept because the directory exists — a table of contents of the tree, a topic-to-filename
table, a `source-files` list. Naming files in prose is normal, and a list, table, or full tree is
welcome wherever its entries carry their own payload: what a directory reserves, what a file
specifies, its domain and scope. The test: strip the paths out and read what is left — if nothing
teaches, it was the listing. A layout a reader is told to create in their own project is a
specification, not a copy, and stays. `programming/spec-driven-docs/00-model.md` owns the full rule
and the heading that decides the judgment (ADR-filesystem-owns-disk-state).

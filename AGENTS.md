# AGENTS

Single source of truth for how `exobrain-tech` works. `README.md` indexes the content; this file
holds the rules. Keep it a digest — rules and pointers, never a rule dump.

## Glossary

- **MONOREPO** — the public `exobrain-tech` KB (this repo) plus the private
  `../exobrain-tech-vault`, together forming one logical knowledge base. References between the two
  are in-repo cross-references within the MONOREPO, not external dependencies, which is what keeps
  them compatible with Self-Containment below (ADR-0003).
- **`$EXOBRAIN_TECH`** — a checkout of the public KB (this repo). Docs and scripts point into it as
  `$EXOBRAIN_TECH/<path>`.
- **`$EXOBRAIN_TECH_VAULT`** — a checkout of the private vault (`exobrain-tech-vault`). Point into
  it as `$EXOBRAIN_TECH_VAULT/<path>`.

## What the product is

Non-negotiable: the product is the library — the knowledge itself. It lives in the eight top-level
buckets (`programming/`, `languages/`, `systems/`, `infra/`, `tools/`, `platforms/`, `workflows/`,
`data/`), exactly as a code project's product is its codebase, and may use whatever file and
directory structure best serves the knowledge within a bucket.

`_docs/` is **not** the product. It holds the metadata and specs _about_ the product — governance,
ADRs, conventions, architecture, guides, and explanation about how this knowledge base itself works
— organized by Diátaxis zone (`decisions/`, `guides/`, `reference/`, `explanation/`) and following
the docs-design method.

Never place a knowledge article under `_docs/`. Placement test: "Is this about how the KB works?" →
`_docs/`. "Is this knowledge the library serves?" → the owning bucket. Cross-link from `_docs/` only
when the product reference must mention it. See ADR-0004.

Public/private boundary: private equipment identity, security posture, recovery material,
credentials, and personal workflows belong in `exobrain-tech-vault`, never here.

## Self-containment

Non-negotiable: this project is self-contained. The knowledge it depends on is held in-repo. An
external reference is allowed only as a public link or citation for further reading — never as a
load-bearing dependency on a resource outside the repository, and in particular never on an
external, local, personalized, or mutating repository, path, or tool. If an external document, repo,
or personal path is required to understand, build, or operate this project, copy its essential
knowledge into the repository (a doc, an ADR, or an inline comment) so the repo stays complete on
its own. See ADR-0003.

## Decisions

Non-negotiable: record every significant, hard-to-reverse decision as an ADR under
`_docs/decisions/`, one decision per file, using the MADR-minimal `template.md` — at or below 350
words, exactly one `Status`, so the rationale lives with the work.

Valid statuses: `Proposed`, `Accepted`, `Implemented`, `Deprecated`, `Superseded`, `Rejected`.

Accepted decisions are never deleted, and never left to mislead: a replaced decision gets a new
superseding ADR, a decision whose context evaporated is marked `Deprecated`, and a decision changed
only in part keeps its status and gains an `Amended by ADR-NNNN` line under `Status`.

## Authoring

- Keep each fact in one source of truth and cross-link instead of duplicating.
- Drafts live in `.draft/`; promotion means rewriting into the right zone, not moving a file.
- `README.md` and every `AGENTS.md` are indexes or digests, not rule dumps.
- Use `_docs/reference/known-issues/` for external-system bugs.
- Every fenced code block needs a language specifier; use `text` when none applies.
- A guide is steps, not an essay: imperative instruction, the command in a fence, and prose reserved
  for a decision, a hazard, or a non-obvious ordering constraint.
  `programming/docs-design/10-guide-shape.md` owns the rule; ADR-0019 records the decision.
- This root digest stays plain and frontmatter-free; per-directory digests carry frontmatter.

## Executable artifacts

A bucket may ship a working artifact — a schema, a script, a worked example — when a reader is
expected to copy it and run it. It is library content: it lives in the owning bucket beside the
chapter that explains it, never under `_docs/`.

Shipping one carries three obligations, all in the same change (ADR-0006):

- a gate in `.pre-commit-config.yaml`,
- a case under `just test`,
- every tool it needs added to the `flake.nix` devShell.

A fenced code block is still right for illustration. The line is whether the reader is meant to copy
the thing and run it: if yes it is a file, because a fence cannot be validated and an unenforced
artifact reads as verified.

## No document narrates its own history

Non-negotiable: a file states what is true now. It never narrates how it got that way — no `formerly`, no
`used to`, no `this replaces`, no `inherited from`, no patch-series markers on the code a change
touched, and no note explaining why something is absent. A reader arriving today has no idea what
yesterday looked like, and telling them costs a sentence they cannot act on.

The test: delete the clause. If nothing a reader can act on disappeared, it was archaeology.

Two homes are exempt, because holding history is their entire job. A decision record says why one
option was chosen over others at the time, and is frozen once accepted. A story's `Revisions` section
records changes to the agreement made after that story's own work began — not where the story came
from. Git history holds the rest, and holds it better.

This binds every change, including the one that removes something: a deletion leaves no trace in
prose, only in the log.

## Filesystem state

Non-negotiable: the filesystem owns its own state, and no document indexes it. Forbidden is the
enumeration kept because the directory exists — a table of contents of the tree, a topic-to-filename
table, a `source-files` list. Naming files in prose is normal, and a list, table, or full tree is
welcome wherever its entries carry their own payload: what a directory reserves, what a file
specifies, its domain and scope. Justify it like any other content, not by the directory's existence.
The test: strip the paths out and read what is left — if nothing teaches, it was the listing. A
layout a reader is told to create in their own project is a specification, not a copy, and stays.
`programming/docs-design/00-foundations.md` owns the full rule; ADR-0007 records the decision.

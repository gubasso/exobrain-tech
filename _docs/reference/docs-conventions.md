# Documentation conventions

How `exobrain-tech` organizes and maintains the specs about its knowledge-base product. The goal is a
small set of homes with clear ownership: each fact has one durable source of truth, each document
serves one reader need, each decision stays short enough to review and supersede.

Sources: <https://diataxis.fr/> and
<https://www.writethedocs.org/guide/writing/docs-principles/>.

## Diataxis zones

Place a document by the reader need it serves:

- **`decisions/`** — why a choice was made. Lean ADRs.
- **`guides/`** — how to complete a task. Runbooks and procedures.
- **`reference/`** — exact facts for lookup. Conventions, schemas, diagnostics, case studies.
- **`explanation/`** — how a subsystem fits together. Architecture and background.

Zone comes first, topic second: `_docs/reference/<topic>/`, not `_docs/<topic>/reference.md`. A
top-level topic folder mixes reader needs and forces readers to infer intent from prose. If a
document needs two homes, choose the owner and link from the other.

## Lean ADRs

Architecture decisions live in `decisions/` as lean ADRs. A useful ADR names the problem, lists the
serious options, records the chosen option, states consequences, and declares status. Keep a filled
ADR body at or below 350 words; if it cannot fit, it is probably multiple decisions — split them.

Lifecycle: `Proposed → Accepted → Implemented → Superseded | Deprecated | Rejected`. **Never
delete an accepted decision — and never let one mislead.** Replaced wholesale → `Superseded`, link
the successor. No longer applicable with no successor → `Deprecated`, say why. Changed only in part
by a later ADR while the decision stands → keep the status and add an `Amended by ADR-NNNN — <what
changed>` line under `Status`, editing the old body only where it would actively mislead. Keep a
`Rejected` option when the rejection prevents repeated debate. Copy `decisions/template.md` to
start a new ADR.

## Single source of truth

Write each durable fact once at its owning home and link from everywhere else. Restatement drifts.
A README points to the ADR; a guide links to a reference page; a digest summarizes an area but the
area's files stay authoritative. Resolve a conflict by editing the non-owner to link to the owner.

Let the filesystem own its own state. The directory listing is the source of truth for what exists,
so no document keeps an index of it — not a tree of the directory, a topic-to-filename table, or a
frontmatter file list, whose objective is to mirror what is on disk and which is wrong the moment a
file moves. Naming files in prose is normal, and a list, table, or full tree belongs anywhere its
entries carry their own payload: what a directory reserves, what a file specifies, its scope. Strip
the paths out and read what is left — if nothing teaches, it was the index. A generated table of
contents of a page's own headings is not disk state and belongs to its generator.
`programming/docs-design/00-foundations.md` owns the rule; see
[ADR-0007](../decisions/ADR-0007-filesystem-owns-disk-state.md).

## Drafts

Drafts live outside the shipped tree, normally under a gitignored `.draft/`. Promotion rewrites a
draft into its durable zone and then deletes the draft. Split a mixed draft by reader need before
promoting. No draft is the only home for a real decision.

## AGENTS.md digests

Each substantial content area carries an `AGENTS.md` digest — a map of that area, loaded first,
derived from the area's files, never the source of truth. It declares a `last-synced` date so
staleness is visible, and is regenerated when the area's knowledge changes. It carries no inventory
of the directory. See
[AGENTS.md digest template](./agents-digest-template.md) and
[Knowledge-base architecture](../explanation/knowledge-base-architecture.md).

## Semantic names and stable headings

Filenames and headings are retrieval hints for humans and agents alike. Expose purpose before the
file is opened: `<topic>-diagnostics.md`, `<subsystem>-architecture.md`, `<task>-runbook.md`. Use
consistent vocabulary across files. Prefer direct headings (`## Placement rules`, `## Verification`)
over clever prose.

## Tracking perishable facts

A fact that expires — a price, benchmark, model roster, or external API shape — carries an owner, a
revalidation cadence, and a `last_checked` date in a small machine-readable registry. The tracked
artifact stays the source of truth; the registry keeps it from going stale.

---
digest-of: programming/docs-design
last-synced: 2026-08-05
source-files:
  - 00-overview.md
  - 01-diataxis-zones.md
  - 02-lean-adrs.md
  - 03-comments-and-code-as-sot.md
  - 04-single-source-of-truth.md
  - 05-drafts-and-promotion.md
  - 06-operational-docs.md
  - 07-ai-agent-considerations.md
  - 08-tracking-and-revalidation.md
  - 09-known-issues.md
  - 10-lean-markdown.md
  - 11-appetite-and-scope.md
  - 12-plan-and-slices.md
  - 99-checklist.md
  - README.md
  - template-adr.md
  - template-slice.md
token-estimate: 2010
---

# AGENTS

## Scope

Language-agnostic documentation design canon: Diataxis zones plus a plan zone, lean ADRs beside
living subsystem pages, load-bearing comments, single-source-of-truth placement, draft promotion,
operational docs, agent-aware maintenance, tracking and revalidation, a lean markdown house style,
appetite-bounded units of work filed one directory each, and a review checklist for documentation
changes.

## Key Points

### Overview (00)

- Default to four zones: decisions, guides, reference, explanation; add `plan/` when the project
  defines its work before building it.
- Use lean ADRs, never-delete lifecycle, SoT placement, and load-bearing comments.
- Apply the pattern when projects have durable docs, multiple readers, or LLM agents in the loop.
- Keep the root docs README as an index, not a junk drawer for durable rules.

### Diataxis Zones (01)

- Guides answer task questions; reference answers lookup questions.
- Explanation teaches mental models; decisions record why.
- Use zone-first placement, then topic directories inside the zone.
- Split documents that try to satisfy multiple reader needs.
- Diataxis organizes what exists; a second axis, `<project>/docs/plan/`, owns what is next —
  scope, milestones, open questions. It is the only zone whose contents are expected to go stale,
  which is why its documents carry statuses and may serve more than one reader need.
- One explanation page per subsystem (`<project>/docs/explanation/<subsystem>.md`) owns the current
  design: components, boundaries, constraints, and an `Unresolved` list. It links the ADR that owns
  each choice rather than deciding anything itself, and defers to code for exact behavior.
- The subsystem page is living and corrected in place; the ADRs it links are frozen historical
  records. Precedent: PEP 1 and the Rust RFC process.

### Lean ADRs (02)

- Write an ADR only for a choice that is cross-cutting, hard to reverse, constraining, or rejects
  a plausible alternative; a run of ADRs on one area means a subsystem page is missing.
- An ADR is a record, not a specification: it says why a choice was made at the time, is never
  rewritten to describe the present, and does not go stale by ageing.
- Use the MADR-minimal sections: context, options, outcome, consequences, status.
- Filled ADR bodies stay at or below 350 words.
- Lifecycle is `Ideation -> Proposed -> Accepted -> Implemented -> Superseded | Deprecated |
  Rejected`.
- `Ideation` is the cheap, non-binding entry state and the only status that may be deleted.
- `Superseded` has a successor; `Deprecated` has none (context evaporated).
- Never delete accepted decisions — and never let one mislead: a decision changed only in part
  keeps its status and gains an `Amended by ADR-NNNN — <what changed>` line under `Status`; edit
  the old body only where it would actively mislead.
- Supporting data belongs in reference or explanation, linked from the ADR.

### Comments and Code as SoT (03)

- Code owns behavior; comments own rationale code cannot express.
- Keep comments for boundary conditions, invariants, surprising constraints, and ADR links.
- Delete comments that narrate obvious code or replace them with better names and types.
- Use tests and types for behavior contracts where they can enforce the rule.

### Single Source of Truth (04)

- Avoid Repetition In Documentation: write once at the owning home, link elsewhere.
- Project-wide rules live in author instructions; decisions in ADRs; exact facts in reference.
- Structure is owned by the filesystem; index files explain purpose and never replicate the tree.
- Resolve conflicts by editing non-owner files to link to the owner.
- ADRs beat subsystem pages for why; subsystem pages beat ADRs for what the design is now; code
  beats prose for exact behavior.
- A digest is a map; source chapters and project docs own the guidance.

### Drafts and Promotion (05)

- Drafts live outside shipped docs, normally in `<project>/.draft/`.
- Provisional is not the same as forward-looking: a binding plan, scope decision, or
  open-questions register is project state and belongs in `<project>/docs/plan/`, in version
  control.
- Promotion rewrites the draft into the right zone and then deletes the draft.
- Split mixed drafts by reader need before promotion.
- Promote rejected paths as rejected ADRs only when the rejection has future value.

### Operational Docs (06)

- Runbooks and workflows are guides.
- Diagnostics and case studies are reference.
- Avoid top-level topic folders beside the Diataxis zones.
- Keep runbooks action-oriented and move exact signal interpretation to reference.

### AI Agent Considerations (07)

- Documentation bloat is context pollution.
- Semantic filenames and stable headings improve retrieval.
- Keep always-loaded author-instruction files under 200 lines, ideally 50–100.
- Give each unit of work one entry document that names the sources a session should load.
- `CLAUDE.md` or equivalent author instructions should include documentation maintenance rules.
- Agents should update docs for durable behavior, operations, or decisions, not every detail.
- Split a large author-instructions file along the eager/lazy seam: subtree-local rules go to a
  nested file (plus a `CLAUDE.md` `@AGENTS.md` bridge), cross-cutting rules stay in root, and root
  points rather than `@import`s.

### Tracking and Revalidation (08)

- A tracking file is a machine-readable registry of perishable artifacts and their re-check cadence.
- The tracked artifact stays the source of truth; the registry keeps it from going stale.
- Overdue scanning is deterministic tooling; revalidation is judgment and should surface uncertain
  drift.
- Primary audience is coding agents sweeping the repo; humans benefit too.

### Known Issues (09)

- Bugs in external systems under test are tracked as known-issue cases under
  `<project>/docs/reference/known-issues/` (reference zone), not a top-level topic folder.
- One case = one directory `KI-<NNNN>-<slug>/` (sequential id, keyed on the internal id, not the
  upstream bug); skeleton is README/issue.yaml/investigation/escalation/optional
  mask/evidence/notes.
- Lifecycle `open → investigating → mitigated | masked → monitoring → resolved`; on resolution the
  directory collapses to one `resolved/` summary (issue/root-cause/resolution/recurrence), raw trail
  in VCS history.
- Link code to a case with a marker/comment carrying the id; mask suppressions carry a revert
  trigger and checklist; a registry index + CI check keep references and directories in sync.

### Lean Markdown (10)

- Keep structural markdown (headings, lists, tables, fenced code with a language, inline code,
  links); drop decorative markdown (bold, italics, bold lead-ins, emphasis as a heading).
- Identifiers, paths, flags, and status values go in inline code; binding requirements go in
  uppercase RFC 8174 keywords, in normative documents only.
- The justification is signal discipline and readability, not tokens — emphasis is 1–2% of a
  corpus; restatement and long repeated link paths dominate.
- No formatter or linter can enforce it: dprint only picks the emphasis character, and
  markdownlint has no rule against inline emphasis. Use a fence-aware project hook with an
  `<!-- allow-emphasis: <reason> -->` escape hatch.

### Appetite and Scope (11)

- A slice is one vertical unit of work: end to end, bounded by an appetite, demonstrable when done.
  It is also the entry document an agent session loads.
- An appetite fixes time and varies scope; an estimate fixes scope and varies time. The appetite is
  chosen before the design and is a budget, not a forecast.
- Denominate the budget in what the project actually spends — calendar time, human review passes, or
  implementation sessions — and keep one unit per project.
- Each slice declares a non-negotiable core and a negotiable remainder. The remainder is the
  contingency that funds the guarantee; a core that fills the appetite means the slice must be split.
- When the appetite binds, cut the remainder, compare against baseline rather than the ideal, and
  never cut quality.
- The appetite may change only when both conditions hold: what remains is true core that survived
  scope hammering, and it is all downhill (no unsolved problems). Uphill remainder means re-shape,
  which is a new slice with a new appetite, not an extension.
- Every appetite change is a committed edit citing the condition met and what was cut first. Silent
  revision is the failure mode the rule exists to prevent.
- The pre-authorized escape per rabbit hole is this shelf's extension, not Shape Up's; fixed
  appetite with cuttable scope is DSDM's discipline as much as Basecamp's.
- Known limitation: a feature-shaped unit never spends an increment on an abstraction. The
  subsystem page absorbs design work, and a slice may be shaped around an abstraction when a named
  test at the new boundary makes it demonstrable.

### Plan and Slices (12)

- The plan zone holds `charter.md`, `milestones.md`, `open-questions.md`, and `slices/`.
  `milestones.md` is the single status surface and derives what it can from the slices.
- One directory per slice, always: `slices/<id>-<slug>/README.md` is the entry document, named so
  the human and agent entry points are the same file. The directory is the default because
  promoting a file to a directory later renames it and breaks every reference.
- `tasks.md` MUST NOT exist unless implementation crosses a context reset; `requirements.md` MUST
  NOT exist unless acceptance is many-to-many onto tests; `design.md` MUST NOT exist at all.
- Fixed heading list, pinnable with `MD043`: Goal, Appetite, Core, In scope, Out of scope, Governed
  by, Acceptance, Rabbit holes, Done when, Revisions.
- `Governed by` names individual sources and is the context filter; naming a directory is a defect.
- Acceptance lines use EARS (ubiquitous, state driven, event driven, optional feature, unwanted
  behaviour, complex) and name a test. Test naming is not part of EARS and requires a hook that
  fails on a missing test, or it should be dropped.
- The pre-production gate: until the current slice is implemented, no new specification page and no
  ADR outside it; questions go to the register.
- A slice is a bet, not a contract. Changing `Goal`, `Core`, `Appetite`, or `Acceptance` after work
  starts requires a committed edit adding a `Revisions` line. Cutting the remainder is not a
  revision.
- `open-questions.md` is triage: every entry names what it blocks and exits as an ADR, a slice
  revision, or a recorded measurement.

### Checklist (99)

- Review placement, ADR length and status, draft handling, cross-links, agent readiness, known-issue
  cases, and hook validation before merging documentation changes.
- The checklist is the pre-merge guard for both human and agent-authored doc edits.

### ADR Template

- Copy `template-adr.md` into `<project>/docs/decisions/template.md`.
- Keep each field brief and split separate decisions into separate ADRs.
- The template is copied into a project; the filled ADR becomes the project source of truth.

### Slice Template

- Copy `template-slice.md` into `<project>/docs/plan/slices/<id>-<slug>/README.md` and commit it
  before the work starts.
- Keep the heading list exactly as given so `MD043` can pin it.
- The template ships as one file on purpose; a sibling file is created only when its gate is met.

## Source Map

| Topic                                      | File                                         |
| ------------------------------------------ | -------------------------------------------- |
| Shelf purpose and defaults                 | `README.md`, `00-overview.md`                |
| Diataxis zones and placement               | `01-diataxis-zones.md`                       |
| Subsystem design pages                     | `01-diataxis-zones.md`                       |
| ADR format and lifecycle                   | `02-lean-adrs.md`, `template-adr.md`         |
| Comments and code rationale                | `03-comments-and-code-as-sot.md`             |
| Source-of-truth rules                      | `04-single-source-of-truth.md`               |
| Draft workflow                             | `05-drafts-and-promotion.md`                 |
| Operational docs                           | `06-operational-docs.md`                     |
| Agent considerations                       | `07-ai-agent-considerations.md`              |
| Tracking and revalidation                  | `08-tracking-and-revalidation.md`            |
| Known issues (external bugs)               | `09-known-issues.md`                         |
| Markdown house style                       | `10-lean-markdown.md`                        |
| Appetite and scope bounds                  | `11-appetite-and-scope.md`                   |
| Plan zone documents and the slice artifact | `12-plan-and-slices.md`, `template-slice.md` |
| Review checklist                           | `99-checklist.md`                            |

## Maintenance Notes

- Regenerate when any chapter file changes or a new chapter is added.
- Keep this digest derived from the listed source files; do not introduce new rules here.
- Agents should load this digest first, then read the source chapter that owns the current change.
- When the digest and a source chapter disagree, treat the source chapter as authoritative and
  regenerate the digest.
- Keep source-file ordering stable so context loaders can compare revisions predictably.

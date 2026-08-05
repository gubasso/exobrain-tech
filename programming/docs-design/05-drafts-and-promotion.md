# 05 — Drafts and Promotion

Drafts are useful while thinking is still messy. They are harmful when readers mistake them for
project state. Keep drafts outside shipped documentation until they are promoted into the right
Diataxis zone.

## Draft location

Use `<project>/.draft/` or an equivalent gitignored workspace for notes that are not yet canonical.
The directory can hold discovery notes, raw outlines, temporary checklists, copied issue text, and
half-shaped design arguments. It is a workshop, not a shelf.

Keep draft paths project-local. Do not put draft-only material in `<project>/docs/`, because docs
readers and agents treat that directory as durable. A draft in docs looks like a weak source of
truth. A draft outside docs is clearly provisional.

If the project has no gitignored draft location, create one before writing long-lived scratch
material. The exact name can vary, but the rule should be explicit in the author-instructions file.

Do not rely on filename warnings such as `draft-final.md` inside shipped docs. Path ownership is
clearer than prose labels. A reader should not need to open a file to know whether it is canonical.

## Plans are not drafts

Provisional is not the same as forward-looking. A document can describe work that has not happened
yet and still be project state: the project's scope and appetite, its ordered milestones, and its
register of open questions are commitments the team is currently working under, not exploration.
They belong in `<project>/docs/plan/` and in version control. See
[01 — Diataxis Zones](./01-diataxis-zones.md) for the zone.

The test is not "is this finished" but "is this binding". A half-formed argument about how a
subsystem might work is a draft. A statement that the next milestone is bounded at two weeks and
excludes three named features is project state, however early it is.

Keeping a plan in the gitignored workspace fails in a specific way. The plan is invisible to review,
so a scope change never appears in a diff. It is invisible to version control, so the record of what
the work taught you is lost with the working tree. And it is invisible to a fresh agent session,
which then reconstructs intent from the specification and the decision records instead — the two
artifacts that say what is true, not what is next.

If exploration and plan are tangled in one draft, split them: promote the binding part to the plan
zone and keep the rest in `.draft/` until it resolves.

## Promotion path

Promotion is a rewrite, not a move. Start with `.draft/<topic>.md`, identify the reader need, then
write the durable document in the proper zone. Delete the draft after promotion unless it contains
temporary research that intentionally remains local and ignored.

The typical path for a decision is:

1. Draft the exploration in `<project>/.draft/<topic>.md`.
2. Extract one decision into `<project>/docs/decisions/ADR-<number>-<topic>.md`.
3. Trim the filled ADR to the lean format from [02 — Lean ADRs](./02-lean-adrs.md).
4. Link any supporting reference or explanation pages.
5. Delete the promoted draft.

Do not preserve every sentence. Drafts contain uncertainty, repeated facts, and abandoned options.
The promoted document should contain only the durable result.

Promotion should also remove project-private context when the target is project-agnostic material.
Replace local people, hosts, package names, incidents, and workspace paths with placeholders. Keep
concrete public names only when they are necessary examples and do not make the pattern specific to
one project.

## Promotion targets

Promote to decisions when the draft records why the project chose one option over alternatives.

Promote to guides when the draft teaches a task sequence: setup, release, recovery, migration, or a
repeatable workflow.

Promote to reference when the draft contains lookup material: schemas, command matrices, field
tables, diagnostics, case studies, or exact examples.

Promote to explanation when the draft teaches a mental model: architecture, subsystem boundaries,
design forces, or conceptual background.

If one draft contains all four, split it. Use the zone map in
[01 — Diataxis Zones](./01-diataxis-zones.md) and the placement rules in
[04 — Single Source of Truth](./04-single-source-of-truth.md).

When the draft records a rejected path, decide whether the rejection is durable. If future
maintainers are likely to rediscover the same option, promote it as a rejected ADR. If the rejection
was only a local note with no future value, delete it with the draft.

## Anti-patterns

- Shipping `<project>/docs/drafts/` as a semi-official holding area.
- Moving a draft into docs without trimming it.
- Keeping both the draft and the promoted document with overlapping claims.
- Letting a draft become the only place a decision is recorded.
- Using drafts as permanent backlog items instead of filing the work in the plan zone.
- Keeping the milestone plan, the scope decision, the effort estimates, or the open-questions
  register in a gitignored workspace, where review and version control cannot reach them.

The danger is ambiguity. A reader should know whether a document is project state by looking at its
path. If a draft has value, promote it. If it does not, delete it.

Another anti-pattern is using drafts as a bypass around review. A large draft merged into docs at
the end of a project usually contains several decisions, procedures, and reference facts tangled
together. Promote incrementally instead.

Do not keep a promoted draft "just in case." If the durable doc lost important context, put that
context in the correct zone. If it did not, the draft is only another place for readers to confuse
with truth.

## Checklist

- [ ] Draft material stays outside `<project>/docs/`.
- [ ] Binding forward-looking material is in `<project>/docs/plan/`, not in the drafts workspace.
- [ ] The promotion target matches one reader need.
- [ ] Decision records use the lean ADR template.
- [ ] Supporting facts move to reference or explanation, not into the ADR body.
- [ ] The promoted document links to the source of truth for any reused fact.
- [ ] The draft is deleted or remains clearly local and ignored.

# 01 — Diataxis Zones

Diataxis organizes documentation by reader need. This matters because a single document cannot serve
every mode well. A reader doing a task wants steps. A reader debugging a field wants lookup
material. A reader learning a system wants explanation. A maintainer reviewing a design wants the
decision record.

Sources: <https://diataxis.fr/> and <https://diataxis.fr/how-to-use-diataxis/>.

## The four reader needs

| Need          | Reader question                                      | Documentation zone |
| ------------- | ---------------------------------------------------- | ------------------ |
| Task          | What do I do next?                                   | Guides             |
| Lookup        | What is the exact value, field, command, or symptom? | Reference          |
| Understanding | How does this area fit together?                     | Explanation        |
| Decision      | Why did the project choose this shape?               | Decisions          |

The zones are not content categories. They are reader promises. A guide promises sequence and
completion. A reference page promises stable lookup and precise facts. An explanation promises
context and trade-offs. A decision record promises the choice and its consequences.

When a document feels confused, identify the reader need it is trying to satisfy. If it has two
strong needs, split it and cross-link. A runbook with a long conceptual preface becomes easier to
use when the runbook stays in guides and links to an explanation page.

Treat the need as a mode, not as a reader identity. The same maintainer can be a learner in the
morning, a task-doer during a release, and a reviewer in the afternoon. The directory path should
tell that maintainer which mode the document supports before they read the first paragraph.

## Default docs layout

Use this table as the default placement map:

| Zone        | Default path                  | Reader need   |
| ----------- | ----------------------------- | ------------- |
| Decisions   | `<project>/docs/decisions/`   | Why           |
| Guides      | `<project>/docs/guides/`      | Task          |
| Reference   | `<project>/docs/reference/`   | Lookup        |
| Explanation | `<project>/docs/explanation/` | Understanding |

The project can add topic directories inside a zone when volume requires it:
`<project>/docs/guides/<topic>/`, `<project>/docs/reference/<topic>/`, and
`<project>/docs/explanation/<topic>/`. The topic comes after the reader need, not before it.

Avoid making `<topic>/` siblings of the zones. A top-level topic folder usually mixes guides,
reference, explanation, and decisions in one place. That asks every reader to open files and infer
intent from prose. The zone-first layout makes intent visible from the path.

The root docs README is only an index into these zones. It may list the zones, name the most common
entry points, and point to the project's author-instructions file. It should not become a fifth zone
for policies, workflows, or troubleshooting notes.

## The second axis: intent

Diataxis organizes what exists. All four reader needs are asked about a system that is already
there: how do I use it, what is its exact shape, how does it fit together, why is it like this. The
framework does not model intent over time, and it does not claim to.

A project that defines its artifacts before it implements them needs something the four zones cannot
hold: what is being built next, what bounds it, and what is still undecided. That material is not a
task, not lookup, not a mental model, and not a settled decision. Without a home it falls into the
gitignored drafts workspace, where the plan that actually drives the work becomes invisible to
review, to version control, and to a fresh agent session.

Add one zone on a second axis:

| Zone | Default path           | Reader question                                |
| ---- | ---------------------- | ---------------------------------------------- |
| Plan | `<project>/docs/plan/` | What are we building next, and what bounds it? |

The plan zone holds the project's scope and appetite, its ordered milestones, and its register of
open questions. It does not hold reference tables, task instructions, or conceptual background; when
a plan document starts explaining a subsystem, that explanation belongs in `explanation/` and the
plan links to it.

The plan zone is the only zone whose contents are expected to become false. A guide that stops being
true is a bug; a milestone that stops being current is the normal course of the work. That is why
plan documents carry statuses and the other four zones do not, and why a plan document may serve
intent, scope, and status at once instead of one reader need.

Work that is finished leaves the plan zone the way any fact does: the durable result moves to the
zone that owns it, and the plan keeps only the pointer.

## Placement rules

Put a document in `decisions/` when it records a choice the project may later revisit. It should be
a lean ADR: short context, serious options, chosen outcome, consequences, and status. See
[02 — Lean ADRs](./02-lean-adrs.md).

Put a document in `guides/` when it tells the reader how to complete a task. A guide can include
prerequisites and verification, but it should not become the canonical home for every field or error
code it mentions. Link to reference material instead.

Put a document in `reference/` when readers need exact facts: schemas, option tables, diagnostic
signals, API shapes, command matrices, case studies, and known symptoms. Reference pages are
organized for lookup, not narrative flow.

Put a document in `explanation/` when readers need the mental model: architecture overview,
subsystem responsibilities, design forces, trade-offs, and conceptual background. Explanation is
allowed to be discursive, but it should link to ADRs instead of restating their decisions.

Operational material follows the same map. Runbooks are guides. Diagnostics and case studies are
reference. See [06 — Operational Docs](./06-operational-docs.md).

## Boundary tests

Ask these questions before creating or moving a file:

- If the reader is actively trying to finish work, it belongs in guides.
- If the reader is comparing exact values, fields, statuses, or symptoms, it belongs in reference.
- If the reader is trying to understand a subsystem, it belongs in explanation.
- If the reader is asking why a choice was made, it belongs in decisions.
- If the reader is asking what is being built next or what bounds it, it belongs in plan.
- If the document needs multiple homes, choose the owner and link from the others.

The last rule is the most important. Cross-links are cheap. Duplicate facts are expensive. A guide
can say "for accepted status values, see reference" and keep moving. A reference page can say "for
why this status exists, see ADR-<number>" and avoid retelling history.

When a page is hard to place, write its intended first sentence as a user need: "I need to deploy
`<feature>`," "I need the exact status values," "I need to understand the subsystem," or "I need to
know why this exists." The sentence usually names the zone.

## Anti-patterns

- Topic-first top level: `<project>/docs/<topic>/runbook.md`,
  `<project>/docs/<topic>/decision.md`, and `<project>/docs/<topic>/overview.md` force readers to
  inspect content to determine purpose.
- README as junk drawer: a root docs README should be an index, not the canonical home for
  decisions, procedures, and troubleshooting.
- Guide as encyclopedia: task docs should link to lookup pages instead of embedding complete
  reference tables.
- ADR as tutorial: a decision record does not teach the whole system. It records the choice.
- Explanation as policy: project-wide rules belong in the author-instructions file or an ADR, not
  only in background prose.
- Case study as runbook: a past event can inform a runbook, but the case study belongs in reference
  and the repeatable procedure belongs in guides.
- Decision hidden in guide: if a guide says "we always do X because Y," the durable why belongs in
  an ADR and the guide should link to it.
- Plan zone as a fifth topic folder: `plan/` is a second axis, not another place to file reference
  or explanation material. If it holds anything a reader would consult after the work ships, that
  content belongs in one of the four reader-need zones.

## Sources

- Diataxis: <https://diataxis.fr/>
- How to use Diataxis: <https://diataxis.fr/how-to-use-diataxis/>

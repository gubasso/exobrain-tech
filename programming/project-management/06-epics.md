# 06 — Epics

Work above three points splits, so a feature worth building is almost never one story. The pieces
then carry their own goals and none carries the goal they share. An epic is where that shared end
state lives: one document, and one field on the entries that serve it.

## The membership field

An entry may carry `epic`, naming a document at `<project>/docs/plan/epics/<id>-<slug>.md`.

```yaml
- id: "013"
  slug: oauth-hardening
  type: story
  points: 3
  needs: ["009"]
  epic: "014"
  tags: [security]
```

The epic document is named the way a story file is, and draws from the same id sequence. One id
names one thing anywhere in the zone: `check-plan` rejects an epic whose id is already a story
entry, and rejects two documents claiming one id. So a three-digit token in any field resolves to
exactly one artifact, and renaming an epic's slug never touches the entries that serve it.

The two lines below the dependency are not the same kind of thing. A tag is a free label and
nothing gates it, so a typo makes a second label nobody notices. An epic is a gated reference:
`check-plan` verifies the document exists, and a mistyped id fails the record.

Membership runs one way, from the entry to the document. The document names no members, because a
member list is an index of the record and would go stale on the next entry that joins or leaves;
[00 — Foundations](../docs-design/00-foundations.md) owns that rule. The cost is real and worth
stating: to see one epic's members you search the lane files for its id. That search is always
right, and a list would only sometimes be.

An epic has no parent and names no epic. Nothing enforces that, because there is nothing to
enforce — an epic has no lane entry, so there is no field in which a second level could be written.

## Membership does not sequence

An epic is not a dependency. `needs` keeps sole ownership of what waits for what, so two members of
one epic may be independently eligible, and one may close a month before another opens. Epics never
enter the ranking rules in [03 — The Plan Record](./03-the-plan-record.md), and `--rank-fix` never
groups an epic's members together — an order the machine chose by kinship would be priority, and
the machine owns legality only.

That leaves grouping to the reader's view rather than the record's order, which is where it
belongs; see [04 — Gating the Plan](./04-gating-the-plan.md).

One entry serves one end state. A story that seems to belong to two epics is usually a story that
was split along the wrong seam, and occasionally a story whose second membership was only ever a
label. Split it again, or make the second one a tag.

## An epic may precede its stories

An epic with no members yet is legal, and it is the normal way one starts: write the end state,
then decompose it. It is a first specification, and the decomposition is a later act.

It belongs in the plan zone rather than the drafts workspace even at that stage. The test in
[05 — Drafts and Promotion](../docs-design/05-drafts-and-promotion.md) is not whether a document is
finished but whether it is binding, and an end state the project intends to reach is binding
whether or not a story has been written against it.

The cost of allowing this is that `epics/` can accumulate intent nobody is pursuing, and no gate
will say so. A directory of end states the project has quietly abandoned is worse than no directory,
so retiring one is a review responsibility rather than a linter's.

## Completion is derived

An epic is closed out when every entry carrying its id sits in `closed.yml`. Nothing stores that.
There is no status field on an epic, and closing a story writes nothing extra.

Closed out is not the same as done. A `cut` member closes without delivering, so an epic can reach
the derived state with its goal unmet. That is why the document carries `Done when`: the record
knows when the work stopped, and only the document knows what was supposed to become true.

Two consequences follow, and both are deliberate. An epic whose every member is closed is a finished
epic, never a stale reference, so nothing reports it. And a member that closes as `reshaped` carries
its successor in `succeeded_by`, but that successor may serve a different epic or none — no gate
catches the drift, because re-scoping is exactly what reshaping is for.

## The epic document

Every heading is present, including an empty `Amends` or `Revisions`. The title line is
`# <id> — <short title>`, and the shape imitates the story's so a reader learns one grammar. The
project applies the drop-in gate from [the heading-shape template](./template-heading-shapes.md).

```text
Goal          the end state several stories reach, as one observable outcome
Example       that end state shown concretely; a simulated transcript is fine
Core          the guarantee no member story may cut
Out of scope  explicit no-gos binding every member
Governed by   individual sources every member session must load
Amends        specs this end state expects to change, or None
Done when     the objective condition that makes the end state true
Revisions     changes to the agreement after the first member opened
```

Four story headings are deliberately absent. `In scope` orders a remainder to cut against a point
budget, and an epic has no budget — its remainder is which stories exist. `Acceptance`, `Tasks`, and
`Rabbit holes` are obligations on a work session, and no session implements an epic. An epic that
grows them has become a story above three points that was never split.

`Amends` stays, and is held to the same check a story's is: every path it names must exist. An epic
declares the specs its end state expects to change, and its members carry the individual assertions;
see [05 — Specs and Stories](./05-specs-and-stories.md).

Write the example from the reader's side, the way
[02 — The Story on Disk](./02-the-story-on-disk.md) requires of a story. The end state has not been
built yet, so the transcript is a simulation — say what happens today and what will happen instead.
A simulated transcript is still concrete; a description of one is not.

## When to open one

Open an epic when a split produced pieces whose shared end state is invisible from any one of them.
That is the whole test. Two stories rarely qualify, and a tag is usually enough to find them again.

Anti-patterns:

- An epic that is a renamed story: it has one member and always will.
- A container opened before the split, holding a goal that would have fit in three points.
- An epic carrying acceptance criteria, which makes every member's own acceptance ambiguous.
- An epic per release or per quarter, which is a tag wearing a document.

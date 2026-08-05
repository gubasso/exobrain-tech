# 02 — Lean ADRs

Architecture decision records preserve why a project chose one path over serious alternatives. They
are not tutorials, meeting notes, or design novels. A useful ADR is short, statused, and durable
enough that future maintainers can trust it.

## When a choice earns an ADR

Write an ADR when at least one of these holds:

- The choice is cross-cutting: more than one area has to know about it.
- It is expensive to reverse once code depends on it.
- It constrains future work, so later decisions must be made inside it.
- It rejects a plausible alternative that someone will otherwise propose again.

Everything else belongs where the work is planned, or in the code and its tests. A choice that is
local, obvious, and fully expressed by a type name or a function signature is not a decision record;
see [00 — Overview](./00-overview.md).

This threshold exists because the other rules in this chapter, applied without it, reward document
count. The word cap says a record that will not fit should be split, and splitting is usually
correct. But leanness enforced per file is unbounded in aggregate: a subsystem can be designed in
350-word installments across a dozen records, each individually clean, with no single artifact
stating what the subsystem is. A reader then has to reconstruct the design from the decisions, which
is the work the design document was supposed to have done.

The signal to watch is a run of consecutive ADRs about one area. When that appears, the missing
artifact is a short design document that the decisions hang off, not another decision.

## Default template

Use the drop-in [ADR template](./template-adr.md). A filled ADR has five sections:

- `Context and Problem Statement`: the problem and why it matters.
- `Considered Options`: the serious alternatives, not every idea mentioned in chat.
- `Decision Outcome`: the chosen option and the shortest honest reason.
- `Consequences`: positive and negative trade-offs.
- `Status`: one canonical lifecycle value.

This is a MADR-minimal shape: enough structure to compare decisions consistently, not enough
structure to invite filler. MADR itself is broader; use only the parts that force clarity. See
<https://adr.github.io/madr/> and <https://ozimmer.ch/practices/2022/11/22/MADRTemplatePrimer.html>.

The template is a format contract. Do not add project-specific sections unless the project has an
active, repeated need for them. If a field is consistently empty, remove the field from the local
template or stop creating ADRs for that class of choice.

Name ADR files so they sort and search well: `ADR-<number>-<decision>.md` or the local equivalent.
The number gives stable references in comments, reviews, and follow-up ADRs. The short decision slug
gives humans and agents enough context from filenames alone. Avoid status or date-only names; status
changes, and dates rarely describe the decision.

The title should name the choice, not the implementation task. Prefer "Use `<system>` for artifact
storage" over "Implement storage support." The first title records a decision. The second sounds
like a ticket.

## Word cap

Filled ADR bodies must be at or below 350 words. The cap is a forcing function:

- One page is reviewable in code review.
- Short records are easy for LLM agents to load and compare.
- A hard limit exposes decisions that should be split.
- The record stays focused on the durable choice, not the whole discovery trail.

If the ADR needs more room, first remove narrative detail. If it still needs more room, split the
decision. A project can link multiple ADRs in a small series: one for storage format, one for API
shape, one for migration behavior. That is better than one large record whose status and
consequences blur together.

The word cap applies to the filled decision body, not to surrounding index pages or templates.
Diagrams, large option matrices, or benchmark data belong in reference or explanation pages linked
from the ADR.

The cap also prevents ADRs from becoming hidden specifications. If a decision requires exact API
fields, command examples, or migration steps, put those in reference or guides and link them. The
ADR should say why those artifacts exist and which option they enact.

## Lifecycle

Use this lifecycle:

`Ideation -> Proposed -> Accepted -> Implemented -> Superseded | Deprecated | Rejected`

`Ideation` means the record is a half-formed idea captured while it is fresh. It is not binding, it
is not ready for review, and it may be incomplete or wrong. `Proposed` means the record is ready for
review but not yet binding. `Accepted` means the project has chosen the direction. `Implemented`
means the code, docs, or operations now enact the choice.
`Superseded` means a later ADR replaced it. `Deprecated` means the decision no longer applies and
no successor exists — the context evaporated (a feature was dropped, a dependency vanished) rather
than a replacement being chosen. `Rejected` means the project explicitly decided not to take that
path.

Every implemented ADR should link to the code, configuration, or documentation that enacts it when
there is a stable target. Every superseded ADR should link to the successor. Every deprecated ADR
should say why it stopped applying. Every rejected ADR should explain enough that the same option
is not reopened without new evidence.

### Why the lifecycle needs a cheap entry

`Ideation` exists to make the vocabulary usable. A lifecycle whose lowest state already means "ready
for review" has no low-commitment entry, so every idea either waits outside the record entirely or
enters as a decision. The observable symptom is a corpus that is almost entirely `Accepted`, with no
`Proposed` and no `Rejected` — not because the project never hesitated, but because hesitation had
nowhere to live. Oxide's RFD process solves this with explicit prediscussion and ideation states and
the rule that notes are encouraged to be timely rather than polished; see
<https://rfd.shared.oxide.computer/rfd/0001>.

`Ideation` is the one status that may be deleted. A record that never left it never became project
state, so the never-delete rule below does not apply to it. That is what makes the state cheap
enough to use: entering costs nothing, and leaving costs nothing. Once a record moves to `Proposed`
or beyond, the never-delete rule takes over permanently.

Keep an ideation record short and honest about its own uncertainty. If it cannot yet name the
options, it is a draft and belongs in the drafts workspace instead; see
[05 — Drafts and Promotion](./05-drafts-and-promotion.md).

Status changes are edits to history, not rewrites of history. When moving from `Accepted` to
`Implemented`, keep the original context and outcome intact. Add the implementation link. When
moving to `Superseded`, add a short pointer to the successor and stop there. The successor ADR owns
the new reasoning.

### Amendments: partial change without supersession

A later ADR often changes one aspect of a decision that otherwise stands — renames a command the
old ADR describes, closes an item the old ADR deferred — without reversing it. Marking the old ADR
`Superseded` would be false (the decision holds), but leaving it untouched misleads: a reader or
agent loading it follows stale details as if current.

Handle this with an amendment pointer: keep the status, and add a line under `## Status` in the
form `Amended by ADR-NNNN — <one line: what changed>`. Edit the old body only where its wording
would actively mislead; never rewrite it to pretend the later design was the original choice. The
amending ADR owns the new reasoning and should name what it amends. `Amended by` is an annotation
on a status, not a status value — the status vocabulary stays closed.

## Never delete

Do not delete accepted, implemented, superseded, or rejected ADRs. They are part of project history.
Deleting them destroys context and makes old reviews, commits, and comments harder to understand.

Deletion is appropriate for drafts and for `Ideation` records, neither of which ever became project
state. Once an ADR reaches `Proposed`, change its status instead of removing it. If the title was
misleading, keep the file and clarify the decision outcome. If the decision was wrong, supersede it.
If the context evaporated with no successor, deprecate it. If the project backed away before
implementation, reject it. If a later ADR changed only part of it, add an amendment pointer (see the
amendments section above).

Never-delete does not mean never-correct — and it does not mean never-mislead. Fix typos, broken
links, and misleading wording. Correct metadata that was wrong when written. Do not edit an old ADR
to pretend a later design was the original choice. If the meaning changed, write a new ADR and
change the status or add an amendment pointer; an old ADR that reads as current when it is not is a
trap for every future reader and agent.

This mirrors the precedent of durable proposal systems: Rust RFCs
<https://rust-lang.github.io/rfcs/0002-rfc-process.html>, Python PEPs
<https://peps.python.org/pep-0001/>, and Kubernetes KEPs
<https://github.com/kubernetes/enhancements/blob/master/keps/sig-architecture/0000-kep-process/README.md>.
Those systems preserve decision history because later readers need the trail, not just the latest
state.

## Comparisons

Nygard's original ADR format is intentionally small: title, status, context, decision, consequences.
It can often land around 200 words when the decision is narrow. Source:
<https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions>.

MADR-minimal is slightly more structured. It names considered options and keeps consequences
visible, often around 250 words for a well-scoped decision. That extra structure helps reviewers see
whether the project considered real alternatives.

Y-Statements compress the choice into one sentence with named parts: in the context of a problem,
facing forces, the project decided for one option and neglected others to achieve qualities while
accepting downsides. A Y-Statement can be around 40 words. Use that form for quick summaries, but
prefer the lean ADR template when the project needs durable review history.

The practical default is the lean ADR. It is short enough for review, structured enough for future
readers, and explicit enough for agents to classify.

Use Y-Statements inside summaries, release notes, or ADR indexes when a one-line decision reminder
is enough. Do not replace the ADR with a Y-Statement when the project needs status, consequences, or
links to implementation. Use Nygard's smallest shape when the project has no need to list options.
Use MADR-minimal when review should show what was seriously considered.

## Status values

Use exactly one status value per ADR:

- `Ideation`: captured while fresh; not binding, not yet ready for review, and deletable.
- `Proposed`: open for review.
- `Accepted`: chosen, not necessarily implemented.
- `Implemented`: enacted by the project.
- `Deprecated`: no longer applies; no successor exists.
- `Superseded`: replaced by a later ADR.
- `Rejected`: explicitly not chosen.

Do not invent status synonyms such as `Done`, `Canceled`, or `Obsolete`. Synonyms make filtering
and agent reasoning harder. If the project needs more detail, add one sentence after the status and
link to the successor or implementation. `Amended by ADR-NNNN` is such an annotation, not a status:
the record keeps its status value and gains the pointer.

Use status as data. Reviewers, scripts, and agents should be able to grep for `Status` and classify
the decision without reading the whole file. If the status line needs explanation every time, the
status vocabulary is too large or the decision is doing too much.

## Sources

- Nygard ADR article: <https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions>
- MADR: <https://adr.github.io/madr/>
- MADR primer: <https://ozimmer.ch/practices/2022/11/22/MADRTemplatePrimer.html>
- Rust RFC process: <https://rust-lang.github.io/rfcs/0002-rfc-process.html>
- Python PEP process: <https://peps.python.org/pep-0001/>
- Kubernetes KEP process:
  <https://github.com/kubernetes/enhancements/blob/master/keps/sig-architecture/0000-kep-process/README.md>

See [04 — Single Source of Truth](./04-single-source-of-truth.md) for where ADR facts should be
linked instead of repeated.

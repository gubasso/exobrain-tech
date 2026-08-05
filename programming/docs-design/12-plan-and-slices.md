# 12 — Plan and Slices

[01 — Diataxis Zones](./01-diataxis-zones.md) adds a plan zone for what is next.
[11 — Appetite and Scope](./11-appetite-and-scope.md) says what bounds a unit of work inside it. This
chapter says what documents the zone holds and what shape the unit of work takes on disk.

The shape here is deliberately small. Every artifact it adds has to survive one question: what does a
reader or an agent session lose if this file does not exist? Anything that fails that question is
ceremony, and ceremony is indistinguishable from diligence until you count.

## The plan zone's documents

```text
<project>/docs/plan/
  charter.md         What the project is for, its pillars, and its no-gos. Two pages at most.
                     Written once, revised per release, never per slice.
  milestones.md      The ordered slices, one row each, with a status. The single status surface:
                     a reader consults this and nothing else to know where the work stands.
  open-questions.md  Questions that could change a decision, and what each one blocks.
  slices/            One directory per unit of work.
```

`charter.md` and `milestones.md` are the only documents in the zone a reader consults without a
slice in hand. Keep both short enough that they are read rather than skimmed.

`milestones.md` SHOULD derive whatever it can from the slices themselves — a task count, a status —
so it cannot silently disagree with them. A status surface maintained by hand is a status surface
that goes stale between the moment the work changes and the moment someone remembers.

## One directory per slice

A slice is one directory, always, whatever its size:

```text
<project>/docs/plan/slices/<id>-<slug>/
  README.md         Always. The entry document.
  tasks.md          Only when implementation will cross a context reset.
  requirements.md   Only when acceptance maps many-to-many onto tests.
```

The directory is the default rather than an escalation because escalation has a cost of its own. A
slice that starts as `slices/<id>-<slug>.md` and later needs a second file must be renamed, and the
rename breaks every reference that already points at it — the milestone row, the open questions, the
commit messages. That happens exactly when the work is already in trouble, which is when a
restructuring step is least likely to be taken. A directory that grows a file costs nothing and
breaks nothing.

`README.md` is the entry document. The name is deliberate: forges render it when a reader opens the
directory, so the human entry point and the agent entry point are the same file.

The additional files are gated, and the gates are prohibitions rather than permissions:

- `tasks.md` MUST NOT exist unless implementation will cross a context reset. Its job is durable
  state across a boundary that erases working memory. Inside a single session the ordered work is
  already in the session.
- `requirements.md` MUST NOT exist unless acceptance maps many-to-many onto tests, or spans more
  than one test lane, so a flat list in `README.md` can no longer be audited by reading it.
- `design.md` MUST NOT exist. Current subsystem design is owned by the explanation zone; see
  [01 — Diataxis Zones](./01-diataxis-zones.md).

State them this way because a slice directory looks like an invitation to fill it. Tools trained on
other spec-driven workflows will produce a full set of files unprompted, and a positive rule ("add
`tasks.md` when you need it") does not survive that pull as well as a negative one.

The signal that the gates have failed is countable rather than felt: a run of slices whose
`tasks.md` restates what `README.md` already said. Look for it during review, not per slice — one
slice never looks like ceremony from the inside.

## The slice document

`README.md` has a fixed heading list, which a project SHOULD pin with markdownlint's
`MD043 required-heading-structure` so it cannot drift into a second charter:

```text
Goal          One sentence naming the observable outcome. Not the mechanism.
Appetite      The fixed budget, in the project's chosen unit. See chapter 11.
Core          The non-negotiable outcome this slice guarantees. Never cut.
In scope      The negotiable remainder, ordered so the least valuable is cut first.
Out of scope  Explicit no-gos for this slice.
Governed by   Every source a session must load, named individually.
Acceptance    One line per assertion, each naming the test that proves it.
Rabbit holes  Known traps, each with a pre-authorized escape.
Done when     The objective completion condition.
Revisions     One line per change to this document after the work started.
```

`Core` and `In scope` are separate headings because chapter 11 requires the non-negotiable outcome
to be declared apart from the remainder that funds it. Collapsing them into one scope list is the
`Everything is core` anti-pattern with better formatting.

`Out of scope` does more work than its length suggests. It is the only heading that constrains by
subtraction, and it is what stops a session from helpfully building the adjacent thing.

Use the drop-in [slice template](./template-slice.md).

## Governed by is the context filter

An agent session loads `README.md` and the sources `Governed by` names. Nothing else.

This is the concrete form of the one-entry-document rule in
[07 — AI Agent Considerations](./07-ai-agent-considerations.md), and the reason it works is that it
names sources instead of describing them. "See the specification" loads a corpus. "Governed by
`docs/explanation/auth.md` and ADR-0031" loads two files.

It is also the shelf's answer to a general principle: context is a finite budget, and the way to
spend it well is to carry lightweight identifiers and resolve them at load time rather than
preloading everything that might matter. See
<https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents>.

A `Governed by` entry that names a whole directory is a defect. If the session needs the directory,
the slice is too large; if it needs three files in it, name the three.

## Acceptance lines and EARS

Each `Acceptance` line states one assertion and names the test that proves it:

```text
When a client exceeds 5 login attempts in 60 seconds, the system shall respond 429.
  -> test_rate_limit_429
```

The assertions are phrased in EARS, the Easy Approach to Requirements Syntax — a constrained
template for requirements, first published in 2009 by Alistair Mavin and colleagues at Rolls-Royce,
and used at Airbus, Bosch, Dyson, Honeywell, Intel, NASA, Rolls-Royce and Siemens. Five patterns
plus their combination:

| Pattern            | Template                                                                     |
| ------------------ | ---------------------------------------------------------------------------- |
| Ubiquitous         | The `<system>` shall `<response>`.                                           |
| State driven       | While `<precondition>`, the `<system>` shall `<response>`.                   |
| Event driven       | When `<trigger>`, the `<system>` shall `<response>`.                         |
| Optional feature   | Where `<feature is included>`, the `<system>` shall `<response>`.            |
| Unwanted behaviour | If `<trigger>`, then the `<system>` shall `<response>`.                      |
| Complex            | While `<precondition>`, when `<trigger>`, the `<system>` shall `<response>`. |

The value is that the opening keyword declares which kind of requirement follows before the content
is read, and that the unwanted-behaviour pattern gives error cases a named slot instead of leaving
them to be remembered. It is the same discipline as the uppercase keywords in
[10 — Lean Markdown](./10-lean-markdown.md), one level up: normativity carried by a fixed vocabulary
rather than by tone.

Two honest limits. EARS constrains phrasing, not correctness — a precisely phrased wrong
requirement passes. And the requirement-to-test naming above is not part of EARS; the official
notation says nothing about testability. It is borrowed from safety-critical traceability practice,
and it carries that practice's failure mode: a named test that does not exist yet is a forward
reference that goes stale silently when a test is renamed.

So a project that names tests in acceptance lines MUST also carry a hook that fails when a named
test cannot be found. Without the hook, drop the naming and keep the assertions — an unenforced
reference is worse than none, because it reads as verified.

## The pre-production gate

One line in the project's author-instructions file:

```text
Until the current slice is implemented, do not add a specification page and do not open an ADR
outside that slice. A question that arises goes to docs/plan/open-questions.md.
```

This is the circuit breaker on documentation itself. Without it a project specifies indefinitely,
because specifying is cheaper than building and always feels productive. It is worth more than any
choice about file layout.

The gate binds agents in particular. A session that hits an undecided question will decide it, write
the record, and continue — producing a decision nobody made.

## A slice is a bet, not a contract

The slice document is committed before the work starts. Committed at the end it is a report;
committed at the start it is the thing later commits are read against.

It is not frozen by being committed. Google's design-doc practice is explicit that a document should
be updated when what it describes has not shipped yet, which is exactly a slice's condition for its
whole life; see <https://www.industrialempathy.com/posts/design-docs-at-google/>. A plan that cannot
absorb what implementation taught is a plan that quietly stops being read.

The revision rule is the same one chapter 11 applies to the appetite. A change to `Goal`, `Core`,
`Appetite`, or `Acceptance` after the work has started MUST be a committed edit that adds one line
under `Revisions` naming what changed and what was learned that changed it.

That keeps both properties at once. The plan can move, so it stays true; every movement is in a
diff, so a project that routinely rewrites its slices mid-flight can see that it does, and can go
look at how it shapes work. An unrecorded revision is the only kind that teaches nothing.

Cutting the remainder is not a revision. Cutting is the designed response to a binding appetite and
needs no ceremony beyond the commit that does it.

## Open questions are triage

`open-questions.md` is where a session puts a question it must not answer alone. It is a triage
desk, not a queue.

Every entry names what it blocks, and every entry leaves by one of three exits: it becomes an ADR,
it becomes a line in the slice under `Revisions`, or it is recorded as measured and closed. An entry
with no exit and no blocking relationship is a note, and notes belong in the drafts workspace; see
[05 — Drafts and Promotion](./05-drafts-and-promotion.md).

The failure mode this guards against is the register becoming a place discovery goes to be filed
rather than acted on — a plan that always wins because the thing that would have changed it was
moved to another document. If questions accumulate faster than they exit, the shaping is wrong, not
the register.

## Anti-patterns

- Slice as second charter: project-wide context restated in a unit of work. That is `charter.md`.
- Slice as design document: how a subsystem is built, written where what to build belongs. That is
  the explanation zone.
- Slice as decision record: a choice made in the slice instead of an ADR, so it is invisible to
  every future reader who does not read this slice.
- Directory filled by default: `tasks.md` and `requirements.md` created because the shape suggested
  them.
- `Governed by` naming a directory, a whole zone, or "the docs".
- Acceptance lines with no test named, or naming tests no hook verifies.
- Rabbit holes listed without escapes, which records a risk without doing anything about it.
- Silent revision: `Goal` or `Core` changes and `Revisions` does not.
- Open questions as an archive: entries that name nothing they block and never exit.
- Milestones maintained by hand until they disagree with the slices they summarize.

## Sources

- Shape Up, Write the Pitch (problem, appetite, solution, rabbit holes, no-gos):
  <https://basecamp.com/shapeup/1.5-chapter-06>
- EARS, official notation reference: <https://alistairmavin.com/ears/>
- Design docs at Google (write before code; update while unshipped):
  <https://www.industrialempathy.com/posts/design-docs-at-google/>
- Effective context engineering for AI agents (just-in-time context):
  <https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents>
- Amazon, working backwards (write the launch document first):
  <https://www.allthingsdistributed.com/2006/11/working_backwards.html>

## See also

- [01 — Diataxis Zones](./01-diataxis-zones.md) for the plan zone and for subsystem pages.
- [11 — Appetite and Scope](./11-appetite-and-scope.md) for what bounds a slice.
- [template-slice.md](./template-slice.md) for the drop-in entry document.
- [99 — Checklist](./99-checklist.md) for the pre-merge review gate.

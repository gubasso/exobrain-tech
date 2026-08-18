# 10 — Guide Shape

A guide promises sequence and completion, and [01 — Diataxis Zones](./01-diataxis-zones.md) places a
document on that promise. [09 — Procedure Artifacts](./09-procedure-artifacts.md) names what crosses
a phase boundary. Neither says what a step looks like. This chapter owns the body: how large a step
is, what voice it uses, where the command sits, and how much prose the page may spend.

## Problem

A guide written as an essay makes the reader parse prose to find the command. The reader is not
studying; they are executing, usually under pressure, and every sentence between them and the next
line to type is a sentence they must read to discover it was not the line to type.

Two habits produce it, and both look like helpfulness.

The first is narration: a sentence introducing the step, the command, then a sentence describing
what just happened. The introduction restates the heading and the description restates the
command's own output. Neither survives the test of deletion.

The second is inlined rationale: the concept behind a step explained at the step. It reads as
thorough and it is a placement failure — the explanation belongs to whatever owns that fact, per
[00 — Foundations](./00-foundations.md), and the guide links it. A guide that explains its own
concepts becomes the third copy of them and drifts from both.

Length is the symptom, not the disease. A guide is not improved by compressing its sentences; it is
improved by having fewer of them because the structural constructs carry what the sentences carried.

## The step

A step is one action a reader performs and can confirm.

- Number steps when order binds. Use a list only when the items are genuinely parallel.
- Write the instruction in the imperative: `Create the project`, not `You will now want to create
  the project` and not `The project is created by`.
- Put anything the reader types in a fenced block with a language. A command inside a sentence is
  the most common form of this chapter's problem.
- Follow the command with at most one sentence, and only when the reason is not visible from the
  command. Put it after, so a reader who does not need it has already moved on.
- A step needing a paragraph is two steps, or one step plus a link to the page that owns the
  concept.
- One command per block where a reader runs them one at a time and checks between. Group commands
  only when they are pasted together.

The test for a sentence in a guide: delete it and ask whether the reader can still perform the step.
If they can, it was narration.

## The prose budget

Prose is not banned; it is reserved. A guide spends it on three things.

- A decision the reader must make, where the guide cannot choose for them. State the options and the
  discriminator.
- A hazard that costs real time, stated where the reader is about to hit it rather than in a
  troubleshooting section they reach afterwards.
- An ordering constraint that the steps do not make obvious, such as a step that must land in the
  same commit as another.

Everywhere else the structural construct wins: a table for a mapping or a comparison, a list for
parallel items, a fenced block for anything typed or emitted. The constructs are in
[08 — Lean Markdown](./08-lean-markdown.md), which also bars the emphasis that essay-shaped guides
lean on to mark the parts a reader actually needs.

## Preconditions and verification

A guide states what must be true before step one and how the reader knows they are done. Both are
part of the procedure.

- Preconditions open the guide as a short list: tools installed, access held, state assumed. Each is
  checkable, and one that has a command gets the command.
- Verification closes it as a step, with the command that proves the outcome and what a correct
  result looks like.
- A guide whose verification is "it should work now" has not been finished.

A guide with phases also names what each phase consumes and produces; that shape is owned by
[09 — Procedure Artifacts](./09-procedure-artifacts.md).

## Length

The heading list is the length signal, not the line count. A reader scanning the headings should see
the whole procedure without scrolling. When it no longer fits, the page is carrying more than one
task, or it is carrying reference material a lookup page should own.

Split by the same test used for placement: what the reader is doing stays in the guide, what they
are looking up moves to reference, and why it is so moves to explanation or a decision record.

## Anti-patterns

- Guide as encyclopedia: a complete option table pasted into a task page instead of linked.
- Guide as essay: the command reachable only by reading the paragraph around it.
- Narrated step: a sentence announcing the command and another describing its output.
- Inlined rationale: the concept explained at the step rather than linked to its owner.
- Buried command: the thing to type sitting in prose instead of a fenced block.
- Verification by assertion: a closing claim that the procedure worked, with no command that shows
  it.

# 11 — Procedure Artifacts

A guide promises sequence and completion, and [01 — Diataxis Zones](./01-diataxis-zones.md) places it
on that promise without saying how its body is built. This chapter gives a multi-phase guide four
devices — an artifact token, an outputs block, an inputs line, and recipe form — so every value the
procedure carries has a named producer and a named consumer. It governs the guides zone only; a page
with no phases has no boundaries to label.

## Problem

A long procedure accumulates values: a passphrase generated in one phase, a key id issued by a vendor
in another, a file written in a third. By the phase that consumes them they arrive as a configuration
block asking for four things at once, and the reader scans back through paragraphs to find which one
minted each. A reader who cannot locate the producer guesses, and a procedure that is guessable is not
a procedure.

Realistic-looking placeholders make it worse. A field shown as `key_id: 002abc4e91` reads as a value to
copy rather than a slot to fill, and a reader working through a recovery at speed will copy it.

The fix is not more prose. It is naming each value once, and stating at every phase boundary what
crosses it.

## The artifact token

An artifact is anything a phase produces that something later needs: a string, a passphrase, a file, a
directory, a name the project has to agree on, a record written to paper. Name each one once, as an
upper-snake token in angle brackets.

```text
<BACKUP_PASSPHRASE>
```

- Characters are `A` to `Z`, `0` to `9`, and `_`, and the first character is a letter.
- The token names the artifact, never the step that made it. `<BUCKET_KEY_ID>`, not `<STEP_3_OUTPUT>`.
- One token means one artifact across the whole docs tree.
- A token never carries a real or realistic value. A reader who pastes it gets an obvious error rather
  than a plausible wrong one.
- In prose the token is inline code. Inside a fenced block it is written bare.

Case is the whole discriminator against the other angle-bracket name this shelf uses. A `placeholder`
is lowercase and stands in for anything project-specific — `<project>`, `<host>`, `<subsystem>` — and
belongs to [10 — Lean Markdown](./10-lean-markdown.md). An artifact token is upper-snake and asserts
more: that some phase produces this, and that the guide says which. A project already spending
upper-snake angle names on something else picks a different delimiter for one of the two and records
the choice, because the two meanings cannot share a spelling.

## The outputs block

Every phase that produces an artifact ends with one: an introductory line, then a fenced `text` block
carrying one line per artifact as `<TOKEN> — what it is; where it is kept`.

````text
Outputs of this phase:

```text
<BACKUP_PASSPHRASE> — the repository passphrase; password manager and paper record
<BUCKET_KEY_ID> — the key id the vendor issues; password manager
```
````

- It is the last element of its section, so a reader skimming to the next heading still passes it.
- It is never a heading. Headings are the link namespace, and a derived list does not earn a target on
  every phase.
- It names the artifact and says where it is kept. It does not define it; the definition stays with
  whatever owns that fact, per [00 — Foundations](./00-foundations.md).
- A phase that carries nothing forward says so on one line and stops, as `Outputs: none` plus the
  reason. Silence is ambiguous, and reads as an omission.

## The inputs line

One plain line directly under the phase heading, before the first step.

```text
Inputs: `<BACKUP_PASSPHRASE>` (§4), `<BUCKET_KEY_ID>` (§3).
```

- Tokens are inline code, and each names its producer: a section reference on the same page, an
  explicit relative link when the producer is another guide.
- A phase that consumes nothing writes `Inputs: none.`
- The line is the reader's re-entry point. Someone resuming the procedure a day later reads one line to
  learn what must already be in hand.

Naming producers exposes the guide's real dependency graph, which is half the value. An inputs line
that has to cite a later section is an ordering defect the prose was hiding; reorder the guide, or
state the exception and cross-link both directions so neither phase is a surprise.

## Recipe form

The three devices only pay off if the phase body between them is scannable.

- Each phase is a numbered list, one action per item, with sub-steps nested one level.
- A command sits in a fenced block under its step, never inside a sentence.
- Prose describing what a command does is replaced by the command with a trailing comment.
- Narrative explaining why belongs in the explanation zone, linked once.

A how-to is directions through a sequence of actions toward a goal rather than a description of a
system (<https://diataxis.fr/how-to-guides/>), and the paragraph that describes what the reader is
about to do is usually the paragraph an inputs line and an outputs block replace.

## Where it applies

- A guide with more than one phase where at least one artifact crosses a phase boundary. This is the
  case the chapter exists for.
- Not a single-phase guide. One outputs block with nothing downstream of it is ceremony.
- Not reference, explanation, or decision records. They have no phases, and a reference page that lists
  artifacts is the inventory, which already owns them by name.
- Not a guide written to be read with no access to the rest of the tree, such as a printed recovery
  procedure. It cannot carry an inputs line that points off-page, so it carries one requirements list
  at the top instead, and the exemption is recorded with the project's other local exceptions.

## Enforcement

Nothing in the usual toolchain checks this. An outputs block is an ordinary fenced `text` block, so a
fence-language rule covers its declaration and nothing more, and `MD043` gates a fixed heading list
while a guide is deliberately not a fixed-shape document; see
[10 — Lean Markdown](./10-lean-markdown.md).

What a project-local hook can check, once the corpus has settled, is one relation: every upper-snake
token appearing anywhere in the tree also appears in some outputs block. Two legitimate cases fail that
check and have to be tolerated rather than repaired.

- An artifact whose consumer is outside the docs — a password-manager entry, a paper record, a
  configuration option in code. It is produced and never consumed on a page.
- A phase documented before it is built, whose block carries the not-yet clause in place of a delivered
  artifact.

Until such a hook exists the review gate is a reviewer, through [99 — Checklist](./99-checklist.md).

## Anti-patterns

- Realistic placeholder: a slot filled with something shaped like a real value, which readers copy
  instead of replacing.
- Step-named token: `<STEP_3_OUTPUT>` encodes the procedure's current shape, so it is wrong the moment
  a step is inserted.
- Outputs heading: promoting the block to `## Outputs` puts a derived list in the link namespace and
  invites inbound links that later phases cannot move.
- Second definition: an outputs block explaining what the artifact is rather than naming it, leaving
  two places to change.
- Notation everywhere: tokens scattered through reference and explanation pages, where nothing produces
  or consumes them.

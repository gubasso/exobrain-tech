# 02 — The Story on Disk

What shape does one story take, and how is it written? One Markdown file carries the conversation;
an optional same-name directory carries only non-narrative artifacts.

## Layout

```text
<project>/docs/plan/stories/
  007-rate-limit.md       the story; always
  007-rate-limit/         traces, fixtures, or diagram sources; only when needed
```

The sibling directory never contains a second narrative document. Durable behavior belongs in a
reference spec, current structure in an explanation page, and a hard-to-reverse choice in an ADR.
The story file never moves when artifacts appear, and record references use its stable id.

## Fixed shape

Every heading is present, including an empty `Tasks` or `Revisions` section. Empty means the
question was considered and there is nothing to record. `Amends` is the exception: it names specs
or the literal `None`, never nothing.

```text
Goal          one observable outcome, not the mechanism
Example       that outcome shown as a transcript, request and response, or before and after
Core          the guarantee that is never cut
In scope      negotiable remainder, least valuable last
Out of scope  explicit no-gos
Governed by   individual sources the work session must load
Amends        specs the story must leave changed, or None
Acceptance    assertions and the tests that prove them
Tasks         decomposition as a GFM checklist
Rabbit holes  known traps with pre-authorized escapes
Done when     objective completion condition
Revisions     changes to the agreement after work began
```

The project applies the drop-in heading gate from
[the heading-shape template](./template-heading-shapes.md). `check-plan` additionally requires a
fenced block under `Example` for `story` and `spike`; it is optional for `chore`.

An epic borrows this grammar for the end state a set of stories reaches, minus the headings that
obligate a work session; see [06 — Epics](./06-epics.md).

## Write the difference

Lean is placement, not length. A sentence still needed after the story closes belongs to its
durable owner, and the story states only what changes.

Cite the claim, not coordinates. State what a source establishes in the carrying sentence, then
link to a stable heading anchor. Never make comprehension depend on opening a file or counting
lines.

Show the outcome from the reader's side. Use concrete values and real output, and say what happens
now when the example describes future behavior. A description of a transcript is not a transcript.

````markdown
## Example

The fourth request in one minute is rejected; today it succeeds.

```console
$ for n in 1 2 3 4; do request /search; done
200 200 200 429
```
````

Leanness and example quality require review. The fence gate proves only that an example-shaped
answer exists.

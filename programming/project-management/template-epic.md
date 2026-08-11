# Template — epic

Copy the block into `<project>/docs/plan/epics/<id>-<slug>.md`, then carry `epic: "<id>"` on the
lane entry of every story that serves it. Take the id from the same sequence stories use: one id
names one thing, so an epic never reuses a story's. Keep the heading list exact; the drop-in
[heading-shape gate](./template-heading-shapes.md) checks it. The document names no member stories
— membership runs from the entry to the document, never back.

An epic may exist before any story does. Writing the end state first is the normal order.

````markdown
# <id> — <short title>

## Goal

<One sentence naming the end state several stories reach together, not the mechanism and not any
one story's outcome.>

## Example

<Show the end state with a transcript, request and response, or before and after. It has not been
built, so this is a simulation: say what happens today and what will happen instead.>

```text
<Concrete invocation and output.>
```

## Core

<The guarantee no member story may cut. If the members can deliver everything else and still fail
this, it belongs here.>

## Out of scope

- <Explicit no-go binding every member.>

## Governed by

- `<path/to/owning/document.md#anchor>` — <the claim this source establishes for every member.>

## Amends

<`None`, or one spec path per line with the assertion this end state expects to change. Every path
must exist; the member stories carry the individual assertions.>

## Done when

<The objective condition that makes the end state true. Every member closing is not this condition
— a cut member closes without delivering.>

## Revisions

<One line per change to the agreement after the first member opened, or `None`.>
````

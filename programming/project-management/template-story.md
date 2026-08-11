# Template — story

Copy the block into `<project>/docs/plan/stories/<id>-<slug>.md` before work begins. Keep the
heading list exact; the drop-in [heading-shape gate](./template-heading-shapes.md) checks it. Place
traces, fixtures, diagram sources, or other non-narrative artifacts in an optional sibling named
`<id>-<slug>/`.

<!-- dprint-ignore-start -->
<!-- dprint's markdown plugin moves a task-list marker off `- [ ] <placeholder>` and onto a later
     unchecked item; see the known issue on task-list markers before removing this fence. -->

````markdown
# <id> — <short title>

## Goal

<One sentence naming the observable outcome, not the mechanism.>

## Example

<Show the outcome with a transcript, request and response, or before and after. Say what happens
today and what will happen. Use at least one fenced block for a story or spike.>

```text
<Concrete invocation and output.>
```

## Core

<The non-negotiable outcome. Never cut it; split the story if it cannot fit at three points.>

## In scope

- <Negotiable item, most valuable first.>
- <Negotiable item, cut first.>

## Out of scope

- <Explicit no-go.>

## Governed by

- `<path/to/owning/document.md#anchor>` — <the claim this source establishes.>

## Amends

<`None`, or one list item per amended spec, each opening with the spec path in inline code followed
by the assertion this story must leave changed.>

## Acceptance

- <Assertion> — `<test name>`.

## Tasks

- [ ] <Implementation task.>

## Rabbit holes

- <Known trap> — escape: <the pre-authorized response.>

## Done when

<The named tests pass unskipped and amended specs state the accepted behavior.>

## Revisions

<One line per change to the agreement after work began, or `None`.>
````

<!-- dprint-ignore-end -->

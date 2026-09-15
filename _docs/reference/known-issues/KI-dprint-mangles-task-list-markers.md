---
upstream: https://github.com/dprint/dprint-plugin-markdown/issues
affects: every document pairing a GFM checklist with an angle placeholder
state: masked
filing: ready
workaround: wrap the block in a dprint-ignore range
retire_when: a markdown plugin newer than 0.22.0 formats the reproduction below unchanged
checked: 2026-09-12
---

# dprint markdown moves a task-list marker off an angle placeholder

`dprint` markdown plugin `0.22.0`, as pinned in [`dprint.json`](../../../dprint.json).

## Symptom

A checklist item whose text begins with an angle placeholder loses its `[ ]` marker, and the marker
reappears on the next unchecked list item in the document — including one under a different heading.
The result is stable under repeated formatting, so the corruption survives review as if it were the
authored content.

## How it works

Three parts, and only three: the `- [ ]` marker, the angle placeholder `<...>` that opens the item's
text, and the plugin's list-item rewriter. The bug is the rewriter reading the placeholder as the
thing it must not touch, and moving the marker out of its way.

Run one pass over the block below and watch where the marker sits after each step.

Step 1, the input. Item one is checked-shaped and starts with a placeholder; the item under `## H`
is a plain unchecked item.

Step 2, `dprint fmt`. The rewriter strips `[ ]` from item one, because a placeholder opening the
text makes the marker read as content rather than as a marker.

Step 3, the same pass reattaches the marker to the next unchecked list item it finds — across the
heading boundary, onto the item under `## H`.

Step 4, a second `dprint fmt` changes nothing. The output is a fixed point, which is why the
corruption survives review: it looks authored.

## Reproduction

<!-- dprint-ignore-start -->
<!-- The bug rewrites its own reproduction: without this range the input block below is formatted
     into a copy of the output block, and the case reads as though nothing happened. -->

```markdown
- [ ] <Implementation task.>

## H

- <Known trap> — escape: <x.>
```

One `dprint fmt` pass rewrites that to:

```markdown
- <Implementation task.>

## H

- [ ] <Known trap> — escape: <x.>
```

<!-- dprint-ignore-end -->

The trigger is the placeholder, not the fence: the same input inside a fenced `markdown` block is
rewritten the same way, while replacing `<Implementation task.>` with plain prose formats cleanly.

## Where it bites

Any document pairing `- [ ]` with an angle placeholder: a template whose checklist items are still
unfilled, and any copy of one that a reader has not completed. Every formatting pass moves the marker
onto the item below it. A completed checklist has no placeholders left, so the trigger needs both
halves together. No document in this repository carries the pair today.

## Workaround

Wrap the affected block in `<!-- dprint-ignore-start -->` and `<!-- dprint-ignore-end -->`. The
range is honored, the authored markers survive, and the exclusion is narrow enough that the rest of
the file stays formatted. Any template that pairs `- [ ]` with an angle placeholder needs the same
guard until the plugin is fixed.

## Exit

Drop the guard once a newer markdown plugin formats the reproduction above unchanged.

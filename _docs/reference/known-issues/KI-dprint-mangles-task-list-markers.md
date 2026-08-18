---
upstream: https://github.com/dprint/dprint-plugin-markdown/issues
affects: every document pairing a GFM checklist with an angle placeholder
state: masked
workaround: wrap the block in a dprint-ignore range
retire_when: a markdown plugin newer than 0.22.0 formats the reproduction below unchanged
---

# dprint markdown moves a task-list marker off an angle placeholder

`dprint` markdown plugin `0.22.0`, as pinned in [`dprint.json`](../../../dprint.json).

## Symptom

A checklist item whose text begins with an angle placeholder loses its `[ ]` marker, and the marker
reappears on the next unchecked list item in the document — including one under a different heading.
The result is stable under repeated formatting, so the corruption survives review as if it were the
authored content.

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

Any document pairing `- [ ]` with an angle placeholder. In this repository that is
`_docs/plan/stories/*.md`, whose `Tasks` section is a GFM checklist, and every pass moves the marker
onto the `Rabbit holes` item below it — contradicting the fixed shape the
[story headings reference](https://github.com/gubasso/plan-xp/blob/develop/docs/reference/story-headings.md)
requires and the `MD043` hook gates.

## Workaround

Wrap the affected block in `<!-- dprint-ignore-start -->` and `<!-- dprint-ignore-end -->`. The
range is honored, the authored markers survive, and the exclusion is narrow enough that the rest of
the file stays formatted. Any template that pairs `- [ ]` with an angle placeholder needs the same
guard until the plugin is fixed.

## Exit

Drop the guard once a newer markdown plugin formats the reproduction above unchanged.

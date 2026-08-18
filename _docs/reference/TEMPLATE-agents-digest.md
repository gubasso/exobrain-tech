# AGENTS.md digest template

Copy this file into a content area as its `AGENTS.md`. A digest is a map of the area — loaded first
by an agent before it reads the underlying files — and never the source of truth. The content files
own the knowledge; this digest summarizes it. Regenerate it when the area's knowledge changes; when
the digest and a source disagree, the source wins.

The digest carries no index of the directory. The filesystem owns what exists, and a checked-in file
list drifts on the next add or rename; see
[ADR-filesystem-owns-disk-state](../decisions/ADR-filesystem-owns-disk-state.md). Name a file when you have something
to say about it, and say that thing — a list or table earns its place by what its entries teach, not
by the directory having contents.

Where an area's shape is itself load-bearing, add a `## Directory domains` section: one entry per
part of the tree stating what it reserves and what belongs there. That is knowledge the listing
cannot supply, so it is a section about the area, not an inventory of it.

Keep the frontmatter accurate: `digest-of` is the area path, `last-synced` is the date you last
reconciled the digest with the area, and `token-estimate` is a rough size.

```markdown
---
digest-of: <path/to/this/area>
last-synced: <YYYY-MM-DD>
token-estimate: <approx tokens>
---

# AGENTS

## Scope

<One or two sentences: what knowledge this area holds, and what it does not.>

## Key points

- <Load-bearing fact from the area, one bullet per idea.>
- <Another fact. Name a file only when you have something to say about it.>

## Maintenance notes

- Regenerate when the area's knowledge changes.
- Keep this digest derived from the sources; do not introduce new rules here.
- Load this digest first, then read the file that owns the current change.
```

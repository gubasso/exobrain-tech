# Bugzilla comment syntax

> <https://bugzilla.readthedocs.io/>

A Bugzilla comment is plain text. Nothing is rendered: `**bold**` shows as asterisks, `## Head` shows
as hashes, and a `|` table shows as pipes with its columns wandering. Every convention below has to
work as literal monospace text, and the good ones also survive if the instance happens to have
Bugzilla's optional Markdown mode enabled — an author cannot tell from the outside which one they
are writing into, so nothing may depend on rendering.

## What Bugzilla interprets

- URLs autolink.
- Bug references autolink: `bug NNNNNN`, and on SUSE instances `bsc#NNNNNN`, `boo#NNNNNN`,
  `bnc#NNNNNN`.
- In-bug references autolink: `comment #N`, `attachment #N`.
- A leading `>` marks quoted text and is styled as a quote.

That is the whole list. Everything else is decoration the author supplies and the reader interprets.

## Headings

One level: all-caps over a dash underline of the same length.

```text
CODE PATH
---------
```

The underline is the only decoration that wins both ways. In monospace it is an unmistakable rule
under the title, and under a Markdown renderer `TEXT` plus `---` is a setext H2, so it upgrades
instead of breaking. A `#`-prefixed heading and a `== Wiki ==` heading both degrade into noise.

Reserve `=====` banner rules for copy-paste framing — a summary line to lift into the Summary field,
a do-not-paste field block — so a reader tells operator instructions from report content at a glance.
That distinction is worth more than a second heading level, which is why `===` is not the level-one
rule even though the setext ladder would put it there.

## Sub-headings

Stop at two levels, and make the second a bare title-case line with no underline. The contrast
between `ALL-CAPS` over a rule and title case over nothing is the hierarchy; the plain line reads as
subordinate precisely because it carries no decoration.

Each alternative fails for a concrete reason:

- Another underline character: `~~~` opens a code fence in CommonMark, `***` is a horizontal rule,
  and `^^^` or `...` are conventions nobody shares. Every invented character costs a lookup.
- Indentation: it signals subordination well in monospace, but four spaces is a literal block — the
  device the body already uses for source and log excerpts — so a sub-heading becomes
  indistinguishable from a snippet.
- RFC numbering (`1.`, `1.1`): the only convention that scales past two levels, which is the tell.
  Needing `1.1` means writing a document, not a bug comment.

Depth is what makes plain text unreadable, because no font weight or size carries the levels — only
decoration invented on the spot. Where a section wants substructure, numbered steps or a run-in
lead-in beat a heading; where it genuinely needs three levels, attach a file and point at it.

## Blocks, tables, and lists

- Literal blocks: indent four spaces. Bugzilla preserves the whitespace, and the same indent is a
  code block under a Markdown renderer. This is how source excerpts and log lines are shown.
- Tables: aligned columns under an ASCII rule of dashes and spaces. A `|`-delimited Markdown table
  renders as pipes and misaligns as soon as a cell grows.
- Lists: `*` or `-` with a hanging indent lining the continuation up under the text.
- Emphasis: none. Word order carries it, and an occasional all-caps word is the only emphasis that
  reads as deliberate rather than as leftover markup.
- Wrapping: hard-wrap at a fixed width. Bugzilla does not reflow, so an unwrapped paragraph becomes
  one very long line in a narrow browser column.

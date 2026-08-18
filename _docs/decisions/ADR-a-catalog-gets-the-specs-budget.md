# A catalog gets the spec's budget

## Context and Problem Statement

`programming/spec-driven-docs/06-format.md` capped every chapter at 200 lines, and nothing checked
it. Wiring the check exposed the flaw the cap had all along: some of the shelf's documents hold one
entry per rule — the gate list, the checklist, the glossary, the shelf index — so their length is a
function of how many rules exist. A fixed chapter cap on those punishes the corpus for growing, and
the gate list broke it as soon as three subjects landed.

## Considered Options

- Hold every document to 200 lines and evict content when one breaks.
- Exempt the one-entry-per-rule documents from the cap entirely.
- Give them the spec's number, 300 lines, as a distinct artifact class.

## Decision Outcome

Chosen option: a distinct class at 300 lines, named a catalog. A spec already carries that number
for the same reason — it is also one entry per rule — so the class is the existing reasoning applied
where it already applied. The principle is the one the spec cap states and the size gate's
TOC-stripping already encodes: a budget bounds authored content, so what is derived from the corpus
is measured differently from what an author chose to say.

Exempting them outright was rejected because an unbounded document is one nobody notices growing.
`08-gates.md` matches a catalog by filename rather than by inspecting it, because no command can tell
an argument from an inventory.

## Consequences

- Good: the size gate can ship green over the current corpus, rather than landing with its own
  exceptions already suppressed.
- Good: a catalog now owes something concrete — one entry per rule, no argument — which is the rule
  that keeps it from becoming a chapter.
- Bad: it is a second number to remember, and the boundary is a judgment made once per file.
- Bad: 300 lines of catalog is still 300 lines of context when the file is loaded.

## Status

Accepted.

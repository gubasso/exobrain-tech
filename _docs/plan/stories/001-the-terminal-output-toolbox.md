# 001 — The Terminal Output Toolbox

## Goal

A program that has to draw a chart, a board, or a graph in a terminal picks from a measured
catalogue instead of repeating the survey.

## Example

The chapter answers what each tool draws, what it costs as a closure, and whether it survives a
pipe — measured by running it, not by reading its README.

```console
$ rg -n 'Closure' programming/cli-design/13-terminal-output-toolbox.md | head -2
14:**Closure size**, via `nix path-info -S` on a `buildEnv` of the candidate set, not on each tool
147:Combined as one environment: 71 MiB for all three, because most of each is glibc.
```

## Core

The measurements, the method that produced them, and the verdict for each tool. Dated evidence, not
a package index.

## In scope

- The three tests that decided every row: closure measured as a set, pipe safety run rather than
  read, and output quality on one real graph.
- What a shell already does unaided: sparklines, eighth-cell bars, half-block rows, and the ladder.
- The glyph tiers and the padding trap that decides when shell-native layout is correct at all.
- The catalogue: adopted, documented, and rejected, each with the reason.
- The naming traps in nixpkgs that would otherwise waste an afternoon.

## Out of scope

- Any renderer. This chapter is a survey; the program that draws is elsewhere.
- Keeping the version numbers current. They are dated evidence with a revalidation trigger.

## Governed by

- `_docs/decisions/ADR-docs-vs-library-boundary.md` — the placement test that sends this
  chapter to a content bucket rather than to `_docs/`: its subject is CLI design, not this
  repository.
- `programming/cli-design/README.md` — the shelf this chapter joins, and the numbering it continues.
- `programming/spec-driven-docs/06-format.md` — the markdown rules every chapter here is held to.

## Amends

- `programming/cli-design/13-terminal-output-toolbox.md` — new: the whole survey.
- `programming/cli-design/README.md` — the index gains its row.

## Acceptance

- The chapter states its measurement date and what revalidates a row — review.
- Every adopted tool carries a closure figure measured as a set — review.
- Relative links and markdown checks pass — `just lint`.

## Tasks

- [x] Measure the candidates and write the survey.
- [x] Land it in the cli-design bucket and add the index row.

## Rabbit holes

- Turning the survey into a renderer because the measurements suggested one — escape: the renderer
  belongs to the program that needs it, and it is a story in that program's record.

## Done when

The chapter is in the bucket, the index names it, and a program choosing a terminal renderer can
start from a measurement rather than from a search.

## Revisions

None.

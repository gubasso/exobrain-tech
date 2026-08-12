# ADR-0008: Separate documentation design from project management

## Context and Problem Statement

The former documentation-design shelf mixed durable documentation design with perishable intent and
work execution, although its own second-axis diagram exposed the seam. Keeping both domains together
blurred ownership and made planning support appear to be documentation-design machinery.

## Considered Options

- Retain one shelf.
- Split the shelves while cross-linking their seam.
- Split and simultaneously redesign planning around XP.

## Decision Outcome

Chosen option: **split the shelves while cross-linking their seam** — the structural boundary is useful
now, while an XP-oriented planning redesign is separate work and remains deferred.

`programming/docs-design/` owns durable documentation design;
`programming/project-management/` owns planning and executing work. The Diataxis chapter retains the
Plan row, boundary test, and anti-pattern, while the full second-axis argument moves to project
management chapter 00. Documentation-design chapters 08–11 become 06–09; appetite and plan/slices
become project-management chapters 01 and 02.

The principal reference direction is project management to documentation design. A slice cites its
governing specs and ADRs; completion promotes durable results to subsystem pages, ADRs, and reference.
Documentation design carries concise seam pointers only. Executable plan artifacts and their gates move
to `programming/project-management/plan/` without behavior changes.

## Consequences

- Good: each shelf has a coherent subject and support surface while planned work and durable artifacts
  remain explicitly integrated.
- Bad: the structural move changes published paths and requires broad link and hook migration.
- Neutral: the current appetite method, milestones prose, lane record, schema, and linter remain as-is.

## Status

Superseded by [ADR-0017](./ADR-0017-the-planning-method-moves-to-plan-xp.md) — the method and its artifacts now live at [plan-xp](https://github.com/gubasso/plan-xp).

Amended by [ADR-0009](./ADR-0009-plan-record-is-lane-files.md),
[ADR-0010](./ADR-0010-the-unit-is-a-story-in-one-file.md),
[ADR-0011](./ADR-0011-a-story-is-a-diff-a-spec-is-a-state.md), and
[ADR-0012](./ADR-0012-the-plan-linter-may-write-the-record.md) — the deferred XP redesign landed, so
the slice unit, the appetite method, and chapters 01 and 02 named above are superseded by stories,
points, the lane-file record, and the writing linter. The shelf split itself is unchanged.

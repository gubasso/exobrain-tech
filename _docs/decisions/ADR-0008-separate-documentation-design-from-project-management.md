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

Implemented by the [documentation-design shelf](../../programming/docs-design/README.md), the
[project-management shelf](../../programming/project-management/README.md), and the
[plan artifacts](../../programming/project-management/plan/).

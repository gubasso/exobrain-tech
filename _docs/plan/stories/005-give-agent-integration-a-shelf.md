# 005 — Give Agent Integration a Shelf of Its Own

## Goal

A reader who asks how a project is built to work with a coding agent finds one shelf that answers
it, and every fact in that answer has one owner.

## Example

One chapter carries the whole domain, at almost five times the cap the library sets for a chapter.

```console
$ wc -l programming/cli-design/05-designing-for-llm-agents.md
958 programming/cli-design/05-designing-for-llm-agents.md
```

Half of it is CLI design, which is what a chapter under `cli-design/` holds. The other half is
skills, instruction files, distribution roots, and evaluations, which belong to no shelf today.

## Core

`programming/agent-integration/` owns the domain. The oversized chapter is cut into it, chapter by
chapter, and what stays behind is CLI design. A fact moves once: where the new chapter would restate
a rule a `cli-design/` chapter already owns, it links that owner instead.

## In scope

- The shelf, with the chapters carved out of `05-designing-for-llm-agents.md`.
- `05` reduced to the CLI half, with its links re-pointed at the chapter that now owns each fact.
- The eval material in `09-testing-and-quality/regression-safeguards.md`, which the shelf takes.
- The shelf's hub and digest, and every link in the library that reached into `05`.

## Out of scope

- The vendor shelves, which a later story splits by vendor.
- The portable skill standard, which a later story writes from the specification and the adapters.
- Any chapter another repository's plan lands here.

## Governed by

- `_docs/specs/SPEC-knowledge-base-boundary.md` — what belongs in a bucket and what does not.
- `_docs/specs/SPEC-docs-format.md` — the register and the size budgets every chapter here meets.
- `AGENTS.md` — the product boundary, and the rule that each fact has one source of truth.

## Amends

- `programming/agent-integration/` — the shelf and its chapters.
- `programming/cli-design/05-designing-for-llm-agents.md` — reduced to CLI design.
- `programming/cli-design/09-testing-and-quality/regression-safeguards.md` — narrowed to the
  safeguard model.
- Every index, digest, and chapter that links into `05`.

## Acceptance

- No chapter under the new shelf is over 200 lines, counted by hand.
- No fact the sweep's ledger names is stated in two files.
- Every link that reached into `05` resolves to the chapter that owns the fact it cites —
  `just lint`.
- `just check` passes.

## Tasks

- [ ] Carve six chapters out of `05` and reduce that chapter to the CLI half.
- [ ] Re-point every link into `05` at the owner of the fact it cites.
- [ ] Write the shelf's hub and digest.

## Rabbit holes

- Rewriting a fact while moving it — escape: move the text, and correct only what the sweep's ledger
  names as wrong.
- Growing a chapter past the cap because the source had more prose — escape: the cap is the budget,
  and the surplus is either a link to the owner or a second chapter.

## Done when

The shelf indexes its own chapters, `05` is a chapter again, and a reader who opens the hub can say
what a project needs in order to be discoverable and operable by a coding agent.

## Revisions

None.

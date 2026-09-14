# 014 — Integrate the Landing Chapters

## Goal

The landing chapters are the only place the library states the lifecycle, and a reader who wants to
see it rather than read it has one worked example and one checklist.

## Example

Six chapters state the pattern and nothing shows it end to end.

```console
$ ls programming/agent-integration/9*.md
90-worked-example.md
```

That example covers a tool an agent runs. It covers no tool that writes into a project, so nothing
shows an unattributed collision, a failure part way through a landing, or a rerun.

## Core

A second worked example, with its own absurd domain, walks a tool that writes into projects from
greenfield through an update, a collision it refuses, a failure before the record is written, a
rerun, verification, and cleanup. A checklist walks every layer before shipping and names the owner
of each section rather than restating it.

## In scope

- The landing worked example.
- The checklist, across governance, playbook, safety, distribution, durable knowledge, landing,
  routing, and evidence.
- The hub and digest, updated once.
- A route from the bootstrap shelf to the landing chapters.

## Out of scope

- Anything the sweep marks unrelated.
- Restating a rule an owner chapter already holds.

## Governed by

- `_docs/specs/SPEC-knowledge-base-boundary.md` — what belongs in a bucket and what does not.
- `_docs/specs/SPEC-docs-format.md` — the register and the size budgets.
- `AGENTS.md` — the rule that each fact has one source of truth.

## Amends

- `programming/agent-integration/91-worked-example-landing.md` and `99-checklist.md` — created.
- `programming/agent-integration/90-worked-example.md`, `README.md`, and `AGENTS.md` — the routes.
- `programming/project-bootstrap/02-governance-and-docs.md` — one route.

## Acceptance

- Every search hit has one verdict, and no count is written down.
- The worked example and the checklist agree with every owner chapter.
- Reference implementations stay non-normative, and no portable chapter carries a product name.
- No neighboring shelf states a durable rule this shelf owns.
- `just check` passes.

## Tasks

- [ ] Sweep the library for every concept the landing chapters own, and give each hit one verdict.
- [ ] Write the landing worked example and the checklist.
- [ ] Update the hub, the digest, and the bootstrap route once each.

## Rabbit holes

- Extending the existing worked example past its budget — escape: a tool that writes into projects
  is a different subject and gets its own example.
- Writing a checklist that restates the rules — escape: each section names its owner, and a failure
  sends the reader there.

## Done when

The shelf owns the lifecycle, shows it once, and checks it once.

## Revisions

None.

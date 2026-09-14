# 011 — State the Staging and Landing Lifecycle

## Goal

A person can see what a tool would write before it writes, and the preview costs nothing in
correctness, because production never reads it.

## Example

The shelf now names the candidate and the record, and says nothing about the order in which either
reaches a project.

```console
$ git grep -c "stage" programming/agent-integration/08-candidate-artifacts-and-receipts.md
0
```

Nothing states what a preview holds, what happens when a write fails half way, or what a command
that removes a directory tree must establish before it removes anything.

## Core

Eight steps, two renders, one invariant. The candidate is rendered to a stage for investigation and
rendered again into production. Production accepts no stage as input, so there is no stale-input
problem and therefore no validity protocol. The stage survives until verification, because that is
when its evidence is worth most.

## In scope

- The eight steps and their order.
- The independence invariant, and the cost of the alternative.
- What the stage holds, and what it exposes without landing.
- Landing by ownership class, validation first, receipt last.
- The failure behavior, stated as what it is rather than as atomicity.
- Cleanup as a separate destructive command with its own refusals.
- The boundary between a portable property and the mechanism that proves it.

## Out of scope

- Version selection and migration judgment, which the next chapter owns.
- Any product, command, schema name, or vendor.

## Governed by

- `_docs/specs/SPEC-knowledge-base-boundary.md` — what belongs in a bucket and what does not.
- `_docs/specs/SPEC-docs-format.md` — the register and the size budgets.
- `_docs/specs/SPEC-project-platform.md` — a portable chapter names a property, not a platform.

## Amends

- `programming/agent-integration/09-disposable-staging-and-direct-landing.md` — created.
- `programming/agent-integration/08-candidate-artifacts-and-receipts.md` — one lifecycle link.
- `programming/agent-integration/README.md` and `AGENTS.md` — the route to it.

## Acceptance

- The ordered example distinguishes the two renders and draws no arrow from staged bytes into
  production.
- The failure text scopes replacement to supported filesystems and says receipt-last, never
  atomicity, transaction, rollback, or all-or-nothing.
- Cleanup names the protected roots and the evidence it validates, without naming a platform path.
- Access-control examples state a property and attribute a mechanism to an implementation.
- `just check` passes.

## Tasks

- [ ] State the eight steps and the independence invariant.
- [ ] State landing by ownership, validation first and receipt last.
- [ ] State the failure behavior honestly, and cleanup's refusals.

## Rabbit holes

- Promising atomicity because the words are available — escape: the guarantee is a whole-file
  boundary under one lock, and everything past that is version control.
- Turning one implementation's file modes into a universal rule — escape: the rule is the property,
  and the mechanism belongs to whoever proved it.

## Done when

The lifecycle is stated end to end, the stage is useful through verification, and production does
not depend on it.

## Revisions

None.

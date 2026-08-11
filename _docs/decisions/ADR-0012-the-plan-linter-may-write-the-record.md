# ADR-0012: The Plan Linter May Write the Record

## Context and Problem Statement

Closing work can make `todo.yml` ranking illegal by making previously blocked entries eligible.
Computing the minimal legal repair is mechanical, but doing it by hand is easy to forget and hard
to review from one lane alone.

## Considered Options

- Report ranking defects and require manual repair.
- Let the existing linter perform a bounded repair on explicit request.
- Run an automatic repair from the validation hook.

## Decision Outcome

Chosen option: an explicit bounded writer — `check-plan --rank-fix` may reorder entry blocks within
`backlog.yml` and `todo.yml`, and `--rank-slots` reports legal choices without writing.

The writer is deterministic, identity-preserving on legal input, idempotent, and non-canonical. It
moves original line blocks without re-serializing, never moves an entry between lanes, and changes
nothing when the dependency graph is cyclic. No hook invokes the writer.

This extends ADR-0006 without amending it: ADR-0006 requires shipped artifacts to be gated but does
not constrain a gated artifact to read-only behavior.

## Consequences

- Good: the machine repairs illegality while preserving human ranking wherever several orders are
  legal.
- Bad: the linter now has a write mode whose byte-preservation properties require dedicated tests.

## Status

Implemented by [the plan linter](../../programming/project-management/plan/check-plan) and its
[test harness](../../programming/project-management/plan/test-check-plan).

Amended by [ADR-0015](./ADR-0015-closed-is-an-append-only-log.md) — the writer's scope extends to
`closed.yml`, ordered by close date.

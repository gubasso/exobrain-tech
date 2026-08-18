# Closed Is an Append-Only Log, and the Linter Warns

## Context and Problem Statement

`closed.yml` was described as completion history that is never re-ranked, but nothing checked it.
Its order could drift from its close dates, so history read as a queue and any velocity computation
had to sort before it could scan. The rule was real and unenforced, which is the failure ADR-executable-artifacts-in-the-library
names: an unenforced reference reads as verified.

Making it a hard failure would block a commit over an ordering the tool can compute itself.

## Considered Options

- Leave the order a convention stated only in prose.
- Fail validation when a close date sits out of sequence.
- Report it, and let the existing writer repair it on request.

## Decision Outcome

Chosen option: **`closed.yml` is ordered by close date, and a violation is a warning the writer
repairs**. `check-plan` gains a third diagnostic class alongside failures and ranking failures: a
warning reaches the reader on stderr and never reaches an exit code, in any mode. `--rank-fix`
extends to `closed.yml` with a stable sort by date, so entries closed on one day keep the order they
were written in and an already-ordered file stays byte-identical.

The distinction the third class encodes: a failure names something no tool can decide, while a
warning names something the tool can fix. Gating on the second buys friction and no falsifiability.
The writer's existing guarantees are unchanged — it still refuses to write while any content check
fails, still moves whole line blocks, and is still never invoked from a hook.

## Consequences

- Good: history is ordered by construction, a velocity scan needs no sort, and the rule is
  falsifiable without becoming an obstacle.
- Bad: the linter's contract is now three classes rather than two, and a reader who ignores stderr
  can let the log drift indefinitely.

## Status

Superseded by [ADR-the-planning-method-moves-to-plan-xp](./ADR-the-planning-method-moves-to-plan-xp.md) — the method and its artifacts now live at [plan-xp](https://github.com/gubasso/plan-xp).

Amends ADR-the-plan-linter-may-write-the-record by extending the writer's scope to a third lane.

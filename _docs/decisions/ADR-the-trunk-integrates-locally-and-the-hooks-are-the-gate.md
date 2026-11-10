# The trunk integrates locally, and the git hooks are the gate

## Context and Problem Statement

ADR-master-is-the-trunk landed each branch through one squash-merged pull request, and a CI workflow
repeated the two hook stages on every push and request. This repository has one writer. The CI held
no check of its own, and it skipped the link check that the push stage runs, so every request waited
for a second run of checks that had already passed on the writer's host. The operator ruled on
2026-11-10 that this repository integrates locally and runs no CI.

## Considered Options

- `local integration through release-kit, with no CI` — chosen.
- `squash-merged pull requests behind CI` — rejected: with one writer, the request adds no review,
  and the CI repeats checks that the hooks already ran.
- `local integration, with CI kept as a second run` — rejected: a check that runs twice in two
  places drifts, and a CI-only failure would gate nothing that the hooks do not.

## Decision Outcome

Chosen option: `local integration through release-kit, with no CI` — the trunk takes exactly the
change that passed its hooks on the integrating host.

- `master` stays the trunk and the only permanent branch.
- release-kit is landed with `integration = "local"`, `merge_form = "rebase"`, the linked-worktree
  checkout mode, and release mode `none`. A branch opens in its own worktree, and `rk integrate`
  rebases it onto the trunk, which fast-forwards and is pushed. Every commit keeps its own message.
- The commit stage and the push stage run every check, each check in one stage. No workflow runs on
  the forge.

Enforced by `project-platform:the-hooks-are-the-only-gate`.

## Consequences

- Good: a change lands in minutes, and one place states every check.
- Good: the link check gates the push, where CI skipped it.
- Bad: a push from a checkout without the hooks installed lands unchecked, so every clone runs
  `pre-commit install` once.

## Status

Accepted

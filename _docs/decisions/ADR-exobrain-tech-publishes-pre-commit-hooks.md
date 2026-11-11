# exobrain-tech publishes pre-commit hooks

## Context and Problem Statement

A rule of the canon that every project must follow needs one check that every project can run. A
`path:` flake reference filled a disk on 2026-11-10, and the pattern sat in several repositories
with nothing to stop it. A check copied into each repository drifts from the rule and from the
other copies, so the rule needs one owner for its text and its check.

## Considered Options

- `publish the hook from this repository in .pre-commit-hooks.yaml` — chosen.
- `copy a local hook into each repository` — rejected: each copy drifts, and a fix reaches no copy
  until someone edits each one.
- `publish the hook from a separate repository` — rejected: it splits the rule from its check, so a
  change to one does not review the other.

## Decision Outcome

Chosen option: `publish the hook from this repository in .pre-commit-hooks.yaml` — the spec that
states the rule and the hook that enforces it change in one commit, and each consumer pins a
released tag.

- `.pre-commit-hooks.yaml` is a public interface. A hook id is never renamed or removed without a
  major release.
- The repository runs each published expression on itself through a local twin in
  `.pre-commit-config.yaml`. A negative control in `.hooks/test-gates.sh` holds the two entries
  equal and runs the published hook over a corpus of lines that must fail and lines that must pass.

Enforced by `project-platform:a-flake-is-read-from-its-git-tree`.

## Consequences

- Good: a consumer gets a fix of the check by moving its pin, and the corpus shows what the fix
  changed.
- Good: the rule and its check cannot disagree in a released tag.
- Bad: a lexical hook cannot follow a reference that a program assembles, so it needs a runtime
  guard beside it.
- Bad: removing a hook id costs a major release.

## Status

Accepted

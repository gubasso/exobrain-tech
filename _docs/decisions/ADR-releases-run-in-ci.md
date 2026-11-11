# Releases run in CI

## Context and Problem Statement

The repository ran no CI, because the commit and push hooks were the whole gate. A CLI that other
projects pin needs tagged releases with built artifacts and their attestation, and only the forge
can mint those. The question is how CI joins the repository without becoming a second gate that
judges what the hooks do not.

## Considered Options

- `three kinds of CI jobs: validation, release, and pull-request checks` — chosen.
- `no CI, with releases cut by hand` — rejected: a hand-cut release carries no attestation, and its
  artifacts depend on one machine.
- `CI as an independent gate` — rejected: a check that only CI runs fails after a green push, so the
  hooks stop being the proof of a change.

## Decision Outcome

Chosen option: `three kinds of CI jobs` — CI mirrors the hooks and adds only what the forge alone
can do.

- A validation job runs exactly the checks of the commit and push hooks, so a push that passed its
  hooks passes CI.
- A release job (release-plz, cargo-dist) creates tags, releases, and artifacts, and checks nothing
  that the hooks do not check.
- A pull-request check judges what only a pull request has, such as its title. No push starts it, so
  it stays outside the rule that the hooks run every check.
- Work keeps integrating locally (`integration = "local"`, `merge_form = "rebase"`): the project has
  one writer, and release-kit merges the release request at the forge in either mode.
  `setup.required_workflow` names the validation workflow that wakes the release gate.
- This replaces the no-CI half of `ADR-the-trunk-integrates-locally-and-the-hooks-are-the-gate`. That
  record stays as it is, and the rule `project-platform:the-hooks-are-the-only-gate` states what
  binds now.

Enforced by `project-platform:the-hooks-are-the-only-gate`.

## Consequences

- Good: a release is reproducible from the forge, with an attestation for each artifact.
- Good: a red CI run after a green push names a check that a hook is missing.
- Bad: the repository depends on the forge's runners and on two release secrets.

## Status

Accepted

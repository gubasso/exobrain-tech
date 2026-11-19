# A permanent hook scope is a declared prek group

## Context and Problem Statement

CI jobs of this repository and of seven other projects set a permanent `SKIP` for the hooks that CI cannot run, such as the link check. The scope of such a hook lived in a workflow, apart from the hook. The operator ruled on 2026-11-19: "SKIP is only for a one-time run in a specific runtime that needs it. The permanent scopes go in the main configuration, the prek config and the CI files, as declared tags, and each place selects its hooks by them. pre-commit: cheap, local. pre-push: heavier, does not repeat pre-commit, local. CI-only hooks run only in CI."

## Considered Options

- `prek groups, selected by each CI step` — chosen.
- `a permanent SKIP in each CI job` — rejected: the scope lives apart from the hook, and each job repeats the list.
- `one stage for each scope` — rejected: a stage decides when git fires a hook, and a stage of its own would drop a local-only check from the push stage.

## Decision Outcome

Chosen option: `prek groups, selected by each CI step` — the hook carries its own scope, so the configuration states every exception.

- `groups: [local-only]` marks a hook that CI cannot run, and its comment states why.
- A CI-only hook takes `stages: [manual]` and `groups: [ci-only]`, so no git hook fires it, and its comment states why it cannot run locally.
- Each CI step names its stage and its group filter. A group filter with no `--stage` also runs the `manual` hooks, so no step leaves the stage out.
- `SKIP` stays for a one-time run, and no committed file sets it.

Enforced by `project-platform:the-hooks-are-the-only-gate`.

## Consequences

- Good: `prek list --group local-only` and `prek list --group ci-only` name every exception, each with its reason beside it.
- Good: prek 0.4.4 brought groups, so the sets keep their floor.
- Bad: prek refuses `--group ci-only` when no hook carries the group, so a project with no CI-only hook leaves that step out.

## Status

Accepted

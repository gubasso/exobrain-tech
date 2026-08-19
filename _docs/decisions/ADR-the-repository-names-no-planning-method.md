# The repository names no planning or project-management method

## Context and Problem Statement

ADR-the-planning-method-moves-to-plan-xp made this repository a consumer of one planning project
through a `flake.nix` input, a `justfile` recipe, gates, and schema URLs. A knowledge base whose
environment fails to build without an external planning tool has adopted that tool's method, and
every reader of this repository inherits the choice.

## Considered Options

- Name no method, and allow the docs root to hold directories serving complementary domains —
  chosen.
- Keep the coupling — rejected: it binds the library to one method and makes an outside project
  load-bearing for the devShell.
- Vendor the method and its schemas here — rejected: it re-adopts the coupling and its maintenance,
  and duplicates a source of truth.

## Decision Outcome

Chosen option: name no method. This supersedes ADR-the-planning-method-moves-to-plan-xp.

The docs root may hold directories serving adjacent domains, planning among them, and nothing in this
repository names or depends on what fills them. The flake input, the update recipe, the plan gates,
and the schema URLs go; the decision log keeps every record of the earlier choice, because a record
states its own moment.

ADR-a-flake-pinned-tool-input-is-a-tool-dependency keeps its rule. That rule is about tools the
repository runs, and it is unchanged by which tools those are.

## Consequences

- Good: the devShell builds with no external project, and the library carries no method.
- Good: self-containment holds without an exception argued per input.
- Bad: the plan zone loses its linter and its schemas, so its shape is held by review.

## Status

Accepted

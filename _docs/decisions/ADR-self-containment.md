# This project is self-contained

## Context and Problem Statement

`exobrain-tech` is the public technical knowledge base. It drifts when critical knowledge lives only in external documents that move, disappear, or fall out of sync. Contributors and agents must be able to understand and operate the repository from the repository alone.

## Considered Options

- Reference external docs freely as the source of truth.
- Keep all load-bearing knowledge in-repo; treat external links as optional further reading.
- Mix both with no rule.

## Decision Outcome

Chosen option: **keep all load-bearing knowledge in-repo** — the repository is complete on its own; an external reference is allowed only as a public link/citation, never as a load-bearing dependency on a resource outside the repository, and in particular never on an external, local, personalized, or mutating repository, path, or tool. Private or personal material stays out of this repo and belongs in `exobrain-tech-vault`. If external knowledge is required, its essential substance is copied into the repo.

## Consequences

- Good: the repo is self-explanatory and resilient to external link rot; agents work from one source.
- Bad: some duplication of external material, and a discipline cost to keep copied knowledge current.

## Status

Accepted

Amended by [ADR-a-flake-pinned-tool-input-is-a-tool-dependency](./ADR-a-flake-pinned-tool-input-is-a-tool-dependency.md) — a public,
lock-pinned tool input is a tool dependency of the same class as `shellcheck`, not the external
dependency this rule bars.

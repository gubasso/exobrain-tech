# The exobrain CLI fills only the gaps of native tools

## Context and Problem Statement

mise, asdf, devbox, Nix flakes, and prek each cover a part of the features of a dev environment with their own commands. Without a rule, the `exobrain` CLI can grow a verb that repeats one of those commands, as the `self-depend` verbs of rk and sdd repeat the pin move of the project's own manager.

## Considered Options

- `use the native command, and let the CLI fill only a named gap` — chosen.
- `a verb that duplicates a native command, such as self-depend` — rejected: the project then has two movers for one pin, and the copy lags the manager it repeats.
- `a hand-written grammar for a file that a maintained parser reads` — rejected: it breaks on the first syntax its author did not foresee.

## Decision Outcome

Chosen option: `use the native command, and let the CLI fill only a named gap` — the operator ruled on 2026-11-21 that exobrain handles only the gaps that no other tool covers.

- A project uses the native command of its tool where one exists. A project that uses mise runs the commands of mise.
- `programming/project-bootstrap/COMPARISON-dev-tool-features.md` names each feature of a dev environment, the native way of each tool, and each gap.
- The CLI carries a verb only for a gap that the document names.
- Where a tool outside the managers covers a gap, the canon reuses that tool first. `flake-edit update <input>` moves the tag pin of a flake input, so the CLI carries no move.
- Where no tool fits, a verb builds on a well-maintained parser.

Enforced by `cli:a-verb-fills-a-named-gap`.

## Consequences

- Good: a project learns one command per feature, from the tool that owns the feature.
- Good: each verb of the CLI has a stated reason.
- Bad: the canon depends on tools outside it, such as `flake-edit`, and follows their releases.
- Bad: the comparison document needs a re-run when a native tool gains a feature.

## Status

Accepted

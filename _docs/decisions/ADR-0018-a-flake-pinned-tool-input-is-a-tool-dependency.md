# ADR-0018: A lock-pinned public tool input is a tool dependency, not an external dependency

## Context and Problem Statement

ADR-0017 makes this repository a consumer of `plan-xp` through a `flake.nix` input tracking that
project's `develop` branch. ADR-0003 bars a load-bearing dependency on a resource outside the
repository, and in particular on an external, local, personalized, or mutating repository, path, or
tool. Read without care, the new input is exactly what that rule forbids.

## Considered Options

- Vendor the linter and both schemas back into this tree.
- Drop the planning method here and keep no plan zone.
- State the reading: a public, lock-pinned tool input is a tool dependency.

## Decision Outcome

Chosen option: state the reading.

ADR-0003 bars a load-bearing dependency on external knowledge, and on a resource that can change or
disappear under the repository. A public flake input pinned in `flake.lock` is neither. It is the
same class of dependency as `shellcheck`, `dprint`, and `check-jsonschema`, which the devShell has
always carried: a versioned tool the repository runs, not a source of truth it reads.

Tracking a moving branch is what makes this load-bearing rather than decorative, and the lock file is
what keeps it true. `flake.lock` names one revision, and that revision is what every gate runs until
someone updates it deliberately. An unpinned input would be a violation of ADR-0003, so a lock entry
is mandatory and removing it is not a maintenance decision.

Vendoring was rejected: a copy of the schemas is the duplication `AGENTS.md` forbids, and a copy that
drifts from the version the gate runs is worse than none.

This amends ADR-0003. Its rule is unchanged; what changes is that the boundary between knowledge and
tooling is now stated rather than assumed.

## Consequences

- Good: the repository keeps one plan record, gated by the tool that defines the format.
- Bad: a careless `nix flake update` can pull a broken `develop`, which CI catches at the update
  commit rather than before it.

## Status

Accepted

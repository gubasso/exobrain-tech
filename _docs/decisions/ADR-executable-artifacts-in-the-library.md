# The library may ship executable artifacts, and must gate them

## Context and Problem Statement

This knowledge base shipped only prose, embedding machine-readable artifacts as fenced code blocks.
That works for eight lines of illustrative JSONC and fails for a JSON Schema or a 250-line linter,
because a fenced block cannot be validated: no hook runs `check-jsonschema` against a schema that
exists only inside markdown. The result is an artifact the repository asserts and never checks, and
an unenforced reference reads as verified.

## Considered Options

- Ship artifacts as real files inside the owning bucket, and gate them here — chosen.
- Keep embedding them as fenced blocks and accept that they are untested — rejected: the failure
  above, restated as a policy.
- Extract fenced blocks to temporary files in CI — rejected: a parser between the artifact and its
  test is one more thing to be wrong.

## Decision Outcome

Chosen option: ship them as real files and gate them. A drop-in a reader copies is trustworthy only
if this repository proves it runs.

An executable artifact is library content, not `_docs/` metadata, so it lives in its bucket beside
the chapter that explains it. Three obligations attach, all in the same change — a gate in
`.pre-commit-config.yaml`, a case in `just test`, and any tool it needs added to `flake.nix`. A
fenced block stays correct for illustration; the line is whether a reader is expected to copy the
thing and run it.

## Consequences

- Good: published drop-ins are proven rather than asserted, and a schema gives editor completion
  where a fenced copy gave none.
- Bad: this repository is no longer markdown-only. `just test` stops being a no-op, the devShell
  grows per-artifact tooling, and a contributor may now break a build.

## Status

Accepted. Enacted by the gates in `.pre-commit-config.yaml`, the `test` recipe in `justfile`, and
the validator packages in `flake.nix`.

Amended by [ADR-separate-documentation-design-from-project-management](./ADR-separate-documentation-design-from-project-management.md) — the
plan-zone artifacts and their gates moved out of `_docs/`.

Amended by [ADR-the-planning-method-moves-to-plan-xp](./ADR-the-planning-method-moves-to-plan-xp.md) — those
artifacts left the repository. This rule binds what it still ships.

# Source the documentation canon upstream

## Context and Problem Statement

The documentation method needs one public owner while this checkout must remain independently
operable. `ADR-self-containment` requires the knowledge and tools used for current operation to stay
local.

## Considered Options

- `pinned local projection with upstream provenance` — chosen.
- `runtime upstream dependency` — rejected: routine work would fail without another checkout.
- `duplicate canonical shelves` — rejected: two living owners would drift.

## Decision Outcome

Chosen option: `pinned local projection with upstream provenance` — the public canon owns the method,
and this checkout owns the projection it currently operates.

Held locally without network or canon access: `.spec-driven-docs/` hooks, MD043 arrays, relative-link
config, verifier, templates, and chapter debt; `_docs/specs/`; `_docs/decisions/`; the pre-commit
integration block; `flake.nix`; and `justfile`.

Referenced only for upgrades: the canon URL, `canon_version`, `canon_ref`, and migration guides.
Provenance records `canon_version`, `canon_source`, `canon_ref`, `installed_at`, and per-file hashes.
The verifier detects managed drift by comparing installed bytes with those hashes.

The projection carries only the gates this repository wires. Comparison-document and plugin gates are
not vendored, because no document and no plugin here would run them. The marked pre-commit block is
authored here rather than generated: the canon owns the scripts it names, this repository owns which
of them run, and the recorded marker hash is what detects an edit to it.

Enforced by `knowledge-base-boundary:no-knowledge-is-external` and
`distribution:verification-operates-offline`.

## Consequences

- Good: current work stays self-contained while releases have one upstream owner.
- Bad: living spec changes require local reconciliation during an upgrade.

## Status

Accepted

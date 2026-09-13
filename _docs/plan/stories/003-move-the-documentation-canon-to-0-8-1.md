# 003 — Move the Documentation Canon to 0.8.1

## Goal

This checkout runs the current documentation canon, and its upgrade is a pin move rather than a
migration.

## Example

The installed instance records a schema the current binary refuses outright.

```console
$ sdd status --target .
invalid manifest: manifest schema_version 1 is older than this binary's; run 'sdd upgrade'
```

## Core

The canon serves its delivered gates from its own binary now. Taking the binary as a lock-pinned
tool input replaces twenty-four vendored scripts and a vendored verifier, and it returns the
instance to a version that can move again.

## In scope

- The canon as a `flake.nix` input, pinned in `flake.lock`, and its package in the devShell.
- The upgrade itself, the debt record it migrates, and the declaration it renders the marked block
  from.
- The six requirements the delivered gates cite that the local specs did not state.
- `.hooks/test-gates.sh` reduced to the gates this repository authors.
- One record for the boundary the change restates.

## Out of scope

- Adopting the canon's guide template shape, which would move this repository's own guide gate.
- Any content change a gate does not demand.

## Governed by

- `_docs/decisions/ADR-source-the-documentation-canon-upstream.md` — the projection this change
  amends.
- `_docs/decisions/ADR-a-flake-pinned-tool-input-is-a-tool-dependency.md` — the reading that makes
  the input a tool rather than an external source.
- `_docs/decisions/ADR-executable-artifacts-in-the-library.md` — why a devShell entry needs a gate
  behind it.

## Amends

- `flake.nix`, `flake.lock` — the input and its pin.
- `.spec-driven-docs/` — the scripts leave, the declaration and the debt record arrive.
- `_docs/specs/` — the six requirements, and the instance spec the upgrade seeds.
- `.hooks/test-gates.sh`, `justfile`, `AGENTS.md` — what the change leaves behind.

## Acceptance

- `sdd verify` reports the current version with no failure.
- Every delivered gate passes — `just lint`.
- `just test` passes, and the harness exercises only this repository's own gates.

## Tasks

- [x] Add the input, the package, and the lock entry.
- [x] Run the upgrade and migrate the debt list.
- [x] State the six missing requirements in the specs that own them.
- [x] Cut the canon's cases from the gate harness.
- [x] Record the boundary.

## Rabbit holes

- Reconciling every drifted adopted spec against its new seed — escape: state what a live gate
  cites, and leave the rest to the change that needs it.
- Two guide templates in two zones — escape: the seeded one points at the authored one, and the
  shapes reconcile when the guide gate is revisited.

## Done when

The canon version this checkout implements is the current one, and no vendored gate script remains.

## Revisions

None.

# Documentation Distribution Specification

## Purpose

Rules governing the pinned documentation payload held under `.spec-driven-docs/`, its ownership
classes, and offline verification. Local content rules remain in their existing domain specs.

## Requirements

### `distribution:managed-payload-is-hash-verified` — Managed payload is hash verified

The instance MUST record and verify a SHA-256 for every managed documentation payload file.

#### Scenario: A vendored gate changes outside an upgrade

- GIVEN a hook whose installed hash is recorded
- WHEN its bytes change locally
- THEN offline verification fails and names the path

Verify: `pre-commit run spec-driven-docs-verify --all-files`

### `distribution:living-specs-remain-local` — Living specs remain local

The instance MUST keep `_docs/specs/` adopted and locally owned across canon upgrades.

#### Scenario: The canon adds a rule the library does not yet satisfy

- GIVEN an upstream living-spec change
- WHEN an upgrade refreshes managed payload
- THEN the local spec is preserved for explicit reconciliation

Verify: reviewer confirms `adopted_files` records a baseline for every local spec and that no
upgrade rewrites one

### `distribution:the-integration-block-is-instance-authored` — The integration block is instance authored

The instance MUST author the marked `.pre-commit-config.yaml` block itself and record its hash, so
that no generator supplies its contents.

#### Scenario: An upgrade offers to refresh the marked block

- GIVEN a block holding this repository's own hook wiring and the reasoning behind it
- WHEN a generator would replace the region between the markers
- THEN the replacement is refused, because the canon owns the scripts the block names and this
  repository owns which of them run and why

Verify: `pre-commit run spec-driven-docs-verify --all-files`

### `distribution:verification-operates-offline` — Verification operates offline

The instance MUST verify current operation with its vendored verifier and no canon checkout.

#### Scenario: Upstream is unavailable

- GIVEN the local manifest, hooks, configs, specs, templates, flake, and task runner
- WHEN `.spec-driven-docs/verify.sh --target . --offline` runs
- THEN it reaches no upstream path or network endpoint

Verify: `pre-commit run spec-driven-docs-verify --all-files`

# Justfile — task runner for the exobrain-tech knowledge base.
# Every recipe runs inside the pinned Nix devShell (see flake.nix) so the
# toolchain matches CI and local runs.
#
# This is mostly a markdown KB, but not only: a bucket may ship an executable
# artifact a reader is expected to copy and run, and ADR-0006 requires such an
# artifact to be gated here rather than asserted. `test` is where that gate
# lives. `build` stays a no-op — nothing is compiled.

# List available recipes.
default:
    @just --list

# Lint markdown: editorconfig conformance plus the full pre-commit hook set.
lint:
    nix develop --command editorconfig-checker
    nix develop --command pre-commit run --all-files

# Verify the artifacts this repository ships and the record it consumes (ADR-0006,
# as amended by ADR-0017). This library no longer ships an executable of its own;
# what it ships is a record, and the tool that proves it comes from a pinned input.
test: test-plan

# _docs/plan/ — this repository's own plan record, gated by the pinned plan-xp
# input rather than by a copy of its schemas in this tree.
#   1. every lane file and the config match the schemas the package provides
#   2. the record is coherent, checked by the linter from that same package
test-plan:
    nix develop --command sh -c 'check-jsonschema \
        --schemafile "$PLAN_XP_SCHEMA_DIR/plan-lane.schema.json" \
        _docs/plan/lanes/backlog.yml \
        _docs/plan/lanes/todo.yml \
        _docs/plan/lanes/doing.yml \
        _docs/plan/lanes/review.yml \
        _docs/plan/lanes/closed.yml'
    nix develop --command sh -c 'check-jsonschema \
        --schemafile "$PLAN_XP_SCHEMA_DIR/plan-config.schema.json" \
        _docs/plan/config.yml'
    nix develop --command check-plan _docs/plan

# Nothing to compile for a knowledge base.
build:
    @echo "no build: exobrain-tech ships documents and drop-in artifacts, not binaries"

# Format the markdown tree with dprint.
fmt:
    nix develop --command dprint fmt

# Format, then lint, then verify the shipped artifacts.
check: fmt lint test

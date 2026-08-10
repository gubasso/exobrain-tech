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

# Verify every executable artifact the library ships (ADR-0006).
test: test-plan

# programming/docs-design/plan/ — the plan-zone schema and linter.
#   1. the schema is valid JSON Schema draft 2020-12
#   2. the worked example satisfies the schema
#   3. the linter is shellcheck-clean
#   4. the linter accepts the worked example
test-plan:
    nix develop --command check-jsonschema --check-metaschema \
        programming/docs-design/plan/plan-lane.schema.json
    nix develop --command check-jsonschema \
        --schemafile programming/docs-design/plan/plan-lane.schema.json \
        programming/docs-design/plan/example/lanes/backlog.yml \
        programming/docs-design/plan/example/lanes/todo.yml \
        programming/docs-design/plan/example/lanes/doing.yml \
        programming/docs-design/plan/example/lanes/review.yml \
        programming/docs-design/plan/example/lanes/closed.yml
    nix develop --command shellcheck programming/docs-design/plan/check-plan
    nix develop --command programming/docs-design/plan/check-plan \
        programming/docs-design/plan/example

# Nothing to compile for a knowledge base.
build:
    @echo "no build: exobrain-tech ships documents and drop-in artifacts, not binaries"

# Format the markdown tree with dprint.
fmt:
    nix develop --command dprint fmt

# Format, then lint, then verify the shipped artifacts.
check: fmt lint test

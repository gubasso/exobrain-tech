# Justfile — task runner for the exobrain-tech knowledge base.
# Every recipe runs inside the pinned Nix devShell (see flake.nix) so the
# toolchain matches CI and local runs. This is a markdown KB: there are no
# compile/test/publish steps, so `build` and `test` are documented no-ops.

# List available recipes.
default:
    @just --list

# Lint markdown: editorconfig conformance plus the full pre-commit hook set.
lint:
    nix develop --command editorconfig-checker
    nix develop --command pre-commit run --all-files

# No test suite for a markdown knowledge base.
test:
    @echo "no tests: exobrain-tech is a markdown knowledge base"

# Nothing to compile for a markdown knowledge base.
build:
    @echo "no build: exobrain-tech is a markdown knowledge base"

# Format the markdown tree with dprint.
fmt:
    nix develop --command dprint fmt

# Format, then lint.
check: fmt lint

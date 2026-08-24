# Justfile — task runner for the exobrain-tech knowledge base.
# Every recipe runs inside the pinned Nix devShell (see flake.nix) so the
# toolchain matches CI and local runs.
#
# This is mostly a markdown KB, but not only: a bucket may ship an executable
# artifact a reader is expected to copy and run, and
# ADR-executable-artifacts-in-the-library requires such an
# artifact to be gated here rather than asserted. `test` is where that gate
# lives. `build` stays a no-op — nothing is compiled.

# List available recipes.
default:
    @just --list

# Lint markdown: editorconfig conformance plus the full pre-commit hook set.
lint:
    nix develop --command editorconfig-checker
    nix develop --command pre-commit run --all-files

# Verify what this repository ships
# (ADR-executable-artifacts-in-the-library).
#
# The library ships documents and, in a few buckets, a shell script a reader is
# expected to copy and run. Those five scripts are not gated yet:
# `_docs/plan/stories/002-gate-the-bucket-shell-scripts.md` is the story that
# gates them, and it is what returns shellcheck and shfmt to the devShell.
test: test-gates verify-instance

# Prove that each gate this repository adds can be made to fail.
#
# A new gate is demonstrated failing against an intentional violation before it
# is trusted (ADR-executable-artifacts-in-the-library), and `.hooks/test-gates.sh`
# is where every one of those violations is written. It runs the gates that ship
# rather than a copy of them, which is what the devShell entries for awk and
# markdownlint-cli2 are for: a tool reachable only inside pre-commit cannot be
# run against a deliberate violation.
test-gates:
    nix develop --command .hooks/test-gates.sh

# Verify the local documentation-canon projection without network access.
verify-instance:
    nix develop --command .spec-driven-docs/verify.sh --target . --offline

# Nothing to compile for a knowledge base.
build:
    @echo "no build: exobrain-tech ships documents and drop-in artifacts, not binaries"

# Format the markdown tree with dprint.
fmt:
    nix develop --command dprint fmt

# Format, then lint, then verify the shipped artifacts.
check: fmt lint test

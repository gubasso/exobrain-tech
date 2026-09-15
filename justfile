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
# expected to copy and run. Every one of those is linted and formatted here.
test: test-gates test-shipped-scripts test-one-system verify-instance

# Lint and format-check every shell script the buckets ship
# (`knowledge-base-boundary:a-shipped-script-is-gated`).
#
# The pre-commit hook runs the same script. This recipe is what puts the gate in
# CI without a hook run, and what lets a reader check one bucket's drop-in after
# editing it.
test-shipped-scripts:
    nix develop --command .hooks/shipped-scripts.sh

# Hold the flake's evaluated output set to the one supported system
# (ADR-linux-is-the-only-supported-project-target).
#
# The claim is about what the flake exposes, so the check evaluates it rather
# than reading flake.nix as text.
test-one-system:
    nix develop --command .hooks/check-one-system.sh

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
    nix develop --command sdd verify --target .

# Nothing to compile for a knowledge base.
build:
    @echo "no build: exobrain-tech ships documents and drop-in artifacts, not binaries"

# Format the markdown tree with dprint, and the shipped shell scripts with
# shfmt. shfmt takes no layout flags: `.editorconfig` carries them.
fmt:
    nix develop --command dprint fmt
    nix develop --command .hooks/shipped-scripts.sh --fix

# Format, then lint, then verify the shipped artifacts.
check: fmt lint test

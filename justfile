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
test: test-spec-shelf test-gates

# programming/spec-driven-docs/worked-example/ — the shelf applied to itself.
#
# The example ships a pre-commit block a reader copies into their own project,
# and ADR-executable-artifacts-in-the-library makes that block an artifact this
# repository must run rather than
# assert. It is run the way a reader would run it: the example is copied to a
# scratch repository whose root is the example's own root, so the `^_docs/`
# patterns inside the block resolve without a single path being rewritten here.
# A second copy of those hook definitions, scoped to the in-tree path, would be
# the duplication AGENTS.md forbids and would drift from the file readers take.
#
# The copy is what makes the run non-destructive: the block's formatting hooks
# write in place, and they write to the scratch tree.
#
# `env -u` is load-bearing. `git commit` exports GIT_DIR, GIT_INDEX_FILE, and
# GIT_WORK_TREE into every hook process, and this recipe runs as a hook. A
# `git add -A` that inherits them writes to the committing repository's index
# from inside the scratch tree, which corrupts the commit in progress and makes
# every later hook report against the wrong file set.
test-spec-shelf:
    nix develop --command env -u GIT_DIR -u GIT_INDEX_FILE -u GIT_WORK_TREE \
        -u GIT_OBJECT_DIRECTORY -u GIT_ALTERNATE_OBJECT_DIRECTORIES sh -c ' \
        set -e; \
        d=$(mktemp -d); \
        trap "rm -rf $d" EXIT; \
        cp -r programming/spec-driven-docs/worked-example/. "$d"; \
        cd "$d"; \
        git init -q .; \
        git add -A; \
        pre-commit run --all-files --config pre-commit-additions.yaml'

# Prove that each gate this repository adds can be made to fail.
#
# `08-gates.md` requires a new gate to be demonstrated failing against an
# intentional violation before it is trusted, and `.hooks/test-gates.sh` is
# where every one of those violations is written. It runs the gates that ship
# rather than a copy of them, which is what the devShell entries for awk and
# markdownlint-cli2 are for: a tool reachable only inside pre-commit cannot be
# run against a deliberate violation.
test-gates:
    nix develop --command .hooks/test-gates.sh

# Nothing to compile for a knowledge base.
build:
    @echo "no build: exobrain-tech ships documents and drop-in artifacts, not binaries"

# Format the markdown tree with dprint.
fmt:
    nix develop --command dprint fmt

# Format, then lint, then verify the shipped artifacts.
check: fmt lint test

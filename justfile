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

# Verify what this repository ships and what it consumes (ADR-0006).
#
# The library ships documents and, in a few buckets, a shell script a reader is
# expected to copy and run. Those five scripts are not gated yet:
# `_docs/plan/stories/002-gate-the-bucket-shell-scripts.md` is the story that
# gates them, and it is what returns shellcheck and shfmt to the devShell.
#
# What is gated here is the plan record, whose format and linter belong to the
# pinned plan-xp input.
test: test-plan test-spec-shelf

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

# programming/spec-driven-docs/worked-example/ — the shelf applied to itself.
#
# The example ships a pre-commit block a reader copies into their own project,
# and ADR-0006 makes that block an artifact this repository must run rather than
# assert. It is run the way a reader would run it: the example is copied to a
# scratch repository whose root is the example's own root, so the `^_docs/`
# patterns inside the block resolve without a single path being rewritten here.
# A second copy of those hook definitions, scoped to the in-tree path, would be
# the duplication AGENTS.md forbids and would drift from the file readers take.
#
# The copy is what makes the run non-destructive: the block's formatting hooks
# write in place, and they write to the scratch tree.
test-spec-shelf:
    nix develop --command sh -c ' \
        set -e; \
        d=$(mktemp -d); \
        trap "rm -rf $d" EXIT; \
        cp -r programming/spec-driven-docs/worked-example/. "$d"; \
        cd "$d"; \
        git init -q .; \
        git add -A; \
        pre-commit run --all-files --config pre-commit-additions.yaml'

# Move the pinned plan-xp revision to the current `develop` tip, then prove the
# record still passes the linter that arrived with it.
#
# The lock is what pins the gate, so this changes what `test-plan` means and
# belongs in a commit of its own. Clearing the cached verdict retires the
# advisory in `.envrc` straight away rather than a day later; the path comes
# from that file because the direnv layout directory is the user's to place.
update-plan-xp:
    @old=$(jq -r '.nodes["plan-xp"].locked.rev' flake.lock); \
    nix flake update plan-xp; \
    new=$(jq -r '.nodes["plan-xp"].locked.rev' flake.lock); \
    rm -f "${PLAN_XP_FRESHNESS_CACHE:-/nonexistent}"; \
    if [ "$old" = "$new" ]; then echo "plan-xp: already current ($old)"; exit 0; fi; \
    echo "plan-xp: $old -> $new"; \
    echo "changes: https://github.com/gubasso/plan-xp/compare/$old...$new"; \
    just test-plan

# Nothing to compile for a knowledge base.
build:
    @echo "no build: exobrain-tech ships documents and drop-in artifacts, not binaries"

# Format the markdown tree with dprint.
fmt:
    nix develop --command dprint fmt

# Format, then lint, then verify the shipped artifacts.
check: fmt lint test

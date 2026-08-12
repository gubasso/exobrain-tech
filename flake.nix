{
  description = "exobrain-tech knowledge-base dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    # The planning method this repository uses, and the linter that gates its
    # record. It is a tool dependency of the same class as shellcheck below, not
    # an external source of knowledge: ADR-0018 states that reading, and
    # flake.lock pinning a concrete revision is what keeps it true while the
    # branch itself moves.
    plan-xp = {
      url = "github:gubasso/plan-xp/develop";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, nixpkgs, flake-utils, plan-xp }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.just                    # task runner
            pkgs.pre-commit              # per-project git hooks
            pkgs.dprint                  # markdown formatter
            pkgs.editorconfig-checker    # .editorconfig lint
            # Node for the markdownlint-cli2 pre-commit hook. pre-commit's node
            # language falls back to nodeenv, which downloads a generic-glibc
            # node that cannot execute on a Nix host (no /lib64 loader); with
            # node+npm on PATH it uses `language_version: system` instead.
            pkgs.nodejs

            # Tools whose upstream hook is `language: python` but ships a
            # prebuilt, dynamically linked binary in the wheel. Such a binary
            # hard-codes the ELF interpreter /lib64/ld-linux-x86-64.so.2, which
            # does not exist on a Nix host, so the hook installs cleanly and
            # then dies at exec with a bare ENOENT. The nixpkgs builds link
            # against the glibc in the store, so taking the tool from here and
            # marking its hook `language: system` removes the failure mode
            # outright — no patchelf, no hand-repaired ~/.cache/pre-commit.
            #
            # Every `language: system` hook resolves off the ambient PATH and
            # gets no environment of its own, so each one needs a named provider
            # in this list. Currently: dprint, typos, committed.
            pkgs.typos
            pkgs.committed

            # This repository keeps a plan record whose format and linter belong to
            # a separate project. check-jsonschema validates `_docs/plan/` against
            # the schemas the pinned plan-xp package provides, and the linter
            # itself comes from that same package rather than from a file in this
            # tree — see ADR-0017 for the split and ADR-0018 for why a lock-pinned
            # public input is a tool dependency rather than an external one.
            #
            # shellcheck and shfmt left with the hook they served. A devShell entry
            # with no gate behind it is the assertion-without-a-gate the charter
            # forbids; `_docs/plan/stories/002-gate-the-bucket-shell-scripts.md` is
            # the story that brings both back, with a hook.
            pkgs.check-jsonschema

            # The plan linter and the two schemas that gate `_docs/plan/`. They
            # come from the pinned input rather than from a copy in this tree,
            # because a copy would be the duplication AGENTS.md forbids and a
            # copy that drifts from the version the gate runs is worse than
            # none.
            plan-xp.packages.${system}.default
          ];
          shellHook = ''
            export PLAN_XP_SCHEMA_DIR="${plan-xp.packages.${system}.default}/share/plan-xp/schema"
            echo "dev shell ready"
          '';
        };
      }
    );
}

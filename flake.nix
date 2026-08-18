{
  description = "exobrain-tech knowledge-base dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    # The planning method this repository uses, and the linter that gates its
    # record. It is a tool dependency of the same class as shellcheck below, not
    # an external source of knowledge:
    # ADR-a-flake-pinned-tool-input-is-a-tool-dependency states that reading, and
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

            # The gate snippet shipped with programming/spec-driven-docs/ is
            # run against its own fixtures by `just test-spec-shelf`, and two of
            # its hooks are `language: system`: the typed-clause check calls rg,
            # and the table-of-contents freshness check calls md_toc. Both are
            # named here for the reason the block below gives — a system hook
            # resolves off the ambient PATH and needs a declared provider.
            pkgs.ripgrep
            pkgs.python3Packages.md-toc

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

            # The freshness advisory in `.envrc` and the `update-plan-xp`
            # recipe read flake.lock and ask GitHub how far the pin has drifted.
            # Both tools are common enough to be on a developer's PATH already,
            # which is exactly why they are named here: an undeclared tool that
            # happens to resolve is the ambient-PATH failure described above,
            # working on the machine that wrote it and nowhere else.
            pkgs.jq
            pkgs.curl

            # This repository keeps a plan record whose format and linter belong
            # to a separate project. check-jsonschema validates `_docs/plan/`
            # against the schemas the pinned plan-xp package provides, and the
            # linter itself comes from that same package rather than from a file
            # in this tree — see ADR-the-planning-method-moves-to-plan-xp for the
            # split and ADR-a-flake-pinned-tool-input-is-a-tool-dependency for why a
            # lock-pinned public input is a tool dependency rather than an
            # external one.
            #
            # There is no shellcheck or shfmt here. A devShell entry with no gate
            # behind it is the assertion-without-a-gate the charter forbids, and
            # `_docs/plan/stories/002-gate-the-bucket-shell-scripts.md` is the
            # story that adds the hook and the tools together.
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

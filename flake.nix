{
  description = "exobrain-tech knowledge-base dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self, nixpkgs, flake-utils }:
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

            # The plan-zone artifacts this KB ships as canonical sources
            # (`programming/docs-design/`) are executable text, not prose, so
            # they are verified here rather than trusted:
            #   check-jsonschema : the lane schema against draft 2020-12, and
            #                      the worked example against the schema. It is
            #                      also the upstream pre-commit hook a project
            #                      adopting the method wires up, so the version
            #                      that validates the shipped copy is the same
            #                      one that will validate a project's.
            #   shellcheck       : the shipped `check-plan` linter must be clean
            #                      before it is published as a drop-in.
            #   shfmt            : canonical shell formatting for the same file,
            #                      so a copied artifact does not drift in style.
            #
            # No Python packaging tool is needed for any of this. Nix supplies
            # check-jsonschema as a binary on PATH, which is why neither poetry
            # nor uv appears here — this repo has no Python project to manage.
            pkgs.check-jsonschema
            pkgs.shellcheck
            pkgs.shfmt
          ];
          shellHook = ''echo "dev shell ready"'';
        };
      }
    );
}

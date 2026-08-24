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

            # Vendored corpus-wide gates call rg, and the root table-of-contents
            # hook calls md_toc. System hooks resolve off the ambient PATH, so
            # both tools need declared providers here.
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

            # awk backs the prose gate, and markdownlint-cli2 backs the heading
            # shapes. The linter has its own pre-commit environment, but a gate
            # that cannot be run outside pre-commit cannot be run against a path
            # the hook misses, or exercised against a deliberate violation —
            # which `just test-gates` does for both.
            pkgs.gawk
            pkgs.markdownlint-cli2

            # There is no shellcheck or shfmt here. A devShell entry with no
            # gate behind it is the assertion-without-a-gate the charter
            # forbids, and the story that adds the hook and the tools together
            # is `_docs/plan/stories/002-gate-the-bucket-shell-scripts.md`.
          ];
          shellHook = ''
            echo "dev shell ready"
          '';
        };
      }
    );
}

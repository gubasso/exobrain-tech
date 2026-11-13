{
  description = "exobrain-tech knowledge-base dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # The Rust toolchain that rust-toolchain.toml names, so the devshell, the
    # package, and CI compile with one compiler.
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # The documentation canon, taken as the tool that serves its own gates.
    # The tag in this URL is the version and flake.lock is the content pin, so
    # a lock entry is mandatory (ADR-a-flake-pinned-tool-input-is-a-tool-dependency).
    # Freshness is the manager's own verb: nix flake update spec-driven-docs,
    # with the tag moved to match, followed by sdd upgrade.
    spec-driven-docs = {
      url = "github:gubasso/spec-driven-docs/v0.17.0";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # The release-kit convention's own binary, `rk`. The workflow hooks of the
    # release-kit block call it by name, and `rk integrate` lands a branch on
    # the trunk. The same tool-input reading as spec-driven-docs applies, and
    # `rk self-depend sync --apply`, run from a shell, moves the tag and the
    # lock together.
    release-kit = {
      url = "github:gubasso/release-kit/v0.9.15";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      spec-driven-docs,
      release-kit,
    }:
    let
      # One system, declared here rather than mapped over a default set. This
      # checkout's only required check runs natively on x86_64 Linux, and a
      # flake that offered more would advertise support nothing proves
      # (ADR-linux-is-the-only-supported-project-target). This binds the
      # toolchain. The library's own subject matter stays platform-neutral.
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };
      toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          # The documentation canon's own binary. It serves every delivered
          # gate the hook block wires and the offline instance check,
          # so the projection holds configuration and no scripts.
          spec-driven-docs.packages.${system}.default
          release-kit.packages.${system}.default

          pkgs.just                    # task runner
          pkgs.dprint                  # markdown formatter
          # The runtime of the markdownlint-cli2 hook, a node hook that
          # .pre-commit-config.yaml runs under `language_version: system`.
          # Under `system`, prek takes the node on PATH and downloads none
          # (https://prek.j178.dev/reference/language-support/).
          pkgs.nodejs

          # A hook tool that the hook runner can build an environment for, such as
          # typos, committed, md-toc, or editorconfig-checker, is pinned in
          # .pre-commit-config.yaml and has no package here. A language
          # library never enters this shell
          # (project-platform:the-most-specific-native-owner-holds-each-dependency).

          # rg runs the Verify commands of the specs that sdd ships, such as
          # SPEC-docs-specs.md and SPEC-spec-to-code.md, which call it from PATH.
          pkgs.ripgrep

          # jq reads the evaluated system set in .hooks/check-one-system.sh,
          # which the one-system pre-push hook and `just test-one-system` run.
          pkgs.jq

          # The link checker behind the `lychee-system` pre-push hook.
          pkgs.lychee

          # The two tools behind `knowledge-base-boundary:a-shipped-script-is-gated`.
          # Its hook is `language: system`, and `just test-gates` runs the gate
          # against a deliberate violation outside the hook runner, so both tools
          # need a provider here.
          pkgs.shellcheck
          pkgs.shfmt

          # The Rust toolchain of the `exobrain` workspace in crates/, from
          # rust-toolchain.toml. A toolchain is no language library. prek
          # also builds the rust hook (ripsecrets) with this cargo.
          toolchain
          # owner: devshell. The cargo subcommands that the push-stage hooks
          # and the justfile run. No hook language builds a cargo subcommand.
          pkgs.cargo-nextest
          pkgs.cargo-deny
          # owner: devshell. The release tools: release-plz proves the
          # release configuration with its dry run, and cargo-dist plans and
          # renders the release workflow that the dist-plan hook checks.
          pkgs.release-plz
          pkgs.cargo-dist
        ];
        shellHook = ''
          echo "dev shell ready"
        '';
      };

      # The `exobrain` binary, built under Nix from the committed lock with
      # the toolchain that rust-toolchain.toml pins.
      packages.${system}.default = pkgs.callPackage ./nix/package.nix {
        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };
      };
      checks.${system} = {
        package = self.packages.${system}.default;
        # The built binary answers at all. nix flake check builds only the
        # checks output, so the package is named there as well.
        smoke = pkgs.runCommand "exobrain-smoke" { } ''
          ${pkgs.lib.getExe self.packages.${system}.default} --version
          touch $out
        '';
      };
      formatter.${system} = pkgs.nixfmt-rfc-style;
    };
}

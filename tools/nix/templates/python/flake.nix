{
  description = "python (uv) dev shell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { nixpkgs, ... }:
    let
      # The one supported system (project-platform:one-supported-system).
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

      # .python-version names the exact interpreter patch. The host's uv installs
      # it for development. A flake that also packages the project builds on the
      # nixpkgs Python of the same patch, and the check below holds the two equal.
      # See ../../02-per-project-devshell.md ("Python and uv in a devshell").
      pythonVersion = pkgs.lib.fileContents ./.python-version;
      python =
        pkgs."python${pkgs.lib.concatStrings (pkgs.lib.take 2 (pkgs.lib.splitVersion pythonVersion))}";

    in
    {
      # The host supplies uv, and uv owns the interpreter, so the devshell holds
      # only the project tools.
      devShells.${system}.default = pkgs.mkShell {
        # The nixpkgs pre-commit puts the libraries of its own Python on
        # PYTHONPATH, which shadows the packages of the project venv. The fix,
        # an override of its virtualenv, is in ../../02-per-project-devshell.md
        # ("Git hooks (pre-commit)").
        packages = [
          pkgs.pre-commit
          pkgs.just
        ];
      };

      # Fails when nixpkgs ships another patch than .python-version names.
      checks.${system}.python-version = pkgs.runCommand "python-version-is-aligned" { } ''
        if [ "${python.version}" != "${pythonVersion}" ]; then
          echo "nixpkgs ships Python ${python.version}, and .python-version names ${pythonVersion}" >&2
          exit 1
        fi
        touch "$out"
      '';
    };
}

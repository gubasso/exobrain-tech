{
  description = "python (uv) dev shell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { nixpkgs, ... }:
    let
      # The one supported system (project-platform:one-supported-system).
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

    in
    {
      # The host supplies uv, and uv owns the interpreter of .python-version,
      # so the devshell holds only the project tools. A project that Nix also
      # packages adds the interpreter check of ../../05-package-a-uv-project.md.
      devShells.${system}.default = pkgs.mkShell {
        # The nixpkgs pre-commit puts the libraries of its own Python on
        # PYTHONPATH, which shadows the packages of the project venv, so a hook
        # or a recipe that runs in the venv drops it: env -u PYTHONPATH uv run.
        # See ../../02-per-project-devshell.md ("Git hooks (pre-commit)").
        packages = [
          pkgs.pre-commit
          pkgs.just
        ];
      };
    };
}

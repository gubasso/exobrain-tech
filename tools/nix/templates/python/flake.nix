{
  description = "python (uv) dev shell";

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

        # The one declared Python version. Keep it within the requires-python
        # of pyproject.toml. See ../../02-per-project-devshell.md ("Aligning the
        # Python version").
        python = pkgs.python313;
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            python
            pkgs.uv
            pkgs.pre-commit
            pkgs.just
            # native build deps for C-extensions, uncomment as needed:
            # pkgs.swig pkgs.openssl pkgs.pkg-config
          ];

          # Precompiled manylinux wheels that uv installs (grpcio, numpy,
          # pydantic-core, …) link against a system libstdc++.so.6 / libz.so.1
          # that Nix keeps off the default loader path. Expose the gcc C++
          # runtime + zlib so `import`ing them works. Without this you get
          # "ImportError: libstdc++.so.6: cannot open shared object file".
          # See ../../02-per-project-devshell.md ("Binary wheels").
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.stdenv.cc.cc.lib
            pkgs.zlib
          ];

          # Hand the pinned interpreter to uv, forbid interpreter downloads,
          # install uv.lock into .venv as it is, and put .venv/bin on PATH.
          # Messages go to stderr: `nix develop --command` and `direnv exec`
          # share stdout with the command. After a change of pyproject.toml,
          # run `uv lock`, then `direnv reload`.
          # See ../../02-per-project-devshell.md ("Layering a language venv").
          shellHook = ''
            root="$(git rev-parse --show-toplevel)"
            export UV_PYTHON="${python}/bin/python3"
            export UV_PYTHON_DOWNLOADS=never
            export UV_PROJECT_ENVIRONMENT="$root/.venv"
            uv sync --frozen --quiet --project "$root" \
              || echo "uv sync failed: after a change of pyproject.toml, run uv lock" >&2
            export VIRTUAL_ENV="$root/.venv"
            export PATH="$root/.venv/bin:$PATH"
          '';
        };
      }
    );
}

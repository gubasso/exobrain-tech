{
  lib,
  rustPlatform,
}:

let
  cargoToml = lib.importTOML ../crates/exobrain/Cargo.toml;
in
rustPlatform.buildRustPackage {
  pname = cargoToml.package.name;
  version = cargoToml.package.version;

  # The workspace root, without the build outputs that a local checkout
  # holds. A flake evaluated from its git tree never sees them, and a path
  # evaluation of the folder must not copy them either.
  src = lib.cleanSourceWith {
    src = ../.;
    filter =
      path: type:
      let
        base = baseNameOf path;
      in
      !(builtins.elem base [
        "target"
        "result"
        ".direnv"
      ]);
  };
  cargoLock.lockFile = ../Cargo.lock;
  buildAndTestSubdir = "crates/exobrain";

  # The integration tests drive git and a stub nix in temporary folders,
  # which the build sandbox does not offer. `just test` runs them, and
  # checks.smoke carries the Nix-side signal.
  doCheck = false;

  meta = {
    description = "The exobrain-tech CLI: guards that enforce the rules of the knowledge base";
    license = lib.licenses.mit;
    mainProgram = "exobrain";
    platforms = [ "x86_64-linux" ];
  };
}

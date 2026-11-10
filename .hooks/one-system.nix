# Every system this flake keys an output by.
#
# This reads the flake's own outputs rather than parsing `nix flake show
# --json`, whose shape differs between Nix builds: some emit a `version` key
# beside the output families, and some nest the tree another level. The
# attribute names below are the fact the check is about, and they do not move.
#
# Takes an absolute path to the flake, because a git+file reference refuses a
# relative one. `.hooks/check-one-system.sh` resolves it.
#
# The path is read as a `git+file` reference. A bare path makes getFlake fetch
# the folder as a `path:` flake, which copies every file into the store, the
# ignored ones included. The `git+file` form copies the tracked tree, as
# `nix flake check` does.
reference:
let
  flake = builtins.getFlake ("git+file://" + toString reference);
  families = [
    "packages"
    "devShells"
    "checks"
    "formatter"
    "apps"
    "legacyPackages"
    "devShell"
    "defaultPackage"
  ];
  present = builtins.filter (name: builtins.hasAttr name flake) families;
  keysOf = name: builtins.attrNames (builtins.getAttr name flake);
in
builtins.concatLists (map keysOf present)

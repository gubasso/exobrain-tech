# 05 — Package a uv project with uv2nix

A uv project that Nix also installs, such as a command that a Home Manager profile carries, builds
its package from `uv.lock` with [uv2nix](https://github.com/pyproject-nix/uv2nix). The lock stays the
one owner of every Python library, and the flake holds none. Development keeps uv on the host and the
interpreter from uv
([Python and uv in a devshell](./02-per-project-devshell.md#python-and-uv-in-a-devshell)). The worked
example is the vitals project ([gubasso/vitals](https://github.com/gubasso/vitals), v0.1.0).

## The flake

Add the three inputs of the
[uv2nix hello-world template](https://github.com/pyproject-nix/uv2nix/blob/master/templates/hello-world/flake.nix),
each following the nixpkgs of the project:

```nix
pyproject-nix = {
  url = "github:pyproject-nix/pyproject.nix";
  inputs.nixpkgs.follows = "nixpkgs";
};
uv2nix = {
  url = "github:pyproject-nix/uv2nix";
  inputs.pyproject-nix.follows = "pyproject-nix";
  inputs.nixpkgs.follows = "nixpkgs";
};
pyproject-build-systems = {
  url = "github:pyproject-nix/build-system-pkgs";
  inputs.pyproject-nix.follows = "pyproject-nix";
  inputs.uv2nix.follows = "uv2nix";
  inputs.nixpkgs.follows = "nixpkgs";
};
```

Build the libraries as wheels on the nixpkgs Python of the exact patch in `.python-version`, the
`python` binding of the devshell chapter, so the shipped program and the uv-managed venv run one
interpreter:

```nix
workspace = uv2nix.lib.workspace.loadWorkspace { workspaceRoot = ./.; };
pythonSet = (pkgs.callPackage pyproject-nix.build.packages { inherit python; }).overrideScope (
  pkgs.lib.composeManyExtensions [
    pyproject-build-systems.overlays.wheel
    (workspace.mkPyprojectOverlay { sourcePreference = "wheel"; })
  ]
);
venv = pythonSet.mkVirtualEnv "vitals-env" workspace.deps.default;
inherit (pkgs.callPackages pyproject-nix.build.util { }) mkApplication;
vitals = mkApplication {
  inherit venv;
  package = pythonSet.vitals;
};
```

`mkApplication` exposes the console scripts of the package alone. The venv itself carries `python`
and the scripts of every library in its `bin/`, which a profile that installs it would put on
`PATH`. Expose the result as the default package and app, so `nix run github:<owner>/<repo>/<tag>`
runs a release:

```nix
packages.${system}.default = vitals;
apps.${system}.default = {
  type = "app";
  program = "${vitals}/bin/vitals";
};
```

## The check

A flake check builds the package and runs it once on fixed fakes, so a broken wheel, a missing
console script, or a failed import stops the push. The program selects its fakes with a setting
that its own tests use; `vitals` reads `VITALS_FAKE_SOURCES=1`:

```nix
checks.${system}.package-runs-once = pkgs.runCommand "vitals-runs-once" { } ''
  export HOME="$TMPDIR"
  VITALS_FAKE_SOURCES=1 ${vitals}/bin/vitals --once > "$TMPDIR/snapshot.json"
  ${pkgs.jq}/bin/jq -e '.gpu.temperature == 52.0' "$TMPDIR/snapshot.json" > /dev/null
  touch "$out"
'';
```

The check runs at the push stage, as a costly check.

## A system library the package cannot carry

A Nix Python does not search `/usr/lib64` or `/run/opengl-driver/lib`, so a library that the driver
of the host installs, such as `libnvidia-ml.so.1` for NVML, does not load from the package. The
package carries no host path. The launcher of each host puts the directory of the library on
`LD_LIBRARY_PATH` and executes the packaged binary:

- On a standalone Home Manager host (`osConfig == null`), the directory is a store path that links
  only `/usr/lib64/libnvidia-ml.so.1`, so no other library of the distribution enters the process.
- On NixOS, the directory is `/run/opengl-driver/lib`.

Without the path, the program reports the source as unavailable, `NVML Shared Library Not Found`,
and draws every other source.

## Consuming the package from a host flake

The host flake takes the project as an input and installs its default package from an app module.
To land every new version through the normal update of the host, follow the trunk of the project:
`git+https://github.com/<owner>/<repo>?ref=master`. Nix resolves no "newest tag", so an input pinned
to a tag moves only by a hand edit of the tag. `nix flake update <input>` moves one input, and the
update of every input moves it with the rest. The trunk is safe to follow when every trunk commit
passed the hooks and CI of the project.

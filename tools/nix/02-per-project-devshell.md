# 02 — Per-project devShell (flake + `.envrc` + direnv)

## Automatic activation (direnv)

`.envrc` only auto-loads if **direnv is hooked into your shell** — this is the piece people miss
(without it nothing activates on `cd`). Install direnv + nix-direnv and add the hook once,
host-level, not per-project:

```bash
# install direnv + nix-direnv (OS package or Home Manager `programs.direnv`), then
# hook the shell — for bash, in ~/.bashrc (zsh/fish have their own hook line):
eval "$(direnv hook bash)"
```

nix-direnv caches the flake evaluation so re-entry is fast. On NixOS / Home Manager,
`programs.direnv = { enable = true; nix-direnv.enable = true; };` installs the hook for you. (In
dctl sandboxes both the hook and a `[whitelist]` prefix are baked in.)

**The hook only fires in prompt-drawing shells.** `direnv hook bash` works via `PROMPT_COMMAND`, so
it activates only when a shell renders a prompt. It does **not** fire in **command shells**
(`bash -c "cmd"`, and `bash -lic "cmd"` despite the `-i`) or in non-interactive/CI/lifecycle shells
— none of them run the prompt loop. So a tool launched that way (a coding agent, a CI step, an
editor task runner) starts with **no devShell** unless you activate it another way:

- **Explicit:** `direnv exec . <cmd>` — loads the dir's `.envrc` and runs `<cmd>` inside it.
- **Eager:** `eval "$(direnv export bash)"` at shell init — loads the current dir's env without a
  prompt (useful when you can't wrap the command, e.g. an agent launched via a command shell; this
  is what dctl sandboxes do, keyed off an env marker so only sandbox command shells trigger it).

Enter the shell with `nix develop`, or run `direnv allow` once so nix-direnv enters it on `cd`.

## Minimal `flake.nix`

Values in angle brackets are placeholders. Copy-paste starting points per language are in
[`templates/`](templates/).

```nix
{
  description = "<project> dev shell";

  inputs = {
    # nixos-unstable = rolling branch (better-tested than nixpkgs-unstable);
    # or pin a stable release, e.g. nixos-26.05. Avoid the bare `nixpkgs`
    # registry shorthand — it resolves per-machine and is not reproducible.
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };
      in {
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.<language-runtime>   # e.g. python314, nodejs_22, zig
            pkgs.<package-manager>    # e.g. uv, pnpm
          ];
          shellHook = ''echo "dev shell ready"'';
        };
      });
}
```

## Layering a language venv (Python / uv)

The devshell syncs the venv of the project and puts it on `PATH`, so `.envrc` holds only
`use flake` and console scripts resolve with no manual activation. The `shellHook` hands the
interpreter of the flake to uv, stops uv from downloading another interpreter, installs `uv.lock`
into `.venv`, and puts `.venv/bin` first on `PATH`:

```nix
let
  python = pkgs.python313; # the one declared Python version
in
{
  devShells.default = pkgs.mkShell {
    packages = [ python pkgs.uv ];
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
```

`--frozen` installs `uv.lock` as it is and never rewrites it. After a change of `pyproject.toml`,
run `uv lock` inside the shell, then `direnv reload`. The root comes from git, so the hook syncs the
same venv from any subdirectory and from a linked worktree. The hook prints its messages on stderr,
because `nix develop --command <cmd>` and `direnv exec . <cmd>` share stdout with the command.

## Aligning the Python version

The flake pins the interpreter, `flake.lock` pins it transitively, and that pin is the one declared
Python version of the shell. `UV_PYTHON` hands it to uv, and `UV_PYTHON_DOWNLOADS=never` keeps uv
from fetching a managed interpreter, so the venv runs on the pin. `requires-python` in
`pyproject.toml` bounds the versions the package supports, a range such as `>=3.11`. It is package
metadata, not the dev pin. Keep the pin inside that range: uv refuses an interpreter outside it.
`pkgs.uv` is a native binary and carries no interpreter, so the shell holds a single Python. A
project that uses Poetry gets a second interpreter from the closure of `pkgs.poetry`. That trap and
its guard are in
[Aligning the Python version under Nix](../../languages/python/python-poetry.md#aligning-the-python-version-under-nix).

Do not run `uv python install`. It downloads an interpreter outside the Nix store that
`flake.lock` does not pin.

## Binary wheels need the C++ runtime (`libstdc++.so.6`)

A pure Nix devShell provides the runtimes and build tools, but **not** the shared C/C++ runtime
libraries on the dynamic loader's search path — Nix reaches those only through RPATHs baked into
nix-built binaries. Precompiled **manylinux wheels** (grpcio's `cygrpc.so`, `numpy`,
`pydantic-core`, `cryptography`, …) are **not** nix-built: uv drops them in as-is, and they
`dlopen`/link the _system_ `libstdc++.so.6` (and often `libz.so.1`) via the normal loader path.
Inside the devShell that path doesn't include them, so `import` fails:

```text
ImportError: libstdc++.so.6: cannot open shared object file: No such file or directory
```

It fails at import time and only under Nix: a plain distro venv finds the host's
`/usr/lib/libstdc++.so.6`. The `NIX_CC` / `NIX_LDFLAGS` / `stdenv` vars are compile-time hooks and
do nothing for an already-built wheel.

Fix: put the gcc C++ runtime and zlib on `LD_LIBRARY_PATH` in the devShell. `mkShell` exports any
attribute as an env var ([`templates/python/flake.nix`](templates/python/flake.nix) carries it):

```nix
devShells.default = pkgs.mkShell {
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    pkgs.stdenv.cc.cc.lib   # ships libstdc++.so.6 — clears the reported error
    pkgs.zlib               # libz.so.1 — grpcio/Google wheels dlopen this too
  ];
};
```

Then `direnv reload` (or re-enter `nix develop`). If a wheel still fails to load, `ldd` its `.so` to
see which library is missing and add that package (e.g. `pkgs.libz`, `pkgs.openssl`, `pkgs.glib`).

`nix-ld` needs the host to install the `nix-ld` module, which an RPM/openSUSE host lacks.
`autoPatchelfHook` moves the wheel into Nix and breaks the split where uv owns the venv.

## Git hooks (pre-commit)

`pre-commit` is per-project, not a global tool. The devshell holds `pre-commit` and only the tools
of the hooks that keep `language: system` under an exception class. pre-commit builds the
environment of every other hook from `.pre-commit-config.yaml`
([Who builds a hook environment](../../programming/project-bootstrap/04-quality-gates.md#who-builds-a-hook-environment)).
The devshell also holds the runtime that pre-commit needs to build a hook on a clean machine: a
project with `node` hooks adds `pkgs.nodejs`, because the generic node that pre-commit downloads
does not run on a host without the standard glibc loader, as on Nix. The host supplies only the
base tools, `nix`, `direnv`, the ELF loader, and the tools that carry credentials
([Who owns each dependency](../../programming/project-bootstrap/03-local-dev-environment.md#who-owns-each-dependency)).

statix is a `no-hook-language` exception: upstream publishes no hook in a language for which
pre-commit builds an environment. The devshell lists `pkgs.statix` beside `pkgs.pre-commit`, and
the hook records that owner:

```yaml
- repo: local
  hooks:
    # owner: devshell (no-hook-language) statix has no hook in a language that pre-commit builds.
    - id: statix
      name: statix
      entry: statix check
      language: system
      pass_filenames: false
```

Install the git hook once inside the shell with `pre-commit install`, which is idempotent. Make it
automatic on shell entry with a `shellHook` running `pre-commit install`, or go fully
Nix-native with [cachix/git-hooks.nix](https://github.com/cachix/git-hooks.nix), the same project as
`pre-commit-hooks.nix`, which generates `.pre-commit-config.yaml` from Nix and installs the hook
from the devShell's `shellHook`. In a sandbox that auto-activates direnv (e.g. dctl), a
non-interactive lifecycle step must call it project-scoped: `direnv exec . pre-commit install`.

## Maintaining the pin

```bash
nix flake lock              # generate flake.lock the first time (needs network)
nix flake update            # bump ALL inputs
nix flake update nixpkgs    # bump one input only
```

## Notes

- Ignore `.direnv/` and `/result` in `.gitignore`.
- Nix supplies runtimes and non-language system tools; it does **not** replace the project's package
  manager by default (no uv2nix / cargo2nix unless you specifically want Nix to build the app).
- A flake holds no language library: no `python3.withPackages`, no `python3Packages.*`, and no
  `nodePackages.*`. A library goes to the manifest of its stack, and the devshell keeps only the
  interpreter and its manager
  ([Who owns each dependency](../../programming/project-bootstrap/03-local-dev-environment.md#who-owns-each-dependency)).
- Offline / no-nix host: author `flake.nix` + `.envrc`, but `flake.lock` can only be generated where
  `nix` and network are available. **Do not fabricate a lock.**

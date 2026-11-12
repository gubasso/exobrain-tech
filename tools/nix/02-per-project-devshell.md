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

## Python and uv in a devshell

The host supplies uv, and uv owns the interpreter, so the devshell of a uv project holds no Python,
no uv, and no `shellHook` for either. The convention, with the pins and the activation for each
environment manager, is
[uv, the interpreter, and the environment manager](../../languages/python/project-bootstrap-spec/00-toolchain-and-layout.md#uv-the-interpreter-and-the-environment-manager).
Under direnv, `.envrc` loads the flake, then lets direnv's `layout uv` sync and activate `.venv`:

```bash
use flake
layout uv
```

`nix develop --command` reads no `.envrc`, so a recipe, a hook, and CI reach the tools of the
project through `uv run`, which syncs `.venv` first.

A project that Nix also packages builds on the nixpkgs Python of the exact patch in
`.python-version`, and a flake check holds the two equal:
[05 — Package a uv project](./05-package-a-uv-project.md#the-interpreter).

## Binary wheels need the C++ runtime (`libstdc++.so.6`)

This section binds a venv built on a Nix interpreter, which a Poetry project still uses. A
uv-managed CPython runs on the host loader and finds the host libraries itself. A pure Nix devShell provides the runtimes and build tools, but **not** the shared C/C++ runtime
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

## Git hooks

The git hook runner, prek, is a host tool, like git. Git runs an installed hook from every shell,
the shells without the devshell included, so the runner lives beside git. A devshell that held a
runner written in Python would also export the Python of that runner on `PYTHONPATH`, and the
variable would shadow the packages of a uv project. So the devshell holds no runner.

The devshell holds only the tools of the hooks that keep `language: system` under an exception
class. prek builds the environment of every other hook from `.pre-commit-config.yaml`
([Who builds a hook environment](../../programming/project-bootstrap/04-quality-gates.md#who-builds-a-hook-environment)).
It builds a Python hook with the uv of the host, a Rust hook with the cargo of the devshell, and a
node hook with a node that it finds or downloads
([prek language support](https://prek.j178.dev/reference/language-support/)). A project with `node`
hooks adds `pkgs.nodejs` and names `node: system` in `default_language_version`, so prek downloads
no node. The host supplies only the base tools, `nix`, `direnv`, `uv`, prek, the ELF loader, and
the tools that carry credentials
([Who owns each dependency](../../programming/project-bootstrap/03-local-dev-environment.md#who-owns-each-dependency)).

statix is a `no-hook-language` exception: upstream publishes no hook in a language for which prek
builds an environment. The devshell lists `pkgs.statix`, and the hook records that owner:

```yaml
- repo: local
  hooks:
    # owner: devshell (no-hook-language) statix has no hook in a language that prek builds.
    - id: statix
      name: statix
      entry: statix check
      language: system
      pass_filenames: false
```

A hook that runs in the project venv calls `uv run` with no prefix, because no runner Python reaches
the shell:

```yaml
# owner: pyproject.toml (project-environment) pytest runs the test suite in the uv environment.
- id: pytest
  name: pytest
  entry: uv run --frozen pytest
  language: system
  pass_filenames: false
```

Install the git hooks once per clone with `prek install`, from any shell. It writes a shim for each
stage in `default_install_hook_types`. The shim records the path of the prek that wrote it and falls
back to `prek` on `PATH` when that path is gone, so an update of the host does not break it. In a
sandbox, a lifecycle step runs `prek install` directly, with no devshell around it.

## Maintaining the pin

```bash
nix flake lock              # generate flake.lock the first time (needs network)
nix flake update            # bump ALL inputs
nix flake update nixpkgs    # bump one input only
```

## Notes

- Ignore `.direnv/` and `/result` in `.gitignore`.
- Nix supplies runtimes and non-language system tools; it does **not** replace the project's package
  manager by default (no uv2nix / cargo2nix unless you specifically want Nix to build the app, as
  [05 — Package a uv project](./05-package-a-uv-project.md) does).
- A flake holds no language library: no `python3.withPackages`, no `python3Packages.*`, and no
  `nodePackages.*`. A library goes to the manifest of its stack, and the devshell keeps the
  toolchain and its manager, except where the host supplies the manager, as it supplies uv
  ([Who owns each dependency](../../programming/project-bootstrap/03-local-dev-environment.md#who-owns-each-dependency)).
- Offline / no-nix host: author `flake.nix` + `.envrc`, but `flake.lock` can only be generated where
  `nix` and network are available. **Do not fabricate a lock.**

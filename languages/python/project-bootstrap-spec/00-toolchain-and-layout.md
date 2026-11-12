# 00 — Toolchain & layout

The Python ecosystem choices for a fresh project: which project/dependency manager to use, what
baseline `pyproject.toml` metadata to set, the src layout, how to pin the interpreter, and where the
virtualenv and Nix devShell fit.

## Project & dependency manager

`uv` is the modern default — a single fast tool that manages the interpreter, the virtualenv, the
lockfile (`uv.lock`, generated: the hooks that rewrite or judge exclude it, see
[generated files](../../../programming/project-bootstrap/04-quality-gates.md#generated-files)), and dependencies. Scaffold with:

- Application: `uv init <name>`.
- Distributable package: `uv init --package <name>` (adds a build backend and `src/` layout).
- In an existing directory: `uv init` (or `uv init --package`).

`uv add <dep>` / `uv add --dev <dep>` manage runtime and dev dependencies. `pyproject.toml` and
`uv.lock` own every Python library of the project, and the flake holds none, as
[who owns each dependency](../../../programming/project-bootstrap/03-local-dev-environment.md#who-owns-each-dependency)
says. Poetry, PDM, and pip-tools (`pip-compile`) are alternatives for a project that already uses
one of them. Each project keeps one manager.

## A single script

A standalone script declares its dependencies in an inline PEP 723 block at the top of the file. uv
reads the block, builds an isolated environment, and runs the script in it:

```python
#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "httpx",
# ]
# ///
```

`uv init --script <file> --python 3.12` writes the block, and `uv add --script <file> <dep>` adds a
dependency to it. Run the script with `uv run --script <file>`, or mark it executable and run it
directly through the shebang. `uv lock --script <file>` pins the versions in a `<file>.lock` beside
it.

A script moves to a project with `pyproject.toml` when its tests need its third-party packages,
because pytest then runs in an environment that holds both.

## `pyproject.toml` baseline metadata (PEP 621)

`pyproject.toml` is the single project manifest. Set the minimum now under `[project]`: `name`,
`version`, `requires-python`, and a short `description`. Declare the build backend in
`[build-system]` (e.g. `hatchling`, or `uv_build` for uv-native builds). Leave publish-grade
metadata (`license`, `authors`, `readme`, `urls`, `classifiers`, `keywords`) to the publishing
phase. Bootstrap only needs enough to build, import, and test.

## src layout

Prefer the **src layout** — the importable package lives under `src/<name>/`, tests under `tests/`.
This forces you to test the installed package rather than the source tree and avoids accidental
implicit imports. `uv init --package` scaffolds this shape, with a `[project.scripts]` entry and the
`uv_build` backend, and it is the default for a command-line tool too. Add `src/<name>/py.typed`, so
a type checker reads the package as typed, and a `__main__.py` that calls the entry point, so
`python -m <name>` runs it. For an application, move the package under `src/` to match.

## uv, the interpreter, and the environment manager

A tool that builds environments belongs to the host, and a tool that a project uses inside its
environment belongs to the project. `nix`, `mise`, `asdf`, `direnv`, and `git` build environments,
and so does uv: it installs interpreters, creates venvs, and runs tools in isolation. The host
supplies uv, and no environment manager of a project supplies it again. uv installs CPython, so uv
owns the interpreter, and no environment manager supplies a Python to a uv project
(`project-platform:the-most-specific-native-owner-holds-each-dependency`).

Pin uv in `pyproject.toml`. A uv outside the range refuses to run the project, on the host, in CI,
and in an editor alike. `uv sync --locked` installs the same `uv.lock` on every uv inside the range,
so pin `==` only where every machine must run one uv:

```toml
[tool.uv]
required-version = ">=0.13.7"
```

Pin the interpreter in `.python-version` with the exact patch, such as `3.15.2`, never a minor line
such as `3.15`. With a minor line, uv installs the newest patch that its own release knows, so the
patch drifts with the uv version. With an exact patch, a uv that does not know the patch fails
loudly, and `required-version` names the first uv that knows it. `requires-python` bounds the
versions the package supports, such as `>=3.15`. It is package metadata, not the pin.

A project that Nix also packages runs the shipped program on a nixpkgs interpreter. The flake reads
`.python-version`, takes the nixpkgs Python of that patch, and a flake check fails when nixpkgs ships
another patch. When nixpkgs and uv disagree on the newest patch, pin the patch that both supply. The
Nix form is in
[Python and uv in a devshell](../../../tools/nix/02-per-project-devshell.md#python-and-uv-in-a-devshell).

`uv run` syncs `.venv` before each command, so a recipe, a hook, and CI need no activation. For an
interactive shell, direnv's `layout uv` syncs `.venv` with `--frozen`, sets `VIRTUAL_ENV`, and puts
`.venv/bin` on `PATH`. A project needs no `shellHook`, no `UV_PROJECT_ENVIRONMENT`, and no
`UV_PYTHON`, because uv finds the project root and uses `.venv` by default. CI installs uv as
[05 — CI](../../../programming/project-bootstrap/05-ci.md) shows.

| Environment manager    | Holds uv          | Holds Python | Activation                                       |
| ---------------------- | ----------------- | ------------ | ------------------------------------------------ |
| Nix flake under direnv | no, the host does | no, uv does  | `use flake`, then `layout uv` in `.envrc`        |
| mise                   | no, the host does | no, uv does  | `python.uv_venv_auto = "source"` in `[settings]` |
| asdf                   | no, the host does | no, uv does  | `layout uv` under direnv; asdf activates no venv |
| none                   | no, the host does | no, uv does  | `layout uv` under direnv, or `uv run` alone      |

mise documents `python.uv_venv_auto` for a uv project with a `uv.lock`, and `_.python.venv` for a
project without uv ([mise Python](https://mise.jdx.dev/lang/python.html), v2026.11.6, read
2026-11-11). Its cookbook also installs uv through `[tools]`, which this convention leaves to the
host. asdf manages tool versions in `.tool-versions` alone
([asdf configuration](https://asdf-vm.com/manage/configuration.html), read 2026-11-11).

The trade-offs:

- The managed CPython is a python-build-standalone build. It runs on the host loader, so a
  manylinux wheel needs no `LD_LIBRARY_PATH`. A NixOS host needs nix-ld for it.
- The interpreter sits in `~/.local/share/uv/python`, outside the Nix store. `required-version` and
  the exact patch in `.python-version` pin it, not `flake.lock`.
- The first `uv sync` on a clean machine downloads the interpreter, and CI caches it.

## Layout detail

For a single package, the default `uv init` layout is enough to start. For a CLI with subcommands or
a distributable library, follow the implementation-kind file: [`cli-project.md`](./cli-project.md)
or [`library-project.md`](./library-project.md). Bootstrap owns the _ordering_ (get an importable,
buildable project first); the kind files own the shape-specific additions.

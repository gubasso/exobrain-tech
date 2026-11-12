# The host supplies uv, and uv owns the Python interpreter

## Context and Problem Statement

The canon put the Python interpreter and uv in the devshell of a project, and a `shellHook` handed
the interpreter to uv with `UV_PYTHON` and forbade downloads. uv installs and manages interpreters
natively, so the hook fought the tool, and the devshell nested one environment manager inside
another. nixpkgs also ships uv days after Astral releases it, so the Python patch that a project
could pin trailed the one nixpkgs packaged.

## Considered Options

- `uv on the host, the interpreter from uv` — chosen.
- `uv in the devshell, the interpreter from uv` — rejected: it nests one environment manager in
  another, and the nixpkgs lag returns with every Python patch.
- `uv and the interpreter in the devshell, bound with UV_PYTHON` — rejected: it fights the native
  interpreter management of uv and needs a `shellHook` and an `LD_LIBRARY_PATH` workaround.

## Decision Outcome

Chosen option: `uv on the host, the interpreter from uv` — a tool that builds environments belongs
to the host, beside `nix`, `direnv`, and `git`, and uv builds Python environments. The operator
decided it on 2026-11-11, and the vitals project proved it.

- `[tool.uv] required-version` in `pyproject.toml` pins uv, and `.python-version` names the exact
  interpreter patch, which uv installs.
- `.envrc` holds `use flake`, then `layout uv`. Recipes, hooks, and CI reach tools through
  `uv run`, and CI installs uv with `astral-sh/setup-uv`.
- A project that Nix also packages takes the nixpkgs Python of the same patch, and a flake check
  fails when nixpkgs ships another patch.

Enforced by `project-platform:the-most-specific-native-owner-holds-each-dependency`.

## Consequences

- Good: the devshell holds no Python and no `shellHook` for it, and a managed CPython runs on the
  host loader, so manylinux wheels need no `LD_LIBRARY_PATH`.
- Bad: a NixOS host needs nix-ld for the managed CPython, and the interpreter lives outside the Nix
  store, pinned by `required-version` and `.python-version` rather than by `flake.lock`.
- Open: whether rustup or a Node version manager moves to the host on the same principle.

## Status

Accepted

Amends [the most specific native owner holds each dependency](./ADR-the-most-specific-native-owner-holds-each-dependency.md),
which put uv and the interpreter in the devshell.

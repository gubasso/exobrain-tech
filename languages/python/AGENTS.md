---
digest-of: languages/python
last-synced: 2026-11-10
token-estimate: 400
---

# AGENTS

## Scope

Python language notes at the top level (outside `cli-spec/`). Includes a Django code-review guide,
general notes, Pydantic patterns, pytest fixtures, framework references, and packaging/environment
conventions.

## Key Points

- **Environment vs dependencies**: the Python runtime is provisioned by the project's Nix flake
  devShell (canonical per-project environment manager). uv manages dependencies inside it, from
  `pyproject.toml` and `uv.lock` for a project and a PEP 723 block for a single script. A flake
  never holds a Python library. Poetry, PDM, and pip-tools are alternatives for a project that
  already uses one, with one manager in each project. See
  `project-bootstrap-spec/00-toolchain-and-layout.md`,
  `../../workflows/development-tools-workflow.md`, and `../../tools/nix/README.md`.
- **Django review guide**: Django-specific review heuristics (N+1 queries, migrations, security,
  template injection).
- **General notes** (`python.md`): project environment/runtime, shell commands via subprocess, type
  hints (mypy cheat sheet), CLI libraries (docopt), rich/pprint, project layout, imports.
- **Pydantic**: Mixin patterns with enums, validator patterns (field/model validators).
- **Testing**: pytest conftest fixture patterns and organization.
- **Packaging**: local Python module with CLI access in a virtualenv; Poetry deps, lockfile export,
  container-deploy patterns for a project that already uses Poetry (`python-poetry.md`).
- **Frameworks**: Eve, Flask, MongoDB integration notes.

## Maintenance Notes

- CLI-spec has its own AGENTS.md; this digest covers only the top-level Python files.
- Framework notes (Eve, Flask, MongoDB) are reference-link collections; load directly when relevant.
- Environment-manager guidance is cross-cutting; its home is
  `workflows/development-tools-workflow.md` — the Python notes only cross-link it.

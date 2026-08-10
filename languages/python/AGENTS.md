---
digest-of: languages/python
last-synced: 2026-07-10
token-estimate: 400
---

# AGENTS

## Scope

Python language notes at the top level (outside `cli-spec/`). Includes a Django code-review guide,
general notes, Pydantic patterns, pytest fixtures, framework references, and packaging/environment
conventions.

## Key Points

- **Environment vs dependencies**: the Python runtime is provisioned by the project's Nix flake
  devShell (canonical per-project environment manager); Poetry manages dependencies inside it. Do
  not replace Poetry with Nix/poetry2nix by default. See
  `../../workflows/development-tools-workflow.md` and `../../tools/nix/README.md`.
- **Django review guide**: Django-specific review heuristics (N+1 queries, migrations, security,
  template injection).
- **General notes** (`python.md`): project environment/runtime, shell commands via subprocess, type
  hints (mypy cheat sheet), CLI libraries (docopt), rich/pprint, project layout, imports.
- **Pydantic**: Mixin patterns with enums, validator patterns (field/model validators).
- **Testing**: pytest conftest fixture patterns and organization.
- **Packaging**: local Python module with CLI access in a virtualenv; Poetry deps, lockfile export,
  container-deploy patterns.
- **Frameworks**: Eve, Flask, MongoDB integration notes.

## Maintenance Notes

- CLI-spec has its own AGENTS.md; this digest covers only the top-level Python files. The
  `release-workflow-spec/` **stub** (release-please / python-semantic-release + PyPI Trusted
  Publishing) is the Python binding of `programming/release-workflow/`, to be expanded when
  adopted.
- Framework notes (Eve, Flask, MongoDB) are reference-link collections; load directly when relevant.
- Environment-manager guidance is cross-cutting; its home is
  `workflows/development-tools-workflow.md` — the Python notes only cross-link it.

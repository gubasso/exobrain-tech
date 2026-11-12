---
digest-of: languages/python/project-bootstrap-spec
last-synced: 2026-11-11
token-estimate: 650
---

# AGENTS

## Scope

Python binding of the general `programming/project-bootstrap/` shelf: the once-per-project
Python setup that takes an empty repo to a scaffolded, gated project ready for feature work. It
**overlays** the general spine (repo, license, governance, dev env, CI, security) and never restates
it; it owns only the Python ecosystem choices and the two implementation-kind orderings (CLI,
library). Publishing is out of scope.

## Key Points

- **Manager:** `uv` is the modern default — one tool for interpreter, virtualenv (`.venv`), lockfile
  (`uv.lock`), and deps. Scaffold `uv init <name>` (app) or `uv init --package <name>`
  (distributable, adds build backend + `src/`). Poetry, PDM, and pip-tools are alternatives for a
  project that already uses one, with one manager in each project.
- **Single script:** a PEP 723 block declares its dependencies, the shebang
  `#!/usr/bin/env -S uv run --script` runs it, and `uv add --script` edits the block. It moves to a
  project when its tests need its third-party packages.
- **Manifest:** `pyproject.toml` (PEP 621). Bootstrap sets only `name`, `version`,
  `requires-python`, `description`, and a `[build-system]` backend (`hatchling` or `uv_build`).
  Publish-grade metadata (`license`, `authors`, `readme`, `urls`, `classifiers`, `keywords`) is
  deferred to the publishing phase — do not duplicate that gate here.
- **Layout:** src layout — importable package under `src/<name>/`, `tests/` outside `src/` so tests
  run against the installed package.
- **uv and the interpreter:** the host supplies uv, and uv owns the interpreter.
  `[tool.uv] required-version` pins uv, `.python-version` names the exact patch, and `.envrc` holds
  `use flake`, then `layout uv`. The devshell holds no Python and no uv (`00-toolchain-and-layout`,
  "uv, the interpreter, and the environment manager").
- **Quality gates (configured in `pyproject.toml`):** `ruff` with `select = ["ALL"]` (format +
  lint) as its upstream hook; mypy strict with the extra error codes and the pydantic and returns
  plugins, from the project venv where a plugin needs the `uv.lock` versions; `pytest` (tests under
  `tests/`) and `pip-audit` (security baseline, PyPI Advisory DB / OSV) as dev dependencies. The
  typing contract is `../typing.md`. Wire all into pre-commit so failures surface locally.
- **CLI kind:** console-script entry under `[project.scripts]`; arg-parsing via `argparse` (stdlib)
  or `click`/`typer`; consistent error type + non-zero exit codes; stdlib `logging` (human on
  stdout, logs on stderr); config precedence flags > env > file, validated early (e.g. `pydantic`).
- **Library kind:** public API via `__all__` + private internals; `py.typed` marker (PEP 561); test
  the installed package (src layout enforces it); a docs skeleton (`mkdocs`/`sphinx`) if the API is
  non-trivial.

## Maintenance Notes

- General spine: `../../../programming/project-bootstrap/`. Detailed CLI structure: `../cli-spec/`.
- `web-service.md` is a declared followup kind; add it when it lands.
- The Python packaging landscape (`uv`, `ruff`) moves fast — re-verify the default-tool choices
  against upstream on a cadence when regenerating.
- No conflicts among the current source files.

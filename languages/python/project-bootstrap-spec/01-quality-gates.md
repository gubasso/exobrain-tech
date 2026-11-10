# 01 — Quality gates

The Python concretion of the general
[quality gates](../../../programming/project-bootstrap/04-quality-gates.md) and
[security baseline](../../../programming/project-bootstrap/06-security-baseline.md) chapters. The
configuration of each tool lives in `pyproject.toml`. Each tool has one owner, as
[03 — Who owns each dependency](../../../programming/project-bootstrap/03-local-dev-environment.md#who-owns-each-dependency)
says: a tool that only a hook runs is pinned in `.pre-commit-config.yaml`, and the test suite is a
dev dependency in `pyproject.toml`.

## Formatter + linter — `ruff`

`ruff` is the modern default: one fast tool that both formats and lints, replacing the older black +
flake8 + isort stack. Configure it under `[tool.ruff]`. Its hooks run:

```bash
ruff format --check
ruff check
```

If you inherit an older project, **black** (format), **flake8** (lint), and **isort** (imports) are
the stack `ruff` supersedes; keep one owner rather than running both.

## Typecheck — `mypy` / `pyright`

Add a static type checker. Pick one, and run it as its hook:

```bash
mypy src        # or:
pyright
```

Configure under `[tool.mypy]` in `pyproject.toml` (pyright reads `pyproject.toml` or
`pyrightconfig.json`). Prefer strict mode for new code.

## Tests — `pytest`

`pytest` is the default test runner; put tests under `tests/` and run against the installed package
(the src layout enforces this):

```bash
pytest
```

## Security — `pip-audit`

`pip-audit` is the Python tool behind the general security baseline: it fails on installed
dependencies with known advisories (PyPI Advisory Database / OSV).

```bash
pip-audit
```

Run it in CI so a vulnerable dependency cannot merge.

## Pre-commit wiring

ruff, mypy, and pyright take their upstream hooks, because only a hook runs them: `ruff-check` and
`ruff-format` from `astral-sh/ruff-pre-commit`, `mypy` from `pre-commit/mirrors-mypy` with the
typed libraries in `additional_dependencies`, or `pyright` from `RobertCraigie/pyright-python`,
which reads the venv that `pyrightconfig.json` names. pytest runs the test suite of the project, so
it stays a dev dependency in `pyproject.toml` and keeps a `language: system` hook with its owner
comment:

```yaml
- repo: local
  hooks:
    # owner: pyproject.toml (project-environment) pytest runs the test suite in the uv environment.
    - id: pytest
      name: pytest
      entry: uv run pytest
      language: system
      pass_filenames: false
```

The general [04 — Quality gates](../../../programming/project-bootstrap/04-quality-gates.md#who-builds-a-hook-environment)
states the exception classes.

## Publish-readiness (later phase)

Publish-grade checks (`uv build`, `twine check`, metadata completeness) belong to the publishing
phase, not bootstrap. Bootstrap only guarantees the project builds, formats, lints, typechecks,
tests, and audits clean.

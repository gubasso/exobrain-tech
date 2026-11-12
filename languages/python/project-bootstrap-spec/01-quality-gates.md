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

## Typecheck — mypy strict

A new project runs mypy with the full typing contract from its first commit, so the first line of
code already meets it. The contract and its types are in [Typing a Python project](../typing.md).
The configuration lives in `pyproject.toml`:

```toml
[tool.mypy]
strict = true
warn_unreachable = true
files = ["src", "tests"]
enable_error_code = [
    "exhaustive-match", "explicit-override", "possibly-undefined", "redundant-expr",
    "truthy-bool", "truthy-iterable", "ignore-without-code", "unused-awaitable", "deprecated",
    "mutable-override", "redundant-self", "unimported-reveal", "explicit-any",
]
plugins = ["pydantic.mypy", "returns.contrib.mypy.returns_plugin"]

[tool.pydantic-mypy]
init_forbid_extra = true
init_typed = true
warn_required_dynamic_aliases = true
```

`exhaustive-match` fails a `match` on a `StrEnum` or on a `Result` that misses a case, and
`explicit-any` fails a written `Any`. A project that loads no plugin can take pyright from
`RobertCraigie/pyright-python` instead.

ruff selects every rule family, and its ignore list holds only a rule that conflicts with
`ruff format`, a rule that excludes another selected one, and a rule the project rejects by name,
each with its reason:

```toml
[tool.ruff.lint]
select = ["ALL"]
ignore = [
    "COM812", # ruff format owns the trailing commas
    "D203",   # excludes D211
    "D213",   # excludes D212
    "CPY001", # the root LICENSE covers every file
]
```

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

ruff and pyright take their upstream hooks, because only a hook runs them: `ruff-check` and
`ruff-format` from `astral-sh/ruff-pre-commit`, and `pyright` from `RobertCraigie/pyright-python`,
which reads the venv that `pyrightconfig.json` names. mypy takes `pre-commit/mirrors-mypy` with the
typed libraries in `additional_dependencies` while it loads no plugin. With the pydantic and returns
plugins, each plugin must load the version of its library that `uv.lock` pins, a reason specific to
the repository: mypy becomes a dev dependency and runs from the project venv. pytest runs the test suite of the project, so
it stays a dev dependency in `pyproject.toml` and keeps a `language: system` hook with its owner
comment:

```yaml
- repo: local
  hooks:
    # owner: pyproject.toml (project-environment) the pydantic and returns plugins of mypy must load the versions that uv.lock pins.
    - id: mypy
      name: mypy (strict)
      entry: uv run --frozen mypy
      language: system
      types: [python]
      pass_filenames: false
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

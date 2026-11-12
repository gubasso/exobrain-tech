# A Python project meets the typing contract, and returns a Result

## Context and Problem Statement

The Python pages advised "strict mode for new code" and typed exceptions caught at the Typer
boundary. mypy cannot prove that a caller handles a raised exception, and a lax check lets `Any`
and an unchecked `match` through. The operator asked for types as close to Rust as Python allows.

## Considered Options

- `mypy strict with the extra codes and both plugins, ruff ALL, and Result from returns` — chosen.
- `strict mode without the extra codes` — rejected: it misses an unhandled `match` case and a
  written `Any`.
- `exceptions throughout, with the AppError hierarchy` — rejected inside the program: mypy cannot
  prove that a caller handles a raised error.
- `rustedpy/result` — rejected: its repository was archived on 2026-06-25.

## Decision Outcome

Chosen option: `mypy strict with the extra codes and both plugins, ruff ALL, and Result from
returns` — vitals ran it from its first commit on 2026-11-11, and the gate held through 70 tests.

- pydantic owns the boundary, frozen dataclasses own internal values, a unit is a `NewType`, and a
  closed set is a `StrEnum`.
- A fallible function returns `Result[T, E]`, `@safe` lives only in an adapter, and a `match` on a
  `Result` goes through `outcome()`, the union `Success | Failure`, so `exhaustive-match` holds it.
- `main` is the one exit boundary and maps a `Failure` to its sysexits code.
- mypy runs from the project venv when its plugins must load the `uv.lock` versions.

## Consequences

- Good: a commit that breaks the contract fails before it lands, and a panel or a caller handles a
  failure as a value.
- Bad: a contributor learns the `returns` containers, and `Result.value_or(None)` needs a helper.

## Status

Accepted

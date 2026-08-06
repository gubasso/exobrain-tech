# ADR-0002 — `main` is a process boundary, not a place for logic

**Status:** Accepted **Date:** 2026-08-06

## Context

[03 — Error handling](../03-error-handling.md) showed a `fn main() -> ExitCode` that called
`AppContext::new` and `dispatch` inline, and both
[`templates/src/main.rs.template`](../templates/src/main.rs.template) and
[`main.rs.machine.template`](../templates/src/main.rs.machine.template) put the subcommand dispatch
directly in `main`. The spec demonstrated the shape without stating the rule or its mechanism, so a
reader could reasonably conclude the alternative — `fn main() -> anyhow::Result<()>` — was equally
acceptable. It is not, and the reason is structural rather than stylistic: `ExitCode` does not
implement `Try`, so `?` is illegal in `main`; and `impl<T, E: Debug> Termination for Result<T, E>`
hardcodes `ExitCode::FAILURE` and an `Error: {err:?}` dump, so a `Result`-returning `main` cannot
emit `64`/`70`/`78` at all. A downstream project (`vivarium`, a microVM supervisor) hit this while
reconciling a `Result<(), u8>` inner function against a documented sysexits matrix.

## Decision

`main` returns `std::process::ExitCode`, contains no logic, and owns exactly two things: the
connection to process globals (`std::env::args_os()`), and the conversion of a failure into a
rendered error chain plus an exit status through `AppError::exit_code()`. The fallible program lives
in a separate `run` function that receives its inputs as parameters and reads no process global —
which makes "the arguments were parsed and validated" a fact of `run`'s signature rather than a
convention, and leaves the parser a pure function of an iterator that the whole argument grammar can
be tested against. `fn main() -> Result<_, _>` is disallowed for any binary with a documented
exit matrix; `std::process::exit` is disallowed outright in favour of returning the `ExitCode`, so
destructors run. `impl Termination` for an owned type is permitted but not the default — it does not
remove `run`, since `?` depends on `Try`.

## Consequences

- [03 — Error handling](../03-error-handling.md) gains `## The process boundary`, a
  `## Binaries a service manager supervises` section for the daemon audience, an `## Anti-patterns`
  table, and three new entries under `## Rules`.
- [10 — Reference projects](../10-reference-projects.md) gains a daemons-and-VMMs section
  (firecracker, cloud-hypervisor, youki, ruff) evidencing the pattern.
- Both `main.rs` templates move dispatch into `run`.
- Downside: two functions where one would compile, and a `run` whose only caller is `main` — the cost
  of keeping `?` and a real exit matrix at the same time.

## Alternatives considered

- **`fn main() -> anyhow::Result<()>`.** Rejected: exits `1` and `Debug`-dumps. Acceptable only when
  success-versus-generic-failure is the whole contract (rust-analyzer, GUI binaries).
- **`impl Termination` for the code enum, `fn main() -> ExitKind`.** Rejected as the default: stable
  and officially exemplified since 1.61, but it still needs `run`, hides rendering inside a
  conversion, and no surveyed project uses it.

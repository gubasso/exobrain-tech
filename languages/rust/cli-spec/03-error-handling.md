# 03 — Error Handling (Rust)

> Prerequisite:
> [General principles — Error Messages](../../../programming/cli-design/02-error-messages.md) for
> the four-part anatomy (what/where/why/hint), stable `err.kind`, BSD sysexits, and audience matrix.
> This chapter is the Rust implementation using `thiserror` + `anyhow`.

`thiserror` per layer for typed errors, `anyhow` only at the binary boundary, one top-level
`AppError` enum with `exit_code()` mapped to BSD sysexits.

## The layer table

| Layer       | Returns                             | Crate                                  | Rule                                                                                                                           |
| ----------- | ----------------------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `domain/`   | `enum DomainError` per module       | `thiserror`                            | Invariant violations only. No I/O variants.                                                                                    |
| `adapters/` | `enum <Sys>AdapterError`            | `thiserror` + `#[from] std::io::Error` | One error type per adapter. Wrap upstream errors with `#[source]`.                                                             |
| `services/` | `enum ServiceError`                 | `thiserror`                            | `#[from]` domain + adapter errors. Do NOT recursively `#[from]` peer service errors.                                           |
| `commands/` | `Result<(), AppError>`              | `thiserror`                            | `AppError` `#[from]`s every service and `std::io::Error`. Plus an `Other(#[from] anyhow::Error)` arm for ad-hoc `.context(…)`. |
| `main`      | `Result<(), AppError>` → `ExitCode` | `anyhow` allowed                       | `AppError::exit_code() -> u8`. Unit-test the matrix.                                                                           |

## Why both `thiserror` and `anyhow`

- `thiserror` builds **named, matchable** errors. Code that needs to react to a specific failure can
  `match` on the variant.
- `anyhow` is for **opaque, context-rich** errors at the application boundary — you don't care which
  exact variant fired, you care about printing a useful chain to the user.

Inside the crate, every error has a name. At the boundary, you optionally lose the name in exchange
for cheap `.context("while doing X")` chains.

**Never** return `anyhow::Error` from a library crate or a public API. Inside a binary's `commands/`
and `main`, it's fine — and the typed `AppError::Other(#[from] anyhow::Error)` variant lets you mix
the two without ceremony.

## Skeleton: `src/error.rs`

```rust
//! Crate-level error type and exit-code mapping.

use thiserror::Error;

/// Top-level error type returned from `commands::*::run`.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("usage: {0}")]
    Usage(String),

    #[error("config: {0}")]
    Config(#[from] crate::config::ConfigError),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Domain(#[from] crate::domain::DomainError),

    #[error(transparent)]
    Adapter(#[from] crate::adapters::AdapterError),

    #[error(transparent)]
    Service(#[from] crate::services::ServiceError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl AppError {
    /// Map to a BSD sysexits exit code. See sysexits(3).
    pub fn exit_code(&self) -> u8 {
        match self {
            AppError::Usage(_)                                 => 64, // EX_USAGE
            AppError::Config(_)                                => 78, // EX_CONFIG
            AppError::Domain(_)                                => 65, // EX_DATAERR
            AppError::Adapter(crate::adapters::AdapterError::NotFound) => 66, // EX_NOINPUT
            AppError::Adapter(crate::adapters::AdapterError::Unavailable) => 69, // EX_UNAVAILABLE
            AppError::Adapter(_)                               => 74, // EX_IOERR
            AppError::Service(_)                               => 70, // EX_SOFTWARE
            AppError::Io(e) if e.kind() == std::io::ErrorKind::NotFound        => 66,
            AppError::Io(e) if e.kind() == std::io::ErrorKind::PermissionDenied => 77, // EX_NOPERM
            AppError::Io(_)                                    => 74,
            AppError::Other(_)                                 => 70,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage_is_64() {
        assert_eq!(AppError::Usage("bad flag".into()).exit_code(), 64);
    }

    #[test]
    fn permission_denied_is_77() {
        let e = AppError::Io(std::io::Error::from(std::io::ErrorKind::PermissionDenied));
        assert_eq!(e.exit_code(), 77);
    }
    // ... one test per arm
}
```

The matrix test isn't optional. Treat exit codes as part of the user-facing API and lock them down.

## BSD sysexits cheat sheet

The full code/constant/when matrix lives in
[`cli-design/02-error-messages.md#exit-codes--bsd-sysexits`](../../../programming/cli-design/02-error-messages.md#exit-codes--bsd-sysexits).
Map every `AppError` variant onto a constant from that table.

Don't use codes outside that set without writing them down. Shell scripts read your exit code.

**Hand-roll the mapping; don't take the [`sysexits`](https://docs.rs/sysexits/) crate.** Its enum has
sixteen variants — `0` and `64..=78` — with no arbitrary-`u8` variant and no `From<u8>`. The moment
your program owns a code outside that range (a `--strict` mode's `1`, a wrapped child's `128+S`
pass-through, a domain code like firecracker's signal range `148..=157`), the crate can express part
of your taxonomy and not the rest, and ownership splits in two. One local enum keeps one source of
truth. Firecracker and every project in the entry-point table below reached the same conclusion.

**Know that sysexits is deprecated upstream.** FreeBSD's own
[`sysexits(3)`](https://man.freebsd.org/cgi/man.cgi?query=sysexits&sektion=3) now says the interface
"has been deprecated and is retained only for compatibility. Its use is discouraged," and notes that
the choice of an appropriate value is often ambiguous. None of the reference projects in chapter 10
use sysexits values. This spec keeps them anyway — see ADR-0003 for why — but the honest consequence
is that **the code is worth less than the message.** Given a fixed budget, spend it on the rendered
diagnostic before spending it on refining the taxonomy.

## Per-layer error type examples

### Domain error (`src/domain/widget.rs`)

```rust
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("widget id must be 1..=64 chars, got {len}")]
    IdLength { len: usize },

    #[error("widget id contains invalid char: {0:?}")]
    IdInvalidChar(char),

    #[error("widget cannot transition from {from:?} to {to:?}")]
    InvalidTransition { from: WidgetState, to: WidgetState },
}
```

Pure: no `std::io::Error`, no `reqwest::Error`. Only states the domain cares about.

### Adapter error (`src/adapters/git.rs`)

```rust
#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("git binary not found in PATH")]
    NotFound,

    #[error("git invocation failed: {0}")]
    Spawn(#[from] std::io::Error),

    #[error("git exited {code}: {stderr}")]
    Failed { code: i32, stderr: String },
}
```

Wraps the external system's failure modes. One enum per adapter family.

### Service error (`src/services/widget.rs`)

```rust
#[derive(Debug, thiserror::Error)]
pub enum WidgetServiceError {
    #[error(transparent)]
    Domain(#[from] crate::domain::widget::DomainError),

    #[error(transparent)]
    Git(#[from] crate::adapters::git::GitError),

    #[error("widget {0} is locked by another process")]
    Locked(crate::domain::widget::WidgetId),
}
```

Composes domain and adapter errors. **Do not** add `#[from]` arms for peer services (e.g.
`OtherServiceError`) — recursive `#[from]` graphs cause ambiguous `?` inference and force you to
disambiguate at call sites. Instead, lift shared infrastructure into adapters or domain.

## The `Other(#[from] anyhow::Error)` arm

Inside `commands/`, sometimes you want `.context("while loading user prefs")` on a one-off call
without inventing a whole error variant. Add this arm to `AppError`:

```rust
#[error(transparent)]
Other(#[from] anyhow::Error),
```

Now this works:

```rust
use anyhow::Context;
let prefs = std::fs::read_to_string(&path)
    .with_context(|| format!("loading prefs from {path:?}"))?;  // -> AppError via #[from]
```

This is `riptask`'s approach (`src/error.rs:48-49`) and it's a good escape hatch.

## The process boundary: `main` delegates, `run` does the work

`main` returns `ExitCode` and contains no logic. The fallible program is a separate function — call
it `run`, `try_main`, or `dispatch` — and `main` is a five-to-ten-line adapter that renders the error
once and classifies it once. This is a rule, not a preference. See ADR-0002.

### Why the split is forced

Three facts compose:

1. `ExitCode` does not implement `Try`. The `?` operator is illegal anywhere in a function whose
   return type is `ExitCode`. A `main` that holds the logic must `match` every fallible call.
2. `fn main() -> Result<(), E>` compiles, but `impl<T: Termination, E: Debug> Termination for
   Result<T, E>` prints `Error: {err:?}` and returns `ExitCode::FAILURE` — always `1`, always the
   `Debug` rendering. It structurally cannot emit `64`, `70`, or `78`.
3. So you can have `?`, or a plain `main`, or specific exit codes — any two. The split buys all
   three.

Point 2 is the one people get wrong. `main() -> anyhow::Result<()>` is fine when "success versus
generic failure" is the whole contract; it is the wrong top-level signature the moment a sysexits
matrix exists. Verify against
[`Termination`](https://doc.rust-lang.org/std/process/trait.Termination.html) rather than assuming.

### `run` takes its inputs as parameters

`main` owns the connection to process globals — `std::env::args_os()`, and any env read that has to
happen before the config layer. Everything past `main` receives what it needs as arguments. A `run`
that reaches out to `args_os()` itself cannot be called twice, cannot be called with synthetic
input, and turns "the arguments were parsed and validated" into a convention rather than a fact of
the signature.

```rust
fn main() -> ExitCode {
    let outcome = match arguments(std::env::args_os()) {
        Ok(config) => run(&config),
        Err(e) => Err(e),
    };
    match outcome { /* render, classify */ }
}

fn run(config: &Config) -> Result<(), AppError> { ... }   // not callable until parsing succeeded
```

The payoff is that the parser becomes a pure function of an iterator, so the whole argument grammar
— missing flag, wrong order, relative path where an absolute one is required, trailing junk — is
unit-testable without spawning a process. That is usually the only genuinely testable part of a thin
binary.

ripgrep (`run(flags::parse())`), ruff (`run(Args::parse_from(args))`), and the Rust Book all do
this; the Book lists "calling the command line parsing logic with the argument values" among the
responsibilities that stay in `main`. With `clap` this falls out for free, since `Cli::parse()`
handles its own failures and exits — so `main` gains no second error path. With a hand-written
parser that returns `Result`, accept the one extra match arm in `main` rather than pushing the
`argv` read down.

### `impl Termination` for your own type does not replace `run`

`Termination` has been stable for user types since 1.61, and the release notes show a `GitBisectResult`
enum doing exactly what an `exit_code()` mapping does. It is legitimate. It also does not remove the
inner function, because `?` depends on `Try`, not on `Termination` — you end up with
`fn main() -> Report { Report(run()) }` and the `match` relocated into `report()`. Rendering inside
`report()` also hides a side effect in a conversion. Reach for it only when several binaries in one
workspace share exactly the same reporting policy.

### The shape

```rust
fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprint_error(&e);
            std::process::ExitCode::from(e.exit_code())
        }
    }
}

fn run() -> Result<(), AppError> {
    let cli = app_template::cli::Cli::parse();
    let ctx = app_template::context::AppContext::new(&cli)?;
    app_template::dispatch(&ctx, cli)
}

fn eprint_error(e: &app_template::error::AppError) {
    eprintln!("app: {e}");
    let mut source = std::error::Error::source(e);
    while let Some(cause) = source {
        eprintln!("  caused by: {cause}");
        source = cause.source();
    }
}
```

The chain walk surfaces `#[source]` and `#[from]` causes and remains the default. When a concrete
need justifies a prettier renderer, choose from the
[diagnostic comparison](./07-dependencies.md#diagnostics-and-panics). Pretty renderers must obey the
centralized machine and color policy; do not assume a renderer's global hooks or detector does so.

The split predates the exit-code argument: the Rust Book prescribes it for binary crates so the
logic is testable and `main` is verifiable by inspection
([ch12-03](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html)).
Exit codes are the second, independent reason.

### What real projects do

Every widely-studied Rust binary splits. What varies is only the return types.

| Project          | `main` returns             | Inner fn returns              | Error type                 | Code space                   |
| ---------------- | -------------------------- | ----------------------------- | -------------------------- | ---------------------------- |
| firecracker      | `ExitCode`                 | `Result<(), MainError>`       | `thiserror` enum           | `FcExitCode` enum + `From`   |
| ruff             | `ExitCode`                 | `anyhow::Result<ExitStatus>`  | `anyhow`                   | `ExitStatus` enum + `From`   |
| ripgrep          | `ExitCode`                 | `anyhow::Result<ExitCode>`    | `anyhow` + downcast        | `0`/`1`/`2`                  |
| cargo            | `()`                       | `CliResult`                   | `anyhow` inside `CliError` | free `i32` on the error      |
| fd               | `()`                       | `Result<ExitCode>`            | `anyhow`                   | own enum + `.exit()`         |
| bat, hyperfine   | `()`                       | `Result<bool>` / `Result<()>` | `anyhow`                   | `0`/`1` via `process::exit`  |
| cloud-hypervisor | `()`                       | `Result<(), Error>`           | `thiserror` enum           | `0`/`1` via `process::exit`  |
| rust-analyzer    | `anyhow::Result<ExitCode>` | same                          | `anyhow`                   | `0`, or std's `1` on failure |

Two patterns fall out. Projects needing more than two codes all introduce a **named code type**
rather than bare integer constants at call sites. And projects whose exit code carries no contract
(rust-analyzer: an LSP server; a GUI app) can accept `Termination for Result`'s exit `1` — that
option is unavailable the moment a matrix is documented.

This spec's default is the firecracker/ruff shape: `main -> ExitCode`, inner `-> Result<_, AppError>`,
one `exit_code()` mapping, rendering at the boundary. Prefer returning `ExitCode` over
`std::process::exit`, which skips destructors on every stack — relevant whenever the program holds
child processes, file sinks, or a `WorkerGuard`. Enable `clippy::exit = "deny"` to enforce it.

## Binaries a service manager supervises

A daemon started by systemd, launchd, or a supervisor tree is a third audience alongside human and
machine-facing, and the rules invert.

**The exit code buys less than you think.** systemd's behaviour is a `0`-versus-nonzero decision plus
whatever `SuccessExitStatus=`, `RestartPreventExitStatus=`, and `RestartForceExitStatus=` explicitly
name. `64`, `70`, and `74` are behaviourally identical unless the unit lists them. Set those
properties deliberately or accept that the codes are diagnostic breadcrumbs in `systemctl status`,
not a control surface. `RestartPreventExitStatus=64 70` — never retry a usage error or an internal
defect, do retry a transient I/O failure — is usually the useful pairing.

**The message buys more.** The unit's `MainPID` has stderr wired to the journal, and that is the only
place a failure can be explained. A supervised binary that classifies its error and prints nothing has
built the less valuable half. Firecracker logs the error to its log face _and_ `eprintln!`s it before
converting; copy that ordering — render, then classify.

**Do not erase the cause on the way out.** The typed error from the layer below is the whole account
of what happened; a supervisor that collapses it to a category has nothing left to write.

## Anti-patterns

| Anti-pattern                                | Why it fails                                                                                                                                                         |
| ------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Result<(), u8>` / `Result<(), (u8, &str)>` | Implements no `Error`, no `Display`, no `source()`. Nothing converts via `?`, so every call site needs a manual `map_err` — the type is what forces the erasure.     |
| `.map_err(\|_\| EX_IOERR)`                  | Discards the `io::Error` that says whether it was `ENOENT`, `EACCES`, or `EISDIR`. Use `.map_err(AppError::ReadSpec)` and keep it as `#[source]`.                    |
| `Result<T, ()>`                             | The API Guidelines condemn `()` as an error type by name. Use a named unit variant.                                                                                  |
| Classification at each call site            | Thirteen independent `map_err`s each pick a code, so nobody can audit which codes a command can return. One exhaustive `match` in `exit_code()` is compiler-checked. |
| `u8` as the code type                       | Admits 256 values where the matrix admits ten. A closed enum makes an off-taxonomy code unconstructible.                                                             |

The failure mode these share is silent drift: the code compiles, the tests pass, and the taxonomy in
the README stops describing the binary.

## Rules

- `main` returns `ExitCode` and holds no logic. The fallible program is `run`. (ADR-0002)
- No `std::process::exit` — return the `ExitCode` so destructors run. Enforce with
  `clippy::exit = "deny"`.
- Render the error chain before classifying it. A binary that returns a code and prints nothing has
  shipped the less useful half.
- No `panic!`, `.unwrap()`, `.expect()` outside `main`, tests, build scripts, and `LazyLock`
  initializers.
- No catch-all `_ => 1` in `exit_code()`. Map every variant explicitly.
- Every public function returning `Result` documents the error variants it can produce.
- Don't wrap `String` errors. If you find yourself reaching for `anyhow!("...")`, ask whether a
  named variant would be clearer — for one-offs at the binary edge, `anyhow!` is fine; in libs, name
  it.

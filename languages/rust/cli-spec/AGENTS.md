---
digest-of: languages/rust/cli-spec
last-synced: 2026-08-07
token-estimate: 800
---

# AGENTS

## Scope

Rust-specific CLI implementation spec applying the general principles from
`programming/cli-design/`. Covers directory layout, crate organization, subcommand patterns,
error handling, logging, config, dependencies, naming, coding style, and reference projects. The
spec references the general facing-category taxonomy and records only Rust idioms.

## Key Points

### Crate Stack Defaults

- Parser: `clap` (derive, env, wrap_help). Config: `figment` (env, toml). XDG: `directories`.
  Errors: `thiserror` + `anyhow`. Logging: `tracing` + `tracing-subscriber` + `tracing-appender`.
  Serialization: `serde` + `serde_json` + `toml`. Self-documentation: `clap_complete` and
  `clap_mangen`. Paths: `camino`. Async: `tokio` (rt, macros). Tests: `assert_cmd` + `predicates`
  - `insta` + `tempfile`. Runner: `cargo nextest`.
- Human-facing color defaults to `anstyle` plus `anstream`; optional `owo-colors` is only an
  ergonomic emitter with `supports-colors` disabled. Resolve one choice and apply it at the stream.
- Tables: `comfy-table` auto-fits but is feature-frozen and seeking a maintainer; `tabled` needs an
  explicit width and its `ansi` feature for colored cells. Use `unicode-width`, never byte/character
  count.
- Progress: `indicatif` fails closed on non-TTY/`TERM=dumb`; combine active bars with
  `tracing-indicatif` and explicitly suppress them in machine mode. Prompts: choose richer,
  solo-owned `inquire` or multi-owner, zeroizing-password `dialoguer` by need.
- Diagnostics: `annotate-snippets` is span-only; `miette` supports spanless diagnostics;
  `color-eyre` misses `NO_COLOR`. Global report and panic hooks make renderer selection exclusive.

### Directory Tree

- `src/main.rs` (<=120 LOC), `cli/` (clap derive only), `commands/` (handlers), `domain/` (pure, no
  I/O), `adapters/` (traits + impls), `services/` (optional), `config/`, `context.rs`, `error.rs`,
  `logging.rs`, conditional `ui/` for human-facing CLIs or `output/`/`protocol/` for machine-facing
  CLIs, `util/`.
- `tests/cmd_<name>.rs` per subcommand. Unit tests inline in `#[cfg(test)]`.
- Templates include a human-facing `main.rs.template` and a machine-facing
  `main.rs.machine.template`.

### Subcommand Pattern (Four-Edit Rule)

- `cli/<name>.rs` (clap Args), `cli/mod.rs` (register variant), `commands/<name>.rs` (free `run`
  fn), `main.rs` (dispatch arm).
- Handler signature: `pub fn run(ctx: &AppContext, args: <Verb>Args) -> Result<(), AppError>`.
- Parse-shape to runtime-shape projection at top of handler via `Request::from_cli(args)`.
- Help: `about` + `after_long_help = include_str!(...)` for human-facing narrative addenda. Keep
  machine-facing `help`/usage, `doctor`, `init`, completion, and man surfaces terse and parseable.

### Error Handling

- `thiserror` per layer: `DomainError` (invariants only), `<Sys>AdapterError` (I/O), `ServiceError`,
  `AppError` (top-level with `#[from]` arms + `Other(anyhow::Error)`).
- `AppError::exit_code() -> u8` mapped to BSD sysexits. Unit-test every arm.
- No `unwrap`/`expect` outside main, tests, build scripts, `LazyLock`.

### The Process Boundary (ADR-main-delegates-to-run)

- `main` returns `ExitCode`, holds no logic, and owns two things: the connection to process globals
  (`args_os()`), and rendering-then-classifying the error. The fallible program is `run`.
- `run` receives its inputs as parameters and reads no process global, so its preconditions are
  facts of the signature and the parser stays a pure, testable function of an iterator.
- Forced, not stylistic: `ExitCode` does not implement `Try` (no `?` in `main`), and
  `Termination for Result` hardcodes exit `1` plus a `Debug` dump, so `fn main() -> Result<_, _>`
  cannot emit a sysexits code. Disallowed for any binary with a documented exit matrix.
- No `std::process::exit` — return the `ExitCode` so destructors run. Enforce with
  `clippy::exit = "deny"`. `impl Termination` for an owned type is permitted, not default: it still
  needs `run`.
- Service-manager-supervised binaries are a third audience beside human/machine-facing. systemd
  distinguishes only zero from non-zero unless `SuccessExitStatus=`/`RestartPreventExitStatus=`/
  `RestartForceExitStatus=` name a code, so the journal message is the payload and the code is a
  breadcrumb. Render before classifying; never erase the cause on the way out.
- Anti-patterns: `Result<(), u8>`, `Result<(), (u8, &str)>`, `Result<T, ()>`,
  `.map_err(|_| CODE)`, and per-call-site code selection.

### sysexits Caveat (ADR-keep-bsd-sysexits-despite-deprecation)

- Keep BSD sysexits as the default taxonomy, but know it is deprecated upstream by `sysexits(3)`
  itself and unused by most reference projects. Consequence: the code is worth less than the
  message; spend the budget on the diagnostic first.
- Hand-roll the mapping. Do NOT take the `sysexits` crate: it admits only `0` and `64..=78`, so any
  owned code outside that range splits the taxonomy across two sources.

### Logging

- `tracing` for emission. `tracing-subscriber` with `EnvFilter` and `tracing-appender` for file
  sink.
- Honor `RUST_LOG`. File: JSON, no ANSI. `mirror_stderr=false` is the machine-facing default;
  human-facing CLIs set mirroring from flags, verbosity policy, or config.
- Hold the `WorkerGuard` for program lifetime.

### Config

- `figment` layered: defaults -> user file -> project file -> env (`APP_*`) -> CLI (serialized
  GlobalArgs).
- `directories::ProjectDirs` for XDG. `deny_unknown_fields` on Config struct. Immutable after
  construction.

### Dependencies

- Manage deps through Cargo's CLI only: `cargo add`/`cargo remove`/`cargo update`. Never hand-edit
  dependency names/versions/features in `Cargo.toml`. Commit `Cargo.lock` for binaries. (ADR-cargo-cli-only-dependencies)
- Current majors: `thiserror` 2, `toml` 1 (TOML spec 1.1), `anstream` 1.0.
- Avoid: `env_logger`, `log`, `structopt`, `failure`, `confy`, `dirs`, `lazy_static`, `serde_yaml`,
  unmaintained `ansi_term`, and maintenance-only legacy color crates as new defaults.
- Prefer: `std::sync::LazyLock` over `once_cell`. `time` over `chrono` when possible.

### Naming and Visibility

- Default `pub(crate)`. `foo.rs + foo/` over `mod.rs`. `<Verb>Args`, `<Verb>Request`,
  `<Layer>Error`.
- Every `pub`/`pub(crate)` item has a `///` doc comment. Module headers with purpose and
  non-purpose.
- Function placement, first match wins: has a receiver -> method; no receiver but converts one
  settled domain type to another -> associated fn on the output (`XdgPaths::resolve`,
  `HiArgs::from_low_args`); no receiver and consumes raw outside-world shape or is one stage of a
  module-owned pipeline -> free fn (`flags::parse`, `commands::dispatch::classify`). "Builds one
  type" is not the test; `C-CONV-SPECIFIC` is, and `&[OsString]` is the least specific type in a CLI.
  `C-METHOD` governs receivers only. Argv classification is always the free-function case.

### Coding Style

- No `.unwrap()` outside safe zones. Trait objects only when justified. Closures via `impl Fn`. No
  crate-root `#![allow(dead_code)]`.
- Clippy pedantic + nursery as warn. `unwrap_used` and `expect_used` as warn.
- CI lint: `rg 'println!|eprintln!' src/ --glob '!src/ui/**' --glob '!src/main.rs'`.

## Maintenance Notes

- Chapter 06 (testing) is a subdirectory; load directly when reviewing test strategy.
- Templates in `templates/` provide bootstrap skeletons; not digested here.
- The spec's own decision records live in `decisions/` and are cited by slug in the sections above.
- Regenerate when any chapter file changes or crate ecosystem shifts.

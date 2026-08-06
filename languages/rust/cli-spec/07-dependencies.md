# 07 — Dependencies (Rust)

> Prerequisite: the canonical principles each crate implements live in
> [`cli-design/`](../../../programming/cli-design/) — specifically
> [`01-logging-and-output`](../../../programming/cli-design/01-logging-and-output.md) (`tracing`,
> `tracing-subscriber`, `tracing-appender`),
> [`02-error-messages`](../../../programming/cli-design/02-error-messages.md) (`thiserror`,
> `anyhow`), [`03-config-precedence`](../../../programming/cli-design/03-config-precedence.md)
> (`figment`, `directories`), and
> [`08-testing-and-quality`](../../../programming/cli-design/09-testing-and-quality/testing-strategy.md)
> (`assert_cmd`, `insta`, `tempfile`, `nextest`). This chapter is the curated default crate list
> with one-line justifications. Facing-category consequences follow
> [General — Facing category & message types](../../../programming/cli-design/00-architecture.md#facing-category--message-types).

Opinionated default dependency list. Each entry has a one-line justification and a "skip if"
condition. Pick deliberately; resist the urge to add "useful-looking" crates without a concrete
need.

## Adding dependencies

**Manage every dependency through Cargo's CLI — never hand-edit dependency names, versions, or
features in `Cargo.toml`.**

- Add or change a crate with `cargo add <crate>` (e.g.
  `cargo add clap --features derive,env,wrap_help`). `cargo add` fetches the latest
  SemVer-compatible version, resolves the whole graph, writes the entry into `Cargo.toml`, and
  updates `Cargo.lock` in one step.
- Remove a crate with `cargo remove <crate>`.
- Bump the lockfile with `cargo update` — deliberately, and read the changelog first.
- Add a dev-dependency with `cargo add --dev <crate>`; add a feature to a crate you already depend
  on with `cargo add <crate> --features <feat>`.
- Do **not** type version strings or feature lists into `Cargo.toml` by hand. Hand-edits drift from
  the resolver, skip the lockfile update, and pin stale versions. Agents and contributors change
  dependencies only through these commands.
- Commit `Cargo.lock` for binaries (not for libraries); `cargo add`/`cargo update` keep it in sync.

The [pinning policy](#pinning-policy) below still governs the resulting entries: majors in
`Cargo.toml`, exact versions in `Cargo.lock`.

## Runtime defaults

Always:

| Crate                | Features                     | Why                                                                                           | Skip if                                                                            |
| -------------------- | ---------------------------- | --------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| `clap`               | `derive`, `env`, `wrap_help` | Standard parser; `env` lets flags fall back to env vars; `wrap_help` makes `--help` readable. | Never.                                                                             |
| `clap_complete`      | —                            | Generates shell completions via a subcommand.                                                 | The CLI is dev-internal and shell completions aren't needed.                       |
| `clap_mangen`        | —                            | Generates man pages; expose them through a subcommand when useful.                            | The CLI is dev-internal and man pages aren't needed.                               |
| `anyhow`             | —                            | Ad-hoc context-rich errors at the binary edge.                                                | You're writing a pure library (use only `thiserror`).                              |
| `thiserror`          | —                            | Typed error enums in `domain/`, `adapters/`, `services/`, `error.rs`.                         | Never.                                                                             |
| `tracing`            | —                            | Structured logging primitive. Emitted everywhere.                                             | Never.                                                                             |
| `tracing-subscriber` | `env-filter`, `fmt`          | Installs the subscriber in `main`.                                                            | Never (in a binary).                                                               |
| `tracing-appender`   | —                            | Non-blocking file sink.                                                                       | Only for a throwaway dev-internal CLI.                                             |
| `serde`              | `derive`                     | Universal serialization.                                                                      | Never.                                                                             |
| `serde_json`         | —                            | JSON I/O.                                                                                     | No JSON I/O anywhere.                                                              |
| `toml`               | —                            | Config file parsing (via figment).                                                            | No config files.                                                                   |
| `figment`            | `env`, `toml`                | Layered config with source provenance.                                                        | Trivial single-file config (use `toml` + `serde` directly).                        |
| `directories`        | —                            | XDG/Windows/macOS config-path resolution.                                                     | No cross-platform config paths needed.                                             |
| `camino`             | `serde1`                     | UTF-8-only paths; kills `Path::to_string_lossy()` boilerplate.                                | The CLI only operates on user-supplied paths it never round-trips through strings. |
| `tokio`              | `rt`, `macros`               | Single current-thread runtime for async.                                                      | Fully sync CLI.                                                                    |

## Human presentation

These are concern-triggered additions for a human-facing CLI, not a bundle every application needs.
Machine output and log sinks remain undecorated under the
[canonical presentation policy](../../../programming/cli-design/01-logging-and-output.md#color).

### Color layers

| Layer             | Default or addition                               | Ruling                                                                                                            |
| ----------------- | ------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Style vocabulary  | [`anstyle`](https://docs.rs/anstyle/)             | Default public style types; no rendering engine or detector.                                                      |
| Stream adaptation | [`anstream`](https://docs.rs/anstream/)           | Default sole owner of pass-through, stripping, and legacy-Windows adaptation.                                     |
| Policy primitives | [`anstyle-query`](https://docs.rs/anstyle-query/) | Conformant low-level OS-string and terminal lookups when implementing the ladder; it does not read `FORCE_COLOR`. |
| Resolved choice   | [`colorchoice`](https://docs.rs/colorchoice/)     | Carry the startup decision as `Auto`, `Always`, `AlwaysAnsi`, or `Never`; renderers do not decide again.          |
| Ergonomic emitter | [`owo-colors`](https://docs.rs/owo-colors/)       | Optional `Display` syntax above `anstream`; keep its non-default `supports-colors` feature disabled.              |

No crate supplies the complete recommended ladder for free. Resolve policy once at startup, make
machine mode unconditional `Never`, and apply or strip escapes at the actual stream. Enabling
`owo-colors`' `supports-colors` feature introduces a second detector with different rules.
[`colorchoice-clap`](https://docs.rs/colorchoice-clap/) is a supporting addition for ordinary CLIs
that expose `--color=auto|always|never`; a passthrough wrapper may have a product reason not to own
that flag.

The default family is maintained by the rust-cli organization in the
[`rust-cli/anstyle` repository](https://github.com/rust-cli/anstyle) under MIT or Apache-2.0. `clap`
uses `anstyle` unconditionally in its
[public `Styles` API](https://docs.rs/clap/latest/clap/builder/struct.Styles.html), and its default
`color` feature brings `anstream`, so ordinary `clap` projects generally add direct edges rather
than a new dependency subtree. This choice is governed by
[ADR-0004](./adr/0004-use-anstyle-and-anstream-for-terminal-colour.md).

#### Color alternatives and avoid list

| Crate                                                      | Ruling                                                                                                                                                                      |
| ---------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`colored`](https://github.com/colored-rs/colored)         | Avoid: process-global color state, stdout terminal probing even for stderr, and MPL-2.0 make it conflict with stream-local policy.                                          |
| [`nu-ansi-term`](https://github.com/nushell/nu-ansi-term)  | Maintenance-only; not a 2026 default even though it remains a compatible escape emitter.                                                                                    |
| [`yansi`](https://github.com/SergioBenitez/yansi)          | Dormant since 2024; do not choose it for a new default.                                                                                                                     |
| [`ansi_term`](https://crates.io/crates/ansi_term)          | Unmaintained under [RUSTSEC-2021-0139](https://rustsec.org/advisories/RUSTSEC-2021-0139.html); do not use.                                                                  |
| [`termcolor`](https://github.com/BurntSushi/termcolor)     | Supported for existing users but maintenance-only for new code; [Cargo migrated to `anstream`](https://github.com/rust-lang/cargo/issues/12627).                            |
| [`supports-color`](https://github.com/zkat/supports-color) | Stale and unsuitable as policy owner: it deliberately treats `NO_COLOR=0`, `NO_COLOR=""`, `FORCE_COLOR=0`, and `FORCE_COLOR=""` differently from the published conventions. |
| [`console`](https://github.com/console-rs/console)         | A broader terminal abstraction with its own cached policy, including Unix-only `NO_COLOR` handling and `CLICOLOR_FORCE` winning over it; not the default color-only layer.  |

### Tables

| Crate                                                   | Choose when                                                                   | Caveat                                                                                                                                                                                                                                                                  |
| ------------------------------------------------------- | ----------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`comfy-table`](https://github.com/Nukesor/comfy-table) | The table should auto-fit terminal width through its default TTY integration. | Version 8.0.0, released 2026-08-05, removed `TableComponent`, `modifiers`, `load_preset`, and `set_style` while the project describes itself as feature-frozen and [seeks a maintainer](https://github.com/Nukesor/comfy-table/issues/202). Revalidate before adoption. |
| [`tabled`](https://github.com/zhiburt/tabled)           | The caller should own width and layout policy.                                | It performs no terminal detection: supply a width, commonly from [`terminal_size`](https://docs.rs/terminal_size/), and enable its non-default `ansi` feature for pre-colored cells.                                                                                    |

Display width is not byte length or character count. Use
[`unicode-width`](https://github.com/unicode-rs/unicode-width) for CJK, combining characters, and
emoji; never size columns with `str::len()` or `.chars().count()`.

### Progress

[`indicatif`](https://github.com/console-rs/indicatif) remains the default for progress and
spinners. Its stderr draw target automatically hides when stderr is not a terminal or `TERM=dumb`.
Normal `tracing` or `log` writes can corrupt an active bar; use
[`tracing-indicatif`](https://github.com/console-rs/tracing-indicatif) when tracing and bars coexist,
not the unhealthy [`indicatif-log-bridge`](https://docs.rs/indicatif-log-bridge/).

`indicatif` inherits `console`'s independent cached color policy: `NO_COLOR` handling is Unix-only,
and `CLICOLOR_FORCE` can override it. Machine mode must explicitly hide or neutralize progress
rather than trust that second policy.

### Prompts

| Crate                                                  | Choose when                                                                                                      | Policy and maintenance caveat                                                                                                       |
| ------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| [`inquire`](https://github.com/mikaelmello/inquire)    | Rich prompts, autocomplete, validators, date selection, or a swappable backend matter.                           | It releases more often but is solo-owned. Replacing its render config bypasses its built-in `NO_COLOR` handling.                    |
| [`dialoguer`](https://github.com/console-rs/dialoguer) | A smaller API, multi-owner maintenance, zeroizing password input, or sharing `console` with `indicatif` matters. | It is more recently committed despite less-frequent releases and inherits `console`'s platform-dependent, cached color limitations. |

Prompts go to stderr and must fail rather than appear in machine or non-interactive mode.

### Diagnostics and panics

| Crate                                                                    | Choose when                                                               | Caveat                                                                                                                                                            |
| ------------------------------------------------------------------------ | ------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`annotate-snippets`](https://github.com/rust-lang/annotate-snippets-rs) | A healthy, focused source-span renderer is the whole need.                | It is the wrong tool when no source span exists.                                                                                                                  |
| [`miette`](https://github.com/zkat/miette)                               | Diagnostics need codes, help, severity, cause chains, and optional spans. | For deterministic machine output force `MietteHandlerOpts::new().unicode(false).color(false)`; its fancy renderer otherwise has independent capability detectors. |
| [`color-eyre`](https://github.com/eyre-rs/eyre)                          | Interactive report and panic presentation with spantraces is wanted.      | It [does not honor `NO_COLOR`](https://github.com/eyre-rs/eyre/issues/236) and installs global report and panic hooks.                                            |

`miette` and `color-eyre` both install global report hooks and must not be combined. The proposed
`miette` migration remains only a proposal. [`human-panic`](https://github.com/rust-cli/human-panic)
is a conditional user-friendly panic-report alternative, but it also owns a panic hook and cannot
be stacked casually with `color-eyre`. Every pretty renderer must receive or obey the centralized
machine/color choice.

## Dev-dependencies

| Crate        | Features | Why                                    | Skip if                                      |
| ------------ | -------- | -------------------------------------- | -------------------------------------------- |
| `assert_cmd` | —        | Process-level CLI tests.               | No integration tests (you should have them). |
| `predicates` | —        | Assertion helpers for `assert_cmd`.    | Same.                                        |
| `insta`      | `yaml`   | Snapshot tests for structured output.  | No structured output to snapshot.            |
| `tempfile`   | —        | Isolated temp dirs per test.           | Same.                                        |
| `trybuild`   | —        | Compile-fail tests for typestate APIs. | No typestate.                                |

## Conditional adds

Add only when a specific concrete need arises.

| Crate                                                      | When                                                                                                                                                      |
| ---------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `colorchoice-clap`                                         | An ordinary human-facing CLI exposes a `--color` option with `auto`, `always`, and `never`; skip when a recorded product constraint forbids that surface. |
| `miette`, `annotate-snippets`, `color-eyre`, `human-panic` | A concrete diagnostic or panic need selects one compatible role from the comparison above.                                                                |
| `chrono` or `time`                                         | Real date/time handling. Prefer `time` (smaller, no globally-mutable timezone state). Use `chrono` only if dep tree already pulls it.                     |
| `glob`                                                     | Glob pattern matching (e.g. `*.txt`).                                                                                                                     |
| `regex`                                                    | Regular expressions. Heavyweight; consider whether `str::contains` suffices first.                                                                        |
| `reqwest`                                                  | HTTP client. Use `rustls` backend, not `native-tls`, to keep static builds working.                                                                       |
| `rusqlite` or `sqlx`                                       | SQL persistence. Pick `sqlx` only if you need async or compile-time query checking.                                                                       |
| `crossterm`                                                | Human-UX TUI input handling. Reach for it only if you need raw mode.                                                                                      |
| `indicatif`                                                | Human-UX progress bars.                                                                                                                                   |
| `dialoguer`                                                | Human-UX interactive prompts.                                                                                                                             |
| `dirs`                                                     | Old XDG helper. **Don't use**; `directories` supersedes it.                                                                                               |
| `walkdir`                                                  | Filesystem traversal with depth/symlink controls.                                                                                                         |
| `ignore`                                                   | `.gitignore`-aware traversal. Bigger than `walkdir`; use only if you actually need gitignore semantics.                                                   |
| `which`                                                    | Locate executables on PATH.                                                                                                                               |
| `tempfile` (runtime, not dev)                              | If your CLI creates real tempfiles (not just tests).                                                                                                      |
| `parking_lot`                                              | Faster mutex/rwlock than std. Only if profiling shows lock contention.                                                                                    |
| `rayon`                                                    | Data parallelism. Use only when the workload is genuinely parallelizable and the CLI isn't I/O-bound.                                                     |
| `once_cell` / `std::sync::LazyLock`                        | Prefer std `LazyLock` (1.80+) over `once_cell::sync::Lazy` for new code.                                                                                  |

## Avoid by default

Deprecated, dead, or superseded:

| Crate                           | Why avoid                              | Use instead                   |
| ------------------------------- | -------------------------------------- | ----------------------------- |
| `env_logger`                    | Old; doesn't integrate with spans.     | `tracing-subscriber`.         |
| `log` (directly)                | Lacks structured fields and spans.     | `tracing`.                    |
| `structopt`                     | Merged into clap; deprecated.          | `clap` derive.                |
| `failure`                       | Unmaintained.                          | `thiserror` + `anyhow`.       |
| `error-chain`                   | Unmaintained.                          | Same.                         |
| `confy`                         | Can't layer multiple files.            | `figment`.                    |
| `dirs`                          | Maintenance moved to `directories`.    | `directories`.                |
| `chrono` (if you have a choice) | Global timezone state, larger surface. | `time`.                       |
| `lazy_static`                   | Older, macro-based.                    | `std::sync::LazyLock`.        |
| `rustc-serialize`               | Pre-`serde`.                           | `serde`.                      |
| `serde_yaml`                    | Unmaintained.                          | `serde_yaml_ng` or skip YAML. |

## Cargo.toml skeleton

Hand-author only the non-dependency tables — `[package]`, `[[bin]]`, `[profile.release]`. Everything
under `[dependencies]`/`[dev-dependencies]` is installed with `cargo add` (see
[Adding dependencies](#adding-dependencies)); never type crate versions in by hand.

```toml
[package]
name        = "app-template"
version     = "0.1.0"
edition     = "2024"
rust-version = "1.85"
license     = "MIT OR Apache-2.0"
description = "One-line description."
repository  = "https://github.com/you/app-template"

[[bin]]
name = "app"
path = "src/main.rs"

[profile.release]
lto           = "thin"
codegen-units = 1
strip         = "symbols"
```

Install the default runtime stack — each command resolves the latest SemVer-compatible version,
writes it into `[dependencies]`, and updates `Cargo.lock`:

```sh
cargo add clap --features derive,env,wrap_help
cargo add clap_complete clap_mangen
cargo add anyhow thiserror
cargo add tracing
cargo add tracing-subscriber --features env-filter,fmt
cargo add tracing-appender
cargo add serde --features derive
cargo add serde_json toml
cargo add figment --features env,toml
cargo add directories
cargo add camino --features serde1
cargo add tokio --features rt,macros

# Human color defaults (add when the CLI is human-facing):
cargo add anstyle anstream

# Examples: add only the presentation concerns the CLI actually uses:
cargo add comfy-table
cargo add indicatif tracing-indicatif
cargo add dialoguer
cargo add miette --features fancy

# Dev-dependencies:
cargo add --dev assert_cmd predicates tempfile
cargo add --dev insta --features yaml
```

The resolved `[dependencies]` block will look roughly like the following. **Illustrative only —
install via `cargo add`; do not hand-copy these versions.** As of 2026-08, the verified core majors
are `thiserror` 2, `toml` 1 (TOML spec 1.1), and `anstream` 1:

```toml
[dependencies]
clap                = { version = "4", features = ["derive", "env", "wrap_help"] }
clap_complete       = "4"
clap_mangen         = "0.2"
anyhow              = "1"
thiserror           = "2"
tracing             = "0.1"
tracing-subscriber  = { version = "0.3", features = ["env-filter", "fmt"] }
tracing-appender    = "0.2"
serde               = { version = "1", features = ["derive"] }
serde_json          = "1"
toml                = "1"
figment             = { version = "0.10", features = ["env", "toml"] }
directories         = "5"
camino              = { version = "1", features = ["serde1"] }
tokio               = { version = "1", features = ["rt", "macros"] }

[dev-dependencies]
assert_cmd  = "2"
predicates  = "3"
insta       = { version = "1", features = ["yaml"] }
tempfile    = "3"
```

Trim aggressively for very small CLIs (drop `tokio`, `figment`, `directories` if you don't need
them). Add to taste from the conditional list above.

## Pinning policy

- Pin to **major versions** in `Cargo.toml` (`"4"`, `"0.3"`). Let `Cargo.lock` pin exact versions.
- Commit `Cargo.lock` for binaries. Don't for libraries.
- Run `cargo update` deliberately, not as a default `just` task. Read the changelog.
- Use `cargo deny` (configured in `deny.toml`) to enforce a license allowlist and ban yanked or
  vulnerable versions.

## Justification: figment over config-rs

Both work. Pick figment because:

- It tracks per-key source provenance. Errors say _"`timeout` in `./app.toml` (line 12) was
  negative"_ instead of _"invalid config"_.
- Its provider model maps cleanly onto our layered precedence (defaults → user → project → env →
  CLI).
- `config-rs` requires more glue for the same outcome and has noisier errors.

If you're committed to `serde_path_to_error` and don't need source tracking, `config-rs` is fine.

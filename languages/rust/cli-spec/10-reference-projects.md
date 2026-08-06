# 10 — Reference Projects (Rust)

> Prerequisite:
> [General principles — Reference Projects](../../../programming/cli-design/10-reference-projects.md)
> for the language-agnostic organizational patterns these projects exemplify. This chapter zooms in
> on the Rust specifics.

Real-world Rust CLIs worth studying. For each: shape, what they do well, what to copy.

## Single-crate CLIs

### [`sharkdp/fd`](https://github.com/sharkdp/fd) — file finder

Single crate, ~10k LOC. Layout:

```text
src/
├─ main.rs
├─ cli.rs               # clap derive
├─ config.rs
├─ walk.rs              # core traversal
├─ output.rs
├─ error.rs
├─ exit_codes.rs        # explicit exit-code mapping
├─ exec/                # --exec implementation
├─ filter/              # type/extension/size filters
└─ fmt/                 # formatting
```

Copy: the dedicated `exit_codes.rs`, the per-feature subdirectories (`exec/`, `filter/`, `fmt/`).

### [`sharkdp/bat`](https://github.com/sharkdp/bat) — pretty `cat`

**The canonical lib+bin pattern.** `src/lib.rs` exposes the rendering engine; `src/bin/bat/` is the
thin CLI on top:

```text
src/
├─ lib.rs               # public API
├─ assets.rs
├─ printer.rs
├─ ...
└─ bin/
   └─ bat/
      ├─ main.rs
      ├─ app.rs         # clap
      └─ ...
```

Copy: the `src/lib.rs` + `src/bin/<name>/` split when your logic is reusable.

### [`extrawurst/gitui`](https://github.com/extrawurst/gitui) — git TUI

Single crate, TUI-centric:

```text
src/
├─ main.rs
├─ app.rs               # top-level state machine
├─ args.rs              # clap
├─ components/          # one file per TUI component
├─ tabs/
├─ popups/
└─ keys/
```

Copy: the `components/` + `tabs/` + `popups/` split for TUI projects. Less relevant for pure CLIs
but instructive.

### [`ouch-org/ouch`](https://github.com/ouch-org/ouch) — universal compressor

```text
src/
├─ main.rs
├─ cli/
├─ commands/
├─ utils/
├─ archive/             # topical: archive formats
└─ non_archive/         # topical: single-file compressors
```

Copy: the `cli/` + `commands/` split this spec recommends. Topical dirs (`archive/`, `non_archive/`)
for plugin-like features.

## Workspace CLIs

### [`BurntSushi/ripgrep`](https://github.com/BurntSushi/ripgrep) — fast grep

Workspace with reusable library crates:

```text
crates/
├─ core/                # bin + flags + glue
├─ matcher/             # grep-matcher: matcher trait
├─ regex/               # grep-regex: regex matcher impl
├─ searcher/            # grep-searcher: search runner
├─ printer/             # grep-printer: output formats
├─ pcre2/
├─ globset/             # globbing
├─ ignore/              # gitignore traversal
└─ ...
```

Copy: extracting reusable subsystems into library crates when they have value independent of the
binary. The `globset` and `ignore` crates are used by dozens of other projects.

### [`rust-lang/cargo`](https://github.com/rust-lang/cargo) — Rust's package manager

```text
src/
├─ cargo/               # the library
└─ bin/
   └─ cargo/
      ├─ main.rs
      └─ commands/      # one file per subcommand, uniform `exec()` signature
```

Each `commands/<name>.rs` exposes:

```rust
pub fn cli() -> clap::Command { ... }
pub fn exec(gctx: &GlobalContext, args: &ArgMatches) -> CliResult { ... }
```

Copy: the uniform `exec()` signature across subcommands. Cargo uses imperative clap (not derive) but
the discipline is the same.

### [`jj-vcs/jj`](https://github.com/jj-vcs/jj) — Jujutsu VCS

```text
cli/src/
├─ main.rs
├─ cli_util.rs
├─ command_error.rs
├─ commands/            # one file per subcommand
└─ ui.rs                # bounded UI module
```

Copy: `command_error.rs` as a focused error module; `ui.rs` as the single rendering surface.

### [`eza-community/eza`](https://github.com/eza-community/eza) — modern `ls`

```text
src/
├─ main.rs
├─ options/             # clap + arg parsing
├─ output/              # rendering
├─ fs/                  # filesystem abstractions
└─ theme/
```

Copy: the `options/` (parsing) + `output/` (rendering) split if your CLI has substantial formatting
logic.

### [`starship/starship`](https://github.com/starship/starship) — shell prompt

```text
src/
├─ main.rs
├─ context.rs           # one big AppContext-like struct
├─ module.rs            # trait for modules
├─ modules/             # one file per prompt module
├─ configs/
├─ formatter/
└─ utils/
```

Copy: the `context.rs` + plugin-like `modules/` pattern when you have user-extensible features.

### [`helix-editor/helix`](https://github.com/helix-editor/helix) — modal editor

Aggressive workspace split:

```text
helix-core/             # pure types: rope, position, syntax
helix-view/             # view layer
helix-term/             # the binary
helix-tui/              # TUI primitives
helix-lsp/              # LSP client
helix-loader/           # config + assets
helix-event/            # event bus
xtask/                  # build automation
```

Copy: workspace boundaries that align with **dependency direction** — `helix-core` depends on
nothing app-specific, `helix-term` depends on everything. Lower-level crates do not depend on
higher-level ones.

### [`atuinsh/atuin`](https://github.com/atuinsh/atuin) — shell history

```text
crates/
├─ atuin/               # CLI binary
├─ atuin-client/        # client-side logic
├─ atuin-server/        # server-side logic
└─ atuin-common/        # shared types
```

Copy: the four-crate split (`bin` + `client` + `server` + `common`) when you ship both client and
server.

### [`zellij-org/zellij`](https://github.com/zellij-org/zellij) — terminal multiplexer

```text
zellij-server/
zellij-client/
zellij-utils/
zellij-tile/            # plugin API
default-plugins/
```

Copy: a separate `<app>-tile` (or `<app>-plugin-api`) crate when you support plugins — keeps the
plugin ABI small and stable.

## Daemons, supervisors, and VMMs

Long-running processes started by a service manager rather than a shell. Study these when the binary
you are writing is supervised — the entry-point discipline is the same, the audience is not. See
[03 — Error handling · Binaries a service manager supervises](./03-error-handling.md#binaries-a-service-manager-supervises).

### [`firecracker-microvm/firecracker`](https://github.com/firecracker-microvm/firecracker) — AWS microVM VMM

**The reference implementation of this spec's error boundary**, arrived at independently. `main`
delegates, `MainError` is a `thiserror` enum with `#[source]` chaining, and one `From` impl owns the
whole mapping:

```rust
fn main() -> ExitCode {
    let result = main_exec();
    if let Err(err) = result {
        error_unrestricted!("{err}");            // log face
        eprintln!("Error: {err:?}");             // journal face
        ExitCode::from(FcExitCode::from(err) as u8)
    } else { ExitCode::SUCCESS }
}
fn main_exec() -> Result<(), MainError> { ... }
impl From<MainError> for FcExitCode { ... }      // exhaustive match
```

Copy: the render-then-classify ordering; the hand-rolled `FcExitCode` enum (it carries signal-derived
codes `148..=157`, which is why the `sysexits` crate was unusable); one `From` impl as the single
audit surface for "which codes can this program return".

### [`cloud-hypervisor/cloud-hypervisor`](https://github.com/cloud-hypervisor/cloud-hypervisor) — VMM

A strong `thiserror` enum with 30-plus `#[source]`-chained variants, a bespoke `cli_print_error_chain`
renderer — and exit codes of only `0` and `1`. **It invests everything in the message and nothing in
the code**, which is a coherent choice when the consumer reads stderr rather than `$?`. It also
cleans up its API socket before exiting, which is the argument against `process::exit` in miniature.

Copy: the error-chain renderer. Don't copy the `process::exit(0|1)` boundary if your exit matrix is
richer than "worked / didn't".

### [`containers/youki`](https://github.com/containers/youki) — OCI runtime

Relevant for the **pass-through** problem: `exec` and `run` must reproduce a contained process's exit
status verbatim, which youki does with `std::process::exit(exit_code)` mid-`main`. That works, and it
is exactly what `clippy::exit` warns about. A `GuestStatus(u8)` variant on your code enum reaching
`ExitCode::from(u8)` gets the same result with a clean unwind.

### [`astral-sh/ruff`](https://github.com/astral-sh/ruff) — linter

The modern consensus form, and the closest thing to this spec's default:

```rust
fn main() -> ExitCode {
    match run(Args::parse_from(args)) {
        Ok(code) => code.into(),
        Err(err) => report_error(&err),
    }
}
pub enum ExitStatus { Success, Failure, Error }
impl From<ExitStatus> for ExitCode { ... }
```

Copy: `report_error(&err) -> ExitCode`, which produces the rendering and the code in one boundary
function; the separate `ExitStatus` type so the _success_ channel also carries a code (linting that
found violations is not an error).

## Quick comparison

| Project  | LOC   | Shape                                          | Key takeaway                            |
| -------- | ----- | ---------------------------------------------- | --------------------------------------- |
| fd       | ~10k  | single crate                                   | Per-feature subdirs.                    |
| bat      | ~15k  | single crate, `src/lib.rs` + `src/bin/<name>/` | Reusable lib + thin bin.                |
| gitui    | ~30k  | single crate                                   | TUI component layout.                   |
| ouch     | ~10k  | single crate                                   | `cli/` + `commands/` matches this spec. |
| jj       | ~80k  | workspace, `cli/` subcrate                     | `command_error.rs`, focused `ui.rs`.    |
| eza      | ~30k  | single crate                                   | `options/` + `output/` split.           |
| starship | ~40k  | single crate                                   | `context.rs` + `modules/`.              |
| cargo    | ~200k | workspace, `bin/cargo/commands/`               | Uniform `exec()` per subcommand.        |
| ripgrep  | ~50k  | workspace                                      | Subsystems as published libs.           |
| helix    | ~120k | workspace, ~8 crates                           | Dependency-direction boundaries.        |
| atuin    | ~40k  | workspace, 4 crates                            | Bin + client + server + common.         |
| zellij   | ~80k  | workspace                                      | Plugin ABI crate.                       |

Supervised processes, compared on the axis that distinguishes them:

| Project          | Domain      | Error type       | Codes                                | Renders the chain?        |
| ---------------- | ----------- | ---------------- | ------------------------------------ | ------------------------- |
| firecracker      | microVM VMM | `thiserror` enum | `FcExitCode` enum, incl. `148..=157` | Yes — log face and stderr |
| cloud-hypervisor | VMM         | `thiserror` enum | `0`/`1` only                         | Yes — bespoke renderer    |
| youki            | OCI runtime | `anyhow`         | child pass-through                   | Yes                       |
| ruff             | linter      | `anyhow`         | `ExitStatus` enum                    | Yes — `report_error`      |

## What to copy from your own repos

From `riptask`:

- The exit-code matrix in `src/error.rs:52-118` — one variant per code, unit-tested.
- The `domain/` + `adapters/` + `services/` vocabulary.
- The `[[bin]]` rename trick in `Cargo.toml` (crate `riptask`, binary `tsk`).
- The help-group manifest with a drift test (`src/cli.rs:9-31`, `src/cli.rs:903-937`).

**Don't copy** from riptask:

- The monolithic `src/cli.rs:1-1558`. Split into `cli/mod.rs` + per-subcommand files.
- Per-command tokio runtime construction (`src/main.rs:95-159`). Build one runtime in `main`.
- Custom env var `RIPTASK_LOG`. Use `RUST_LOG`.
- The ambiguous `models/` vs `domain/` split. Pick `domain/` only.

From `ripwork`:

- The parse-shape / runtime-shape split: `cli/<name>.rs` + `workflows/<name>.rs` (this spec calls it
  `commands/<name>.rs`).
- `pub(crate)` everywhere (`src/runtime/mod.rs:1-7`).
- The `ui/` module as the only place that prints (`CLAUDE.md:46-52`).
- Figment-based config (`src/config.rs:1-13`).
- Per-file `//!` headers stating purpose and non-purpose (`src/main.rs:1-10`).
- ADR references in `Cargo.toml` comments (`Cargo.toml:19-20`).
- `trybuild` for typestate invariants (`tests/trybuild.rs`).

**Don't copy** from ripwork:

- Dead `src/lib.rs:1-13` that exports nothing. Delete it or make it real.
- Recursive `#[from]` chains in `src/error.rs:144-188`. Lift shared infra instead.
- Mixing workflow-args and `common.rs`/`preflight.rs` in one directory. Keep `cli/` strictly
  clap-derive.

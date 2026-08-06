# 02 — Error Messages

Error messages have four audiences: the end user trying to recover, the ops engineer triaging a
paged alert, the developer reading a bug report, and the LLM coding agent debugging from a log. A
good error speaks to all four without being verbose. This chapter is how.

## The expressive-error anatomy

Every error report has four parts. Write them in this order; omit a part only when it would be
redundant.

1. **What** — the operation that failed, in one concrete line.
2. **Where** — the specific input, file, or step that triggered it.
3. **Why** — the root cause, walked from the error chain.
4. **Hint** — actionable next step, when one is known.

Example (good):

```text
app: failed to load config
  where: /home/user/.config/app/config.toml (line 12)
  why:   timeout_secs must be a positive integer, got -1
  hint:  set [defaults] timeout_secs = 30 (or any positive value) and retry
```

Example (bad — same failure):

```text
Error: An error occurred while processing your request. Please try again later.
```

The bad example tells the user nothing. It costs them a debugging session. The good example shows
the file, the line, the actual bad value, and the fix.

### Hints earn their keep

A hint is only useful when:

- The fix is specific (`set X to Y and retry`), not generic (`check your config`).
- The user has a realistic path to apply it from the terminal.
- It doesn't lie about the problem (don't suggest a fix that won't actually work).

When you don't know the fix, **omit the hint** — don't fabricate. A missing hint is honest; a wrong
hint is worse than nothing.

---

## Audiences

| Audience         | What they want                         | How the error serves them                                                   |
| ---------------- | -------------------------------------- | --------------------------------------------------------------------------- |
| **End user**     | Recover, now.                          | `What`, `Where`, `Hint`. The `Why` if it's intelligible.                    |
| **Ops engineer** | Triage at 2am.                         | Stable error key (`err.kind=…`), exit code, log record.                     |
| **Developer**    | Reproduce + fix.                       | Full chain (`Caused by:`), `RUST_LOG=trace` style verbose mode.             |
| **LLM agent**    | Read logs, infer cause, suggest a fix. | Structured `err.kind` + `err.msg` fields in the program-log. Stable schema. |

Where the caller's error goes follows the **universal Unix stdout/stderr contract** and does **not**
change with facing category (see
[00 — Facing category & message types](./00-architecture.md#facing-category--message-types) and the
channels matrix in [01 — Logging & Output](./01-logging-and-output.md)). The caller's error always
goes to `stderr` with a non-zero exit code; `stdout` carries only a successful result. What differs
is the _format_, not the channel: a **human-facing** tool writes a prose `What`/`Where`/`Hint`
message to `stderr`, while a **machine-facing** tool writes a **structured (JSON) error to
`stderr`**. In **both** cases a structured record is also written to the program-log file, and all
of them must agree on the facts.

---

## Stable error keys

Every error variant gets a **stable kind identifier** — a short, machine-matchable string that does
not change between versions.

| Bad                            | Good                          |
| ------------------------------ | ----------------------------- |
| `"Error: could not find file"` | `err.kind=ConfigNotFound`     |
| `"Network problem"`            | `err.kind=NetworkUnavailable` |
| `"Invalid argument"`           | `err.kind=BadFlagValue`       |

The kind appears:

1. In the program-log record (`err.kind=ConfigNotFound`).
2. Optionally in the user-UX message (some tools show `[E0309]` style codes; do this only if your
   error space is small enough that codes are memorable).
3. In documentation / runbook entries (`See "ConfigNotFound" in TROUBLESHOOTING.md`).

LLM agents pattern-match on these. Renaming a kind is a breaking change.

---

## Error layering

Inside the program, errors are typed per layer (see also
[Rust 03 — Error Handling](../../languages/rust/cli-spec/03-error-handling.md)). The pattern is
universal:

```text
┌────────────────────────────────────────────────────────┐
│  main / boundary    AppError → exit code               │
├────────────────────────────────────────────────────────┤
│  commands           AppError (sum of services + I/O)   │
├────────────────────────────────────────────────────────┤
│  services           ServiceError (sum of domain + I/O) │
├────────────────────────────────────────────────────────┤
│  domain             DomainError (invariants only)      │
├────────────────────────────────────────────────────────┤
│  adapters           AdapterError (one per system)      │
└────────────────────────────────────────────────────────┘
```

Rules:

- **Each layer has its own error type.** Domain errors don't mention I/O; adapter errors don't
  mention business rules.
- **Lower layers wrap upstream errors with a cause link.** The chain is preserved end-to-end.
- **The top layer (AppError or equivalent) maps every variant to an exit code.** No catch-all
  `_ => 1`.
- **Never return opaque "anyhow / Exception" from a library.** Use the typed enum. Opaque wrappers
  are for the binary boundary only.

The chain walk that prints `caused by:` lines is how the user (and the LLM reading the log) sees the
_why_.

---

## Exit codes — BSD sysexits

Use `sysexits(3)` codes for predictable mapping. Don't invent new codes without writing them in your
README.

| Code | Constant              | When                                            |
| ---- | --------------------- | ----------------------------------------------- |
| `0`  | success               | Normal exit.                                    |
| `1`  | (catch-all)           | **Avoid.** Pick something specific.             |
| `2`  | (shell builtin error) | **Avoid.** Conflicts with `bash` syntax errors. |
| `64` | `EX_USAGE`            | Wrong CLI usage (bad flag, missing arg).        |
| `65` | `EX_DATAERR`          | Input data was malformed.                       |
| `66` | `EX_NOINPUT`          | Input file did not exist / unreadable.          |
| `69` | `EX_UNAVAILABLE`      | Service required but not available.             |
| `70` | `EX_SOFTWARE`         | Internal bug.                                   |
| `73` | `EX_CANTCREAT`        | Could not create output file.                   |
| `74` | `EX_IOERR`            | I/O error during execution.                     |
| `75` | `EX_TEMPFAIL`         | Transient; retry may help.                      |
| `77` | `EX_NOPERM`           | Permission denied.                              |
| `78` | `EX_CONFIG`           | Config file invalid.                            |

Reference: [`sysexits(3)`](https://man.freebsd.org/cgi/man.cgi?query=sysexits&sektion=3).

**Treat the matrix as part of the user-facing API.** Unit-test that each error variant maps to its
declared code. Shell scripts depend on these.

**The caveat, stated plainly.** That same `sysexits(3)` page says the interface "has been deprecated
and is retained only for compatibility. Its use is discouraged," and that "the choice of an
appropriate exit value is often ambiguous." Surveyed real-world CLIs mostly do not use these values —
ripgrep, ruff, and fd define small domain code spaces instead. This guidance keeps sysexits anyway,
because pre-agreed numbers are guessable to the scripts and coding agents that read them where a
private numbering must be learned per tool. But the honest consequence is that **the exit code is
worth less than the message.** Spend effort on the rendered diagnostic first. The reasoning, and the
rule against pulling the numbers from a crate, are recorded in
[Rust CLI spec ADR-0003](../../languages/rust/cli-spec/adr/0003-keep-bsd-sysexits-despite-deprecation.md).

A process supervised by systemd or launchd is a further case: the manager's behaviour is a
`0`-versus-nonzero decision plus whatever `SuccessExitStatus=`/`RestartPreventExitStatus=` name
explicitly, so distinct categories buy nothing operationally unless the unit lists them. For a daemon,
the journal message is the payload and the code is a breadcrumb.

### Codes are program-wide categories, not per-subcommand namespaces

One taxonomy for the whole program. `78` means "config error" whether it came from `start`, `init`,
or `config` — the code identifies the _kind_ of failure, not the command that raised it. Do **not**
mint a fresh code space per subcommand ("exit 3 means X for `push`, Y for `pull`"). That is API
surface that rots, it collides as commands multiply, and it tempts consumers to branch on
implementation detail that should have stayed diagnostic text.

So is "one specific code per failure case, per subcommand" the goal? No — that is the cumbersome
anti-pattern. The per-command view is a **mapping onto the fixed set**, not a new code space. For a
multi-subcommand CLI, express it as **one central command × code matrix** (single source of truth);
each subcommand's own docs describe its failure _conditions_ in prose and point at the matrix,
rather than carrying a competing code table.

### Exit code = category; message = instance

This is the split that lets the code set stay small:

- The **exit code** carries the _category_ — coarse, stable, for machine branching.
- **stderr** (the `what`/`where`/`why`/`hint`) plus the **`err.kind`** carry the _instance_ — the
  specific file, value, or step.

You never need a distinct code per distinct failure, because the message and the kind already carry
the specifics. `curl` exit `7` always means "failed to connect"; _which_ host and _why_ live in the
stderr line. Scripts branch on `7`; humans and agents read the message.

### How fine-grained? A spectrum

Reference CLIs sit at different points, all defensible:

| Style            | Examples                    | Shape                                                  |
| ---------------- | --------------------------- | ------------------------------------------------------ |
| Coarse           | `grep`, `diff`, `ripgrep`   | `0` ok/match · `1` no-match/differ · `2` **any** error |
| Small-structured | `gh`, `sysexits` categories | a handful of branchable classes (`gh`: `0/1/2/4`)      |
| Fine-grained     | `rsync` (~15), `curl` (~90) | one code per failure class scripts actually branch on  |

**Default to small-structured** for a multi-subcommand tool: `0`, a usage class, and the handful of
sysexits categories a real consumer would branch on. Go fine-grained only where automation genuinely
acts differently per class — `curl 28` (timeout, retry) vs `curl 3` (bad URL, don't). Rule of thumb:
**add a distinct code only when a real consumer will branch on it**; otherwise fold it into a
category and let stderr carry the specifics.

### Stability is a promise (append-only)

Documented codes are a permanent API. Once shipped:

- **Never reassign** a code's meaning across versions.
- **Only append** — new codes take fresh, unused numbers (`curl` ships ~90 and has retired none).
- Tell consumers to branch on **`0` vs non-zero, or the documented categories** — never on an
  undocumented number, and never assume codes beyond the documented set won't appear later.

This is what makes coarse codes safe to extend: a script written against today's matrix keeps working
when tomorrow adds a category.

### The `--strict` exception

A read-only checker (`doctor`, a linter) may adopt the warnings-as-errors convention: `--strict`
promotes a soft `warn` to exit `1`. This is the one sanctioned use of a bare `1` — a _policy signal_
on a side-effect-free command, not an operational failure — and it must be documented as the
explicit exception to "never a generic `1`." Operational failures still use the sysexits categories.

### Child processes: pass the status through

When your CLI **starts another process** (a wrapper, an `exec`/`shell` verb), the boundary flips:
before the child starts, use sysexits for your own failures; **once the child is running, propagate
its status verbatim** (`0..255`, `128+N` for signal death, `127` not-found / `126` not-executable).
That whole contract is owned by
[07 — Process & POSIX](./07-cli-wrapper-design/process-and-posix.md#3-process-model--exec-vs-spawn-signals-exit-codes-tty);
don't restate it here.

---

## Printing the chain

Two display levels:

**Default (terse, for end users):**

```text
app: failed to load config
  caused by: timeout_secs must be a positive integer
  caused by: parse error at line 12
```

**Verbose (`-v` or higher), or in the program-log always:**

```text
app: failed to load config
  err.kind: ConfigInvalid
  where:    /home/user/.config/app/config.toml (line 12)
  caused by: timeout_secs must be a positive integer (kind=DomainError::BadValue)
  caused by: parse error at line 12 (kind=ConfigError::Toml)
  hint:      set [defaults] timeout_secs = 30 and retry
```

The walk visits `source()` (or your language's equivalent) until it's `None`. Indent each level.
Dedupe — if a wrapper's message is `caused by: <inner.message>`, don't print the inner twice.

In the program-log, the same information appears as fields:

```text
ts=... level=error op=config.load err.kind=ConfigInvalid err.path=/home/user/.config/app/config.toml err.line=12 err.msg="timeout_secs must be a positive integer"
```

---

## Pretty-printing libraries

When the default chain walk isn't enough, reach for a dedicated library:

| Language | Library                                                                  | What it adds                                                   |
| -------- | ------------------------------------------------------------------------ | -------------------------------------------------------------- |
| Rust     | [`miette`](https://docs.rs/miette/)                                      | Source-snippet rendering, ASCII art pointers, structured help. |
| Rust     | [`color-eyre`](https://docs.rs/color-eyre/)                              | Colored `anyhow`-style chains with spantraces.                 |
| Python   | [`rich.traceback`](https://rich.readthedocs.io/en/latest/traceback.html) | Colorized tracebacks with source lines.                        |
| Go       | `errors.Is` / `errors.As` + custom formatter                             | Idiomatic chain walking.                                       |
| Bash     | Custom `trap ERR` handler + `set -E`                                     | Linenumber + last command.                                     |

Gate the heavyweight ones behind a `--pretty-errors` flag or a debug build feature — they're for
interactive humans, not for piping into another tool.

---

## Error messages for the LLM agent

When an LLM agent runs your CLI and reads its logs to debug:

1. **Stable `err.kind`** lets the agent pattern-match against a known taxonomy.
2. **Structured fields** (`err.path`, `err.line`, `err.value`) let the agent reason about the
   failure without parsing prose.
3. **Predictable chain depth** — don't randomize whether you wrap N times.
4. **No noisy stack traces in the default log**. Stack traces (when emitted) go behind `-vvv` or
   into a separate `err.trace` field with a stable encoding.
5. **One `op` per top-level command invocation**, with `status=error` and an `err.*` group when it
   fails. The agent can grep `status=error` to find every failure in a session.

See [05 — Designing for LLM Agents](./05-designing-for-llm-agents.md) for the broader pattern.

---

## Anti-patterns

- **Stringly-typed errors**: `return Err("something failed")`. No `kind`, no chain, no exit-code
  mapping.
- **Generic "an error occurred"**: tells the user nothing. Always include `what` and `where`.
- **Panic-as-error**: panicking on user input is a bug. Panics are for invariant violations the
  programmer made.
- **Swallowing context**: `.map_err(|_| MyError::Generic)?` loses the cause. Always preserve the
  chain.
- **Catch-all exit code `1`**: the matrix is the API; map every variant.
- **Per-subcommand code namespaces**: the same number meaning different things in different
  subcommands. Codes are program-wide categories; map each command onto the shared set.
- **Reassigning a code's meaning** across versions: it's a permanent API. Append new codes; never
  repurpose an old one.
- **Inventing new exit codes** without documentation. Stick to `sysexits` unless you have a very
  good reason.
- **Multi-paragraph error blobs on stderr** in non-verbose mode. The user wants three lines: what,
  where, fix.
- **Translating `err.kind`** into localized text. The kind is an API key, not a user-facing label.

---

## Checklist

For every error variant, confirm:

- [ ] It has a stable `err.kind` identifier.
- [ ] It maps to a specific (non-`1`) exit code, drawn from the **program-wide** category set (not a
      per-subcommand number); new codes are appended, never reassigned.
- [ ] Its `what` / `where` / `why` / `hint` are clear when rendered.
- [ ] Its chain preserves the underlying cause (`#[from]`, `errors.Unwrap()`, `from e`, etc.).
- [ ] It appears in a unit test that locks down the exit code.
- [ ] Its program-log record includes `err.kind` and `err.msg` as separate fields.
- [ ] It does not leak sensitive data (file contents, secrets, credentials).

## See also

- [Rust 03 — Error Handling](../../languages/rust/cli-spec/03-error-handling.md) — `thiserror` +
  `anyhow` stack, `#[from]` mechanics, `AppError::exit_code()`.
- [01 — Logging & Output](./01-logging-and-output.md) — how errors travel through the program-log
  layer.
- [05 — Designing for LLM Agents](./05-designing-for-llm-agents.md) — agent-readable failure
  schemas.

## References

- [`sysexits(3)` (FreeBSD man page)](https://man.freebsd.org/cgi/man.cgi?query=sysexits&sektion=3)
- [GNU `grep` exit status (`0`/`1`/`2`)](https://www.gnu.org/software/grep/manual/grep.html#Exit-Status)
- [GitHub CLI exit codes (`gh`: `0`/`1`/`2`/`4`)](https://cli.github.com/manual/gh_help_exit-codes)
- [`curl` exit codes (append-only, ~90)](https://everything.curl.dev/cmdline/exitcode.html)
- [BurntSushi: Error Handling in Rust](https://burntsushi.net/rust-error-handling/)
- [Alexis King: Parse, don't validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
- [`miette`](https://docs.rs/miette/) · [`color-eyre`](https://docs.rs/color-eyre/) ·
  [`rich.traceback`](https://rich.readthedocs.io/en/latest/traceback.html)

---
digest-of: programming/cli-design
last-synced: 2026-08-19
token-estimate: 1450
---

# AGENTS

## Scope

Language-agnostic CLI design canon: architecture, logging, errors, config, coding style, LLM-agent
design, preflight/health checks, naming, reference projects, and a pre-ship checklist.
Language-specific implementations live in `languages/<lang>/cli-spec/`.

## Key Points

### Architecture (00)

- Split parse-shape (CLI parser structs) from runtime-shape (domain types); projection happens once
  at the top of each handler.
- Declare the facing category at design time, `human-facing` or `machine-facing`, never as a runtime
  `isatty()` flip.
- One `AppContext` built in `main` and passed by reference: config, paths, runtime handle, clock,
  and either a human-UX `Ui` or a machine-output facility. No globals.
- Directory roles: `cli/` (parse-shape), `commands/` (handlers), `domain/` (pure types, no I/O),
  `adapters/` (external I/O), `services/`, `config/`, `ui/` or the structured-output boundary,
  `util/`.
- Four-edit rule for subcommands: `cli/<name>`, `cli/root`, `commands/<name>`, `main` dispatch. One
  crate by default; a workspace only at ~8k LOC or when a real second consumer appears.

### Logging and Output (01)

- Three message types: human-UX (human-facing default), machine-output (machine-facing default,
  human-facing opt-in), and log-messages (both, always). Log-messages default to
  `$XDG_STATE_HOME/<app>/<app>.log` in structured `key=value` or JSON with no ANSI; the terminal
  mirror is opt-in.
- Machine-output does not paginate by default; where output can be large, document
  `--limit`/`--page`/`--cursor`/`--offset`.
- Verbosity: none=warn, `-v`=info, `-vv`=debug, `-vvv`=trace. Human-facing tools default to
  colorful terminal output; machine modes and non-terminal
  destinations are undecorated. Resolve presentation once at startup for the actual destination
  stream: active `FORCE_COLOR` wins over active `NO_COLOR`, and empty values are inert.

### Error Messages (02)

- Four-part anatomy: what, where, why, hint. Stable `err.kind` per variant, machine-matchable and
  never renamed. Per-layer typed errors aggregated at a top-level `AppError`.
- BSD sysexits exit codes (64=usage, 65=data, 66=noinput, 69=unavailable, 70=software, 74=ioerr,
  78=config). No catch-all `1`.
- Codes are program-wide categories, not per-subcommand namespaces; one central command×code matrix
  maps the per-command view onto the fixed set. Exit code is the category and stderr plus `err.kind`
  is the instance, which is what keeps the code set small.
- Fine-grained spectrum: coarse (grep/diff `0/1/2`) → small-structured (gh `0/1/2/4`, sysexits;
  default) → fine (rsync ~15, curl ~90). Add a code only when a real consumer branches on it.
- Codes are append-only and permanent: never reassign. The one sanctioned bare `1` is a read-only
  checker's `--strict` promoting `warn`→`1`.
- Child processes: pass status through verbatim (`128+N`, `126`/`127`) — owned by Ch. 07.
- Rust pretty-renderer selection, color policy, and global-hook caveats live in the nested Rust CLI
  digest and dependency chapter.

### Config Precedence (03)

- `CLI > env > project file > user file > defaults` for every key.
- XDG paths for config/state/cache/data. Never `~/.<app>/`.
- The loader tracks per-key source provenance, unknown keys fail loudly, and `--print-config`
  exposes the result.

### Coding Style (04)

- Explicit errors; parse don't validate; newtypes for domain primitives. Composition over
  inheritance, and free functions when there is no state.
- Constructor placement follows `C-CONV-SPECIFIC`, not "builds one type": a boundary stage taking
  raw argv/bytes stays a free function in its module, while a settled-type-to-settled-type step goes
  on the output type. `C-METHOD` governs receivers only.
- Files <=400 LOC. Comments say why, not what, and module headers state purpose and non-purpose.
  Strict lints project-wide; a per-line allow needs a justification.

### LLM Agent Design (05)

- Default path: CLI plus a thin Skill wrapper; MCP only for stateful/auth/multi-tenant needs. Three
  layers: CLI (mechanism), SKILL.md (playbook), AGENTS.md (constitution).
- Every output is a prompt: include affected IDs and next-command suggestions.
- `--help` is documentation, and the self-documenting machine surfaces are `help`/usage, `doctor`,
  `init`, completion, and man pages via a subcommand.
- Verb-noun structure mirroring kubectl/docker/gh. Familiar flag names (`--dry-run`, `--force`,
  `--yes`). Deterministic and idempotent operations.

### Preflight & Health Checks (06)

- Every subcommand validates its prerequisites at entry and fails fast before any side effect —
  never a half-applied mutation or an opaque late error.
- One first-class `doctor` aggregates every environment check (`--scope`, `--json`); it must not
  probe just one path.
- One probe set, three call sites: `doctor`, per-command guards, and `init`/setup — no independent
  per-command checks that drift.
- Each check carries a stable ID (doubling as `err.kind`) and is classified hard (blocks, non-zero
  exit plus remediation) or soft (warn plus documented fallback).
- Read-only/inert commands (`status`, `version`, `help`, `doctor`, list/view) never gate.

### Naming and Docs (08)

- A surface element earns its place by discriminating: a subcommand from its siblings, a flag
  between invocations. A one-child namespace collapses to the bare verb, a flag is declared where it
  is read, and a read-only name (`view`/`status`/`list`) is a tie-breaker, never a reason to split.
- Visibility defaults to least-public, `pub(crate)` before `pub`. Names are `<Verb>Args`
  (parse-shape), `<Verb>Request` (runtime), `<Layer>Error`, and concept-name newtypes.
- `--help` is generated from the parser, not hand-authored; narrative goes in intro/epilog hooks,
  and module headers say what it is and what it isn't.

### Reference Projects (10)

- Ten patterns from real CLIs: single-crate, lib+bin, domain-crates, client+server+common, plugin
  ABI, uniform exec(), focused error+ui modules, options/output split, context+modules, and the
  dependency-direction workspace.

### XDG scaffolding & `init` (11)

- Config is read-only at runtime; `init`/scaffold never writes to `$XDG_CONFIG_HOME`, and the why
  lives in `design-decisions/config-state-ownership/`. Four safe patterns: print starter config to
  stdout; sane defaults; write state/cache only; explicit non-clobbering write to a named path.
- Anti-pattern: `init` appending discovered runtime facts into `~/.config/*` gives two writers and
  breaks under Home Manager. Fix with a config/state split plus merge. The only sanctioned write to
  a user-owned surface is a flag naming the exact target, off by default, confirmed, reversible.
- `init` reuses the `doctor` probe subset (see 06), with `xdg-ninja` as the conformance check.
  Deleting `init` is a sanctioned outcome: keep `init --write` where a binding is worth recording,
  drop the verb where everything resolves per-invocation.

### Config generation from types (12)

- Config is read-only to the tool, so copy-don't-scaffold: generate the example the user copies.
- The config type is the single source of truth; generate a JSON Schema and an annotated
  `*.example.*` from it, hard-failing when a public field lacks a description.
- Staleness is rendered bytes against on-disk bytes, never a cache or a stamp, which requires
  deterministic byte-stable rendering: no dates, stable key order, no absolute paths.
- Regenerate-and-stage in pre-commit, verify-only in CI. Cost: `git commit -p` on a generated file
  is undetectable by the gate.
- The generator is a separate build target (`cargo xtask`, a dev-only Python package, a Go `tools/`
  main), never a hidden subcommand shipping schema deps to every user. Non-reflectable surfaces ship
  as hand-maintained examples under the same discipline.

### Checklist (99)

- Pre-ship sanity check across architecture, logging, errors, config, style, LLM agents, naming,
  testing, CI, and wrapper specifics.

## Maintenance Notes

- Chapters 07 (CLI wrapper design) and 09 (testing & quality) are subdirectories; load them directly.
- Language-specific specs (`rust/cli-spec/`, `python/cli-spec/`, `bash/cli-spec/`) apply these
  principles to concrete ecosystems. Regenerate when the shelf's knowledge changes.

---
digest-of: programming/cli-design
last-synced: 2026-07-29
source-files:
  - 00-architecture.md
  - 01-logging-and-output.md
  - 02-error-messages.md
  - 03-config-precedence.md
  - 04-coding-style-rust-zig.md
  - 05-designing-for-llm-agents.md
  - 06-preflight-and-health-checks.md
  - 08-naming-and-docs.md
  - 10-reference-projects.md
  - 11-xdg-scaffolding.md
  - 12-config-generation-from-types.md
  - 99-checklist.md
  - README.md
token-estimate: 1650
---

# AGENTS

## Scope

Language-agnostic CLI design canon: architecture, logging, errors, config, coding style, LLM-agent
design, preflight/health checks, naming, reference projects, and a pre-ship checklist.
Language-specific implementations live in `languages/<lang>/cli-spec/`.

## Key Points

### Architecture (00)

- Split parse-shape (CLI parser structs) from runtime-shape (domain types). Projection happens once
  at the top of each handler.
- Declare the facing category at design time: `human-facing` or `machine-facing`. This is not a
  runtime `isatty()` flip.
- One `AppContext` built in `main`, passed by reference. Holds config, paths, runtime handle, clock,
  and either a human-UX `Ui` or a machine-output/protocol facility. No globals.
- Directory roles: `cli/` (parse-shape), `commands/` (handlers), `domain/` (pure types, no I/O),
  `adapters/` (external I/O), `services/` (optional shared orchestration), `config/`, `ui/`
  (human-facing) or structured-output boundary (machine-facing), `util/`.
- Four-edit rule for subcommands: `cli/<name>`, `cli/root`, `commands/<name>`, `main` dispatch.
- Single crate by default; workspace only at ~8k LOC or when a real second consumer appears.

### Logging and Output (01)

- Three message types: human-UX (human-facing default), machine-output (machine-facing default,
  human-facing opt-in), and log-messages (both categories, always).
- Default log-messages to `$XDG_STATE_HOME/<app>/<app>.log` in structured `key=value` or JSON, no
  ANSI. Terminal mirror is opt-in.
- Machine-output does not paginate by default; if output can be too large, document
  `--limit`/`--page`/`--cursor`/`--offset` in `--help`.
- Verbosity: none=warn, `-v`=info, `-vv`=debug, `-vvv`=trace.
- Respect `NO_COLOR`/`FORCE_COLOR` for human-UX; never color machine-output or log files.

### Error Messages (02)

- Four-part anatomy: what, where, why, hint.
- Stable `err.kind` per variant (machine-matchable, never rename).
- BSD sysexits exit codes (64=usage, 65=data, 66=noinput, 69=unavailable, 70=software, 74=ioerr,
  78=config). No catch-all `1`.
- Per-layer typed errors aggregated at top-level `AppError`.
- Codes are **program-wide categories, not per-subcommand namespaces**; the per-command view is a
  mapping onto the fixed set, expressed as one central command×code matrix (single source of truth).
- **Exit code = category; stderr + `err.kind` = instance** — this is what keeps the code set small.
- Fine-grained spectrum: coarse (grep/diff `0/1/2`) → small-structured (gh `0/1/2/4`, sysexits;
  default) → fine (rsync ~15, curl ~90). Add a code only when a real consumer branches on it.
- **Append-only stability**: codes are a permanent API — never reassign, only append; consumers
  branch on `0`/non-zero or documented categories.
- Sanctioned bare `1`: a read-only checker's `--strict` promoting `warn`→`1` (documented exception).
- Child processes: pass status through verbatim (`128+N`, `126`/`127`) — owned by Ch. 07.

### Config Precedence (03)

- `CLI > env > project file > user file > defaults` for every key.
- XDG paths for config/state/cache/data. Never `~/.<app>/`.
- Loader tracks per-key source provenance. Unknown keys fail loudly.
- `--print-config` subcommand for debugging.

### Coding Style (04)

- Explicit errors; parse don't validate; newtypes for domain primitives.
- Composition over inheritance; free functions when no state.
- Constructor placement: assembly belongs on the produced type.
- Files <=400 LOC. Comments say why, not what. Module headers state purpose and non-purpose.
- Strict lints project-wide; per-line allow only with justification.

### LLM Agent Design (05)

- Default path: CLI + thin Skill wrapper. MCP only for stateful/auth/multi-tenant needs.
- Three-layer model: CLI (mechanism), SKILL.md (playbook), AGENTS.md (constitution).
- Every output is a prompt: include affected IDs and next-command suggestions.
- `--help` is documentation; machine-output is default for machine-facing tools and opt-in for
  human-facing tools; `doctor` reports health checks.
- Self-documenting machine surfaces: `help`/usage, `doctor`, `init`, completion, and man pages via a
  subcommand.
- Verb-noun structure mirroring kubectl/docker/gh. Familiar flag names (`--dry-run`, `--force`,
  `--yes`).
- Deterministic and idempotent operations.

### Preflight & Health Checks (06)

- Every subcommand validates its prerequisites at entry and fails fast **before** any side effect —
  never a half-applied mutation or an opaque late error.
- One first-class `doctor` aggregates **all** environment checks (`--scope`, `--json`); it must not
  probe just one path.
- One probe set, three call sites: `doctor` (whole catalog), per-command guards (the subset that
  command needs), and `init`/setup — no independent per-command checks that drift.
- Each check has a stable ID (doubles as `err.kind`) and is classified **hard** (blocks, non-zero
  exit + remediation) or **soft** (warn + documented fallback).
- Read-only/inert commands (`status`, `version`, `help`, `doctor`, list/view) never gate.

### Naming and Docs (08)

- Visibility defaults to least-public. `pub(crate)` before `pub`.
- `<Verb>Args` (parse-shape), `<Verb>Request` (runtime), `<Layer>Error`, concept-name newtypes.
- `--help` is generated from parser, not hand-authored. Narrative goes in intro/epilog hooks.
- Module headers: "what it is, what it isn't."

### Reference Projects (10)

- Ten patterns from real CLIs: single-crate, lib+bin, domain-crates, client+server+common, plugin
  ABI, uniform exec(), focused error+ui modules, options/output split, context+modules,
  dependency-direction workspace.

### XDG scaffolding & `init` (11)

- Config is read-only at runtime; `init`/scaffold must never write to
  `$XDG_CONFIG_HOME`. The _why_ lives in
  `design-decisions/config-state-ownership/`.
- Four safe scaffold patterns: print starter config to stdout; sane defaults
  (config optional); write state/cache only; explicit non-clobbering write to a
  named path.
- Anti-pattern: `init` appending discovered runtime facts into `~/.config/*` →
  two writers, breaks under Home Manager. Fix with config/state split + merge.
- The only sanctioned write to a user-owned surface is explicit: a flag naming the
  exact target, off by default, confirmed, and reversible/non-clobbering — never a
  silent side effect.
- `init` reuses the `doctor` probe subset (see 06). Run `xdg-ninja` as the
  conformance check.
- **Deleting `init` is a sanctioned outcome.** Once the config scaffold is
  forbidden, test whether it still earns its place: config optional via defaults? no
  durable state to persist? another verb already owns the rest? If all three, remove
  the verb. Discriminator vs the explicit-write pattern: is there a **binding worth
  recording** (keep `init --write`) or is everything resolved per-invocation (drop it)?

### Config generation from types (12)

- Config is read-only to the tool, so **copy-don't-scaffold**: generate the example
  the user copies rather than writing their config.
- The config type is the single source of truth; generate a JSON Schema (editor/CI
  validation) and an annotated `*.example.*` (required active, optional commented,
  fake placeholders, generated header) from it.
- Generation hard-fails if a public field lacks a description; the example must
  round-trip through the real loader.
- Freshness gated by a pre-commit hook (`pass_filenames: false`, `always_run: true`);
  CI runs the same command with `--check`.
- **Staleness is rendered-bytes vs on-disk bytes — never a cache or stamp.** A cache
  adds an artifact, an invalidation rule, and a way for the gate to report fresh when
  it is not. Requires deterministic, byte-stable rendering: no dates, stable key
  order, no absolute paths.
- **Regenerate-and-stage in pre-commit, verify-only in CI**, so a model change and
  its example land in one commit. Cost: generated files must be staged whole, so
  `git commit -p` on one is a mistake the gate cannot detect.
- The generator is a **separate build target** (Rust `cargo xtask`, a dev-only
  Python package, a Go `tools/` main) — never a hidden subcommand shipping schema
  deps to every user. In Rust it needs a library target to import the config types
  from, which is often what trips a "no workspace yet" rule.
- Non-reflectable surfaces ship as hand-maintained examples under the same
  discipline. **Most real tools have a mix**, split by who owns each schema; a
  wrapper's own config reflects, the fragments it composes do not.

### Checklist (99)

- Pre-ship sanity check across architecture, logging, errors, config, coding style, LLM agents,
  naming, testing, regression safeguards, CI, and wrapper specifics.

## Source Map

| Topic                                                                     | File                                 |
| ------------------------------------------------------------------------- | ------------------------------------ |
| Facing category, parse/runtime shape, AppContext                          | `00-architecture.md`                 |
| Message types, log schema, channel matrix                                 | `01-logging-and-output.md`           |
| Error anatomy, sysexits, error layering                                   | `02-error-messages.md`               |
| 5-layer config merge, XDG, provenance                                     | `03-config-precedence.md`            |
| 18 coding-style rules                                                     | `04-coding-style-rust-zig.md`        |
| CLI+Skill+AGENTS.md model, agent-facing patterns                          | `05-designing-for-llm-agents.md`     |
| Preflight guards + doctor aggregation (hard/soft)                         | `06-preflight-and-health-checks.md`  |
| Visibility, naming tables, help generation, docs                          | `08-naming-and-docs.md`              |
| Organizational patterns from 12 CLIs                                      | `10-reference-projects.md`           |
| Scaffold/`init` without mutating config; when to delete `init`; xdg-ninja | `11-xdg-scaffolding.md`              |
| Generate config examples from types; copy-don't-scaffold                  | `12-config-generation-from-types.md` |
| Pre-ship checklist                                                        | `99-checklist.md`                    |

## Maintenance Notes

- Chapters 07 (CLI wrapper design) and 09 (testing & quality) are subdirectories not included as
  source files in this digest; load them directly when reviewing wrapper design or testing. They
  include light category-scoping tags.
- Language-specific specs (`rust/cli-spec/`, `python/cli-spec/`, `bash/cli-spec/`) apply these
  principles to concrete ecosystems.
- Regenerate when any chapter file changes or new chapters are added.

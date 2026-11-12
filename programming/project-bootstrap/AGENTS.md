---
digest-of: programming/project-bootstrap
last-synced: 2026-11-11
token-estimate: 900
---

# AGENTS

## Scope

The language-agnostic, **once-per-project** setup that takes an empty repo to a scaffolded,
quality-gated baseline ready for feature work: repo creation, foundations, governance docs, dev
environment, quality gates, CI, and a security baseline. This tree is the source of truth for _what_
a new project must have and _in what order_; language `project-bootstrap-spec/` bindings overlay it
with concrete tooling.

## Key Points

- **Three-layer ownership model.** Setup descends three layers, each owning a disjoint set of facts:
  **General** (this tree — the universal, cross-language spine), **Language**
  (`languages/<lang>/project-bootstrap-spec/` — ecosystem choices that overlay the spine and
  never restate it), and **Implementation-kind** (`<kind>.md` files — CLI/library/service
  bootstrap-time ordering, delegating detailed _how_ to existing specs).
- **One owner per fact / single source of truth.** Each step lives in exactly one place; other
  layers link, never duplicate. A tool that owns its own shelf is referenced, not copied.
- **Bootstrap runs once, at creation.** Repo → foundations → gates, ending at a scaffolded,
  quality-gated baseline ready for feature work.
- **The runbook is the spine.** `runbook.md` is the ordered _what_ (8 steps); chapters explain the
  _why_ behind each step; language and kind bindings are overlays.
- **Chapter topics 00–06.** 00 bootstrap model (once-per-project phase, three-layer model); 01
  repository foundation (`.gitignore`, SPDX `LICENSE`, `README` skeleton); 02 governance & docs
  (`CLAUDE.md`, `AGENTS.md` convention, MADR-minimal ADR scaffold, README-as-index); 03 local dev
  environment (Nix devShell + `.envrc`, who owns each dependency, `.editorconfig`); 04 quality gates
  (formatter, linter, git hooks in two stages, hook exception classes, generated files, task runner); 05 CI (the two hook stages repeated in
  the devshell, the CI-only list); 06 security baseline (secrets hygiene, dependency audit,
  OpenSSF Scorecard).
- Dependency owners. Each dependency has one owner, the most specific file that declares it
  natively: the manifest of the stack (`pyproject.toml` with `uv.lock`, a PEP 723 block,
  `Cargo.toml`, `package.json`), then `.pre-commit-config.yaml` for each hook tool that the hook
  runner can build, then the devshell for the toolchains, the task runner, and what no owner above
  can declare, then the host for the base tools, the git hook runner prek, the ELF loader, and the
  credentialed tools. The hook configuration owns a tool that only a hook uses, a linter or a type
  checker included. No devshell holds a hook runner. A project uses a host tool by name, with no
  probe and no fallback, and its README lists the host requirements.
  A flake holds no language library. Python uses uv. A hook that keeps `language: system` carries
  an `# owner:` comment with one of five classes.
- CI parity. The commit stage runs the fast checks and the push stage the costly ones; together
  they run every CI check, and the push stage does not rerun the commit stage. CI repeats both
  stages in the devshell. A CI-only check needs a reason a long runtime is not, and is listed with
  the report-only and scheduled jobs.
- Generated files. A generator runs natively and is configured only as its tool documents; every
  hook that rewrites or judges a generated file (lock file, release changelog) excludes it in the
  tool's own configuration, and the reading hooks (parser, secret scan, link check) keep it. A file
  a person edits after generation, or a marked region, belongs to the project.

## Maintenance Notes

- This is a **generated digest**; regenerate it (updating `last-synced` and `token-estimate`)
  whenever the shelf's knowledge changes. Do not add guidance absent from the sources.
- Language bindings and implementation-kind files live under `languages/<lang>/`; add or
  refresh them independently of this general spine.
- No conflicts among the current source files.

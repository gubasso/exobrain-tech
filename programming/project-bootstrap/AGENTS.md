---
digest-of: programming/project-bootstrap
last-synced: 2026-11-02
token-estimate: 800
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
- **SoT-vs-cog contract.** This repo is the SoT for _what_; `cog bootstrap-*` skills (one per
  domain) automate _how_. If they disagree, **fix the runbook first, then cog** — the documented
  step wins.
- **Chapter topics 00–07.** 00 bootstrap model (once-per-project phase, three-layer model); 01
  repository foundation (`.gitignore`, SPDX `LICENSE`, `README` skeleton); 02 governance & docs
  (`CLAUDE.md`, `AGENTS.md` convention, MADR-minimal ADR scaffold, README-as-index); 03 local dev
  environment (Nix devShell + `.envrc`, who supplies each tool, `.editorconfig`); 04 quality gates
  (formatter, linter, git hooks in two stages, task runner); 05 CI (the two hook stages repeated in
  the devshell, the CI-only list); 06 security baseline (secrets hygiene, dependency audit,
  OpenSSF Scorecard); 07 automation with cog (the contract and domain→helper map).
- Tool split. The devshell holds the toolchain, `pre-commit`, the task runner, and every tool a
  recipe or a `system` or `script` hook calls; pre-commit manages the environments it builds, with
  the devshell supplying what it cannot fetch (e.g. `nodejs`); the host supplies only `git`, `wt`,
  `bash`, `awk`, `sed`, `grep`, the coreutils, `nix`, `direnv`, and the host deploy tools.
- CI parity. The commit stage runs the fast checks and the push stage the costly ones; together
  they run every CI check, and the push stage does not rerun the commit stage. CI repeats both
  stages in the devshell. A CI-only check needs a reason a long runtime is not, and is listed with
  the report-only and scheduled jobs.

## Maintenance Notes

- This is a **generated digest**; regenerate it (updating `last-synced` and `token-estimate`)
  whenever the shelf's knowledge changes. Do not add guidance absent from the sources.
- Language bindings and implementation-kind files live under `languages/<lang>/`; add or
  refresh them independently of this general spine.
- No conflicts among the current source files.

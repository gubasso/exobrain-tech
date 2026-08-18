---
digest-of: programming/project-bootstrap
last-synced: 2026-07-10
token-estimate: 700
---

# AGENTS

## Scope

The language-agnostic, **once-per-project** setup that takes an empty repo to a scaffolded,
quality-gated baseline ready for feature work: repo creation, foundations, governance docs, dev
environment, quality gates, CI + branch protection, and a security baseline. This tree is the source
of truth for _what_ a new project must have and _in what order_; language `project-bootstrap-spec/`
bindings overlay it with concrete tooling, and it hands off to the later `release-workflow/` phase.

## Key Points

- **Three-layer ownership model.** Setup descends three layers, each owning a disjoint set of facts:
  **General** (this tree — the universal, cross-language spine), **Language**
  (`languages/<lang>/project-bootstrap-spec/` — ecosystem choices that overlay the spine and
  never restate it), and **Implementation-kind** (`<kind>.md` files — CLI/library/service
  bootstrap-time ordering, delegating detailed _how_ to existing specs).
- **One owner per fact / single source of truth.** Each step lives in exactly one place; other
  layers link, never duplicate. Branch protection (`tools/git/branch-protection/`) and release setup
  keep their own homes and are referenced, not copied.
- **Bootstrap precedes release.** Bootstrap runs once at creation (repo → foundations → gates) and
  is distinct from and precedes the recurring `release-workflow/` phase; bootstrap makes the project
  _ready_, release setup makes it _publishable_. The two cross-link at repo creation and branch
  protection.
- **The runbook is the spine.** `runbook.md` is the ordered _what_ (8 steps); chapters explain the
  _why_ behind each step; language and kind bindings are overlays.
- **SoT-vs-cog contract.** This repo is the SoT for _what_; `cog bootstrap-*` skills (one per
  domain) automate _how_. If they disagree, **fix the runbook first, then cog** — the documented
  step wins.
- **Chapter topics 00–07.** 00 bootstrap model (once-per-project phase, three-layer model); 01
  repository foundation (`.gitignore`, SPDX `LICENSE`, `README` skeleton); 02 governance & docs
  (`CLAUDE.md`, `AGENTS.md` convention, MADR-minimal ADR scaffold, README-as-index); 03 local dev
  environment (Nix devShell + `.envrc`, `.editorconfig`); 04 quality gates (formatter, linter,
  pre-commit, task runner); 05 CI & release-readiness (first CI workflow, branch protection,
  hand-off); 06 security baseline (secrets hygiene, dependency audit, OpenSSF Scorecard); 07
  automation with cog (the contract and domain→helper map).

## Maintenance Notes

- This is a **generated digest**; regenerate it (updating `last-synced` and `token-estimate`)
  whenever the shelf's knowledge changes. Do not add guidance absent from the sources.
- Related trees are referenced, not owned: `../release-workflow/` (next phase),
  `../../tools/git/branch-protection/` (platform runbooks/rulesets), and `../spec-driven-docs/` (the
  single-source-of-truth and Diátaxis standards this shelf obeys).
- Language bindings and implementation-kind files live under `languages/<lang>/`; add or
  refresh them independently of this general spine.
- No conflicts among the current source files.

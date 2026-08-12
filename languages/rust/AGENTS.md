---
digest-of: languages/rust
last-synced: 2026-07-10
token-estimate: 500
---

# AGENTS

## Scope

Rust language notes at the top level: general Rust guidance, framework notes, database integration
patterns, and pointers to the bootstrap, CLI, and release/publishing sub-shelves (each of which
carries its own nested digest).

## Key Points

- **General notes** (`rust.md`): libraries, cargo tools, git2, module layout, arrays/vectors, and
  iterator patterns.
- **Patterns** (`patterns.md`): general Rust idioms and patterns.
- **Axum**: Web framework reference notes.
- **DateTime handling**: serde + SQLx + chrono integration patterns.
- **SQLx**: Database query patterns and migrations-related notes.
- **Bootstrap**: the `project-bootstrap-spec/` sub-shelf is the rust binding of the general
  `programming/project-bootstrap/` shelf — toolchain/layout, quality gates, and the CLI
  implementation-kind. Own nested `AGENTS.md`.
- **Release & publishing**: the `release-workflow-spec/` sub-shelf is the unified rust binding of
  the general `programming/release-workflow/` shelf — release-plz on `develop`, `master`
  promotion onto the release tag, crates.io Trusted Publishing (register `release-plz.yml`, not
  `release.yml`), crate metadata, token scopes, helper scripts, SemVer/yank, cargo-dist binary
  distribution, and a per-new-project runbook. Own nested `AGENTS.md`.
- **Cookbook**: the `cookbook/` sub-shelf is a single-file, TLDR ship-it runbook (scaffold → quality
  gates → branch security → CI → release/publish) that inlines and footnotes the bootstrap and
  release specs. It is the sanctioned SoT/DRY exception (repo `AGENTS.md`; ADR-0002) — do **not**
  de-duplicate it against the specs. Own nested `AGENTS.md`.

## Maintenance Notes

- `cli-spec/`, `project-bootstrap-spec/`, and `release-workflow-spec/` each have their own digest
  and are not expanded here.
- The former top-level `code-review-guide.md` was migrated out of this repo into the cog skill-refs
  corpus; it is no longer a source file here.
- Regenerate when the area's knowledge changes.

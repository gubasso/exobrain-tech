---
digest-of: languages/rust
last-synced: 2026-07-10
token-estimate: 500
---

# AGENTS

## Scope

Rust language notes at the top level: general Rust guidance, framework notes, database integration
patterns, and pointers to the bootstrap and CLI sub-shelves (each of which carries its own nested
digest).

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

## Maintenance Notes

- `cli-spec/` and `project-bootstrap-spec/` each have their own digest and are not expanded here.
- Regenerate when the area's knowledge changes.

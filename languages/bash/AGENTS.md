---
digest-of: languages/bash
last-synced: 2026-07-23
token-estimate: 300
---

# AGENTS

## Scope

Bash language notes at the top level, routing into the Bash CLI and bootstrap specs. Bash review
heuristics live in the owning specs rather than in a separate top-level rule dump.

## Key Points

- **CLI scripting rules**: `cli-spec/` owns entry-point shape, strict-mode caveats, module layout,
  ShellCheck discipline, error handling, temp cleanup, testing, install/XDG, and Bash idioms.
- **Bootstrap rules**: `project-bootstrap-spec/` owns the order for turning a fresh Bash project into
  a runnable, lintable, gated script project and cross-links to `cli-spec/` for detailed technique.
- **Review routing**: use the child digests and owning specs for Bash review heuristics; cross-link
  instead of copying rules into this top-level digest.

## Maintenance Notes

- `cli-spec/` and `project-bootstrap-spec/` each have their own AGENTS.md. This digest is the
  top-level router and should not duplicate detailed Bash rules.

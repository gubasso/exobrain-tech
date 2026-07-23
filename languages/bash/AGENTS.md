---
digest-of: languages/bash
last-synced: 2026-07-23
source-files:
  - README.md
  - cli-spec/AGENTS.md
  - project-bootstrap-spec/AGENTS.md
  - release-workflow-spec/AGENTS.md
token-estimate: 300
---

# AGENTS

## Scope

Bash language notes at the top level, routing into the Bash CLI, bootstrap, and release workflow
specs. Bash review heuristics live in the owning specs rather than in a separate top-level rule dump.

## Key Points

- **CLI scripting rules**: `cli-spec/` owns entry-point shape, strict-mode caveats, module layout,
  ShellCheck discipline, error handling, temp cleanup, testing, install/XDG, and Bash idioms.
- **Bootstrap rules**: `project-bootstrap-spec/` owns the order for turning a fresh Bash project into
  a runnable, lintable, gated script project and cross-links to `cli-spec/` for detailed technique.
- **Release workflow**: `release-workflow-spec/` owns tag, changelog, GitHub Release,
  `install.sh`, AUR, OBS, and release runbook guidance.
- **Review routing**: use the child digests and owning specs for Bash review heuristics; cross-link
  instead of copying rules into this top-level digest.

## Source Map

| Topic                                                        | File                                           |
| ------------------------------------------------------------ | ---------------------------------------------- |
| Top-level Bash index                                         | `README.md`                                    |
| CLI project spec and Bash review heuristics                  | `cli-spec/` (separate AGENTS.md)               |
| Project bootstrap ordering and quality gates                 | `project-bootstrap-spec/` (separate AGENTS.md) |
| Release workflow (tag → GitHub Release → install.sh/AUR/OBS) | `release-workflow-spec/` (separate AGENTS.md)  |

## Maintenance Notes

- `cli-spec/`, `project-bootstrap-spec/`, and `release-workflow-spec/` each have their own
  AGENTS.md. This digest is the top-level router and should not duplicate detailed Bash rules.
- The `release-workflow-spec/` binding (bash release + distribution: tag, git-cliff changelog,
  Makefile, install.sh, AUR, OBS) is the bash binding of the general
  `programming/release-workflow/` shelf.

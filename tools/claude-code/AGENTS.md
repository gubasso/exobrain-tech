---
digest-of: tools/claude-code
last-synced: 2026-07-26
token-estimate: 450
---

# AGENTS

## Scope

Top-level index for Claude Code and Codex CLI operational guidance. Subdirectories cover
orchestration, plan rounds, skill authoring, and implementation review practices.

## Key Points

- **Codex wrapper**: All Codex invocations use `codex-session` (not raw `codex`). Auto-selection is
  the default; omit `--account` for quota-aware selection or pass `--account auto` explicitly.
- **Sandbox**: Resume-compatible workflows use `--dangerously-bypass-approvals-and-sandbox` for all
  calls. One-shot workflows may use native sandbox flags. See `codex-conventions.md` §Unified
  Sandbox for Resume Workflows.
- **Safety rules**: Stages 1-2 read-only, stage 3 is the only write stage. Always `< /dev/null` to
  prevent stdin blocking. 600s Bash timeout for all Codex calls.
- **Session resumption**: `exec resume <thread-id>` preserves context. Original and resumed calls
  must use the same sandbox flags (see Resume Constraint in `codex-conventions.md`).
- **Behavioral orientation**: Every prompt starts with READ-ONLY or WRITE orientation block.
- **Git**: Codex must never run git commands; all git operations belong to the Claude Code
  orchestrator.
- **Model aliases, effort, and pricing**: no longer held here. They moved to the `cog` repository's
  own reference zone; look there rather than in this shelf.
- **Memory-file loading**: root/ancestor `CLAUDE.md` + `@imports` load eagerly at launch; a nested
  `CLAUDE.md` loads lazily on subtree access; import paths resolve relative to the importing file;
  Codex builds its `AGENTS.md` chain eagerly root→cwd at launch. See `memory-file-loading.md`.

## Maintenance Notes

- Each subdirectory has its own AGENTS.md for detailed digests.
- Claude Code conventions should be re-verified when major upstream behavior changes.

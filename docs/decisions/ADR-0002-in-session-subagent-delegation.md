# ADR-0002: In-session subagent delegation over headless `claude -p`

## Context and Problem Statement

`plan-queue-runner` drove each implementation round by shelling out to a headless `claude -p`
process running `/prex`. Headless mode has no event loop after the model's final turn: a backgrounded
Bash/Codex task is killed ~5 s after the result. So when the child backgrounded the long Codex
implementation call it was reaped before `/prex` stages 4–5 ran — the round silently stayed `doing`
while the child exited `0`. The `claude -p` host existed only because subagents historically could
not spawn subagents, and `/prex` must delegate internally (plan review, code review, review loop).

## Considered Options

- **A. Headless `claude -p` + a prose "never background" mandate.** Advisory only; the model overrode
  it on large rounds. Rejected.
- **B. Headless `claude -p` + a deterministic `PreToolUse` hook that denies backgrounded Codex
  calls.** Works, but adds a hook and keeps the fragile per-round process host. Deferred.
- **C. Run each round as an in-session foreground subagent.** Claude Code ≥ v2.1.172 supports nested
  subagents; foreground calls block the parent until the whole chain returns. The work runs in the
  long-lived parent process — real event loop, synchronous blocking, no per-round process exit — and
  `/prex` spawns its review stages as nested subagents (depth ≤ 3). Chosen.

## Decision Outcome

Chosen: **C**. A generic delegate subagent replaces `claude -p` wherever a fresh full Claude
execution is delegated. `plan-queue-runner` dispatches each round (and `/gc`) to it via the Agent
tool; round completion is verified deterministically by re-reading `QUEUE.yaml` (`status == done`).

We deliberately did **not** add the option-B backgrounding hook (architecture-only). Foreground
blocking plus the orchestrator's status check make the reaping failure structurally impossible. The
"never background a Codex call" rule stays in every Codex-driving skill (`prex`, `review-loop`,
`plan-writer-multi`, `ask`) as the prose safeguard.

## Consequences

- Good: synchronous blocking; shared live event loop; no per-round process; native nesting; one
  uniform delegation primitive; unattended privilege preserved via the inherited permission mode.
- Bad: the orchestrating session must stay alive for the whole run; we rely on foreground discipline
  rather than a hard hook (mitigated above). If a headless-orchestrator use re-emerges, revisit B.

## Status

Implemented (2026-06-17) in the standalone `cog` project, which now owns the Claude agent and
skill trees. Canon: canonical write pending in the public KB; see `tools/claude-code/orchestration/`.

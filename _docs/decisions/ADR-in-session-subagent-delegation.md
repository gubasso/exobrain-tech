# In-session subagent delegation over headless `claude -p`

## Context and Problem Statement

`plan-queue-runner` drove each implementation round by shelling out to a headless `claude -p`
process. Headless mode has no event loop after the model's final turn: a backgrounded task is killed
about five seconds after the result. So when the child backgrounded the long implementation call it
was reaped before the later stages ran, and the round silently stayed `doing` while the child exited
`0`. The headless host existed only because subagents historically could not spawn subagents, and
the round must delegate internally for plan review, code review, and the review loop.

## Considered Options

- Run each round as an in-session foreground subagent — chosen: nested subagents are supported, and
  a foreground call blocks the parent until the whole chain returns.
- Headless `claude -p` plus a prose "never background" mandate — rejected: advisory only, and the
  model overrode it on large rounds.
- Headless `claude -p` plus a deterministic `PreToolUse` hook denying backgrounded calls — deferred:
  revisit if a headless orchestrator re-emerges.

## Decision Outcome

Chosen option: the in-session subagent. The work runs in the long-lived parent process — real event
loop, synchronous blocking, no per-round process exit. A generic delegate subagent replaces
`claude -p` wherever a fresh full execution is delegated, and round completion is verified
deterministically by re-reading the queue record.

The backgrounding hook was deliberately not added. Foreground blocking plus the orchestrator's
status check make the reaping failure structurally impossible, and the "never background" rule stays
in every driving skill as the prose safeguard.

## Consequences

- Good: synchronous blocking, a shared live event loop, native nesting, one uniform delegation
  primitive, and unattended privilege preserved through the inherited permission mode.
- Bad: the orchestrating session must stay alive for the whole run, and the guarantee rests on
  foreground discipline rather than a hard hook.

## Status

Implemented (2026-06-17) in the standalone `cog` project, which now owns the agent and skill trees.

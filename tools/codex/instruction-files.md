# Codex instruction-file adapter

Verified 2026-09-14 against <https://learn.chatgpt.com/docs/agent-configuration/agents-md>.

Codex's instruction-file facts, against the portable rules in
[agent-integration/01 — Instruction files](../../programming/agent-integration/01-instruction-files.md).

## Which file Codex reads

`AGENTS.md`, and `AGENTS.override.md` ahead of it in the same directory. Other filenames are read
only where `project_doc_fallback_filenames` names them.

## The chain, and when it is built

Codex builds the chain once, eagerly, at launch.

1. The global scope: `AGENTS.override.md` then `AGENTS.md` under the Codex home directory, `~/.codex`
   unless `CODEX_HOME` says otherwise.
2. The project scope: a walk from the Git root down to the working directory, at most one file per
   directory.

The files concatenate from the root downward, so a file closer to the working directory appears
later and overrides what came before it.

## The consequence of an eager chain

A session launched from the repository root walks root to root, so it never reads a nested
`AGENTS.md` deeper in the tree. A rule that must be seen from a root-launched session lives in the
root file.

This is the difference that decides where a rule goes. See
[01 — Instruction files](../../programming/agent-integration/01-instruction-files.md#where-a-rule-goes-under-eager-and-lazy-loading)
for the portable rule it produces.

## The size limit

The chain stops once its combined size reaches `project_doc_max_bytes`, 32 KiB by default. Past that
point a file is not read at all, and no warning marks the boundary in the session. Raising the limit
or splitting the instructions across directories are the two ways out, and only the second one also
reduces what every session pays for.

## See also

- [agent-integration/01 — Instruction files](../../programming/agent-integration/01-instruction-files.md) —
  what the root file carries whatever the vendor.
- [skills](./skills.md) — the same vendor's skill facts.

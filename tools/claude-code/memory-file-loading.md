# Claude Code memory-file loading

How Claude Code and the `AGENTS.md`-native CLIs discover author-instruction files, and when each one
enters the context window. This is the tool-specific source of truth behind the language-agnostic
pattern in
[documentation canon 05 — Agent context](https://github.com/gubasso/spec-driven-docs/blob/v0.1.0/method/05-agent-context.md).

## Claude Code

- Claude Code reads **`CLAUDE.md`**, not `AGENTS.md`. To share one source with the other agents, a
  `CLAUDE.md` imports the shared file with `@AGENTS.md`.
- **Root and ancestor `CLAUDE.md` files load eagerly** at session start, concatenated from the
  filesystem root down to the working directory (broad → specific).
- **A nested `CLAUDE.md` in a subdirectory loads on demand** — only when Claude reads a file in that
  subdirectory. This is the lazy mechanism that scopes rules to a subtree.
- **`@import` expansion is eager**: an imported file is pulled into context alongside the file that
  imports it. A root `@import` therefore saves no context; a nested `CLAUDE.md`'s `@import` loads
  only when that nested file loads.
- **Import paths resolve relative to the importing file**, not the working directory. A nested
  `docs/x/CLAUDE.md` imports its sibling as `@AGENTS.md`, never `@docs/x/AGENTS.md` (which would
  resolve to `docs/x/docs/x/AGENTS.md`). Imports recurse up to four hops; an `@` inside a code span
  or fenced block is not treated as an import.
- **`.claude/rules/` with `paths:` frontmatter** is the alternative path-scoped lazy mechanism: a
  rule file loads only when Claude works with files matching its globs.

## Codex and other `AGENTS.md`-native CLIs

- Codex reads nested `AGENTS.md` files, but **builds its instruction chain eagerly, once at launch**,
  walking from the Git root down to the current working directory; files closer to the cwd override
  earlier ones. It is **not** triggered lazily by touching files in a subtree.
- Consequence: a Codex session launched **from the repo root** never loads a nested `AGENTS.md`
  deeper in the tree (root → cwd is just the root). A rule that must be seen from a root-launched
  session has to live in the root `AGENTS.md`.

## Design consequence

Split a large author-instructions file only along the eager/lazy seam: subtree-local rules go to a
nested `AGENTS.md` (plus a one-line `CLAUDE.md` bridge for Claude Code), while cross-cutting rules
stay in root so both the eager Codex chain and Claude sessions working elsewhere still see them.
Leave a plain pointer — not an eager `@import` — from root to the nested file.

## Sources

- Claude Code memory & imports: <https://code.claude.com/docs/en/memory>
- AGENTS.md standard (nested files, nearest-wins): <https://agents.md/>
- Codex AGENTS.md configuration (eager root→cwd chain):
  <https://learn.chatgpt.com/docs/agent-configuration/agents-md>

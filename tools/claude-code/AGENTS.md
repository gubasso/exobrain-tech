---
digest-of: tools/claude-code
last-synced: 2026-09-14
token-estimate: 420
---

# AGENTS

## Scope

Claude Code's own behavior, dated and sourced from that vendor's documentation, as adapters to the
portable rules in `programming/agent-integration/`. Nothing here states a rule that holds for
another runtime, and nothing here documents another product's skills or contracts.

## Key points

### Instruction files (memory-file-loading.md)

- Claude Code reads `CLAUDE.md`, never `AGENTS.md`. A repository that keeps one source bridges with
  `@AGENTS.md` or a symlink.
- The working directory and its ancestors load at launch. A `CLAUDE.md` below the working directory
  loads when Claude reads a file in that subtree. Files concatenate root-down, so the closest one is
  read last.
- An `@import` expands when the file naming it loads, and its path resolves against that file rather
  than the working directory. Four hops maximum; an `@` inside code is text.
- `.claude/rules/` is the path-scoped alternative: a rule with `paths` globs loads only against a
  matching file, and a rule without them loads at launch.

### Skills (skills.md)

- Six discovery roots. Precedence runs enterprise, personal, project, and any of those over a
  bundled skill; a nested package stays loaded under a qualified name.
- Identity comes from the directory for a personal or project package, and frontmatter `name` is a
  display label. The specification does the opposite.
- Fourteen frontmatter fields beyond the specification's, including `model`, `effort`, `context:
  fork`, `paths`, and `hooks`. Only the specification's six are read outside Claude Code.
- `allowed-tools` grants for the invoking turn and clears at the next message; `disallowed-tools`
  removes for the same window. Invocation itself is governed by `Skill(<name>)` permission rules.
- The body loads once and stays, so it carries standing instructions. Re-attachment after
  summarization keeps 5,000 tokens per skill inside a 25,000-token budget.

## Maintenance notes

- Both pages carry a verified date and the vendor URL they were read from. Re-read the source before
  changing a fact, and advance the date in the same edit.
- A rule that turns out to hold for any runtime belongs in the shelf chapter the page links, and the
  page keeps only this vendor's version of it.

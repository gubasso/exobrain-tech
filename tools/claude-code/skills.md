# Claude Code skills adapter

Verified 2026-09-14 against <https://code.claude.com/docs/en/skills>.

Claude Code's differences from the portable baseline in
[agent-integration/03 — Skills](../../programming/agent-integration/03-skills.md). The baseline is
the conforming package the Agent Skills specification defines; everything below is this vendor's
addition to it or exception from it.

## Discovery roots and precedence

| Root       | Path                                                       | Loads                                              |
| ---------- | ---------------------------------------------------------- | -------------------------------------------------- |
| Enterprise | `.claude/skills/<name>/SKILL.md` under managed settings    | At session start, for every user of the deployment |
| Personal   | `~/.claude/skills/<name>/SKILL.md`                         | At session start                                   |
| Project    | `.claude/skills/<name>/SKILL.md` at the repository root    | At session start                                   |
| Nested     | `<subdir>/.claude/skills/<name>/SKILL.md`                  | On first file access inside that subdirectory      |
| Additional | `.claude/skills/<name>/SKILL.md` under an `--add-dir` path | At session start, for that session only            |
| Plugin     | `<plugin>/skills/<name>/SKILL.md`                          | When the plugin loads                              |

Precedence on a name collision runs enterprise, then personal, then project, and any of those over a
bundled skill. A project-root package wins over a nested one, and both stay loaded because the
nested one answers to a qualified name.

## Fields beyond the specification

The specification's `name`, `description`, `license`, `compatibility`, `metadata`, and the
experimental `allowed-tools` are read as written. Claude Code adds the following, and a package that
uses one is a package only Claude Code reads in full.

| Field                            | What it does                                                                |
| -------------------------------- | --------------------------------------------------------------------------- |
| `when_to_use`                    | Extra trigger text, appended to `description` in the listing                |
| `argument-hint`                  | Autocomplete hint for the slash-command form                                |
| `arguments`                      | Named positional arguments, substituted as `$name` in the body              |
| `disable-model-invocation`       | `true` removes the skill from the model's context; only a person invokes it |
| `user-invocable`                 | `false` hides the skill from the slash menu; only the model invokes it      |
| `disallowed-tools`               | Tools removed from the pool while the skill is active                       |
| `model`, `effort`                | Per-skill overrides of the session's model and effort level                 |
| `context`, `agent`, `background` | `context: fork` runs the body as an isolated subagent's whole prompt        |
| `hooks`                          | Hooks registered when the skill invokes, for the rest of the session        |
| `paths`                          | Globs that limit automatic invocation to matching files                     |
| `shell`                          | `bash` or `powershell`, for the dynamic injection below                     |

## Identity is derived from the path

For a personal or project package the command name is the directory name, and frontmatter `name`
sets the display label alone. A nested package that collides with a root one answers to its
subdirectory path with `:` as the separator, as `/apps/web:deploy`. A plugin package is prefixed
with the plugin's name. Name matching ignores case, spacing, and invisible characters.

`synced` is reserved as a directory name.

## Activation and reload

Descriptions sit in context every turn; the body loads only when the skill invokes, and stays in
context for the rest of the session. The body is not re-read on a later turn, so it carries standing
instructions rather than one-time steps. After summarization each re-attached skill keeps its most
recent 5,000 tokens, and all of them share 25,000.

The listing truncates `description` plus `when_to_use` at 1,536 characters, which is wider than the
specification's 1,024-character cap on `description` alone.

Changes under `~/.claude/skills/`, `.claude/skills/`, and any `--add-dir` path apply inside the
running session. A plugin's hooks, agents, and MCP configuration need `/reload-plugins`.

## The permission model

`allowed-tools` pre-approves tools for the turn the skill invokes on, and the grant clears at the
next message. It grants and never restricts. `disallowed-tools` removes tools for the same window.
Both accept Bash rules, as `Bash(git add *)`.

Invocation itself is governed by permission rules over `Skill(<name>)`, which match the skill's own
name, its aliases, and the unqualified form.

The body may inject shell output before the model reads it, inline as `` !`command` `` or as a
fenced `!` block. The setting `disableSkillShellExecution` turns that off, and a skill that depends
on it has to keep working when the injected text comes back empty.

## String substitution

`$ARGUMENTS`, `$ARGUMENTS[N]`, `$N`, and `$name` carry the invocation's arguments.
`${CLAUDE_SKILL_DIR}` resolves to the directory holding `SKILL.md`, which is how a bundled script is
named without depending on the working directory. `${CLAUDE_PROJECT_DIR}`, `${CLAUDE_SESSION_ID}`,
and `${CLAUDE_EFFORT}` carry session state.

## Exceptions to the baseline

- The specification requires `name` and `description`. Claude Code treats both as optional, falling
  back to the directory name and to the first non-empty line of the body.
- The specification derives identity from `name`, which must match the directory. Claude Code
  derives the command from the path and uses `name` as a label.
- Outside Claude Code, only `name`, `description`, `license`, `compatibility`, `metadata`, and
  `allowed-tools` are read. A package that depends on any other field above is not portable.

## See also

- [agent-integration/03 — Skills](../../programming/agent-integration/03-skills.md) — the baseline
  this adapter deviates from.
- [agent-integration/04 — Skill distribution](../../programming/agent-integration/04-skill-distribution.md) —
  what the roots above are an instance of.
- [memory-file-loading](./memory-file-loading.md) — the same vendor's instruction-file facts.

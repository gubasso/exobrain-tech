# OpenCode skills adapter

Verified 2026-09-14 against <https://opencode.ai/docs/skills/>.

OpenCode's differences from the portable baseline in
[agent-integration/03 — Skills](../../programming/agent-integration/03-skills.md).

## Discovery roots and precedence

Six roots, searched in this order. The first match answers.

| Order | Path                                        | Scope   |
| ----- | ------------------------------------------- | ------- |
| 1     | `.opencode/skills/<name>/SKILL.md`          | Project |
| 2     | `~/.config/opencode/skills/<name>/SKILL.md` | User    |
| 3     | `.claude/skills/<name>/SKILL.md`            | Project |
| 4     | `~/.claude/skills/<name>/SKILL.md`          | User    |
| 5     | `.agents/skills/<name>/SKILL.md`            | Project |
| 6     | `~/.agents/skills/<name>/SKILL.md`          | User    |

A project root is searched by walking up from the working directory until the Git worktree's own
root is reached.

Reading another vendor's root and the standard root alongside its own is this runtime's own choice,
and it is what lets one installed package serve several runtimes here without a link.

## Fields beyond the specification

None. OpenCode reads `name`, `description`, `license`, `compatibility`, and `metadata`, which is the
specification's set. An unrecognized field is ignored, which is the portability rule the
specification states.

`allowed-tools` is not among the fields read, so the permission model below is the whole of what
governs a skill here.

## Identity is derived from both

`name` must match the containing directory, and must match `^[a-z0-9]+(-[a-z0-9]+)*$`: one to 64
characters, lowercase alphanumeric, single hyphens, no leading or trailing hyphen, no `--`. That is
the specification's rule, checked.

## Activation

The agent loads a skill by calling the native `skill` tool with the skill's name, as
`skill({ name: "git-release" })`. Loading happens on demand, when the agent decides a skill applies.

## The permission model

`opencode.json` carries pattern-matched permissions over skill names, with wildcards, as
`internal-*`. Each pattern resolves to one of three behaviors.

| Permission | Effect                             |
| ---------- | ---------------------------------- |
| `allow`    | The skill loads immediately        |
| `deny`     | The skill is hidden from the agent |
| `ask`      | The person approves the load       |

The setting holds globally and can be overridden for one agent.

## Exceptions to the baseline

None that weaken the specification. This runtime reads the specification's fields, enforces the
specification's naming rule, and adds a permission layer outside the package.

## See also

- [agent-integration/03 — Skills](../../programming/agent-integration/03-skills.md) — the baseline
  this adapter conforms to.
- [agent-integration/04 — Skill distribution](../../programming/agent-integration/04-skill-distribution.md) —
  why reading another runtime's root matters to a project shipping one package.

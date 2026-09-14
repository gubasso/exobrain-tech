# Gemini CLI skills adapter

Verified 2026-09-14 against <https://geminicli.com/docs/cli/skills/> and
<https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/creating-skills.md>.

Gemini CLI's differences from the portable baseline in
[agent-integration/03 — Skills](../../programming/agent-integration/03-skills.md).

## Discovery tiers and precedence

Four tiers, lowest priority first. The last one that carries a name is the one that answers.

| Tier      | Path                                             |
| --------- | ------------------------------------------------ |
| Built-in  | The skills shipped with the CLI                  |
| Extension | The skills bundled inside an installed extension |
| User      | `~/.gemini/skills/` or `~/.agents/skills/`       |
| Workspace | `.gemini/skills/` or `.agents/skills/`           |

Inside one tier the `.agents/skills/` form wins, which is the standard root other runtimes read, so
a package placed there is reachable from more than this vendor.

## The depth limit

`SKILL.md` is discovered at the root of a skills directory or exactly one directory below it.
Anything deeper is not found, and nothing reports the omission. The filename is matched exactly, so
a case-sensitive filesystem rejects `skill.md`.

## Fields beyond the specification

None. Gemini CLI reads `name` and `description`.

Both are required in practice rather than by warning: a `SKILL.md` missing either field, missing the
`---` delimiters, or carrying any text at all before the opening `---` is skipped in silence. A
heading, a comment, and a blank line each count as text.

## Identity is derived from the frontmatter

The `name` field names the skill, not the directory. The specification requires the two to match,
and this runtime does not check it, so a package that disagrees with itself loads here and fails
validation elsewhere.

## Activation and reload

At session start the CLI scans every tier and injects each enabled skill's name and description into
the system prompt. When a request matches a description the model calls the `activate_skill` tool. A
confirmation prompt then shows the skill and the directory access it asks for, and the body plus the
package's folder structure enter the conversation on approval.

`/skills reload`, or `/skills refresh`, rescans every tier.

## The permission model

Activation is the consent point, and it is explicit: the person approves the named skill before its
body is read. Approval also adds the package's own directory to the agent's allowed file paths,
which is what lets a bundled reference or asset be read at all. Nothing outside that directory is
granted by activating the skill.

## Exceptions to the baseline

- A package more than one directory below a discovery root does not exist, as far as this runtime is
  concerned.
- `name` need not match the directory.
- A malformed package is skipped rather than reported.

## See also

- [agent-integration/03 — Skills](../../programming/agent-integration/03-skills.md) — the baseline
  this adapter deviates from.

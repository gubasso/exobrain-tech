# Codex skills adapter

Verified 2026-09-14 against <https://learn.chatgpt.com/docs/build-skills>.

Codex's differences from the portable baseline in
[agent-integration/03 — Skills](../../programming/agent-integration/03-skills.md).

## Discovery roots and precedence

| Scope      | Path                                                                             |
| ---------- | -------------------------------------------------------------------------------- |
| Repository | `.agents/skills`, in the working directory, each parent, and the repository root |
| User       | `$HOME/.agents/skills`                                                           |
| Admin      | `/etc/codex/skills`                                                              |
| System     | The skills bundled with Codex                                                    |

Repository scope wins over user, user over admin, admin over system. Two skills that share a name
are not merged and both stay available, which is the opposite of the resolution most runtimes
perform, so a name collision here surfaces as a choice the model makes rather than as one entry.

Codex reads the standard `.agents/skills` root and no root of its own, so a package placed there is
reachable from any runtime that also reads it.

## Fields beyond the specification

None. Codex reads `name` and `description`, which the specification requires, and the frontmatter is
the whole discovery contract. A field another vendor adds is ignored rather than rejected, which is
the portability rule the specification states.

Model and effort are session settings, chosen through the profile Codex launches with. A package
that sets them in frontmatter carries a field Codex does not read, and the choice is not the
package's to make here.

## Identity is derived from the frontmatter

The `name` field identifies the skill. The directory holding `SKILL.md` is an organizational path
and nothing more, so a package may sit at any depth under a discovery root.

## Activation and reload

At session start Codex injects each discovered skill's name, description, and file path. The body is
read from disk when the skill is selected.

The skill list takes "at most 2% of the model's context window, or 8,000 characters when the context
window is unknown". A description can therefore be shortened, so the vendor says to "Front-load the
key use case and trigger words so a host can still match the skill if descriptions are shortened."
Read 2026-10-08 at <https://learn.chatgpt.com/docs/build-skills>.

A person selects a skill explicitly with `$<name>` in the CLI and the editor extension, or `@<name>`
in ChatGPT. The model selects one implicitly when the request matches a description.
`policy.allow_implicit_invocation: false` in `agents/openai.yaml` turns implicit selection off,
which leaves the skill reachable by name alone.

Codex detects a changed skill without a restart. A change to `~/.codex/config.toml` needs one.

## Authoring guidance

The skills guide, read 2026-10-08 at <https://learn.chatgpt.com/docs/build-skills>:

- "Keep each skill focused on one job."
- "Prefer instructions over scripts unless you need deterministic behavior or external tooling."
- "Write imperative steps with explicit inputs and outputs."

The vendor's skill-creator skill, read 2026-10-08 at
<https://github.com/openai/skills/blob/main/skills/.system/skill-creator/SKILL.md>:

- "Always use imperative/infinitive form."
- "Information should live in either SKILL.md or references files, not both."
- A package holds no README, install guide, quick reference, or changelog.
- The body stays under 5,000 words, and a reference file over 100 lines opens with a table of
  contents.

The instruction-file chain has its own byte limit, stated in
[instruction-files](./instruction-files.md). The portable sources beside these are in
[agent-integration/21 — Skill format evidence](../../programming/agent-integration/21-skill-format-evidence.md).

## The permission model

Codex ships no per-skill permission model. A script under `scripts/` runs under the workspace's own
sandbox and approval settings, the same as any other command the session runs, so the boundary a
skill operates inside is the session's rather than the package's.

## Exceptions to the baseline

- A package's directory name need not match `name`, and nesting under a discovery root is free.
  A package that relies on either is portable to Codex but not from it.
- A name collision resolves to two entries rather than one.

## See also

- [agent-integration/03 — Skills](../../programming/agent-integration/03-skills.md) — the baseline
  this adapter deviates from.
- [instruction-files](./instruction-files.md) — the same vendor's instruction-file facts.

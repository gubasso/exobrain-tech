# 03 — Skills

A skill is the playbook layer's artifact. It tells the agent when to reach for a mechanism and how
to compose it into a workflow. It does not restate the mechanism's own reference material, and it
teaches judgment rather than syntax.

## The central rule

A skill has one authored owner for each kind of content it carries. Sequencing and judgment live in
`SKILL.md`. Durable knowledge lives in a reference file the package owns. Deterministic mechanics
live in a script or in the mechanism's own command.

The rule generalizes: a fact has one owner. Where two files state the same rule, one of them is
already wrong and no reader can tell which.

## The package

A conforming skill is a directory holding one `SKILL.md` and whatever that file points at.

```text
<skill-name>/
├── SKILL.md          # the playbook: frontmatter, then the body
├── references/       # what the agent reads when a step calls for it
├── scripts/          # what the agent runs, never reads
└── assets/           # templates the agent copies and fills in
```

The three subdirectories are conventions, not requirements. What makes the package a skill is
`SKILL.md` with valid frontmatter; everything else is progressive disclosure, and a skill that needs
none of it is one file.

A package carries what it needs to run. It reads its own references by a path relative to the skill
directory, and it does not depend on a repository the reader may not have. An external URL is
further reading, and the skill still works when nobody can reach it.

## The frontmatter contract

Two fields are required, and four are optional. The Agent Skills specification defines them, and it
is the source of every rule in this section: <https://agentskills.io/specification>, read
2026-09-14.

| Field           | Required | What it must satisfy                                                                                                                |
| --------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `name`          | Yes      | 1 to 64 characters, lowercase letters, digits, and single hyphens, no leading or trailing hyphen, and it matches the directory name |
| `description`   | Yes      | 1 to 1024 characters, stating both what the skill does and when to use it                                                           |
| `license`       | No       | The license name, or the name of a bundled license file                                                                             |
| `compatibility` | No       | Up to 500 characters naming an environment requirement                                                                              |
| `metadata`      | No       | A map of string keys to string values, for a reader's own tooling                                                                   |
| `allowed-tools` | No       | Pre-approved tools, and marked experimental                                                                                         |

An unrecognized field is ignored rather than rejected, and that rule is what keeps a package
portable. The baseline is the set above, never the intersection of what runtimes do today, because
an intersection shrinks every time one of them drops a field. Angle brackets belong nowhere in the
frontmatter, because the text goes into a system prompt and a bracketed string reads there as
something other than data.

## The description is the whole trigger surface

The agent sees every installed skill's name and description at once, and no body at all. The
description is the only evidence it has when it decides to load the skill, so a description that
explains what the skill does answers the wrong question. State what the person was trying to do.

```yaml
---
name: dispatch-messages
description: |
  Use this skill when the user dispatches a message through the carrier service, manages
  the flock, checks delivery status, or troubleshoots a roost. Triggers include "send a
  message", "dispatch to roost X", "check message MSG-*". Do not use it for generic
  messaging over email or chat.
---
```

Three parts carry the routing.

1. What the skill produces, in one clause, in the words the person uses.
2. When to use it, as the literal phrasings a person types, including the informal ones.
3. The boundary against the nearest skill that is not this one, where a near one exists.

An agent under-triggers a skill whose description reads like a title, and over-triggers one that
names a domain in the abstract. Ambiguity between two skills costs more than a missing skill, so the
boundary clause earns its characters. Routing is probabilistic, and a description is a claim about
it, so [05 — Evaluations](./05-evaluations.md) states how the claim is checked.

## Progressive disclosure is a budget

Each layer of the package is paid for at a different rate, and a fact belongs in the layer that
matches how often the agent needs it.

| Layer           | Read                               | Cost                 |
| --------------- | ---------------------------------- | -------------------- |
| `description`   | Always, for every installed skill  | Highest              |
| `SKILL.md` body | On every invocation of this skill  | High                 |
| `references/`   | Only when the body says to read it | Low                  |
| `scripts/`      | Only when executed                 | Nothing at load time |

The specification's guidance follows the same shape: roughly 100 tokens of metadata for every
installed skill, under 5,000 tokens of body once a skill activates, and resources on demand. Keep
`SKILL.md` under 500 lines.

One test decides every paragraph of the body.

> Delete this paragraph. Does the agent now make a likely task error?

Yes keeps it in the body. Only on some runs moves it to a reference. No deletes it. Most first
drafts fail the test the same three ways: a fact a capable agent already knows, a rule restated from
the reference that owns it, and text that tells the agent the task matters.

Point at a reference with its condition attached, so the agent knows when the read is worth it.

```markdown
For a Rust project, read `references/rust.md` before you edit the manifest.
```

Keep the package one level deep. The body points at a reference, and a reference points at no
further reference, because each hop costs a read and hides which file owns the rule.

## The body

The body is a table of contents with judgment in it, not a manual.

- Write in the third-person imperative. "Run the dry-run first", never "you should" or "I will".
  An agent reads a conditional modal as optional and skips it.
- Put the condition before the instruction it controls.
- Point at the mechanism's own help output rather than copying flags. A copied flag list is wrong
  the first time a flag changes, and nothing tells you when.
- State why a rule exists. A model follows a reason further than it follows an instruction in
  capital letters, and a rule with no reason is one the agent discards when the case looks
  different.
- Number a multi-step workflow, with the exact command at each step, and spell out the branch where
  one exists.
- Keep one word for one meaning across the whole package.
- Where a step produces a structured artifact, put a filled-in template under `assets/` and tell the
  agent to copy it. An agent matches a pattern more reliably than it follows a description of one.

## Scripts run, references are read

A helper that must behave identically every run belongs in `scripts/`, and the skill instructs the
agent to execute it. Reference material belongs in `references/`, and the skill instructs the agent
to read it. An agent asked to re-derive deterministic logic every run will occasionally derive it
differently.

Extract any chunk that is deterministic and more than a trivial one-liner. Single use is reason
enough, because the body's cost is paid at load rather than at call. Four guardrails keep the rule
from overshooting.

1. Split a chunk that tangles judgment with mechanism. Extract the deterministic half, which is
   usually a mapping, a validation, or a piece of scaffolding, and keep the decision as prose.
2. Keep a trivial one-liner inline. Wrapping a single existence check costs more than it saves.
3. Be coarse. A few scripts that each do a meaningful unit and emit one structured result beat a
   cloud of small helpers stitched together with parsing between every call.
4. Leave prompt content to the model. Extract the scaffolding that writes the file and sets the
   flags, never the natural-language text the skill passes onward.

A script runs without a terminal and never prompts. It bounds its output and says so when it
truncates. It writes its machine-readable result to standard output and its progress to standard
error, so a caller parses one without stripping the other. It returns a distinct exit code per
outcome, defaults to the safe action, and offers a dry run wherever it changes or deletes anything.

A skill that calls a tool declares the tool, checks for it, and fails with a message naming the
cause and the fix. A hard dependency blocks the run; a soft one degrades a single feature, and the
skill says which feature it turned off.

Intermediate work belongs outside the reader's project, in a directory the skill creates and
removes. A deliverable is not intermediate work and goes to its real destination.

## Validation loops

The highest-value shape a skill can carry is a loop that checks the work before it commits it.

```markdown
## Validation loop

1. Draft the artifact.
2. Run the dry-run command and read its output.
3. Where it reports an error, revise and repeat from step 2.
4. When the dry-run passes, run the command without it.
5. Verify the result with the tool's own verify command.
```

The loop works because each step's check is the mechanism's own, not the agent's judgment about
whether the work looks right.

## See also

- [00 — The four layers](./00-model.md) — what the playbook layer never carries.
- [04 — Skill distribution](./04-skill-distribution.md) — where a package installs.
- [05 — Evaluations](./05-evaluations.md) — how a skill is checked rather than assumed.
- [15 — Skill security](./15-skill-security.md) — the rules that bound what a run reaches.

## The adapters

Each of these states one runtime's additions to the baseline above, dated and sourced from that
runtime's own documentation. None of them weakens the baseline.

- [tools/claude-code — skills](../../tools/claude-code/skills.md)
- [tools/codex — skills](../../tools/codex/skills.md)
- [tools/gemini-cli — skills](../../tools/gemini-cli/skills.md)
- [tools/opencode — skills](../../tools/opencode/skills.md)

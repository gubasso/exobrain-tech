# 03 — Skills

A skill is the playbook layer's artifact. It tells the agent when to reach for a mechanism and how
to compose it into a workflow. It does not restate the mechanism's own reference material, and it
teaches judgment rather than syntax.

## The package

A conforming skill is a directory holding one `SKILL.md` and whatever that file points at.

```text
<skill-name>/
├── SKILL.md          # the playbook: frontmatter, then the body
├── reference/        # what the agent reads when a step calls for it
├── scripts/          # what the agent runs, never reads
└── assets/           # templates the agent copies and fills in
```

The three subdirectories are conventions, not requirements. What makes the package a skill is
`SKILL.md` with valid frontmatter; everything else is progressive disclosure, and a skill that needs
none of it is one file.

## The frontmatter is the triggering mechanism

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

Two fields carry the contract. `name` identifies the skill. `description` is the whole of what
decides whether it triggers, because the body loads only after triggering. Every word about when to
use the skill belongs in the description, and every word about how to use it belongs in the body.

What makes a description work:

- Concrete triggers. Name the phrasings and the task types, not the domain in the abstract. An agent
  under-triggers a skill whose description reads like a title.
- Negative scope, stated. The nearby task this skill is not for is what prevents a false positive,
  and no other field can say it.
- The limits the specification sets: `name` at 64 characters, `description` at 1024.

Some runtimes read a third field, `allowed-tools`, restricting what the skill may call. The
specification marks it experimental, so a skill that depends on it depends on one runtime's current
behavior. Every other field a runtime reads is that vendor's extension, and it belongs in that
vendor's adapter rather than in a portable package.

## The body

The body is a table of contents with judgment in it, not a manual.

- Keep it short enough to load without thought. Past roughly 500 lines, split into `reference/` and
  point at the files.
- Write in the third-person imperative. "Run the dry-run first", never "you should" or "I will".
- Point at the mechanism's own help output rather than copying flags. A copied flag list is wrong
  the first time a flag changes, and nothing tells you when.
- State why a rule exists. A model follows a reason further than it follows an instruction in
  capital letters, and a rule with no reason is one the agent discards when the case looks
  different.
- Number a multi-step workflow, with the exact command at each step, and spell out the branch where
  one exists.

## Templates beat prose

An agent matches a pattern more reliably than it follows a description of one. Where a step produces
a structured artifact, put a filled-in template under `assets/`, and tell the agent to copy it.

## Scripts run, references are read

A helper that must behave identically every run belongs in `scripts/`, and the skill instructs the
agent to execute it. Reference material belongs in `reference/`, and the skill instructs the agent
to read it. The distinction matters because an agent asked to re-derive deterministic logic every
run will occasionally derive it differently.

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

## Security

A skill is instructions a model follows, so it is an input like any other. Two consequences:

- A skill from outside the project is code review, not a download. Read what its scripts do before
  installing it, because the agent will run them.
- A skill that reaches for credentials, a network endpoint, or another product's command states why
  in the body. A reader who cannot tell what a skill touches cannot approve it.

## Read it as the agent would

Before a skill ships, feed the whole package to a model and ask it to simulate executing the skill
against a real request, flagging every step where it would have to guess. What comes back is the
list of ambiguities the author could not see, and it costs one prompt.

## See also

- [00 — The four layers](./00-model.md) — what the playbook layer never carries.
- [04 — Skill distribution](./04-skill-distribution.md) — where a package installs.
- [05 — Evaluations](./05-evaluations.md) — how a skill is checked rather than assumed.
- [tools/claude-code](../../tools/claude-code/skill-authoring/skill-spec.md) — one vendor's field
  table, dated, as an adapter to this baseline.

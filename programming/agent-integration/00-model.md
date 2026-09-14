# 00 — The Four Layers

What a project carries for a coding agent falls into four layers. Each layer answers a different
question, each has one class of artifact, and a project that skips one makes the other three work
harder than they should.

## The layers

| Layer             | The question it answers                    | The artifact                                                                                              |
| ----------------- | ------------------------------------------ | --------------------------------------------------------------------------------------------------------- |
| Mechanism         | What does the agent run?                   | The command-line tool, its subcommands, and the protocol server where one is warranted                    |
| Playbook          | What does the agent load for this task?    | The skill: when to reach for the mechanism, and how to compose it into a workflow                         |
| Durable knowledge | What does the agent read on demand?        | The project's documentation, specifications, runbooks, and whatever makes an embedded corpus discoverable |
| Governance        | What binds every session, before any task? | The root instruction file, and the boundary on what the agent may do without being asked                  |

The layers compose. Governance is loaded before the agent reads the request. The playbook loads when
the task matches it. Durable knowledge is fetched by name when a step needs it. The mechanism runs.

## What each layer carries, and never carries

Mechanism carries determinism. The same inputs produce the same outputs, and every output is
machine-readable on request. It never carries judgment about when to run: a tool that refuses a
reasonable command because a policy says so has moved governance into the wrong layer.

Playbook carries judgment. When to reach for the mechanism, which order the steps take, what to
check before a destructive one. It never carries the mechanism's reference material: a skill that
lists flags rots the first time a flag changes, and the mechanism's own help output is the source of
truth for those.

Durable knowledge carries what outlives the task. Specifications, decisions, runbooks, and the
catalog that lets an agent find them by name rather than by walking the tree. It never carries
triggering: a document is read because a step named it, not because the agent guessed it applied.

Governance carries what holds across every task in the project. The conventions, the routing to
everything else, and the actions an agent takes only where the request named them. It never carries
what a single task needs: a rule that binds one workflow belongs to that workflow's playbook.

## Who ships what

A project that owns a land ships the skills that operate that land. The skills live in that
project's own tree, and its own command installs and removes them. A project that documents another
product's command, or ships a skill that drives it, has taken on maintenance it cannot honor: the
other product changes its surface, and the copy is wrong before anyone notices.

Shared material that several skills read installs once, under the user's state directory, and each
skill names it by absolute path. Two agent runtimes agree on no relative layout, so a path that
reaches the shared file from one does not reach it from the other.

A skill's identity is its name, and a name has one owner. Installing a second copy of one skill
under a second scope makes two entries under one name, and which one answers is not something the
project controls.

## The layer boundaries a design gets wrong

Three mistakes recur, and each one is a fact placed in the wrong layer.

The mechanism absorbs the playbook. The tool grows a subcommand per workflow, each one encoding the
order of steps for a case its author imagined. The agent's judgment is what the playbook layer
exists for, and a workflow frozen into a binary cannot be adapted to a request that differs.

The playbook absorbs durable knowledge. The skill grows a reference section, then a schema, then a
troubleshooting catalog. Everything it copies is a second home for a fact the project already
states, and the two drift.

Governance absorbs everything. The instruction file collects every rule anyone wanted an agent to
follow, and stops being read, because a file that large is a file the agent skims. Governance routes
to the other three layers and states only what has no other owner.

## See also

- [01 — Instruction files](./01-instruction-files.md) — the governance layer's artifact.
- [03 — Skills](./03-skills.md) — the playbook layer's artifact and its portable package shape.
- [05 — Evaluations](./05-evaluations.md) — how a project checks that the layers work together.
- [90 — Worked example](./90-worked-example.md) — one project shown across every layer at once.
- [cli-design/05 — Designing for LLM coding agents](../cli-design/05-designing-for-llm-agents.md) —
  the mechanism layer, where the mechanism is a command-line tool.

# Agent Integration

How a project is built so that a coding agent can discover it, operate it, and be trusted with it.
The subject is the project's side of the relationship: the instruction file it carries, the actions
it lets an agent take, the skills it ships, where those skills install, the documentation it makes
discoverable, and how any of it is checked. The agent's own runtime is not the subject, and a fact
about one vendor's runtime lives in that vendor's shelf as a dated adapter.

The mechanism an agent runs is usually a command-line tool, and designing one is
[cli-design](../cli-design/README.md). This shelf is everything above the binary.

## The four layers

The shelf's spine is a model of what a project carries, in four layers. Each answers a different
question, and the index follows it.

| Layer             | The question                            | Where it is stated                                        |
| ----------------- | --------------------------------------- | --------------------------------------------------------- |
| Mechanism         | What does the agent run?                | [cli-design](../cli-design/README.md) for the tool itself |
| Playbook          | What does the agent load for this task? | [03](./03-skills.md), [04](./04-skill-distribution.md)    |
| Durable knowledge | What does the agent read on demand?     | the project's own documentation                           |
| Governance        | What binds every session?               | [01](./01-instruction-files.md)                           |

[00 — The four layers](./00-model.md) states the model itself: what each layer carries, what it
never carries, who ships what, and the three boundary mistakes a design makes.

## The chapters

| Chapter                                               | What it holds                                                                                                            |
| ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| [00 — The four layers](./00-model.md)                 | The model, the ownership rule, and the boundaries a design gets wrong                                                    |
| [01 — Instruction files](./01-instruction-files.md)   | What the root file carries, why it routes instead of restating, and the generated digest beside it                       |
| [03 — Skills](./03-skills.md)                         | The portable package, the frontmatter that triggers it, the body, scripts against references, validation loops, security |
| [04 — Skill distribution](./04-skill-distribution.md) | The two scopes, one source for many runtimes, and what an installer owes a home                                          |
| [05 — Evaluations](./05-evaluations.md)               | Why an eval is a separate signal from a test, the three levers, and the tests an agent writes badly                      |
| [90 — Worked example](./90-worked-example.md)         | One project across every layer: the command tree, the outputs, the skill, the instruction lines, and the eval            |

## The irreducible defaults

A project that does nothing else does these.

- Ship a mechanism an agent can discover from the mechanism: complete help output, machine-readable
  output on request, and stable exit codes.
- Make every output a turn in the conversation. A silent success and a bare failure are both dead
  ends.
- Put the workflow in a skill and the conventions in the instruction file, and let each point at the
  other rather than copying it.
- State the boundary. An agent takes an irreversible action only where the request named it.
- Check the agent's use of the tool, not only the tool. One prompt, ten samples, and a rate you
  watch over time.

## What this shelf is not

It is not a survey of agent runtimes, and it names no vendor as the way. It is not a place for one
person's configuration, which belongs in the vault. And it is not a home for another product's
documentation: a project that ships a land documents its own skills, and this shelf states why.

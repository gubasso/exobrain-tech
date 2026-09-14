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

| Layer             | The question                            | Where it is stated                                                                   |
| ----------------- | --------------------------------------- | ------------------------------------------------------------------------------------ |
| Mechanism         | What does the agent run?                | [cli-design](../cli-design/README.md) for the tool itself                            |
| Playbook          | What does the agent load for this task? | [03](./03-skills.md), [04](./04-skill-distribution.md), [15](./15-skill-security.md) |
| Durable knowledge | What does the agent read on demand?     | [06](./06-documentation-and-discovery.md)                                            |
| Governance        | What binds every session?               | [01](./01-instruction-files.md)                                                      |

[00 — The four layers](./00-model.md) states the model itself: what each layer carries, what it
never carries, who ships what, and the three boundary mistakes a design makes.

## The chapters

| Chapter                                                                                      | What it holds                                                                                                                                  |
| -------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| [00 — The four layers](./00-model.md)                                                        | The model, the ownership rule, and the boundaries a design gets wrong                                                                          |
| [01 — Instruction files](./01-instruction-files.md)                                          | What the root file carries, why it routes instead of restating, and the generated digest beside it                                             |
| [03 — Skills](./03-skills.md)                                                                | The portable package, the frontmatter contract, the description as the trigger surface, the body, scripts against references, validation loops |
| [04 — Skill distribution](./04-skill-distribution.md)                                        | The two scopes, one source for many runtimes, and what an installer owes a home                                                                |
| [05 — Evaluations](./05-evaluations.md)                                                      | Why an eval is a separate signal from a test, the four suites, the rate and its floor, and the tests an agent writes badly                     |
| [06 — Documentation and discovery](./06-documentation-and-discovery.md)                      | Project artifacts against tool references, the three-step route to a corpus, and version matching by construction                              |
| [08 — Candidate artifacts and receipts](./08-candidate-artifacts-and-receipts.md)            | What a tool would write now, what it wrote last time, the three ownership classes, and why an unattributed file is never overwritten           |
| [09 — Disposable staging and direct landing](./09-disposable-staging-and-direct-landing.md)  | The eight steps, why production never reads the stage, landing by ownership, the failure behavior, and guarded cleanup                         |
| [10 — Versions and migration](./10-operator-selected-versions-and-agent-guided-migration.md) | Who selects a version, the order an agent reads evidence, the four confidence classes, and what the tool owes the agent                        |
| [11 — Router and gates](./11-router-and-gates.md)                                            | One skill that drives the sequence and owns none of it, gates split by judgment, and why a proxy is not a proof                                |
| [12 — Prior art](./12-prior-art.md)                                                          | Dated primary sources for file replacement, path containment, template updates, and recorded state                                             |
| [13 — Reference implementations](./13-reference-implementations.md)                          | Two tools, mapped concept to name, non-normative, with the unimplemented parts marked                                                          |
| [15 — Skill security](./15-skill-security.md)                                                | Fetched content as data, least tools, validation before a shell, the plan before the destruction, and the adversarial suite                    |
| [90 — Worked example](./90-worked-example.md)                                                | One project across every layer: the command tree, the outputs, the skill, the instruction lines, and the eval                                  |

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

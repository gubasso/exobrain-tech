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

| Layer             | The question                            | Where it is stated                                                                                             |
| ----------------- | --------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| Mechanism         | What does the agent run?                | [cli-design](../cli-design/README.md) for the tool, [Tool acquisition](./tool-acquisition.md) for obtaining it |
| Playbook          | What does the agent load for this task? | [Skills](./skills.md), [Skill distribution](./skill-distribution.md), [Skill security](./skill-security.md)    |
| Durable knowledge | What does the agent read on demand?     | [Documentation and discovery](./documentation-and-discovery.md)                                                |
| Governance        | What binds every session?               | [Instruction files](./instruction-files.md)                                                                    |

[The four layers](./model.md) states the model itself: what each layer carries, what it
never carries, who ships what, and the three boundary mistakes a design makes.

## The chapters

Read the chapters in the order of the table below. The model comes first. Then come the layers and
their evaluation, from the instruction file to the documentation an agent reads on demand.
The landing chapters follow, from tool acquisition to the router and gates, with their prior art
and their reference implementations. Then come the skill chapters, from security and format to the
setup skill and migration, with the evidence and the research beneath them. The worked examples
and the checklist come last.

| Chapter                                                                              | What it holds                                                                                                                                  |
| ------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| [The four layers](./model.md)                                                        | The model, the ownership rule, and the boundaries a design gets wrong                                                                          |
| [Instruction files](./instruction-files.md)                                          | What the root file carries, why it routes instead of restating, and the generated digest beside it                                             |
| [Skills](./skills.md)                                                                | The portable package, the frontmatter contract, the description as the trigger surface, the body, scripts against references, validation loops |
| [Skill distribution](./skill-distribution.md)                                        | The two scopes, one source for many runtimes, and what an installer owes a home                                                                |
| [Evaluations](./evaluations.md)                                                      | Why an eval is a separate signal from a test, the four suites, the rate and its floor, and the tests an agent writes badly                     |
| [Documentation and discovery](./documentation-and-discovery.md)                      | Project artifacts against tool references, the three-step route to a corpus, and version matching by construction                              |
| [Tool acquisition](./tool-acquisition.md)                                            | The manager axis against the venue axis, the pin each manager records, the freshness loop, and what a tool owes a project that depends on it   |
| [Candidate artifacts and receipts](./candidate-artifacts-and-receipts.md)            | What a tool would write now, what it wrote last time, the three ownership classes, and why an unattributed file is never overwritten           |
| [Disposable staging and direct landing](./disposable-staging-and-direct-landing.md)  | The eight steps, why production never reads a staged byte, landing by ownership, the failure behavior, and guarded cleanup                     |
| [Versions and migration](./operator-selected-versions-and-agent-guided-migration.md) | Who selects a version, the order an agent reads evidence, the four confidence classes, and what the tool owes the agent                        |
| [Router and gates](./router-and-gates.md)                                            | One skill that drives the sequence and owns none of it, gates split by judgment, and why a proxy is not a proof                                |
| [Prior art](./prior-art.md)                                                          | Dated primary sources for file replacement, path containment, template updates, and recorded state                                             |
| [Reference implementations](./reference-implementations.md)                          | Two tools, mapped concept to name, non-normative, with the unimplemented parts marked                                                          |
| [Skill security](./skill-security.md)                                                | Fetched content as data, least tools, validation before a shell, the plan before the destruction, and the adversarial suite                    |
| [Skill format](./skill-format.md)                                                    | The package, links with their conditions, one file per task, the section order, and the writing conventions                                    |
| [Skill size budget](./skill-size-budget.md)                                          | Why a budget counts characters, the caps on the manual and a reference, and the loader limits behind them                                      |
| [Setup skill skeleton](./setup-skill-skeleton.md)                                    | The scope principle, the pre-flight, provisioning, the gate, the tasks, the post-flight, and the report                                        |
| [What the CLI owes a setup skill](./what-the-cli-owes-a-setup-skill.md)              | Every deterministic step a setup skill hands to its tool: classify, compare, land, replace, environment, lint                                  |
| [Migrating from an archive](./migrating-from-an-archive.md)                          | Archive the predecessor, snapshot what no history holds, land clean, and rebuild one group at a time                                           |
| [Skill format evidence](./skill-format-evidence.md)                                  | What the specification and each vendor say about a skill's format and size, dated, with quotes                                                 |
| [Skill format research](./skill-format-research.md)                                  | Long-context and instruction-count findings, the size measurement, the installers, and the conflicts                                           |
| [Worked example](./worked-example.md)                                                | One project across every layer: the command tree, the outputs, the skill, the instruction lines, and the eval                                  |
| [Worked example: landing](./worked-example-landing.md)                               | One tool that writes into other projects, from greenfield through an update, a collision, a failure part way, a rerun, and cleanup             |
| [Checklist](./checklist.md)                                                          | What to walk before shipping, by layer, each section naming its owner                                                                          |

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
- Let the project pin the tool through the manager it already runs, and ship the loop that moves
  that pin, so no consumer sits at a release nobody moves.

## What this shelf is not

It is not a survey of agent runtimes, and it names no vendor as the way. It is not a place for one
person's configuration, which belongs in the vault. And it is not a home for another product's
documentation: a project that ships a land documents its own skills, and this shelf states why.

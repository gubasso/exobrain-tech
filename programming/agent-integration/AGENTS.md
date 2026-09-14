---
digest-of: programming/agent-integration
last-synced: 2026-09-14
token-estimate: 1200
---

# AGENTS

## Scope

How a project is built to be discovered, operated, and trusted by a coding agent: the instruction
file, the authorization boundary, the skills it ships, where they install, and how the result is
evaluated. Designing the command-line tool an agent runs is `programming/cli-design/`. A fact about
one agent runtime is that vendor's shelf under `tools/`, dated.

## Key points

### The model (00)

- Four layers, each with one class of artifact: mechanism, what the agent runs; playbook, what it
  loads for a task; durable knowledge, what it reads on demand; governance, what binds every
  session.
- Each layer has what it never carries. A tool that encodes a workflow, a skill that copies a flag
  list, and an instruction file that collects every rule are the same mistake in three places.
- A project that owns a land ships the skills that operate it, in its own tree, installed by its own
  command. Shared material several skills read installs once under the user's state directory and is
  named by absolute path, because two runtimes agree on no relative layout.
- A skill's name is its identity, and a name has one owner. One package under two scopes is two
  entries under one name.

### Instruction files (01)

- The root file is hand-authored rules and routing. Judgment has no generator.
- A per-directory digest is the second shape: generated from the subtree, regenerated rather than
  edited, carrying frontmatter and no rule of its own.
- Every sentence is loaded in every session, so routing beats restating and length is the budget
  that keeps the file read.

### Skills (03)

- A skill has one authored owner per kind of content: judgment in `SKILL.md`, durable knowledge in a
  reference, mechanics in a script. A fact stated twice is already wrong in one place.
- A conforming package is a directory with `SKILL.md`; `references/`, `scripts/`, and `assets/` are
  progressive disclosure, not requirements. The package is self-contained and one level deep.
- Two required fields and four optional ones. An unrecognized field is ignored, and that rule is
  what makes a package portable. `allowed-tools` is marked experimental.
- The description is the whole triggering mechanism, because the body loads only after triggering.
  Three parts: what it produces, the phrasings a person types, the boundary against a near sibling.
- Progressive disclosure is a budget, not a style. The deletion test decides every paragraph: remove
  it, and ask whether the agent then makes a likely task error.
- Extract any deterministic chunk past a trivial one-liner, single use included. Four guardrails:
  split judgment from mechanism, keep the one-liner, stay coarse, leave prompt text to the model.
- A validation loop whose checks are the mechanism's own is the highest-value shape a skill carries.

### Distribution (04)

- Two scopes: user, owned by the person; project, committed with the land it drives.
- One canonical directory, linked into each runtime's root, is what keeps several runtimes on one
  copy. Whether a runtime follows the link is a dated vendor fact.
- An installer writes only what it declared, removes only what it wrote, and reports what it did.

### Evaluations (05)

- A test suite says the mechanism works; an eval says the agent uses it correctly. They fail
  differently and a project needs both.
- Four suites: trigger, behavior, portability, adversarial.
- An agent is non-deterministic, so the measurement is a rate over samples, not a verdict on one
  run. Ten samples is the smallest number that distinguishes a rate from an anecdote.
- Grade the outcome, not the route. Pin a tool sequence only where the sequence is the safety
  requirement.
- A verifier decides anything code can decide, from the transcript. A judge model grades the rest
  against a written rubric.
- Compare against a baseline run with the skill absent. A skill that does not beat it costs tokens
  for nothing.
- The dominant failure of an agent-written test suite is testing the library instead of the project;
  the five detection heuristics are stated once, in `cli-design/09`.
- Snapshot the three agent-facing surfaces: help output, machine-output schema, exit codes.

### Skill security (15)

- A skill from outside the project is code review, not a download.
- Fetched and reader-supplied content is data, never an instruction, and the rule is stated where
  the skill reads it rather than once at the top.
- Least tools, validated output before a shell or a path or a parser, the plan shown before a
  destructive step, an explicit target for anything irreversible, and no secret anywhere.
- The description says whether the skill contacts the network.

## Maintenance notes

- `02` is reserved for the authorization boundary and lands from another project's plan. The index
  in `README.md` names only chapters that exist.
- Where a chapter would restate a rule a `cli-design/` chapter owns, it links that owner. Check that
  before adding a rule here.
- A rule that holds for one runtime belongs in that runtime's adapter under `tools/`, dated. `01`
  links the instruction-file adapters, and `03` and `04` link all four skills adapters.

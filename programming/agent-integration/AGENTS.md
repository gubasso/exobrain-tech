---
digest-of: programming/agent-integration
last-synced: 2026-09-14
token-estimate: 900
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

- A conforming package is a directory with `SKILL.md`; `reference/`, `scripts/`, and `assets/` are
  progressive disclosure, not requirements.
- The description is the whole triggering mechanism, because the body loads only after triggering.
  It carries concrete triggers and explicit negative scope, within the specification's limits of 64
  and 1024 characters.
- `allowed-tools` is marked experimental by the specification; every other extra field is a vendor
  extension and belongs in that vendor's adapter.
- Scripts are executed and references are read. A skill that asks an agent to re-derive
  deterministic logic gets a different derivation some of the time.
- A validation loop whose checks are the mechanism's own is the highest-value shape a skill carries.

### Distribution (04)

- Two scopes: user, owned by the person; project, committed with the land it drives.
- One canonical directory, linked into each runtime's root, is what keeps several runtimes on one
  copy. Whether a runtime follows the link is a dated vendor fact.
- An installer writes only what it declared, removes only what it wrote, and reports what it did.

### Evaluations (05)

- A test suite says the mechanism works; an eval says the agent uses it correctly. They fail
  differently and a project needs both.
- An agent is non-deterministic, so the measurement is a rate over samples, not a verdict on one
  run. Ten samples is the smallest number that distinguishes a rate from an anecdote.
- The dominant failure of an agent-written test suite is testing the library instead of the project;
  the five detection heuristics are stated once, in `cli-design/09`.
- Snapshot the three agent-facing surfaces: help output, machine-output schema, exit codes.

## Maintenance notes

- `02` is reserved for the authorization boundary and lands from another project's plan. The index
  in `README.md` names only chapters that exist.
- Where a chapter would restate a rule a `cli-design/` chapter owns, it links that owner. Check that
  before adding a rule here.

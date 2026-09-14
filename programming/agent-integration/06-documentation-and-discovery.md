# 06 — Documentation and Discovery

Durable knowledge is the layer an agent reads on demand: what a project's own documentation owes an
agent, and how a tool the project depends on makes its own knowledge reachable. The governance layer
routes and the playbook layer teaches a task. This layer explains the system, and it is the only one
an agent reads a fraction of.

## Two knowledge sets

A tool that writes into a project carries two bodies of knowledge, and confusing them is what makes
a project's documentation fill up with a dependency's manual.

| Set               | What it is                                                                                   | Who selects it                      |
| ----------------- | -------------------------------------------------------------------------------------------- | ----------------------------------- |
| Project artifacts | The documentation, specifications, configuration, and runbooks chosen to live in the project | The project, by its configuration   |
| Tool references   | The installed version's methods, help, schemas, changelog, guidance, and skill resources     | The tool, by what it was built with |

Project artifacts are the target's own files. They are committed, reviewed, and read by anyone
working in the project, and a tool that produces them owns their generated form and nothing beyond
it.

Tool references explain the tool. They exist wherever the tool is installed, they change when the
installed version changes, and they belong to nobody's repository. An agent reads them to understand
what the tool does and what an upgrade changed.

The rule that follows is short: a tool lands the project artifacts the project selected, and exposes
its references without landing them. A reference corpus copied into a project is a second copy of a
manual that ages the moment the tool moves.

## The three-step route

An agent cannot hold a corpus in context, and a corpus nothing points at might as well not exist.
The route between those two failures has three steps, and each one is a different artifact.

1. One always-loaded line. The instruction file names the entry point and nothing more. This is the
   only part every session pays for.
2. One described index. The index names each topic with a stable identifier, a summary, its
   aliases, and its kind. It is read when a task needs the corpus, and it is what turns a vague
   question into an exact request.
3. One exact reader. Given a topic identifier, one command or one path returns that topic and no
   other.

```text
instruction file  ->  "the corpus is reachable at <entry point>"
index             ->  id, summary, aliases, kind, for every topic
reader            ->  one topic, by id
```

The failure this route prevents is the eager one: an instruction file that embeds the corpus, or
imports it, so every session carries a manual it did not need. Routing beats restating here for the
same reason it does in [01 — Instruction files](./01-instruction-files.md), and the budget is the
same budget.

The second failure is the index nobody can use. An index that lists filenames answers only a
question the reader already knew how to ask. An index that summarizes each topic and names its
aliases answers the question the reader actually has.

## Version matching is structural

The running executable serves the references it was built with. That is the whole rule, and it holds
because of how the references are reached rather than because of a policy anyone enforces.

A mutable trunk, an unrelated local checkout, and a web search are each a different version than the
one installed. Any of them can be right, none of them is guaranteed to be, and nothing in the
answer says which. An agent reading upgrade guidance from a trunk six releases ahead of the binary
it is about to run will do the wrong thing confidently.

Treat an upstream source as an optional last reference, consulted after the installed corpus and
labeled as what it is. It never substitutes for the version-matched text.

## What lands and what does not

A tool projects the project artifacts the target's configuration and capabilities select. It does
not copy its reference corpus into the project because the corpus exists.

The distinction survives a useful middle case. A project may select a document the tool ships, and
that document then lands as a project artifact, chosen by configuration. What does not happen is the
whole corpus arriving as a side effect of the tool being installed.

## Human and machine forms

The two audiences want different things from an index, and one document serves neither well.

A human index optimizes routing: a title a person recognizes, a summary that says whether to read
on, and an order that groups related topics. A machine form declares a versioned schema and stable
identifiers, so a reader can select a topic without parsing prose.

Both describe the same corpus. Neither is an executable plan: an index says what exists and where,
never what to write or in which order. A document that tells a tool what to do has stopped being
documentation and become configuration, and it belongs wherever the project's configuration lives.

An identifier is stable or it is not an identifier. A topic renamed keeps its id, and the old name
becomes an alias, because every reference to the old id is in somebody's transcript already.

## Three layers, three jobs

The layers this shelf names each own one job, and the discovery route touches all three.

- The instruction file routes. It names the entry point, and states no topic's content.
- A skill teaches a task. It points at the topic a step needs, with the condition for reading it,
  the way [03 — Skills](./03-skills.md) states.
- Durable knowledge explains the system. It is read when a step calls for it and at no other time.

Each owns one role and restates none of the others. An instruction file that summarizes a topic has
made a second copy that drifts. A skill that inlines a reference has paid for it on every
invocation.

## See also

- [00 — The four layers](./00-model.md) — why durable knowledge is a layer of its own.
- [01 — Instruction files](./01-instruction-files.md) — the always-loaded line this route starts at.
- [03 — Skills](./03-skills.md) — how a skill points at a topic rather than carrying it.

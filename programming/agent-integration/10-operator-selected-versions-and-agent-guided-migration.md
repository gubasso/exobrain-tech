# 10 — Operator-Selected Versions and Agent-Guided Migration

Upgrading a tool that writes into a project has two halves that want different owners. Choosing and
installing the version is the project's decision, made with the manager it already uses. Working out
what the new version's output means for this particular project is judgment, and it is the part an
agent is for.

## Acquisition belongs to the project

The person and the project's existing tool manager select a version and install it. Staging and
landing do not install, execute, fetch, or decode a different version of the tool.

A landing tool may document its own installation method, and may offer a command that assists it.
What it must not do is reach inside the manager: a landing step that resolves a version, downloads
it, and runs it has become a second package manager, with a second resolver, a second cache, and a
second set of trust decisions the project never authorized.

An update is therefore a distinct, separately authorized event, and it happens before migration
starts. After it, the installed executable reports its own identity and version, stages its own
projection, and serves the knowledge matching that version, per
[06 — Documentation and discovery](./06-documentation-and-discovery.md).

## The order an agent reads evidence

Nine sources, and the order matters because each one is more specific than the next.

1. The working tree. What the project holds right now.
2. The central receipt. What this tool put there, and in what form.
3. Version control history. When a destination arrived, in what shape, and beside what else.
4. The staged candidate, and its omissions and conflicts. What the new version would write.
5. The changelog for the installed version.
6. The migration guidance for the installed version.
7. The installed documentation, read exactly, by topic.
8. The project's own checks. What the project itself says about the result.
9. An upstream source, optionally, last, and labeled as what it is.

The ninth is the one that needs the warning. An upstream trunk is a different version than the one
installed. It can be right. Nothing in it says whether it is. An agent that reads upgrade guidance
from a trunk several releases ahead of the binary it is about to run will act on it confidently and
be wrong, so the trunk is a final reference and never a substitute for the version-matched text.

## Four confidence classes

Evidence varies. What the agent may do does not grow when the evidence shrinks.

| What is available   | What it supports                                                         |
| ------------------- | ------------------------------------------------------------------------ |
| Receipt and history | Attributed destinations, their last written form, and how they got there |
| Receipt alone       | Attribution and form, with no account of what the project did afterwards |
| History alone       | Arrival and change over time, with no statement of what this tool owns   |
| Neither             | The files, and nothing about their origin                                |

Each row down is less certainty, not more permission. Unknown ownership is shown to the person
rather than resolved by inference, and the refusal from
[08 — Candidate artifacts and receipts](./08-candidate-artifacts-and-receipts.md) holds at every
row: an unattributed whole file is not overwritten, however plausible the guess.

A heuristic may inform the agent. It never becomes the answer a person is shown. "This file matches
what the tool would generate" is evidence. "This file is ours" is a claim the receipt makes or
nobody does.

## What the agent does

The mechanical steps belong to the tool. These are the ones that need judgment, in the order the
lifecycle in [09 — Disposable staging and direct landing](./09-disposable-staging-and-direct-landing.md)
runs them.

- Preserve the project's intent. A configuration the project tuned means something, and the reason
  is rarely written down beside it.
- Prepare prerequisites the new version expects, before landing rather than during it.
- Resolve the collisions the stage reported, one at a time, with the evidence attached.
- Decide what happens to a retired destination. The tool released it and deleted nothing; whether
  the file still earns its place is a project question.
- Adapt the project's own documentation where the landing changes what it describes.
- Invoke the production landing. The tool renders fresh, and the agent does not copy the stage.
- Read the real diff, not the predicted one.
- Run the project's checks, repair what they find, and run them again.
- Clean the stage last, after verification has passed.

## What the tool owes the agent

Judgment needs evidence, and evidence the agent cannot reach is evidence nobody has.

- One always-visible route to a described documentation index, per the three-step route in `06`.
- A reader that returns one exact topic by its identifier.
- A machine-readable inventory of the stage: what it contains, what it omitted, what it could not
  claim.
- Help for the landing command and for the cleanup command, complete enough to act on.
- A router skill that links to these rules rather than restating them.

## Four things this pattern refuses

- Automatic cross-release reconstruction. The tool does not fetch an older version of itself to
  manufacture a record it does not have.
- Acquisition inside landing. The manager the project already uses owns that, and a landing step
  that duplicates it duplicates its failure modes too.
- Blind documentation copying. A reference corpus is exposed, not landed, and what lands is what the
  project selected.
- A heuristic presented as certainty. An inference is labeled as one wherever a person reads it.

## See also

- [06 — Documentation and discovery](./06-documentation-and-discovery.md) — the version-matched
  knowledge an update makes available.
- [08 — Candidate artifacts and receipts](./08-candidate-artifacts-and-receipts.md) — the record
  that decides which confidence class a destination falls into.
- [09 — Disposable staging and direct landing](./09-disposable-staging-and-direct-landing.md) — the
  lifecycle these judgments run inside.

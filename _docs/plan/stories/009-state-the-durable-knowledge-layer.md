# 009 — State the Durable-Knowledge Layer

## Goal

An agent can learn an installed tool precisely, at the version it is about to run, without any part
of that tool's manual being copied into the project.

## Example

The shelf names four layers and states three of them. The fourth has a row in a table and no
chapter.

```console
$ ls programming/agent-integration/0*.md
00-model.md  01-instruction-files.md  03-skills.md  04-skill-distribution.md  05-evaluations.md
```

Nothing states what a project's documentation owes an agent, how a corpus becomes reachable without
being loaded, or why the version of a document matters as much as its content.

## Core

Two knowledge sets, kept apart. Project artifacts are what the project's configuration selects into
its own tree. Tool references are what the installed version carries, and a tool exposes them rather
than landing them. Between an agent and either set runs one route: an always-loaded line, a
described index, and a reader that returns one exact topic.

## In scope

- The definitions of both knowledge sets, and the rule about what lands.
- The three-step route, and the eager failure it prevents.
- Version matching as a structural property of how references are reached.
- The split between a human index and a machine form, and the stability of an identifier.

## Out of scope

- Candidate artifacts, receipts, staging, and migration, which later chapters own.
- Any product, command, schema, or vendor name.

## Governed by

- `_docs/specs/SPEC-knowledge-base-boundary.md` — what belongs in a bucket and what does not.
- `_docs/specs/SPEC-docs-format.md` — the register and the size budgets.
- `_docs/specs/SPEC-project-platform.md` — the rule that a portable chapter names a property rather
  than a platform.

## Amends

- `programming/agent-integration/06-documentation-and-discovery.md` — created.
- `programming/agent-integration/00-model.md`, `01-instruction-files.md`, `README.md`, `AGENTS.md` —
  the routes to it.

## Acceptance

- The chapter is self-contained, inside its budget, and names no product or vendor.
- Project artifacts and tool references cannot be confused in a definition or an example.
- Every discovery example runs instruction, index, exact topic, over version-matched content.
- `just check` passes.

## Tasks

- [ ] Define the two knowledge sets and what lands from each.
- [ ] State the three-step route and the failure at each end of it.
- [ ] State version matching as structural, not as a policy.

## Rabbit holes

- Describing a real index format — escape: the chapter states what an index must carry, and an
  implementation chooses the shape.
- Letting the index become a plan — escape: an index says what exists, never what to write.

## Done when

The durable-knowledge layer has a chapter, and the three other layers link to it for the job it owns.

## Revisions

None.

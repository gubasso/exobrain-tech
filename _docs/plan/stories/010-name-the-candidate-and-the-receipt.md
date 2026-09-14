# 010 — Name the Candidate and the Receipt

## Goal

A tool that writes files into a project can say what the project should hold now and what it put
there last time, without either answer requiring it to understand a different release.

## Example

The shelf explains how an agent reaches knowledge and says nothing about how a tool reaches a file.

```console
$ git grep -l "receipt\|ownership class" programming/agent-integration/
programming/agent-integration/04-skill-distribution.md
```

That chapter states what an installer owes a home in three sentences. It names no record, no
ownership class, and no behavior for a file whose author the tool cannot identify.

## Core

Two artifacts, kept apart. A candidate is desired state, computed from the installed executable's
own sources, the target's resolved configuration, and the target's observed capabilities. A receipt
is attributed installed state, held centrally, carrying a digest that distinguishes a file the tool
wrote from a file the tool wrote and somebody edited.

## In scope

- What a candidate is, and the three things it is not.
- The receipt's fields, and the bounded schema-reading concern that is not a cross-release protocol.
- The three ownership classes, stated as meanings rather than labels.
- The refusal on an unattributed collision, and the report on a retirement.
- Provenance in the record rather than in the content.

## Out of scope

- Staging, direct landing, verification, and cleanup, which the next chapter owns.
- Version selection and migration judgment, which the chapter after that owns.
- Any product, command, schema name, or vendor.

## Governed by

- `_docs/specs/SPEC-knowledge-base-boundary.md` — what belongs in a bucket and what does not.
- `_docs/specs/SPEC-docs-format.md` — the register and the size budgets.
- `_docs/specs/SPEC-project-platform.md` — a portable chapter names a property, not a platform.

## Amends

- `programming/agent-integration/08-candidate-artifacts-and-receipts.md` — created.
- `programming/agent-integration/README.md` and `AGENTS.md` — the route to it.

## Acceptance

- The chapter is inside its budget and every term has one definition.
- No sentence implies that one executable decodes another release's content.
- No product command, source path, registry, language, or vendor name appears.
- The hub and digest summarize without copying the rules.
- `just check` passes.

## Tasks

- [ ] Define the candidate by its three inputs and its three exclusions.
- [ ] Define the receipt's fields and the role of the digest.
- [ ] State the ownership classes and the behavior for each state a tool meets.

## Rabbit holes

- Letting the receipt grow into a payload format — escape: it records what was written, never how to
  write it, and reading an old schema is not reading old content.
- Resolving an unattributed collision by heuristic — escape: the refusal is the rule, and the
  evidence goes to whoever can judge it.

## Done when

The shelf owns candidate state and attributed installed state, and the two cannot be confused.

## Revisions

None.

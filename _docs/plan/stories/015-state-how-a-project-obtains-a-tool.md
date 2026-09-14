# 015 — State How a Project Obtains a Tool

## Goal

A tool author can build a self-pin matrix, and a consumer can say what a tool owes a project that
takes it as a development dependency, from one chapter and no source tree.

## Example

The shelf explains what a tool lands into a project and says nothing about how the project gets the
tool.

```console
$ git grep -l "tool manager\|pin" programming/agent-integration/
programming/agent-integration/06-documentation-and-discovery.md
```

That chapter states that the running executable serves the references it was built with. It never
says which file records the version, what moves it, or what happens when nothing does.

## Core

Two axes, crossed once. A manager is what the project declares its development tools in and records
a version for. A venue is where a release is published. Every pair renders or is manual with a
reason from a closed set, and a pin a person moves by hand goes stale, so the tool moves its own
under four properties a consumer can check.

## In scope

- The manager axis, the venue axis, and why a shell loader is neither.
- The matrix, its three verdicts, and the closed reason set behind a manual pair.
- The form each manager records, and the one-mechanism rule.
- The freshness loop: one transaction, one rate limit, two registers, no unreviewed write.
- The obligation a tool distributed as a development dependency carries.
- Serving a fragment against editing an owned manifest.

## Out of scope

- Publishing, tagging, and cutting a release.
- Resolving a library dependency at build time.
- What a tool writes into a project's directories, which `cli-design/11` owns.
- Any product, binary, repository, or vendor of this author's.

## Governed by

- `_docs/specs/SPEC-knowledge-base-boundary.md` — what belongs in a bucket and what does not.
- `_docs/specs/SPEC-docs-format.md` — the register and the size budgets.
- `_docs/specs/SPEC-project-platform.md` — a portable chapter names a property, not a platform.

## Amends

- `programming/agent-integration/07-tool-acquisition.md` — created.
- `programming/agent-integration/README.md` and `AGENTS.md` — the route to it.

## Acceptance

- The chapter is inside its budget and every term has one definition.
- Every manager and venue citation carries the date it was read.
- No product command, source path, crate, repository, or vendor of this author's appears.
- The hub and digest summarize without copying the rules.
- `just check` passes.

## Tasks

- [ ] State the two axes and the matrix, with a closed reason set.
- [ ] State the pin forms and the one-mechanism rule.
- [ ] State the freshness loop as four checkable properties.
- [ ] State the obligation and what discharges it.

## Rabbit holes

- Letting one tool's matrix read as the shape of the world — escape: the table is an example and the
  discipline is the payload.
- Growing the chapter into a second one for the dependency half — escape: the difference is one
  paragraph, because that half adds no mover.

## Done when

A reader who has opened no repository of this author's can build the matrix, knows why a manual pair
is manual, and knows whether a tool meets the obligation.

## Revisions

None.

# The planning method and its tooling move to plan-xp

## Context and Problem Statement

The project-management shelf grew a method, two schemas, a cross-file linter, a test harness, and a
worked example. All of it is language-agnostic and none of it is about this knowledge base: it is a
product other projects would adopt, sitting inside a library that exists to serve knowledge.

It also carried a hidden requirement. An outbound reference resolved against a project root guessed
two directories above the zone, and the documents it named were fixed at reference pages organised by
capability — free inside one repository, and a demand that adopters take on an entire documentation
methodology anywhere else.

## Considered Options

- Keep the shelf and the artifacts here, and accept that the product is embedded in a library.
- Reserve a tooling root in this repository and move the artifacts into it.
- Split the method and its tooling into their own public project.

## Decision Outcome

Chosen option: split. The method, the schemas, the linter, the harness, and the worked example become
[plan-xp](https://github.com/gubasso/plan-xp), rewritten so a host project is required only to have a
documentation directory.

This repository becomes an ordinary consumer: `plan-xp` is a `flake.nix` input, `_docs/plan/` is the
record, and `just test` gates it with the linter from that input.

A tooling root here was proposed and rejected. It solved placement and not audience: the artifacts
would still have been reachable only by copying pieces out of this checkout.

This supersedes ADR-separate-documentation-design-from-project-management, ADR-plan-record-is-lane-files, ADR-the-unit-is-a-story-in-one-file, ADR-a-story-is-a-diff-a-spec-is-a-state, ADR-the-plan-linter-may-write-the-record, ADR-an-epic-is-a-field-and-a-document, ADR-closed-is-an-append-only-log, and ADR-plan-parameters-live-in-a-config-file.
One record for eight is correct rather than lazy: the eight are superseded by a single decision —
that this repository no longer owns those choices — and eight near-identical records would be the
clutter. Each choice survives, restated as plan-xp's own record.

## Consequences

- Good: the method is adoptable without this repository, and the library holds only knowledge.
- Bad: a reader following the shelf finds it gone, and this repository now depends on a tool it also
  maintains.

## Status

Accepted

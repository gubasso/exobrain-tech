# 012 — Separate Acquisition from Migration

## Goal

Upgrading a tool that writes into a project splits cleanly: the project chooses and installs the
version with the manager it already uses, and an agent works out what the new output means for this
particular project.

## Example

The shelf states a lifecycle that assumes a version is already installed and never says who
installed it.

```console
$ git grep -c "acquisition\|install the version" programming/agent-integration/
0
```

Nothing stops a landing step from resolving a version, downloading it, and running it, which is a
second package manager with a second resolver and a second set of trust decisions.

## Core

Acquisition belongs to the project. Landing does not install, execute, fetch, or decode another
version. What is left is judgment, and the chapter states the evidence an agent reads, in order, and
the four confidence classes that evidence produces. Each class down is less certainty rather than
more permission.

## In scope

- The acquisition boundary, and the update as a separate authorized event.
- The nine evidence sources in order, with the upstream source last and labeled.
- The four confidence classes, and the refusal that holds across all of them.
- The semantic work that belongs to the agent.
- What the tool owes the agent so the judgment has evidence.
- The four things the pattern refuses.

## Out of scope

- The router skill and the gate boundary, which the next chapter owns.
- Any product, command, manager recipe, or vendor.

## Governed by

- `_docs/specs/SPEC-knowledge-base-boundary.md` — what belongs in a bucket and what does not.
- `_docs/specs/SPEC-docs-format.md` — the register and the size budgets.
- `_docs/specs/SPEC-project-platform.md` — a portable chapter names a property, not a platform.

## Amends

- `programming/agent-integration/10-operator-selected-versions-and-agent-guided-migration.md` —
  created.
- `programming/agent-integration/08-candidate-artifacts-and-receipts.md` and
  `09-disposable-staging-and-direct-landing.md` — one link each.
- `programming/agent-integration/README.md` and `AGENTS.md` — the route, and a digest compressed to
  stay inside its budget.

## Acceptance

- The chapter names roles rather than product commands or manager recipes.
- Each confidence class states the evidence it has and what it still cannot authorize.
- Project artifacts and tool references stay distinct wherever discovery appears.
- Every example cleans the stage after production and verification.
- `just check` passes.

## Tasks

- [ ] State the acquisition boundary and the separately authorized update.
- [ ] State the evidence order and the four confidence classes.
- [ ] State the agent's semantic work and what the tool owes it.

## Rabbit holes

- Letting less evidence become more freedom — escape: the unattributed refusal holds at every row.
- Reading an upstream trunk as the guidance for an installed version — escape: it is a different
  version, and nothing in it says whether it applies.

## Done when

Acquisition and migration have different owners, and the chapter says what each one may decide.

## Revisions

None.

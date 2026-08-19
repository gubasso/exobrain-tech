# Spec-Driven Docs

A documentation framework for projects worked on by coding agents. One artifact states what is true
now and is loaded before work starts. A second states why, is never revised, and is read only when
someone asks. The separation is what keeps a growing project legible to both an agent and a person.

## Problem

A project that records its decisions and nothing else ends up with hundreds of documents, each true
when written, none of them a statement of what binds today. An agent loading that corpus finds
contradictions and resolves them arbitrarily. A person loading it cannot tell which records still
apply. Both failures grow with the project rather than with any single document.

## Model

```text
                 asks                  changes            loaded
  SPEC-<domain>.md  what is true now   edited in place    always
  ADR-<slug>.md  why, at the time      never              on demand only
  code           exact behavior        continuously       as the work needs

  precedence: code > spec > ADR
```

## Chapters

| #        | Chapter                                | One-line hook                                                      |
| -------- | -------------------------------------- | ------------------------------------------------------------------ |
| 0        | [Model](./00-model.md)                 | Which artifact owns a fact, and what wins when two disagree.       |
| 1        | [Placement](./01-placement.md)         | The docs root, the zones, and why specs are centralized.           |
| 2        | [Specs](./02-specs.md)                 | The current contract for one domain, in a fixed shape.             |
| 3        | [Rules](./03-rules.md)                 | EARS grammar, citable identifiers, and a command that proves it.   |
| 4        | [Decisions](./04-decisions.md)         | The append-only log: slug names, never revised, never loaded.      |
| 5        | [Agent Context](./05-agent-context.md) | What loads, what does not, and how large the always-loaded may be. |
| 6        | [Format](./06-format.md)               | Structural markdown, an imperative register, and the budgets.      |
| 7        | [Lifecycle](./07-lifecycle.md)         | Changing a spec, promoting a draft, retiring a workaround.         |
| 8        | [Gates](./08-gates.md)                 | The hook for every rule, and the honest list of unenforced ones.   |
| 9        | [Spec to Code](./09-spec-to-code.md)   | A spec may lead its code; enactment and comments cite it by ID.    |
| 10       | [Procedures](./10-procedures.md)       | The shape of a step, and what crosses a phase boundary.            |
| 11       | [Operational](./11-operational.md)     | What a runbook, a diagnostic, and a case study each contain.       |
| 99       | [Checklist](./99-checklist.md)         | What must pass before a documentation change merges.               |
|          | [Glossary](./glossary.md)              | Resolve a term at its owning chapter.                              |
| Template | [Spec](./TEMPLATE-spec.md)             | Start a domain's contract.                                         |
| Template | [Decision record](./TEMPLATE-adr.md)   | Start a log entry.                                                 |
| Template | [Digest](./TEMPLATE-agents-digest.md)  | Start an author-instructions router.                               |
| Template | [Docs rules](./TEMPLATE-docs-rules.md) | Wire the framework into author instructions.                       |

## Adopting

1. Choose the docs root: `docs/` for a codebase, `_docs/` for a content tree.
2. Name one domain that already has rules people apply, and write `<root>/specs/SPEC-<domain>.md`
   from [TEMPLATE-spec.md](./TEMPLATE-spec.md).
3. Give every requirement a rule ID and a verification command.
4. Wire the heading shapes and the identifier checks from [08 — Gates](./08-gates.md).
5. Land a gate for every budget in [06 — Format](./06-format.md) that is stated as a count. A
   project that publishes these numbers and enforces none of them has documented an intention.
6. Paste [TEMPLATE-docs-rules.md](./TEMPLATE-docs-rules.md) into the root author-instructions file.
7. Add domains as they acquire rules. A domain with no verifiable rule does not get a spec.

Start with one spec. A framework adopted all at once produces empty specs, and an empty spec costs
context on every session while teaching nothing on any of them.

## Worked example

[worked-example/](./worked-example/) is this shelf applied to itself: five specs whose rules are the
chapters above, two decision records, the two `MD043` heading arrays, and the pre-commit block that
gates all of it. It runs — `just test-spec-shelf` copies it to a scratch tree and puts its own hooks
over its own files — so the gate snippets are proven rather than illustrated.

## Related

One document genre has its own shelf, because its rules are about evidence rather than about
documentation: a page comparing this project to its alternatives is specified by
[comparison-docs](../comparison-docs/README.md).

## House format

This shelf obeys [06 — Format](./06-format.md). Chapters stay at or below 200 lines and catalogs at
or below 300, both enforced by a hook rather than by intent, none carries bold or italics, and prose
is spent only on a decision, a hazard, or a non-obvious constraint.

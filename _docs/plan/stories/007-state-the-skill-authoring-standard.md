# 007 — State the Skill Authoring Standard

## Goal

Someone who has opened no vendor's documentation and no other repository can author and evaluate an
Agent Skill from this shelf alone, and can find each runtime's differences in the adapter the
chapter links.

## Example

The shelf states the package shape and the two fields that trigger it, and stops there.

```console
$ wc -l programming/agent-integration/03-skills.md programming/agent-integration/05-evaluations.md
126 programming/agent-integration/03-skills.md
 79 programming/agent-integration/05-evaluations.md
```

What the specification requires of a name, what the optional fields carry, why an unrecognized field
is ignored, which paragraph earns its place in a body, when a routine becomes a script, and how a
rate is measured are each stated elsewhere or nowhere.

## Core

The two chapters carry the standard whole. The specification defines the conforming package and the
field contract. Each runtime's addition is a row in that runtime's adapter. A rule that holds for
any runtime is the library's own, attributed to nobody, and stated once.

## In scope

- The frontmatter contract, the portability rule behind it, and the description as the whole trigger
  surface.
- Progressive disclosure as a budget, with the deletion test that decides a paragraph.
- Script extraction, its four guardrails, and what a script owes its caller.
- The security rules, in their own chapter because the first will not hold them.
- The four evaluation suites, the fixture shapes, the sample floor, and the baseline.

## Out of scope

- Any file under another product's source tree.
- Any chapter another repository's plan lands here.
- A vendor fact, which belongs in that vendor's adapter.

## Governed by

- `_docs/specs/SPEC-knowledge-base-boundary.md` — what belongs in a bucket and what does not.
- `_docs/specs/SPEC-docs-format.md` — the register and the size budgets.
- `AGENTS.md` — the product boundary, and the rule that each fact has one source of truth.

## Amends

- `programming/agent-integration/03-skills.md` — the standard.
- `programming/agent-integration/05-evaluations.md` — the evaluation rules.
- `programming/agent-integration/15-skill-security.md` — created.
- `programming/agent-integration/README.md` and `AGENTS.md` — the index and the digest.

## Acceptance

- Every chapter stays inside its budget.
- Every rule is attributed to the specification, to a named vendor source, or carried as the
  library's own.
- No rule is stated in two files.
- `just check` passes, and `lychee` resolves the specification and every vendor URL.

## Tasks

- [ ] Write the field contract and the portability rule from the specification.
- [ ] Widen the skills chapter with the placement, extraction, and body rules.
- [ ] Split the security rules into their own chapter.
- [ ] Widen the evaluations chapter with the four suites and the measurement.

## Rabbit holes

- Carrying another repository's house rule as canon — escape: a rule stated as universal is here, a
  rule stated as a house rule stays where its house is.
- Writing the baseline as the intersection of what runtimes do — escape: the baseline is what the
  specification defines, and an intersection shrinks every time a runtime drops a field.

## Done when

The shelf states the standard whole, and the four adapters carry every difference from it.

## Revisions

None.

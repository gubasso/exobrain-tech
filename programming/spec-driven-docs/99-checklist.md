# 99 — Checklist

The pre-merge gate for a documentation change. Every box is answerable yes or no from the diff or from
a command. This file is derived: it restates rules in test form and owns none of them. If a box and
its owning chapter disagree, the chapter wins.

## Model

Owner: [00 — Model](./00-model.md).

- [ ] Every durable fact touched by the change has exactly one owner, and the reviewer can name it.
- [ ] No document restates a fact another document owns; non-owners link instead.
- [ ] A behavior contract is carried by a name, a type, or a test where one can carry it.
- [ ] No passage indexes the filesystem for its own sake; a listing that stays earns it by what its
      entries teach.
- [ ] Where the change touched both a spec and a decision record, the spec states the present and the
      record was not edited to match it.

## Placement

Owner: [01 — Placement](./01-placement.md).

- [ ] The docs root matches the product: `docs/` for a codebase, `_docs/` for a content tree, and no
      product content sits under it.
- [ ] Each document has one primary reader need and lives in the matching zone.
- [ ] Every spec is at `<root>/specs/SPEC-<domain>.md` and none is co-located with what it governs.
- [ ] Every file whose kind this framework fixes carries its uppercase prefix: `SPEC-`, `ADR-`, or
      `TEMPLATE-`. Guides, reference, and explanation pages carry none.
- [ ] No exploratory material entered the docs root.

## Specs

Owner: [02 — Specs](./02-specs.md).

- [ ] The spec uses `## Purpose` then `## Requirements`, and introduces no section outside the shape.
- [ ] Requirements are ordered with the most consequential first.
- [ ] No narrative sits between two requirements.
- [ ] No spec links or names a decision record, and no entry document does either; the reference
      runs from the record to the rule ID.
- [ ] A behavior change in this diff is reflected in the spec.
- [ ] A retired requirement was deleted, not marked obsolete.
- [ ] A supporting artifact sits in `<root>/specs/SPEC-<domain>/` only when it has no reader who
      arrives without the spec; anything with an independent reader is in reference.
- [ ] The spec is within 300 authored lines, and a spec over 100 lines carries a generated TOC.

## Rules

Owner: [03 — Rules](./03-rules.md).

- [ ] Every requirement is one `###`<id>`— <title>` heading with a statement, a scenario, and a
      `Verify:` line.
- [ ] Every statement is one sentence in one of the five patterns, with an RFC 2119 keyword.
- [ ] Every statement names a subject that can act.
- [ ] Every rule ID is `<spec-slug>:<rule-slug>` and unique across the project.
- [ ] No rule ID changed because its sentence was reworded.
- [ ] Prohibitions are at or below five per spec, and each is paired with the action replacing it.
- [ ] Every scenario names the contested case rather than restating the rule.

## Decisions

Owner: [04 — Decisions](./04-decisions.md).

- [ ] The choice cleared the threshold: cross-cutting, expensive to reverse, constraining, or
      rejecting a plausible alternative.
- [ ] The filename is `ADR-<imperative-slug>.md` and carries no digit.
- [ ] No merged record was renamed or deleted.
- [ ] No record was edited to describe a later design.
- [ ] The body is at or below 350 words, uses the five sections, and carries exactly one `Status`.
- [ ] Every considered option carries a disposition; rejections state why, deferrals name a reopening
      condition.
- [ ] An enforceable consequence of this record is stated as a requirement in a spec, and the record
      cites the rule ID.

## Agent context

Owner: [05 — Agent Context](./05-agent-context.md).

- [ ] The always-loaded files are within budget: 100 lines at the root, 150 in a subtree.
- [ ] Subtree-local rules live in the subtree, and the root file points rather than imports.
- [ ] Every source an entry document needs is linked directly from it, not through another document.
- [ ] Filenames indicate their contents.

## Format

Owner: [06 — Format](./06-format.md).

- [ ] No bold or italic text anywhere in the change.
- [ ] Every fenced block declares a language.
- [ ] RFC 2119 keywords appear only in normative statements.
- [ ] Prose is spent only on a decision, a hazard, or a non-obvious constraint.
- [ ] No document narrates its own history or explains an absence.
- [ ] The change recommends one default rather than surveying alternatives it does not recommend.
- [ ] One term is used for one concept.

## Lifecycle

Owner: [07 — Lifecycle](./07-lifecycle.md).

- [ ] A behavior change in this diff updated the owning spec in the same change.
- [ ] The change states what was added, modified, or removed, and does not restate what it left alone.
- [ ] No exploratory material entered the docs root, and any promoted draft was deleted.
- [ ] A fact depending on an external source has an entry in the tracking registry.
- [ ] A workaround added here names the condition that retires it, and a workaround whose condition is
      met was removed along with its record.

## Spec to code

Owner: [09 — Spec to Code](./09-spec-to-code.md).

- [ ] A spec change in this diff is cited in the enacting entry document as a typed clause:
      `ADDED`, `MODIFIED`, or `REMOVED`, then the rule ID in inline code.
- [ ] No requirement carries a stored status marker, and no stored coverage artifact was added.
- [ ] The cited type matches the diff: a new requirement is `ADDED`, a reworded one `MODIFIED`, a
      deleted one `REMOVED`.

## Gates

Owner: [08 — Gates](./08-gates.md).

- [ ] A rule added by this change is checked by a hook, or listed as unenforced.
- [ ] The hooks pass on the changed files.

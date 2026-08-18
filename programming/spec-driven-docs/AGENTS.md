---
digest-of: programming/spec-driven-docs
last-synced: 2026-08-18
token-estimate: 700
---

# AGENTS

## Scope

A spec-driven documentation framework for projects worked on by coding agents. Covers the artifact
model, placement, the spec and its requirement blocks, the decision log, agent context budgets, the
markdown register, lifecycle, the spec-to-code seam, and the gates that hold all of it.

The planning method itself — how stories are written, ranked, and moved — belongs to the planning
tool; this shelf owns only the contract its work record satisfies. Guide shape, procedure artifacts,
comment discipline, and the shapes of runbooks, diagnostics, and case studies are held by
`../docs-design/`.

## How to use this shelf

Load this file, find the owning chapter below, then read that chapter and the sources it names. Do not
read the shelf linearly; [05 — Agent Context](./05-agent-context.md) owns focused loading.

## Where the rules live

| Question the agent arrives with                                | Owning chapter        |
| -------------------------------------------------------------- | --------------------- |
| Which artifact owns this fact, and what wins on conflict?      | `00-model.md`         |
| Where does this document go?                                   | `01-placement.md`     |
| What shape is a spec, and when does a domain get one?          | `02-specs.md`         |
| How is a rule written, identified, and verified?               | `03-rules.md`         |
| When does a choice earn a record, and what may I change?       | `04-decisions.md`     |
| What loads at the start of a session, and how large may it be? | `05-agent-context.md` |
| Which markdown may I use, and how long may this be?            | `06-format.md`        |
| How does a spec change, and what happens to a draft?           | `07-lifecycle.md`     |
| How does a spec written first become work, and who tracks it?  | `09-spec-to-code.md`  |
| What hook enforces this rule?                                  | `08-gates.md`         |
| What must pass before this change merges?                      | `99-checklist.md`     |
| What does this shelf mean by a term?                           | `glossary.md`         |

Templates: `TEMPLATE-spec.md`, `TEMPLATE-adr.md`, `TEMPLATE-agents-digest.md`,
`TEMPLATE-docs-rules.md`.

## Non-negotiables

- A spec states what is true now and is loaded; a decision record states why and is not.
- Precedence is code, then spec, then decision record.
- A change that alters current behavior updates the owning spec in the same change.
- A merged decision record is never renamed, never deleted, and never edited to describe the present.
- Every requirement carries a rule ID, one EARS sentence with an RFC 2119 keyword, a scenario, and a
  verification command.
- Cite a rule by its ID so the citation can be checked with a grep.
- Specs live at `<root>/specs/SPEC-<domain>.md`, never beside what they govern.
- A file whose kind is fixed carries it as an uppercase prefix: `SPEC-`, `ADR-`, `TEMPLATE-`.
- Use no bold or italics, and give every fenced block a language.
- A rule no command can check is listed as unenforced, not presented as gated.
- A requirement whose `Verify:` command fails is unimplemented, not broken; its state is derived by
  running the command, never stored.

## Maintenance

- Regenerate when this shelf's chapters change.
- This digest is a router, never a rules home; the owning chapter wins on disagreement.

---
digest-of: programming/docs-design
last-synced: 2026-08-11
token-estimate: 650
---

# AGENTS

## Scope

Language-agnostic documentation design canon: ownership and placement of durable facts, reader-need
zones, decision records, subsystem pages, agent context, drafts, reference maintenance, markdown
mechanics, procedure artifacts, and the documentation review gate.

Planning scope, slices, and the plan record route to [project management](../project-management/AGENTS.md).

## How to use this shelf

Load this file, find the owning chapter below, then read that chapter and the sources it names. Do not
read the shelf linearly; [04 — Agent Context](./04-agent-context.md) owns focused loading.

## Rule ownership

| Question the agent arrives with                                    | Owning file                       |
| ------------------------------------------------------------------ | --------------------------------- |
| Which artifact owns a fact, and may a directory be enumerated?     | `00-foundations.md`               |
| Where does a document or operational artifact go?                  | `01-diataxis-zones.md`            |
| Is this the product, or is it about the product?                   | `01-diataxis-zones.md`            |
| Does this choice deserve an ADR, and what lifecycle applies?       | `02-lean-adrs.md`                 |
| Where is the current design of a subsystem written?                | `03-subsystem-pages.md`           |
| What should a session load, and how large may an entry file be?    | `04-agent-context.md`             |
| Where do scratch notes live, and how do they ship?                 | `05-drafts-and-promotion.md`      |
| How is a perishable fact tracked and revalidated?                  | `06-tracking-and-revalidation.md` |
| How is an external-system bug recorded and retired?                | `07-known-issues.md`              |
| Which markdown constructs carry structure, and how is shape gated? | `08-lean-markdown.md`             |
| How does a guide name values crossing phase boundaries?            | `09-procedure-artifacts.md`       |
| What must pass before a documentation change merges?               | `99-checklist.md`                 |
| What does this shelf mean by a documentation term?                 | `glossary.md`                     |

## Non-negotiables

- One owner per durable fact; every other mention links to it.
- Use `docs/` for documentation about a codebase and `_docs/` for documentation about a knowledge
  base; product content does not live under that root.
- Never delete an accepted decision; supersede, deprecate, reject, or amend it.
- Drafts stay out of the docs root; promotion rewrites them for the owning reader need.
- Use no bold or italics, and give every fenced block a language.
- A fixed shape has one `MD043` array applied by one hook; free-form documents are not gated.
- A multi-phase guide names what each phase consumes and produces; artifact tokens carry no real value.
- The filesystem owns its state; a tree, list, or table stays only when its entries teach something.

## Maintenance

- Regenerate when this shelf's knowledge changes.
- This digest is a map, never a rules home; the owning chapter wins on disagreement.

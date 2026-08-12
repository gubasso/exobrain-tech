# Documentation Design

Language-agnostic principles for organizing durable project documentation: clear ownership, reader-need
zones, lean decisions, living subsystem descriptions, operational reference, agent context, and
validated markdown and procedure shapes.

## Problem

Documentation decays when a fact has several homes, drafts masquerade as project state, or one page
mixes tasks, lookup, explanation, and decisions. This shelf gives durable material one owner and one
reader promise so humans and agents can find, verify, and update it without reconstructing the truth
from code, chat history, or old issues.

## Chapters

| #        | Chapter                                                        | One-line hook                                                       |
| -------- | -------------------------------------------------------------- | ------------------------------------------------------------------- |
| 0        | [Foundations](./00-foundations.md)                             | Which artifact owns a fact, and what wins when two disagree.        |
| 1        | [Diataxis Zones](./01-diataxis-zones.md)                       | The docs root, reader-need homes, and operational placement.        |
| 2        | [Lean ADRs](./02-lean-adrs.md)                                 | MADR-minimal records, a 350-word cap, and a never-delete lifecycle. |
| 3        | [Subsystem Pages](./03-subsystem-pages.md)                     | The living design page: records freeze, descriptions stay live.     |
| 4        | [Agent Context](./04-agent-context.md)                         | Context pollution, semantic names, and focused entry documents.     |
| 5        | [Drafts and Promotion](./05-drafts-and-promotion.md)           | Keep drafts out of shipped docs and promote by reader need.         |
| 6        | [Tracking and Revalidation](./06-tracking-and-revalidation.md) | Track perishable facts and revalidate their sources on a cadence.   |
| 7        | [Known Issues](./07-known-issues.md)                           | Track bugs in external systems under test; expand then collapse.    |
| 8        | [Lean Markdown](./08-lean-markdown.md)                         | Spend markdown on structure, and gate contractual document shapes.  |
| 9        | [Procedure Artifacts](./09-procedure-artifacts.md)             | Name what each phase of a guide produces and what it consumes.      |
| 99       | [Checklist](./99-checklist.md)                                 | Review durable documentation before merging it.                     |
|          | [Glossary](./glossary.md)                                      | Resolve documentation terms at their owning chapters.               |
| Template | [ADR](./template-adr.md)                                       | Start a decision record with the shelf's fixed shape.               |
| Template | [Docs rules](./template-docs-rules.md)                         | Integrate documentation maintenance into author instructions.       |
| Template | [Heading shapes](./template-heading-shapes.md)                 | Gate the ADR heading contract with a live hook.                     |

Forward-looking work — stories, epics, and the lane record that ranks them — is a method with its
own tooling and is outside this shelf. It lives at
[plan-xp](https://github.com/gubasso/plan-xp), which this repository consumes as a pinned
`flake.nix` input.

## Reading

Read ownership before placement, then compare frozen ADRs with living subsystem pages. Agent context
and drafts explain how material is loaded and promoted; tracking, known issues, markdown mechanics,
procedure artifacts, and the review checklist apply those foundations to durable documentation.

An agent should load [AGENTS.md](./AGENTS.md), choose the owning chapter from its routing table, and
read that chapter plus the sources it names.

## Adopting this shelf

Take the docs layout from [01 — Diataxis Zones](./01-diataxis-zones.md), copy
[template-adr.md](./template-adr.md), integrate [template-docs-rules.md](./template-docs-rules.md), and
use [99 — Checklist](./99-checklist.md) at review time. Create only zones with real content, migrate
existing material by ownership, and keep durable facts at one source of truth.

## House format

The markdown rules are [08 — Lean Markdown](./08-lean-markdown.md), which this shelf obeys. What is
specific to the shelf's own layout:

- Frontmatter: only `AGENTS.md` carries it.
- One `#` per file. `##` for sections, `###` only for genuinely parallel sub-parts. Never `####`.
- An unheaded opening states what the file owns, followed by normative sections, anti-patterns where
  useful, and sources where the chapter carries three or more.
- No `## See also`; this table and inline links provide navigation.
- Paragraphs carry one point, normally three sentences or fewer.
- Cite inline at the claim when one claim uses a source.
- In-shelf links take the form `[NN — Title](./NN-slug.md)`.
- `<angle>` placeholders stand for project-specific values.
- Hand-wrap at 100 columns.

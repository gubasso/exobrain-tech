---
digest-of: programming/project-management
last-synced: 2026-08-11
token-estimate: 620
---

# AGENTS

## Scope

Language-agnostic planning and work-execution canon: story shape and estimation, lane coordination,
validation, derived views, and the boundary between planned change and durable specifications.

Documentation ownership and markdown mechanics route to
[documentation design](../docs-design/AGENTS.md).

## Rule ownership

| Question the agent arrives with                             | Owning file                                  |
| ----------------------------------------------------------- | -------------------------------------------- |
| Why is planning a second axis?                              | `00-plan-zone-as-a-second-axis.md`           |
| What is a story and what do points count?                   | `01-stories-and-estimation.md`               |
| How is one story placed and written?                        | `02-the-story-on-disk.md`                    |
| How do lanes, dependencies, questions, and ranking compose? | `03-the-plan-record.md`                      |
| What validates the record and what views derive from it?    | `04-gating-the-plan.md`                      |
| How do acceptance assertions become durable specs?          | `05-specs-and-stories.md`                    |
| How does a project instantiate the plan or a story?         | `template-plan-zone.md`, `template-story.md` |
| Where is the story heading contract wired?                  | `template-heading-shapes.md`                 |
| What must pass before a planning change merges?             | `99-checklist.md`                            |
| What does this shelf mean by a planning term?               | `glossary.md`                                |

## Non-negotiables

- Points count irreducible human judgment; work above three points splits.
- A story is one Markdown file; a same-name directory holds only non-narrative artifacts.
- The lane files own coordination and the story owns what its work session must act on.
- Ranking respects dependencies, and eligible `todo` entries precede ineligible ones.
- `Governed by` states individual source claims, and `Amends` names specs left changed.
- The schema owns one lane's shape; `check-plan` owns cross-file coherence.

## Maintenance

- Regenerate when this shelf's knowledge changes.
- This digest is a map, never a rules home; the owning chapter wins on disagreement.

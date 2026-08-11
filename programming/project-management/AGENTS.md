---
digest-of: programming/project-management
last-synced: 2026-08-11
token-estimate: 450
---

# AGENTS

## Scope

Language-agnostic planning and work-execution canon: intent over time, appetite and scope, slices, plan
records, templates, executable validation artifacts, and the project-management review gate.

Documentation ownership, agent-context, subsystem-page, and markdown mechanics route to
[documentation design](../docs-design/AGENTS.md).

## Rule ownership

| Question the agent arrives with                                       | Owning file                                  |
| --------------------------------------------------------------------- | -------------------------------------------- |
| Why is planning a second axis rather than a fifth documentation need? | `00-plan-zone-as-a-second-axis.md`           |
| How is a unit of work bounded, cut, extended, or reshaped?            | `01-appetite-and-scope.md`                   |
| Which plan documents exist, and what shape does a slice take?         | `02-plan-and-slices.md`                      |
| What is the current lane record, and what validates it?               | `plan/README.md`                             |
| How do I start a plan zone or one slice entry?                        | `template-plan-zone.md`, `template-slice.md` |
| Where do the slice and milestones heading arrays and hooks live?      | `template-heading-shapes.md`                 |
| What must pass before a planning change merges?                       | `99-checklist.md`                            |
| What does this shelf mean by a planning term?                         | `glossary.md`                                |

## Non-negotiables

- Fix an appetite before designing the slice; protect the core by cutting the ordered remainder first.
- A slice is one directory with one entry `README.md`; additional files exist only when their gates are
  met.
- `Governed by` names individual governing specs and ADRs, never a directory or a whole zone.
- Acceptance names objective tests, rabbit holes name escapes, and material revisions remain visible.
- Plan status has one owner, open questions name what they block, and finished durable results leave
  the plan zone.
- The lane schema owns one file's shape; the linter owns cross-file coherence.

## Maintenance

- Regenerate when this shelf's knowledge changes.
- This digest is a map, never a rules home; the owning chapter wins on disagreement.

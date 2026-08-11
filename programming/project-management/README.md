# Project Management

This shelf owns planning and executing work: intent over time, bounded units of work, slices, the plan
record, and the executable artifacts that validate it.

The [documentation-design shelf](../docs-design/README.md) owns the durable specs, ADRs, subsystem pages, and reference material that govern or result from slices.

## Chapters and support

| #        | Chapter or support                                               | One-line hook                                                         |
| -------- | ---------------------------------------------------------------- | --------------------------------------------------------------------- |
| 00       | [Plan Zone as a Second Axis](./00-plan-zone-as-a-second-axis.md) | Separate perishable intent from documentation about what exists.      |
| 01       | [Appetite and Scope](./01-appetite-and-scope.md)                 | Fix the budget, vary the scope, and gate every change to it.          |
| 02       | [Plan and Slices](./02-plan-and-slices.md)                       | Shape the plan zone and one bounded directory per unit of work.       |
| 99       | [Checklist](./99-checklist.md)                                   | Review scope, slices, plan coherence, and completion handoff.         |
|          | [Glossary](./glossary.md)                                        | Resolve planning terms at the chapter that defines each one.          |
| Template | [Plan zone](./template-plan-zone.md)                             | Start a charter, status surface, and open-question register.          |
| Template | [Slice](./template-slice.md)                                     | Start one work entry with scope, sources, acceptance, and revisions.  |
| Template | [Heading shapes](./template-heading-shapes.md)                   | Gate slice and milestones headings with live hook selectors.          |
| Artifact | [Plan record](./plan/)                                           | Validate lane files and cross-file plan coherence with working tools. |

Read the second-axis model first, then choose the work bound, learn the on-disk slice and plan shape,
and finish with the review checklist. Methods, stories, and measurements may be added later; this shelf
does not claim they exist yet.

## Adopting this shelf

Choose one appetite unit, copy only the plan files the project maintains, use the slice and heading
templates where their gates apply, and copy the executable plan artifacts when adopting the lane record.
Keep the plan truthful while work is active, and promote completed durable results to their owning
documentation zones.

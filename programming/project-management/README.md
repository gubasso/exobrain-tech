# Project Management

Language-agnostic planning and work execution: intent over time, bounded stories, lane records,
validation, derived views, and the handoff from planned change to durable documentation.

## Chapters and support

| #        | Chapter or support                                               | Reader question                                          |
| -------- | ---------------------------------------------------------------- | -------------------------------------------------------- |
| 00       | [Plan Zone as a Second Axis](./00-plan-zone-as-a-second-axis.md) | Why is planning separate from reader-need documentation? |
| 01       | [Stories and Estimation](./01-stories-and-estimation.md)         | What is the work unit and what do points count?          |
| 02       | [The Story on Disk](./02-the-story-on-disk.md)                   | What shape does a story take and how is it written?      |
| 03       | [The Plan Record](./03-the-plan-record.md)                       | Where does work sit, wait, and rank?                     |
| 04       | [Gating the Plan](./04-gating-the-plan.md)                       | What validates and derives from the record?              |
| 05       | [Specs and Stories](./05-specs-and-stories.md)                   | Where does accepted behavior become durable state?       |
| 99       | [Checklist](./99-checklist.md)                                   | What must pass before a planning change merges?          |
|          | [Glossary](./glossary.md)                                        | Where is each planning term owned?                       |
| Template | [Plan zone](./template-plan-zone.md)                             | How does a project instantiate the record?               |
| Template | [Story](./template-story.md)                                     | How does one work entry begin?                           |
| Template | [Heading shapes](./template-heading-shapes.md)                   | How is the story heading contract gated?                 |
| Artifact | [Plan record](./plan/)                                           | Which copyable files validate the plan?                  |

The [documentation-design shelf](../docs-design/README.md) owns the durable specs, ADRs, subsystem
pages, and markdown mechanisms that govern or result from stories.

## Adoption

Start with the second-axis boundary, copy the plan-zone and story templates, then copy the schema
and linter. Keep lane membership and ranking in the record, keep the implementing session's
obligations in the story, and promote accepted behavior to its durable owner.

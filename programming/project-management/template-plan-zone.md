# Template — plan zone

Copy each block into the named path under `<project>/docs/plan/`. The complete record uses all five
lane files even when one is empty. Start work with [the story template](./template-story.md), and
copy the [schema and linter](./plan/) beside the plan.

## `charter.md`

```markdown
# <project> — Charter

## What this is for

<The outcome the project exists to produce.>

## Pillars

- <A property every story is judged against.>

## No-gos

- <Something this project will not do, and why.>

## Iteration

<A cadence such as two weeks. Keep one cadence; changing it restarts the velocity series.>
```

## `lanes/backlog.yml`

```yaml
# yaml-language-server: $schema=../plan-lane.schema.json
lane: backlog
stories: []
```

## `lanes/todo.yml`

```yaml
# yaml-language-server: $schema=../plan-lane.schema.json
lane: todo
stories:
  - id: "<id>"
    slug: <slug>
    type: <story | spike | chore>
    points: <1 | 2 | 3>
    needs: []
```

## `lanes/doing.yml`

```yaml
# yaml-language-server: $schema=../plan-lane.schema.json
lane: doing
stories: []
```

## `lanes/review.yml`

```yaml
# yaml-language-server: $schema=../plan-lane.schema.json
lane: review
stories: []
```

## `lanes/closed.yml`

```yaml
# yaml-language-server: $schema=../plan-lane.schema.json
lane: closed
stories:
  - id: "<id>"
    slug: <slug>
    type: <story | spike | chore>
    points: <1 | 2 | 3>
    outcome: <done | cut | reshaped>
    closed: <yyyy-mm-dd>
```

## `stories/`

```text
stories/
  007-rate-limit.md       the story; always
  007-rate-limit/         artifacts; only when needed
```

The optional sibling directory holds traces, fixtures, and diagram sources. It never holds a
second narrative document. This is a layout a project creates, not an index of repository state.

## `open-questions.md`

```markdown
# Open questions

## Q-001 — <the question?>

Raised: <when and in what context>.
Blocks: <004, 005> — <why these ids cannot proceed>.
Exit: <ADR | story revision | measurement>, <what closes it>.
```

`Blocks:` is a comma-separated id list before an optional em dash and reason. A question that
blocks nothing belongs in the drafts workspace.

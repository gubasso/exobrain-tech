# Template — plan zone

Copy each block into the named path under `<project>/docs/plan/`. The complete record uses all five
lane files even when one is empty. Start work with [the story template](./template-story.md), reach
for [the epic template](./template-epic.md) when one goal outgrows a single story, and copy the
[schemas and linter](./plan/) beside the plan.

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

<A cadence such as two weeks, and what the project does with it. Keep one cadence; changing it
restarts the velocity series. `config.yml` carries the same cadence in the form a script reads.>
```

## `config.yml`

Parameters a plan-zone script needs and cannot derive from the record. Nothing links to it — the
path is the contract.

```yaml
# yaml-language-server: $schema=./plan-config.schema.json
iteration:
  start: "<yyyy-mm-dd>"
  length_days: <14>
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
    epic: "<id>"
```

`epic` is optional and names a document under `epics/` by id. It never affects ranking.

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

## `epics/`

```text
epics/
  014-session-hardening.md    one end state several stories reach
```

Create it only when a goal outgrows one story. Ids come from the same sequence stories use, so one
id names one thing. The document names no member stories; the entries name it. An epic may exist
before any member does.

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

## `README.md`

The zone's entry point. It teaches how to read the record; it never lists what the record contains.

```markdown
# Plan

What this project is building next, and what bounds it.

## Where to start

The topmost entry of `lanes/doing.yml` is the work in flight. When that lane is empty, the topmost
entry of `lanes/todo.yml` is what to start: ranking is legal by construction, so the first entry is
always eligible. Open its story under `stories/`, and load the individual sources its `Governed by`
section names.

## What bounds it

`charter.md` states the outcome, the pillars every story is judged against, and the no-gos.
`config.yml` carries the iteration cadence a script reads. An entry may name an epic under
`epics/`, which holds an end state no single story delivers.

## What can stop it

`needs` on an entry names the ids that must close first. `open-questions.md` names decisions that
block specific ids until they are answered. Neither is a stored status; both are read from the
record.

## What already happened

`lanes/closed.yml` is an append-only log ordered by close date. It is history, not a queue.
```

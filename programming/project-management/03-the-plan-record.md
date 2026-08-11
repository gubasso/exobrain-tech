# 03 — The Plan Record

Where does work sit, what does it wait on, and what is next? Five YAML lane files own coordination;
the story owns the work session.

## Lanes and entries

Every plan zone has `backlog.yml`, `todo.yml`, `doing.yml`, `review.yml`, and `closed.yml`, even
when one is empty. The file is the lane, so entries carry neither `status` nor `priority`.

```yaml
lane: todo
stories:
  - id: "009"
    slug: release-readiness
    type: story
    points: 2
    needs: ["006"]
```

Sequence position is the human ranking. `needs` is a flow sequence of ids that must be in
`closed.yml` before the dependent story may enter `doing` or `review`. Closed entries also carry
`outcome` and an ISO `closed` date; a `reshaped` outcome names `succeeded_by`.

An entry may also carry `epic`, naming by id a document that holds an end state several stories
reach. Ids are unique across stories and epics alike, so one id names one thing. It is a gated
reference and never a sequencing fact; see [06 — Epics](./06-epics.md).

## Eligibility and legal ranking

An entry is eligible when every id in `needs` is in `closed.yml` and no open question blocks it.
Two rules keep ranking consistent with that predicate:

- R1: in `todo.yml`, no ineligible entry sits above an eligible one.
- R2: in `backlog.yml` and `todo.yml`, no entry sits above an entry it needs.

The first entry in `todo.yml` is therefore the work to start. Promotion from backlog remains a
human choice. Ordering inside a set of equally legal entries is also a human choice; the machine
owns legality, not priority, and a legal order a human or an agent chose is never disturbed.

`doing` and `review` already require every dependency closed and no blocking question, so the two
rules are satisfied there by the lane gate alone. Position still reads top-first: the topmost entry
of `doing.yml` is the work in flight, and it is the current story whenever that lane is occupied.

`closed.yml` is the exception, and is an append-only log rather than a queue. Its order is its close
dates, ascending, with a day's closes keeping the order they were written in. A date out of sequence
is reported and never gated, because the writer can restore it; see
[04 — Gating the Plan](./04-gating-the-plan.md).

## Blocking is a badge

Blocked is derived, not a lane or stored field. Dependency blocking is recorded by `needs` on the
entry. Decision blocking is recorded by `Blocks:` in `open-questions.md` using a comma-separated id
list before an optional em dash and reason.

```text
Blocks: 004, 005 — the storage choice changes both interfaces
```

An open question cannot block work already in `doing` or `review`, and it leaves when every target
has closed. A question that blocks nothing belongs in the drafts workspace.

## Costs

A lane move changes two files. Closing a story may make another eligible, so the same change also
runs `check-plan --rank-fix` and may update `todo.yml`. That extra diff buys a record whose top
never contradicts its dependency graph.

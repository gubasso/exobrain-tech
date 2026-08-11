# 00 — The Plan Zone as a Second Axis

This shelf owns planning and executing work—intent, scope, slices, and the plan record—while
documentation design owns the durable artifacts produced by completed work.

## The plan zone

Diataxis organizes what exists: all four reader needs are asked about a system that is already there.
It does not model intent over time, and does not claim to. A project that defines its artifacts before
implementing them needs a home for what is being built next, what bounds it, and what is still
undecided — material that is not a task, not lookup, not a mental model, and not a settled decision.

```text
               what exists  (Diataxis)
┌──────────────────────────┬──────────────────────────┐
│ guides/                  │ reference/               │
│ "what do I do next?"     │ "what is the exact       │
│                          │  value, field, symptom?" │
├──────────────────────────┼──────────────────────────┤
│ explanation/             │ decisions/               │
│ "how does this area      │ "why did the project     │
│  fit together?"          │  choose this shape?"     │
└──────────────────────────┴──────────────────────────┘
                  ▲
                  │  all four are asked about a system
                  │  that is already there
─ ─ ─ ─ ─ ─ ─ ─ ─ ┼ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─   second axis: intent
                  │
┌─────────────────┴─────────────────────────────────────┐
│ plan/                                                 │
│ "what are we building next, and what bounds it?"      │
│ the only zone whose contents are expected to go stale │
└───────────────────────────────────────────────────────┘
```

The plan zone is the only zone whose contents are expected to become false. A guide that stops being
true is a bug; a milestone that stops being current is the normal course of the work. That is why plan
documents carry statuses and the other four do not, and why a plan document may serve intent, scope,
and status at once instead of one reader need. Work that finishes leaves the zone the way any fact
does: the durable result moves to the zone that owns it and the plan keeps only the pointer.

What bounds a unit of work is in [01 — Appetite and Scope](./01-appetite-and-scope.md); what documents
the zone holds is in [02 — Plan and Slices](./02-plan-and-slices.md).

## Sources

- Diataxis: <https://diataxis.fr/>
- How to use Diataxis: <https://diataxis.fr/how-to-use-diataxis/>

# Plan

What this repository is building next, and what bounds it.

The method and the tooling are [plan-xp](https://github.com/gubasso/plan-xp), pinned in `flake.nix`.
This directory is the record it validates; `just test-plan` is what runs it.

## Where to start

The topmost entry of `lanes/doing.yml` is the work in flight. When that lane is empty, the topmost
entry of `lanes/todo.yml` is what to start: ranking is legal by construction, so the first entry is
always eligible. Open its story under `stories/`, and load the individual sources its `Governed by`
section names.

## What bounds it

`charter.md` states the outcome, the pillars every story is judged against, and the no-gos.
`config.yml` carries the two path keys and the iteration cadence a tool reads.

## What can stop it

`needs` on an entry names the ids that must close first. `open-questions.md` names decisions that
block specific ids until they are answered. Neither is a stored status; both are read from the
record.

## What already happened

`lanes/closed.yml` is an append-only log ordered by close date. It is history, not a queue.

## A note on the schemas

The `yaml-language-server` modelines above each lane file point at the public raw URL, and they are
editor convenience only. The gate is authoritative, and it validates against
`$PLAN_XP_SCHEMA_DIR`, which the devShell exports from the locked package. No copy of either schema
lives in this tree: a copy would be the duplication `AGENTS.md` forbids, and a copy that drifts from
the version the gate runs is worse than none.

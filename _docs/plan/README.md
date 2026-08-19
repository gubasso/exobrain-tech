# Plan

What this repository is building next, and what bounds it.

This directory is the record of that. The repository names no method that fills it, so its shape is
held by review rather than by a linter.

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

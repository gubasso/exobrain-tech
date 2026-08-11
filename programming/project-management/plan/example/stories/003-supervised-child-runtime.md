# 003 — Supervised Child Runtime

## Goal

Determine whether a crashed child can restart without losing its request boundary.

## Example

The spike runs one crash case and records the observed boundary.

```console
$ runtime-probe --crash-child
child restarted; request failed once; supervisor remained ready
```

## Core

Answer whether supervision preserves the parent process and isolates the failed request.

## In scope

- One representative crash.
- Restart and request observations.

## Out of scope

- Shipping the supervisor.

## Governed by

None.

## Amends

None.

## Acceptance

- The probe records process and request outcomes — `runtime_probe::child_crash`.

## Tasks

- [x] Build the probe.
- [x] Record the result.

## Rabbit holes

- Platform variance — escape: test the supported runtime only.

## Done when

The question has a reproducible answer.

## Revisions

None.

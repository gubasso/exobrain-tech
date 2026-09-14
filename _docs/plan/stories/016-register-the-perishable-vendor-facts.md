# 016 — Register the Perishable Vendor Facts

## Goal

Every document holding a vendor fact that expires has a timer, so a stale adapter surfaces as a
failing gate rather than as a reader acting on last quarter's behavior.

## Example

Seven documents carry a verification date and nothing reads it.

```console
$ grep -c "" _docs/reference/tracking.yaml
20
$ grep -rl "Verified 2026-09-14" tools/ | wc -l
6
```

The registry the specification calls the record of facts that expire is empty, while six adapters
and one baseline chapter state behavior that ships on somebody else's release schedule.

## Core

One entry per document, each naming why the fact ages, the pages to re-read, and the steps that end
with the date advancing. The baseline entry declares the adapters as its dependents, because a
changed specification changes what every exception is an exception to.

## In scope

- An entry for each of the four vendor skill adapters.
- An entry for the Claude Code memory-file document.
- An entry for the Agent Skills baseline the skills chapter states.
- A cadence the project can honor, since an overdue entry blocks every commit.

## Out of scope

- Re-reading the sources, which all carry today's date.
- The dates inside the documents, which stay where the reader meets them.
- Any fact that changes only when this repository changes.

## Governed by

- `_docs/specs/SPEC-tracking.md` — the registry shape, the freshness gate, and the pinned revision.
- `_docs/specs/SPEC-docs-format.md` — the register and the size budgets.

## Amends

- `_docs/reference/tracking.yaml` — seven entries.

## Acceptance

- `sdd gate tracking-registry` passes and `sdd track status` reports every entry current.
- Every entry names a path that exists and revalidation steps that end at `last_checked`.
- No entry restates the fact it tracks.
- `just check` passes.

## Tasks

- [ ] Write one entry per perishable document.
- [ ] Declare the adapters as dependents of the baseline entry.
- [ ] Confirm the gate and the offline status report.

## Rabbit holes

- Copying the vendor behavior into the entry — escape: the entry says where to look, never what the
  source says.
- A cadence short enough to block work it cannot pay for — escape: one quarter, which a reader can
  shorten for a source that proves faster.

## Done when

A vendor page that moves stops the next commit that comes after the cadence, and the entry says what
to re-read.

## Revisions

None.

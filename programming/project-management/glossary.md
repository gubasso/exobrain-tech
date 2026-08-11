# Glossary

Each planning term points to the chapter that owns its rules.

- `badge` — a presentation marker derived from the record, such as dependency-blocked or
  question-blocked. Rules: [03 — The Plan Record](./03-the-plan-record.md).
- `canonical YAML subset` — the flat, one-level, flow-sequence form accepted by `check-plan`.
  Rules: [04 — Gating the Plan](./04-gating-the-plan.md).
- `chore` — necessary work without a customer-visible outcome. Rules:
  [01 — Stories and Estimation](./01-stories-and-estimation.md).
- `cycle time` — elapsed time from the first entry into `doing` until close, derived from repository
  history. Rules: [04 — Gating the Plan](./04-gating-the-plan.md).
- `eligible` — every needed id is closed and no open question blocks the entry. Rules:
  [03 — The Plan Record](./03-the-plan-record.md).
- `epic` — the entry field naming one end state document under `epics/` by id, and that document.
  Single-valued, never a sequencing fact, and never a stored completion state. Rules:
  [06 — Epics](./06-epics.md).
- `lane` — one YAML file whose membership supplies workflow state. Rules:
  [03 — The Plan Record](./03-the-plan-record.md).
- `legal ranking` — an order satisfying the eligibility partition and dependency extension rules.
  Rules: [03 — The Plan Record](./03-the-plan-record.md).
- `needs` — the entry field naming ids that must close before work may proceed. Rules:
  [03 — The Plan Record](./03-the-plan-record.md).
- `plan config` — the schema-validated `config.yml` holding parameters a script reads and the record
  cannot supply. Rules: [04 — Gating the Plan](./04-gating-the-plan.md).
- `points` — `1 | 2 | 3` units of irreducible human judgment. Rules:
  [01 — Stories and Estimation](./01-stories-and-estimation.md).
- `review dwell` — time between entry into and exit from `review`, derived from history. Rules:
  [04 — Gating the Plan](./04-gating-the-plan.md).
- `rework rounds` — entries into `doing` beyond the first, derived from history. Rules:
  [04 — Gating the Plan](./04-gating-the-plan.md).
- `spec` — a reference page stating current capability behavior. Rules:
  [05 — Specs and Stories](./05-specs-and-stories.md).
- `spike` — time-boxed research whose output is an answer rather than behavior. Rules:
  [01 — Stories and Estimation](./01-stories-and-estimation.md).
- `story` — one vertical, demonstrable unit of planned change. Rules:
  [01 — Stories and Estimation](./01-stories-and-estimation.md).
- `task` — an implementation step inside a story, not a separate plan record. Rules:
  [02 — The Story on Disk](./02-the-story-on-disk.md).
- `velocity` — points from done entries per configured iteration, derived rather than stored.
  Rules: [01 — Stories and Estimation](./01-stories-and-estimation.md).
- `warning` — a diagnostic the writer can repair, reported without affecting an exit code. Rules:
  [04 — Gating the Plan](./04-gating-the-plan.md).

# 99 — Checklist

The review gate for planning changes. Each check is derived from an owning chapter or executable
artifact; the owner wins if wording differs.

## Story opening

Owners: [01 — Stories and Estimation](./01-stories-and-estimation.md) and
[02 — The Story on Disk](./02-the-story-on-disk.md).

- [ ] The entry uses `story`, `spike`, or `chore` and `1 | 2 | 3` points.
- [ ] The point value names the human judgments it counts; no gate covers this review.
- [ ] Core is distinct from ordered negotiable scope, and work above three points splits.
- [ ] Every fixed heading is present, and a story or spike carries an Example fence.
- [ ] The Example demonstrates concrete reader-side behavior; no gate covers its quality.
- [ ] Prose describes the change, while durable facts live at their owners; no gate covers leanness.
- [ ] Source references state their claims and link to anchors rather than line coordinates.
- [ ] `Governed by` names individual sources, and `Amends` names specs the session must change.
- [ ] Acceptance assertions name tests, tasks are a checklist, and every known trap has an escape.

## Record changes

Owners: [03 — The Plan Record](./03-the-plan-record.md) and
[04 — Gating the Plan](./04-gating-the-plan.md).

- [ ] All five lane files exist and match the schema.
- [ ] Every id is unique, every entry has one matching story file, and dependencies exist without a
      cycle.
- [ ] Work in `doing` or `review` has no open need or blocking question.
- [ ] `todo` keeps eligible entries first, and backlog and todo respect dependency order.
- [ ] Closing a story includes its outcome and close date, then runs `check-plan --rank-fix` before
      read-only validation.
- [ ] A closed story that amends a spec leaves its acceptance assertion in that spec in present
      tense; no gate covers this content transfer.
- [ ] No tool reorders `closed.yml`, and no hook invokes the writer mode.

## Verification

- [ ] Relative links and markdown checks pass.
- [ ] The schema, example lanes, linter, shell checks, and behavioral harness pass.
- [ ] Every digest ownership row points to a file that exists and owns the named rule.

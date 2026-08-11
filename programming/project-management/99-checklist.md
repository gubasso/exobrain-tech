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

- [ ] All five lane files exist and match the schema, and any `config.yml` matches its own.
- [ ] Every id is unique across stories and epics, every entry has one matching story file, and
      dependencies exist without a cycle.
- [ ] Every `epic` names a document that exists, and no entry serves two end states.
- [ ] Work in `doing` or `review` has no open need or blocking question.
- [ ] `todo` keeps eligible entries first, and backlog and todo respect dependency order.
- [ ] Closing a story includes its outcome and close date, appends it to the bottom of `closed.yml`,
      then runs `check-plan --rank-fix` before read-only validation.
- [ ] A `reshaped` entry's successor carries the epic the work actually serves; no gate covers this.
- [ ] A closed story that amends a spec leaves its acceptance assertion in that spec in present
      tense; no gate covers this content transfer.
- [ ] Only `--rank-fix` reorders `closed.yml`, and no hook invokes the writer mode.

## Epic changes

Owner: [06 — Epics](./06-epics.md).

- [ ] The epic states an end state no single member delivers, shown as a concrete example.
- [ ] Its id comes from the shared sequence, and the file is named `<id>-<slug>.md`.
- [ ] It lists no member stories, carries no status, and names no parent epic.
- [ ] Every fixed heading is present, and `Amends` names existing spec paths or `None`.
- [ ] `Done when` states the condition that makes the end state true, not that every member closed.
- [ ] The epic earns a document: a one-member epic is a story, and a label is a tag.

## Verification

- [ ] Relative links and markdown checks pass.
- [ ] Both schemas, example lanes, config, linter, shell checks, and behavioral harness pass.
- [ ] Every digest ownership row points to a file that exists and owns the named rule.

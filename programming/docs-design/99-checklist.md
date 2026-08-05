# 99 — Checklist

Use this checklist before merging new or changed documentation. If a box is unchecked, fix the doc
or record an explicit exception in the owning source of truth.

## New or changed docs

- [ ] The file has one primary reader need: task, lookup, understanding, or decision. Plan-zone
      documents are exempt: they may carry intent, scope, and status together.
- [ ] The file lives in the matching zone from [01 — Diataxis Zones](./01-diataxis-zones.md).
- [ ] The file does not restate a fact already owned elsewhere.
- [ ] Cross-links point to the owning source instead of copying it.
- [ ] No filesystem trees are pasted into markdown; index files (README/AGENTS) explain purpose and
      do not replicate the directory tree.
- [ ] Project-agnostic docs use `<angle>` placeholders for project-specific names.
- [ ] Perishable facts (prices, benchmarks, model rosters, API shapes) have an owner, a revalidation
      cadence, and an up-to-date tracking entry when needed.
- [ ] The root docs README remains an index, not a duplicate source of rules.

See [08 — Tracking and Revalidation](./08-tracking-and-revalidation.md).

## ADRs

- [ ] New ADRs use the sections from [02 — Lean ADRs](./02-lean-adrs.md).
- [ ] The filled ADR body is at or below 350 words.
- [ ] The ADR has exactly one canonical `Status:`.
- [ ] Status is one of `Ideation`, `Proposed`, `Accepted`, `Implemented`, `Deprecated`,
      `Superseded`, or `Rejected`.
- [ ] The decision cleared the ADR threshold: cross-cutting, hard to reverse, constraining, or
      rejecting a plausible alternative. A run of ADRs on one area means a design doc is missing.
- [ ] Implemented ADRs link to the code, config, or docs that enact them when a stable target
      exists.
- [ ] Superseded ADRs link to the successor; deprecated ADRs say why they stopped applying.
- [ ] An ADR changed only in part by a later ADR keeps its status and carries an
      `Amended by ADR-NNNN` line under `Status`.
- [ ] Rejected ADRs explain enough to prevent repeated debate without new evidence.
- [ ] The project did not delete an accepted or implemented decision.
- [ ] Supporting data too large for the ADR lives in reference or explanation and is linked.

## Placement

- [ ] Project-wide rules live in `<project>/CLAUDE.md` or the local equivalent.
- [ ] Architecture decisions live in `<project>/docs/decisions/`.
- [ ] Step-by-step workflows and runbooks live in `<project>/docs/guides/`.
- [ ] Lookup, diagnostics, and case studies live in `<project>/docs/reference/`.
- [ ] Cross-cutting overview or architecture lives in `<project>/docs/explanation/`.
- [ ] The current design of one subsystem lives in `<project>/docs/explanation/<subsystem>.md`, not
      spread across the decision records.
- [ ] Binding forward-looking material — scope, milestones, open questions — lives in
      `<project>/docs/plan/`, not in the gitignored drafts workspace.
- [ ] Non-obvious local code rationale lives in code comments, not distant prose.
- [ ] Behavior contracts are expressed by names, types, and tests where possible.
- [ ] The file that owns the fact is clear enough that future edits have one target.

See [04 — Single Source of Truth](./04-single-source-of-truth.md).

## Drafts

- [ ] Drafts stay outside `<project>/docs/`.
- [ ] Promotion rewrites the draft into a durable doc instead of moving it unchanged.
- [ ] The promoted document has one reader need.
- [ ] The old draft is deleted or remains clearly local and ignored.
- [ ] No draft is the only home for a real project decision.
- [ ] Promotion removed temporary reasoning and repeated facts.

See [05 — Drafts and Promotion](./05-drafts-and-promotion.md).

## Markdown style

- [ ] The file contains no bold and no italics.
- [ ] Identifiers, paths, flags, commands, and status values are inline code.
- [ ] Binding requirements use uppercase RFC 8174 keywords, and only in normative documents.
- [ ] Every fenced code block declares a language; use `text` when none applies.
- [ ] No paragraph is doing a heading's job.
- [ ] Any exception carries an `<!-- allow-emphasis: <reason> -->` comment.

See [10 — Lean Markdown](./10-lean-markdown.md).

## Plan and appetite

- [ ] Every unit of work names an appetite, fixed before the design and stated in the project's
      chosen unit.
- [ ] The unit of work declares a non-negotiable core and a negotiable remainder, not one
      undifferentiated scope list.
- [ ] The core leaves room to cut inside the appetite; a core that fills the budget means the work
      needs splitting.
- [ ] Anticipated traps carry a pre-authorized escape, and unanticipated ones go to the
      open-questions register rather than silently expanding scope.
- [ ] Completion is objective: named tests pass unskipped.
- [ ] Any appetite change is a committed edit citing the condition met — core only, and all
      remaining work downhill — and what was cut first.
- [ ] No unit of work was brought inside its appetite by dropping tests, review, or a security
      control.

See [11 — Appetite and Scope](./11-appetite-and-scope.md).

## Plan zone and slices

- [ ] The unit of work is one directory under `<project>/docs/plan/slices/` whose `README.md` is the
      entry document, and it was committed before the work started.
- [ ] The slice uses the fixed heading list and did not grow a section that belongs in `charter.md`,
      an ADR, or a subsystem page.
- [ ] `tasks.md` exists only because implementation crosses a context reset; `requirements.md` only
      because acceptance is many-to-many onto tests; no `design.md` exists.
- [ ] `Governed by` names individual files and records, never a directory, a zone, or "the docs".
- [ ] Acceptance lines are EARS-phrased and each names a test; a hook fails when a named test cannot
      be found, or the naming was dropped.
- [ ] Every rabbit hole carries a pre-authorized escape rather than only a warning.
- [ ] Any change to `Goal`, `Core`, `Appetite`, or `Acceptance` after the work started is a committed
      edit with a `Revisions` line saying what was learned.
- [ ] Open-questions entries name what they block, and closed ones left by becoming an ADR, a slice
      revision, or a recorded measurement.
- [ ] The current design of a subsystem lives in `<project>/docs/explanation/<subsystem>.md`, which
      links the ADRs that own the choices rather than restating them.
- [ ] No accepted ADR was rewritten to describe the present.

See [12 — Plan and Slices](./12-plan-and-slices.md).

## Agent readiness

- [ ] Filenames expose purpose before content is opened.
- [ ] Headings are stable and direct.
- [ ] `CLAUDE.md` or the local author-instructions file does not contradict the docs.
- [ ] `AGENTS.md` digests summarize source files only.
- [ ] Oversized ADRs, repeated tables, stale drafts, and broad summaries have been trimmed.
- [ ] Always-loaded author-instruction files stay under 200 lines.
- [ ] Work in flight has one entry document naming the sources a session should load, and it is the
      slice `README.md`.
- [ ] Filenames can be understood from search results without opening the files.

See [07 — AI Agent Considerations](./07-ai-agent-considerations.md).

## Known issues

- [ ] A bug in an external system under test is a known-issue case under
      `<project>/docs/reference/known-issues/`, not a top-level topic folder or a stray draft.
- [ ] The case has a stable `KI-<NNNN>` id and the expected skeleton (index, metadata,
      investigation, escalation, evidence; a mask ledger only when a workaround exists).
- [ ] Any temporary mask carries a revert trigger and checklist, and the code site references the
      id.
- [ ] A resolved case is collapsed to one summary that keeps issue, root cause, resolution, and
      recurrence signal; the raw trail is left in version-control history.
- [ ] The registry/index row matches the case directory and its status.

See [09 — Known Issues](./09-known-issues.md).

## Verification

- [ ] Cross-links pass the project's link-check hook.
- [ ] Spelling and markdown hooks pass.
- [ ] Operational docs are under guides or reference, not topic directories beside the Diataxis
      zones.
- [ ] Comments added with the docs change are load-bearing per
      [03 — Comments and Code as SoT](./03-comments-and-code-as-sot.md).
- [ ] The reviewer can name the source of truth for every durable fact touched by the change.
- [ ] Any generated digest was updated from source files and does not introduce new guidance.

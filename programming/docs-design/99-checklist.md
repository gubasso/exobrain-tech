# 99 — Checklist

The pre-merge gate for a documentation diff. Every box is answerable yes or no from the diff or from a
command; a box that only restates a path is not here, because the diff already shows the path. This
file is derived — it may restate a rule in test form, but it never owns one, and it carries no
rationale, definitions, or citations. If a box and its owning chapter disagree, the chapter wins.

## Ownership

Owner: [00 — Foundations](./00-foundations.md).

- [ ] Every durable fact touched by the change has exactly one owner, and the reviewer can name it.
- [ ] No file restates a fact another file owns; non-owners link instead.
- [ ] Cross-links name the reason to follow them, and no passage indexes the filesystem for its own
      sake; a listing that stays earns it by what its entries teach.
- [ ] The current design of a subsystem is in `<project>/docs/explanation/<subsystem>.md`, not spread
      across decision records; see [03 — Subsystem Pages](./03-subsystem-pages.md).
- [ ] Comments added with the change are load-bearing, and behavior contracts are carried by names,
      types, and tests where they can be.

## Placement

Owner: [01 — Diataxis Zones](./01-diataxis-zones.md).

- [ ] The docs root matches the project's product: `docs/` for a codebase, `_docs/` for a knowledge
      base, and nothing that is product content sits under it.
- [ ] Each durable file has one primary reader need and lives in the matching zone; the plan exception
      is owned by the [project-management checklist](../project-management/99-checklist.md).
- [ ] Operational material is zone-first: runbooks and setup in guides, diagnostics and case studies in
      reference, never a topic directory beside the zones.
- [ ] The root docs README is still an index.

## Decisions

Owner: [02 — Lean ADRs](./02-lean-adrs.md).

- [ ] The decision cleared the threshold: cross-cutting, hard to reverse, constraining, or rejecting a
      plausible alternative. A run of ADRs on one area means a subsystem page is missing.
- [ ] The filename carries the `ADR-` prefix and a slug: `ADR-<number>-<decision>.md`. Templates and
      index files in the decisions directory carry no prefix.
- [ ] The filled body is at or below 350 words and uses the five template sections.
- [ ] Exactly one `Status`, from `Ideation`, `Proposed`, `Accepted`, `Implemented`, `Deprecated`,
      `Superseded`, `Rejected`.
- [ ] Implemented records link what enacts them; superseded link the successor; deprecated say why they
      stopped applying; rejected explain enough to prevent repeated debate.
- [ ] A record changed only in part keeps its status and carries an `Amended by ADR-NNNN` line.
- [ ] No accepted or implemented decision was deleted, and none was rewritten to describe the present.
- [ ] Supporting data too large for the record lives in reference or explanation and is linked.

## Drafts

Owner: [05 — Drafts and Promotion](./05-drafts-and-promotion.md).

- [ ] Drafts stay outside `<project>/docs/`, and no draft is the only home for a real decision.
- [ ] Promotion rewrote the draft into one zone rather than moving it, and removed temporary reasoning.
- [ ] The draft is deleted or remains clearly local and ignored.

## Agent readiness

Owner: [04 — Agent Context](./04-agent-context.md).

- [ ] Filenames and headings expose purpose from search results, without opening the file.
- [ ] The author-instructions file does not contradict the docs and stays under 200 lines.
- [ ] Any `AGENTS.md` digest summarizes the area's knowledge, introduces no new guidance, and keeps
      no index of the directory.
- [ ] Work in flight has one entry document naming the sources a session should load.

## Reference artifacts

Owners: [06 — Tracking and Revalidation](./06-tracking-and-revalidation.md) and
[07 — Known Issues](./07-known-issues.md).

- [ ] Perishable facts have an owner, a cadence, and an up-to-date `last_checked` entry.
- [ ] A bug in an external system under test is a case under
      `<project>/docs/reference/known-issues/` with a stable id, the expected skeleton, and a registry
      row that matches the directory.
- [ ] Any temporary mask carries a revert trigger and checklist, and the code site references the id; a
      resolved case is collapsed to one summary.

## Markdown

Owner: [08 — Lean Markdown](./08-lean-markdown.md).

- [ ] The file contains no bold and no italics.
- [ ] Identifiers, paths, flags, commands, and status values are inline code.
- [ ] Binding requirements use uppercase RFC 8174 keywords, and only in normative documents.
- [ ] Every fenced code block declares a language; use `text` when none applies.
- [ ] No paragraph is doing a heading's job.
- [ ] Any exception carries an `<!-- allow-emphasis: <reason> -->` comment.
- [ ] Every fixed shape has exactly one `MD043` array, in its own file under `.markdownlint/`, and no
      free-form document is gated by one.
- [ ] One hook entry names that file and selects the documents it applies to; an array no hook reads is
      decoration, so the shape was proven to fail before it was trusted.
- [ ] The project config does not mention `MD043` at any value, including `false`, which would override
      every shape while the hooks stay green.
- [ ] A project with repository-invariant tests gates that absence and the hook-to-shape wiring there,
      rather than trusting a checklist for two failures the linter reports as success.
- [ ] No document carries a `markdownlint-configure-file` comment.

## Procedures

Owner: [09 — Procedure Artifacts](./09-procedure-artifacts.md).

- [ ] A guide with more than one phase names, at every phase, what it consumes and what it produces.
- [ ] Each artifact token is upper-snake in angle brackets, names the artifact rather than the step,
      and appears in exactly one outputs block.
- [ ] No token carries a real or realistic value, and no example configuration block shows one.
- [ ] Each outputs block is the last element of its section and is not a heading, and a phase that
      produces nothing says so rather than staying silent.
- [ ] Each inputs line names a producer for every token it lists; a citation of a later phase is a
      recorded exception, cross-linked both ways.
- [ ] Phases are numbered actions with commands in fenced blocks, not prose describing commands.
- [ ] Reference, explanation, and decision records carry no tokens, and any exempted guide records why.

## Verification

- [ ] Cross-links pass the project's link-check hook, and relative links resolve.
- [ ] Spelling and markdown hooks pass.
- [ ] Project-agnostic docs use `<angle>` placeholders for project-specific names.
- [ ] Any generated digest was regenerated from its sources.

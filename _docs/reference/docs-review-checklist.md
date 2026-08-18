# Documentation review checklist

Use this before merging new or changed documentation or content in `exobrain-tech`. If a box is
unchecked, fix it or record an explicit exception in the owning source of truth.

## Placement and single source of truth

- [ ] The file has one primary reader need: task, lookup, understanding, or decision.
- [ ] The file lives in the matching zone (or, for knowledge, in the right content directory).
- [ ] The file does not restate a fact already owned elsewhere; cross-links point to the owner.
- [ ] No passage indexes the filesystem for its own sake — no tree, topic-to-filename table, or
      file-list frontmatter kept because the directory has contents. Any listing that stays earns it
      by what its entries teach. A generated in-page ToC is exempt.
- [ ] Project-agnostic material uses `<angle>` placeholders for project-specific names.

## ADRs

- [ ] New ADRs use the lean sections and stay at or below 350 words.
- [ ] The filename is `ADR-<slug>.md`, carries no digit, and was not changed after merge.
- [ ] The ADR has exactly one `Status:` from
      `Proposed | Accepted | Implemented | Deprecated | Superseded | Rejected`.
- [ ] No accepted or implemented decision was deleted; superseded ADRs link to their successor and
      deprecated ADRs say why they stopped applying.
- [ ] No code comment names a decision record; an agreement is cited from code by its rule ID.

## Drafts

- [ ] Drafts stay outside the shipped tree.
- [ ] Promotion rewrote the draft into a durable home and removed repeated facts.
- [ ] No draft is the only home for a real decision.

## Agent readiness

- [ ] Filenames and headings expose purpose and can be understood from search results.
- [ ] Every affected content area's `AGENTS.md` digest reflects the area's current knowledge,
      introduces no new rules, and keeps no index of the directory.
- [ ] Oversized ADRs, repeated tables, and stale drafts were trimmed.

## Verification

- [ ] Link, spelling, and markdown hooks pass.
- [ ] The reviewer can name the source of truth for every durable fact the change touched.

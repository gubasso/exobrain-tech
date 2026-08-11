# Template — docs rules

Copy this block into `<project>/AGENTS.md`, `<project>/CLAUDE.md`, or the local author-instructions
file under `## Documentation Maintenance`. Link to the owning shelves instead of pasting their
chapters, then state local exceptions below the block.

```markdown
## Documentation Maintenance

- Keep documentation about a codebase under `<project>/docs/` and documentation about a knowledge
  base under `<project>/_docs/`; product knowledge never goes under that metadata root.
- Put decisions in `docs/decisions/`, guides and runbooks in `docs/guides/`, exact lookup in
  `docs/reference/`, current design in `docs/explanation/`, and forward intent in `docs/plan/`.
- Keep the current design of a subsystem in one explanation page; its linked ADRs stay frozen.
- Name every ADR `ADR-<number>-<decision>.md`, keep its filled body at or below 350 words, and give
  it exactly one `Status`.
- Never delete an accepted decision. Supersede, deprecate, reject, or amend it according to the
  repository's ADR lifecycle.
- Record plan state in five lane files, keep each story in `docs/plan/stories/<id>-<slug>.md`, and
  use the optional same-name directory only for non-narrative artifacts.
- Estimate stories at `1 | 2 | 3` points of irreducible human judgment. Protect the declared core,
  cut the ordered remainder first, and split work that exceeds three points.
- Gate every fixed heading contract with one `MD043` array applied by one hook entry. Let the
  project-management heading-shape template own the document list.
- Record a goal larger than one story as one epic at `docs/plan/epics/<id>-<slug>.md`, carry
  membership as `epic:` on the lane entry, and never list member stories in the epic. Stories and
  epics share one id sequence, so an id names one thing.
- Work from the current story — the topmost entry of `doing.yml`, else the topmost of `todo.yml` —
  and the individual sources its `Governed by` section names.
- Keep durable behavior in capability specs. A story names changed specs under `Amends`, and the
  accepted assertions reach those specs in the same commit as the behavior.
- Write each durable fact once at its owning home and cross-link from everywhere else.
- State the claim a source establishes and link to a heading anchor. A story never cites a line
  range; elsewhere, coordinates are permitted only after the prose already carries the claim.
- Keep story prose to the change. Move sentences still needed after close to their durable owner.
- Show each story or spike outcome as a concrete transcript, request and response, or before and
  after under `Example`.
- Let the filesystem own state: indexes explain purpose and never replicate a directory tree.
- Keep drafts in `<project>/.draft/` or another ignored workspace, outside durable docs.
- Keep code comments load-bearing: rationale, invariants, boundary conditions, and links to owners.
- Track external-system bugs as cases under `docs/reference/known-issues/`, and track perishable
  facts in a machine-readable registry with a cadence and `last_checked` date.
- Use no bold or italics. Put identifiers, paths, flags, and statuses in inline code, and give every
  fenced block a language.
- Preserve lowercase `<angle>` placeholders in project-agnostic material.
- Make each phase of a multi-phase guide name its input and output artifacts using upper-snake
  `<ANGLE>` tokens that never carry real values.
- Report documentation changes by ownership: which source changed, which links were added, and
  which gates passed.
```

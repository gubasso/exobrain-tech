---
digest-of: tools/jj
last-synced: 2026-08-29
token-estimate: 350
---

# AGENTS

## Scope

Jujutsu working notes: the workflows a reader runs, and the companion documents that explain what a
workflow's commands cost. Notes are written against jj 0.44, and a note that outgrows that version
is updated rather than dated.

## Key Points

- Every workflow here is a guide under `_docs/specs/SPEC-guides.md`: prerequisites, ordered steps,
  one imperative action each, a verifying last step.
- A guide opens with a `## The whole run` fence carrying the entire command sequence, so a reader
  who already knows the material never reads past it.
- Every command is followed by its output in a `text` fence, simulated from the one worked example
  the guide runs end to end. Change ids, bookmark names, and paths stay consistent across every
  transcript in the same guide.
- Prose earns its place only as a decision, a hazard, or an ordering constraint, and stays at one
  line. Anything longer moves to a companion document that walks a scenario with commit graphs.
- Example names carry the kind-prefix convention `README.md` defines: `bkmrk-` and `wkspc-` while
  the name is jj's alone, dropped the moment the name becomes a branch. Names arriving from a remote
  carry no prefix.
- The example vocabulary is shared across the bucket: `master` is the trunk, `feat-x` the change
  under work, `origin` the remote. A guide that needs an older line names it `release/1.1`.
- A revset or flag a guide relies on is verified against the upstream reference before it ships, and
  the reference page is linked from the guide's `## Reference` section.

## Maintenance Notes

- A new workflow gets a row in `README.md` under `## Doing`; a new companion gets one under
  `## Understanding`.
- Rules stated in the root `AGENTS.md` or in `_docs/specs/` are applied here, not restated.

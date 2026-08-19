# AGENTS

The root author-instructions file for this project. It states what to load, what to update, and what
never to touch, and it stays inside the 100-line budget so it can be read in full on every session.

## Documentation

- Load the specs of the domains you touch before acting: `_docs/specs/SPEC-<domain>.md`.
- Do not load the decision log unless someone asks why a rule exists.
- When a spec and a decision record disagree, follow the spec and leave the record alone.
- When you change current behavior, update the owning spec in the same change.
- Cite a rule by its ID, `<domain-slug>:<rule-slug>`, in commits, reviews, and comments.
- Add a decision record only when the choice is cross-cutting, expensive to reverse, constraining, or
  rejects an alternative someone will propose again.
- Never rename or delete a merged decision record, and never edit one to describe the present.
- Keep exploratory material in `.draft/`; promotion is a rewrite into the owning zone.
- Report documentation changes by ownership: which spec changed, which rule IDs, which hooks passed.

## Budgets

Every budget stated as a count is gated in `pre-commit-additions.yaml`, so none of them is advice.

| Artifact                         | Budget                                               |
| -------------------------------- | ---------------------------------------------------- |
| Root author-instructions file    | 100 lines                                            |
| Subtree author-instructions file | 150 lines                                            |
| Spec                             | 300 lines excluding the TOC; TOC generated above 100 |
| Decision record                  | 350 words                                            |
| Chapter                          | 200 lines                                            |
| Catalog                          | 300 lines                                            |

A document that wants more room splits. The number is never raised to admit it.

## Known issues

- Record an external-system bug in `_docs/reference/known-issues/` as `KI-<slug>.md`, and name that
  case id from the suppression that works around it.

# Template — Documentation maintenance section

Copy the block below into the project's root author-instructions file. It tells an agent what to load,
what to update, and what never to touch. State local exceptions inline rather than restating this
framework; the shelf owns the rules and this section points at them.

```markdown
## Documentation

- Load the specs of the domains you touch before acting: `<root>/specs/SPEC-<domain>.md`.
- Do not load the decision log unless someone asks why a rule exists.
- When a spec and a decision record disagree, follow the spec and leave the record alone.
- When you change current behavior, update the owning spec in the same change.
- Cite a rule by its ID, `<domain-slug>:<rule-slug>`, in commits, reviews, and comments.
- Add a decision record only when the choice is cross-cutting, expensive to reverse, constraining, or
  rejects an alternative someone will propose again.
- Never rename or delete a merged decision record, and never edit one to describe the present.
- Keep exploratory material in `.draft/`; promotion is a rewrite into the owning zone.
- Report documentation changes by ownership: which spec changed, which rule IDs, which hooks passed.
```

Two habits keep this section short. Write each rule once and link rather than restate it, and push
anything that binds only one subtree into that subtree's own author-instructions file.

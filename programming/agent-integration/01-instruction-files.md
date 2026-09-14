# 01 — Instruction Files

The instruction file at a project's root is the governance layer's artifact. Every session loads it
before the agent reads the request, so it is the one place a project states what holds for all work
and routes to everything else.

## What belongs in it

Conventions that bind every task: the stack, the commands that build and test, the commit and
request shape, and the boundary on what the agent does without being asked. Pointers to the deeper
documents, never the documents themselves. A rule that binds one workflow belongs to that workflow's
skill, and a fact that outlives the task belongs to the documentation the file points at.

The test is whether a rule changes what the agent does on work it has not seen yet. A convention
does. A procedure does not, and it goes to the playbook.

## Routing beats restating

Every sentence in the file is loaded in every session, whether or not the task needs it. That is the
budget: a file that states what three documents already say has spent context on a fact the agent
could have fetched when a step called for it.

The file's own size is what keeps it read. A long instruction file is skimmed, and a skimmed rule is
no rule. Keep it to what a reader can hold at once, state each convention in one sentence, and let
the pointer carry the rest.

## The file is hand-authored

The root instruction file is rules and routing, written by the people who own the project. It
carries judgment, and judgment has no generator.

A per-directory digest is a second shape with a different job: a summary of what a subtree holds,
generated from that subtree's own documents, so an agent working in one area loads that area's
context without the whole tree's. A digest is regenerated rather than edited, and it carries
frontmatter naming what it digests, when it was last synced, and how large it is. It states no rule
of its own, because a rule stated in a generated file is a rule nobody can trace to a source.

Conflating the two is the common mistake. A project that treats its root file as generated loses the
place where its conventions live. A project that hand-edits a digest has written a rule into a file
the next generation overwrites.

## What a directory of documents needs anyway

An instruction file routes, and routing needs destinations that explain themselves. Every directory
carries a `README.md` that says what the area is for and what belongs in it, routing by meaning
rather than listing what the filesystem already holds. See
[project-bootstrap/02 — Governance and docs](../project-bootstrap/02-governance-and-docs.md) for
that convention and the decision scaffold beside it.

## Vendor differences

Which filename an agent reads, what loads eagerly against lazily, how an import inside the file
resolves, and where the user-scoped copy sits are all vendor facts, and they change. Each vendor
shelf carries them, dated, sourced from that vendor's own documentation.

One consequence holds whatever the vendor: a file loaded into every session is paid for in every
session, so the cost of a rule is its length times the number of sessions that never needed it.

For the loading facts, see [tools/claude-code](../../tools/claude-code/memory-file-loading.md).

## See also

- [00 — The four layers](./00-model.md) — why governance is a layer and what it never carries.
- [03 — Skills](./03-skills.md) — where a procedure goes instead.

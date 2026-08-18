# The filesystem owns its own state, and no document indexes it

## Context and Problem Statement

This repository indexed its own directories: a `## Source map` table in the root `AGENTS.md` and in
47 area digests, and a `source-files:` frontmatter list in every digest. Each is an `ls` result under
version control, correct until the next add or rename, kept alive by a regenerate-on-add ritual. The
ritual does not hold: audited at deletion time, the tables named a dozen files that are not there. A
checked-in map naming a file that does not exist is the failure ADR-executable-artifacts-in-the-library already names — an
unenforced reference reads as verified, and a stale one is worse than none.

## Considered Options

- Keep the maps and enforce regeneration with a hook.
- Keep the maps but require every row to carry a purpose sentence.
- Delete the maps, and bar the shape rather than the syntax.

## Decision Outcome

Chosen option: delete the maps and bar the shape. Generating the listing correctly would only
automate a fact the filesystem already owns, and the purpose-sentence rule is what licensed these
tables in the first place.

Barred is the enumeration kept because the directory exists: a table of contents of the tree, a
topic-to-filename table, a `source-files` list. Naming files in prose is normal, and a list, table,
or full tree stays wherever its entries carry their own payload — what a directory reserves, what a
file specifies, its scope — justified like any other content. The test: strip the paths out and read
what is left. A layout a reader creates in their own project is unaffected.

`programming/docs-design/00-foundations.md` owns the rule and the root `AGENTS.md` states it. Two
`pygrep` hooks gate the two shapes that are unambiguous in regex; the rest is judgment.

## Consequences

- Good: digests stop shipping a stale inventory and shrink to the knowledge they map.
- Bad: 48 digests and the digest standard change in one pass, and the boundary is a judgment call
  rather than a mechanical rule.

## Status

Accepted.

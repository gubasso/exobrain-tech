# ADR-0020: spec-driven-docs is the documentation method

## Context and Problem Statement

`programming/docs-design/` states its rules as prose an author is trusted to apply, and most of them
no command can see. A coding agent given that shelf resolves its ambiguities arbitrarily, and a
reviewer cannot tell an obeyed rule from an ignored one. The repository needs a method whose rules
carry identifiers a citation can name and commands a gate can run, and it needs one method, not two.

## Considered Options

- Keep docs-design and add gates to it.
- Adopt spec-driven-docs and delete docs-design in the same change.
- Adopt spec-driven-docs, and delete docs-design once its uncovered subjects have a home.

## Decision Outcome

Chosen option: adopt spec-driven-docs, and delete docs-design once its uncovered subjects have a home
— the new shelf covers ownership, placement, decision records, agent context, drafts, reference
maintenance, markdown mechanics and the review gate, and it gates them, so every pointer that names a
method now names it. Five subjects have no owner there: guide shape, procedure artifacts, comment
discipline, subsystem pages, and the shapes of runbooks, diagnostics and case studies. Deleting the
shelf that holds them would destroy knowledge two ADRs and the root author-instructions file depend
on, and porting them in the same change would bury a method swap inside a content migration.

Retrofitting gates onto docs-design was the alternative to the whole shelf, and it is the larger job:
the rules would have to acquire identifiers, verification commands and a fixed spec shape first,
which is the new shelf.

## Consequences

- Good: every rule the method states carries an identifier and a command, and citations are greppable.
- Good: the worked example runs under `just test-spec-shelf`, so the gate snippets are proven.
- Bad: two shelves state documentation rules until the five subjects move, and a reader can arrive at
  the wrong one.
- Bad: the root author-instructions file points at both, which is one pointer more than a reader
  should need.

## Status

Accepted. Supersedes [ADR-0001](./ADR-0001-documentation-governance.md).

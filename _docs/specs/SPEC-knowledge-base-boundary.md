# Knowledge Base Boundary Specification

## Purpose

What may enter this repository and what may not: the split between the library and its metadata, the
split between public knowledge and private material, and the rule that keeps the repository from
adopting a method it does not teach. It binds placement and dependency, not shape — how a document is
written belongs to `SPEC-docs-format.md`.

## Requirements

### `knowledge-base-boundary:no-knowledge-is-external` — No load-bearing knowledge is external

Knowledge this repository depends on MUST be held in-repo, and a tool it runs MUST resolve to one
pinned revision. An external reference is a citation for further reading, never a source of truth.

#### Scenario: A gate reads a schema that lives in another project

- GIVEN a hook validating this repository's own files against a schema fetched from elsewhere
- WHEN that project renames the file or changes what the schema says
- THEN the gate changes meaning with no commit here to explain it, so the schema was knowledge and
  belongs in-repo

The boundary is knowledge, not location. A public tool input pinned in `flake.lock` is a tool of the
same class as the formatter beside it, because the lock names the revision every gate runs until
someone moves it deliberately; an unpinned input is not, because it can change under the repository
(ADR-a-flake-pinned-tool-input-is-a-tool-dependency).

Verify: reviewer confirms the change adds no unpinned input, and no external document that is read
rather than cited

### `knowledge-base-boundary:no-method-is-named` — The repository names no planning method

Outside the decision log, this repository MUST NOT name a planning, project-management, or workflow
method as a dependency, a tool, a schema source, or a link target.

#### Scenario: A docs-root directory serves an adjacent domain

- GIVEN `_docs/plan/`, holding what this repository is building next
- WHEN a method is adopted to shape it
- THEN the directory stays and the method stays unnamed here, because naming it makes every consumer
  of this repository inherit the choice

Verify: reviewer confirms the change names no method

### `knowledge-base-boundary:a-retired-name-does-not-return` — A retired name does not return

A name listed in `.hooks/unnamed-methods.txt` MUST NOT appear outside the decision log.

#### Scenario: A removed dependency is reintroduced by a later change

- GIVEN a method removed from the flake, the hooks, and the prose
- WHEN a later change reintroduces the name in a comment or a link
- THEN the commit fails, because the list is what stops the removal from being undone one file at a
  time

Verify: `pre-commit run no-named-method --all-files`

### `knowledge-base-boundary:private-material-stays-in-the-vault` — Private material stays in the vault

Equipment identity, security posture, recovery material, credentials, and personal workflows MUST
live in `exobrain-tech-vault`, not here.

#### Scenario: A guide needs a real host to be useful

- GIVEN a runbook for a machine in the author's own fleet
- WHEN it would name that host, its addresses, or its keys
- THEN the public half keeps the technique and the private half keeps the identity

Verify: reviewer confirms the change names no host, address, credential, or recovery path

### `knowledge-base-boundary:a-knowledge-article-is-not-metadata` — Knowledge never lives in `_docs/`

A knowledge article MUST live in the bucket that owns its subject, never under `_docs/`.

#### Scenario: A document explains a tool this repository also uses

- GIVEN a page about a formatter the repository runs
- WHEN it teaches the formatter rather than stating how this repository is governed
- THEN it belongs to the owning bucket, and `_docs/` cross-links to it

Verify: reviewer applies the placement test — is this about how the KB works, or is it knowledge the
library serves

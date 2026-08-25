# Knowledge Base Boundary Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`knowledge-base-boundary:no-knowledge-is-external` — Current operation is self-contained](#knowledge-base-boundaryno-knowledge-is-external--current-operation-is-self-contained)
    - [Scenario: A gate reads a schema that lives in another project](#scenario-a-gate-reads-a-schema-that-lives-in-another-project)
  - [`knowledge-base-boundary:the-canon-is-named-once` — The upstream canon is named once](#knowledge-base-boundarythe-canon-is-named-once--the-upstream-canon-is-named-once)
    - [Scenario: A chapter cites the canon passage that argues the same rule](#scenario-a-chapter-cites-the-canon-passage-that-argues-the-same-rule)
  - [`knowledge-base-boundary:a-tool-input-is-pinned` — A tool input resolves to one pinned revision](#knowledge-base-boundarya-tool-input-is-pinned--a-tool-input-resolves-to-one-pinned-revision)
    - [Scenario: An input is added without a lock entry](#scenario-an-input-is-added-without-a-lock-entry)
  - [`knowledge-base-boundary:no-method-is-named` — The repository names no planning method](#knowledge-base-boundaryno-method-is-named--the-repository-names-no-planning-method)
    - [Scenario: A docs-root directory serves an adjacent domain](#scenario-a-docs-root-directory-serves-an-adjacent-domain)
  - [`knowledge-base-boundary:a-retired-name-does-not-return` — A retired name does not return](#knowledge-base-boundarya-retired-name-does-not-return--a-retired-name-does-not-return)
    - [Scenario: A removed dependency is reintroduced by a later change](#scenario-a-removed-dependency-is-reintroduced-by-a-later-change)
  - [`knowledge-base-boundary:private-material-stays-in-the-vault` — Private material stays in the vault](#knowledge-base-boundaryprivate-material-stays-in-the-vault--private-material-stays-in-the-vault)
    - [Scenario: A guide needs a real host to be useful](#scenario-a-guide-needs-a-real-host-to-be-useful)
  - [`knowledge-base-boundary:a-knowledge-article-is-not-metadata` — Knowledge never lives in `_docs/`](#knowledge-base-boundarya-knowledge-article-is-not-metadata--knowledge-never-lives-in-_docs)
    - [Scenario: A document explains a tool this repository also uses](#scenario-a-document-explains-a-tool-this-repository-also-uses)

<!--TOC-->

## Purpose

What may enter this repository and what may not: the split between the library and its metadata, the
split between public knowledge and private material, and the rule that keeps the repository from
adopting a method it does not teach. It binds placement and dependency, not shape — how a document is
written belongs to `SPEC-docs-format.md`.

## Requirements

### `knowledge-base-boundary:no-knowledge-is-external` — Current operation is self-contained

Understanding, linting, formatting, testing, and building this checkout MUST require no file outside
it. Upstream provenance and upgrade availability MAY be external because current operation does not
exercise either one.

#### Scenario: A gate reads a schema that lives in another project

- GIVEN a hook validating this repository's own files
- WHEN the upstream canon checkout is unavailable
- THEN the vendored schema, hooks, specs, configuration, and verifier keep every current operation
  available

The boundary is current operation. A public tool input pinned in `flake.lock` is local operational
state because the lock fixes the revision every gate runs. Canon provenance is metadata for upgrade,
not an input to routine verification (ADR-source-the-documentation-canon-upstream).

Verify: `pre-commit run spec-driven-docs-verify --all-files`

### `knowledge-base-boundary:the-canon-is-named-once` — The upstream canon is named once

Where the upstream documentation canon is named, this repository MUST name it only in the root
`AGENTS.md`, as a project URL carrying no version.

#### Scenario: A chapter cites the canon passage that argues the same rule

- GIVEN a library page restating a rule the canon also states
- WHEN it links that canon passage at a pinned tag
- THEN the commit fails, because the version now has a second home and a bucket has taken a
  dependency it does not own (ADR-the-upstream-canon-is-named-once)

The decision log and the vendored payload are exempt: a record states its own moment, and the
payload is canon-written provenance. The version implemented lives in
`.spec-driven-docs/manifest.json`, which the verifier reads and an upgrade rewrites.

Verify: `pre-commit run canon-named-once --all-files`

### `knowledge-base-boundary:a-tool-input-is-pinned` — A tool input resolves to one pinned revision

Every tool input this repository runs MUST resolve to one revision recorded in `flake.lock`.

#### Scenario: An input is added without a lock entry

- GIVEN a flake input added for a new gate
- WHEN the lock is not updated alongside it
- THEN the input can change under the repository with no commit here to explain it, which is the
  dependency the pin exists to remove (ADR-a-flake-pinned-tool-input-is-a-tool-dependency)

Verify: `jq -e 'del(.nodes.root) | [.nodes[] | .locked.rev] | all(type == "string")' flake.lock`

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

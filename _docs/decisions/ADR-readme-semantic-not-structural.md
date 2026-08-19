# README.md defines domain semantics, not disk structure

## Context and Problem Statement

A `README.md` is the human-facing index for a directory, and the tempting way to write one is to
mirror the filesystem: an ASCII tree, or a bullet per child. The filesystem is the source of truth
for what exists and it drifts fast, so a mirrored README misdescribes the tree the moment a file
moves, and the copy must be hand-maintained forever.

## Considered Options

- README defines the area's organization, semantics, and domain, linking a file only where the link
  carries a semantic role — chosen.
- README mirrors the tree or enumerates every child — rejected: a lower-fidelity copy of `ls` that
  goes stale on every add, rename, and delete.
- README carries no file or directory reference at all — rejected: it also removes the curated
  pointer to a landmark entry, which is navigation by meaning.

## Decision Outcome

Chosen option: a README states what kind of content the directory reserves, how the area is
organized, and what its parts mean.

- Prohibited: ASCII directory trees and `ls`-style enumerations that mirror the filesystem.
- Allowed when justified: a link to a file or directory whose role this README defines. The test is
  whether the link exists because of that responsibility, or because `ls` would show it.
- Exempt: a table-of-contents block between generator markers, which lists the page's own headings
  and belongs to the generator. In-page heading ToCs only, never a hand-written sibling listing.
- Exemption withdrawn: an `AGENTS.md` digest is not exempt. A digest maps knowledge, not files.

## Consequences

- Good: a README stops going stale on every add, rename, and delete, and navigation by meaning
  survives a refactor.
- Bad: existing READMEs carrying trees must be rewritten, and the justified-link line is a judgment
  call rather than a mechanical rule.

## Status

Accepted. Operative rule lives in the repo's `AGENTS.md`, which also reads the older "README = index
file" wording as a semantic index.

Amended by [ADR-filesystem-owns-disk-state](./ADR-filesystem-owns-disk-state.md) — the `AGENTS.md`
digest exemption is withdrawn.

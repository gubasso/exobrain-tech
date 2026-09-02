# Master is the trunk

## Context and Problem Statement

This repository had one branch, `develop`, inherited from a two-branch model it never ran: nothing
was ever integrated into a separate release line. The library taught that model too, so the docs and
the repository agreed only by accident. A branching convention that binds contributors, gates, and
every example in two buckets needs to be one choice, stated once.

## Considered Options

- `one trunk named master` — chosen.
- `keep develop as the default branch` — rejected: it names an integration line for a repository
  that integrates nowhere else, and the second permanent branch it implies never existed.
- `rename to main and keep the two-branch shape` — rejected: it changes the name and leaves the
  model, which is the half that costs something.

## Decision Outcome

Chosen option: `one trunk named master` — `master` is the only permanent branch and the repository
default. Work reaches it through a short-lived branch and one squash-merged pull request, so one
request is one commit and the history stays linear. No second long-lived branch is kept; an older
line, if one is ever needed, is a just-in-time `release/<major>.<minor>` branch that takes changes
by cherry-pick and is deleted once its tags pin its commits.

The library states the model as knowledge in `workflows/trunk-based-development.md`, naming no
external project, per `knowledge-base-boundary:no-method-is-named` and ADR-self-containment. The git
and jj buckets carry the commands and are written against it.

## Consequences

- Good: the repository and the knowledge it serves run one model, so an example is also an
  instruction.
- Good: the trunk is the only thing to keep releasable, and there is no integration branch to
  reconcile.
- Bad: every git and jj example moved at once, and the jj transcripts had to be re-derived rather
  than renamed, because a squash merge and a merge commit leave a repo in different states.
- Bad: `develop` survives as a GitHub redirect, so an old link still resolves and hides the change
  from anyone following one.

## Status

Accepted

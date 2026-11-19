# Commit messages follow the 50/72 convention

## Context and Problem Statement

The common hook set runs committed on commit-msg, and its committed.toml capped the header and each body line at 72 characters with no softer aim for the header. The operator ruled on 2026-11-19: "use Tim Pope's 50/72 convention". The operator added: "we will keep the convention from now on... we'll not rewrite anything... a rule that becomes valid since we land the new setup".

## Considered Options

- `50 as a soft aim for the header, 72 as a hard cap on every line` — chosen.
- `a hard cap of 50 on the header` — rejected: a scoped Conventional Commits header often needs more than 50 characters to name its area and its change.
- `rewrite the older commits to the new limits` — rejected: the operator ruled that the rule starts when a project lands the new setup.

## Decision Outcome

Chosen option: `50 as a soft aim for the header, 72 as a hard cap on every line` — `git log` then fits 80 columns after its 4-space indent.

- `committed.toml` keeps `line_length = 72` and `subject_length = 72`. committed cannot warn at 50, so the aim of 50 lives in its comment and in chapter 04.
- The body wraps at 72.
- The convention applies to the commits made after a project lands the new `committed.toml`, and nobody rewrites an older commit.

## Consequences

- Good: one ceiling for every line, and a header that reads well in a one-line log.
- Bad: no tool warns on a header between 51 and 72 characters, so the aim depends on the author.

## Status

Accepted

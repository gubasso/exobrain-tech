# 009 — Release Readiness

## Goal

A release fails early when its version and changelog disagree.

## Example

The check names the mismatch and the edit that repairs it.

```console
$ release-check
error: tag v2.4.0 does not match manifest 2.3.0
```

## Core

Gate a release on version and changelog consistency.

## In scope

- Manifest and tag agreement.
- A changelog entry for the version.

## Out of scope

- Publishing packages.

## Governed by

None.

## Amends

None.

## Acceptance

- A version mismatch fails — `release_check::version_match`.
- A missing changelog entry fails — `release_check::changelog`.

## Tasks

- [ ] Add both checks.

## Rabbit holes

- Registry state — escape: leave publication to separate work.

## Done when

Both negative cases fail with actionable messages.

## Revisions

None.

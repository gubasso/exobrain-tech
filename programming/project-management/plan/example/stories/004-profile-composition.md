# 004 — Profile Composition

## Goal

A user can combine a base profile with one environment override.

## Example

The composed view shows the override without losing base values.

```console
$ profile show base+staging
region=eu-west-1
log_level=debug
```

## Core

Compose one base and one override deterministically.

## In scope

- Scalar override precedence.
- Missing-key inheritance.

## Out of scope

- Three-way composition.

## Governed by

None.

## Amends

None.

## Acceptance

- Overrides win and untouched values survive — `profile::compose`.

## Tasks

- [x] Define precedence.
- [ ] Complete the composed view.

## Rabbit holes

- Recursive structures — escape: reject them for this story.

## Done when

The named test passes unskipped.

## Revisions

None.

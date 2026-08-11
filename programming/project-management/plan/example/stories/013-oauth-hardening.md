# 013 — OAuth Hardening

## Goal

A session rejects an identity token whose stable claim cannot be established.

## Example

The rejected token names the missing claim without echoing token contents.

```console
$ auth verify ambiguous.jwt
error: no stable subject claim
```

## Core

Reject ambiguous identity without exposing credentials.

## In scope

- Stable-claim validation.
- A redacted error.

## Out of scope

- Provider enrollment.

## Governed by

None.

## Amends

None.

## Acceptance

- An ambiguous token is rejected safely — `oauth::stable_subject`.

## Tasks

- [ ] Resolve Q-004.
- [ ] Implement the validation.

## Rabbit holes

- Provider-specific aliases — escape: require an explicit mapping.

## Done when

The question is closed and the named test passes.

## Revisions

None.

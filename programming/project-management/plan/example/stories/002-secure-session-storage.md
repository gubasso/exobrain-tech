# 002 — Secure Session Storage

## Goal

Sessions survive a process restart without exposing tokens in plaintext.

## Example

A restarted process can recover the session while the stored value remains opaque.

```console
$ sessionctl restart && sessionctl get demo
session demo restored
```

## Core

Persist encrypted session state and recover it after restart.

## In scope

- Encrypt session values at rest.
- Restore a session after restart.

## Out of scope

- Cross-region replication.

## Governed by

None.

## Amends

None.

## Acceptance

- A restart preserves the session — `session_storage::restart`.

## Tasks

- [x] Add the encrypted store.
- [x] Exercise restart recovery.

## Rabbit holes

- Key rotation — escape: record it as separate work.

## Done when

The named test passes unskipped.

## Revisions

None.

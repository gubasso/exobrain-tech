# 014 — A Session Survives Its Host

## Goal

A session outlives a restart, a stolen disk, and an expired provider token without a human
reissuing it.

## Example

Today a restart recovers the session but the stored value is readable and a rotated provider key
orphans it. At the end state both hold.

```console
$ sessionctl restart && sessionctl get demo
session demo restored

$ strings /var/lib/sessionctl/demo.state | grep -c token
0

$ sessionctl rotate-provider-key && sessionctl get demo
session demo restored
```

## Core

A recovered session is never readable at rest, and provider key rotation never invalidates one.

## Out of scope

- Cross-region replication of session state.
- Any change to the session identifier a caller already holds.

## Governed by

None.

## Amends

None.

## Done when

A restart, a disk inspection, and a provider key rotation each leave a working session, and the
storage format carries a version field so the next change can migrate it.

## Revisions

None.

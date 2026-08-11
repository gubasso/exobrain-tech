# 006 — Proxy Guide

## Goal

An operator can configure the supported proxy path without reconstructing the flags.

## Example

The guide shows the smallest successful invocation.

```console
$ service --proxy http://proxy.internal:8080 health
ok
```

## Core

Document the supported proxy flag and one verification command.

## In scope

- HTTP proxy configuration.
- A health check.

## Out of scope

- Proxy deployment.

## Governed by

None.

## Amends

None.

## Acceptance

- The documented command reaches health — `docs::proxy_example`.

## Tasks

- [ ] Write and verify the guide.

## Rabbit holes

- Provider-specific authentication — escape: name it as unsupported.

## Done when

The example is verified and the docs check passes.

## Revisions

None.

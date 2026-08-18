# Keep BSD sysexits, hand-rolled, and treat the message as the primary surface

**Status:** Accepted **Date:** 2026-08-06

## Context

This spec has leaned on BSD sysexits since its first version:
[03 — Error handling](../03-error-handling.md) maps every `AppError` variant onto a constant, and
[`cli-design/02-error-messages.md`](../../../../programming/cli-design/02-error-messages.md) owns the
code table. Two facts had not been recorded against it. First, FreeBSD's own
[`sysexits(3)`](https://man.freebsd.org/cgi/man.cgi?query=sysexits&sektion=3) states the interface
"has been deprecated and is retained only for compatibility. Its use is discouraged," and that "the
choice of an appropriate exit value is often ambiguous." Second, none of the reference projects in
[chapter 10](../10-reference-projects.md) use sysexits values — ripgrep, ruff, and fd use small
domain code spaces; firecracker hand-rolls one including signal-derived codes `148..=157`;
cloud-hypervisor uses `0`/`1`. A reader following this spec deserves to know it is diverging from
observed practice, and why.

## Decision

Keep BSD sysexits as the default taxonomy. It gives pre-agreed, guessable semantics to the audience
this spec actually targets — shell scripts and coding agents — where a private numbering would have
to be learned per tool, and the deprecation is about the C header's portability rather than the
numbers' legibility. Record the caveat in chapter 03 rather than leaving it implicit. Do not adopt
the [`sysexits`](https://docs.rs/sysexits/) crate: its enum admits only `0` and `64..=78`, with no
arbitrary-`u8` variant, so any program owning a code outside that range splits exit-code ownership
between the crate and a local type. Hand-roll one enum. Where the budget is limited, spend it on the
rendered diagnostic before the taxonomy: the code is worth less than the message.

## Consequences

- [03 — Error handling](../03-error-handling.md) gains the deprecation note and the crate rejection
  under `## BSD sysexits cheat sheet`.
- [`cli-design/02-error-messages.md`](../../../../programming/cli-design/02-error-messages.md) gains
  the same caveat at the language-agnostic table it owns.
- A program that needs codes outside `0` and `64..=78` (a `--strict` mode's `1`, a wrapped child's
  `128+S`) extends its own enum rather than mixing two sources.
- Downside: the `EX_*` constants are maintained by hand in every project, and this spec now knowingly
  recommends an interface its upstream discourages.

## Alternatives considered

- **Drop sysexits for a small `0`/`1`/`2` space.** Rejected: throws away legibility for the script
  and agent consumers this spec exists to serve.
- **Adopt the `sysexits` crate for the `64..=78` subset.** Rejected: structurally cannot represent
  owned codes, so the taxonomy would live in two places.

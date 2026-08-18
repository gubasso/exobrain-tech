# Use anstyle and anstream for terminal colour

**Status:** Accepted **Date:** 2026-08-06

## Context

Human-facing Rust CLIs need one style vocabulary and one stream boundary that can pass, strip, or
adapt ANSI without each renderer making a separate terminal decision. The earlier dependency list
treated `anstream`, `owo-colors`, and `nu-ansi-term` as peers even though they own different layers.
Changing the default dependency requires a local ADR.

The [`rust-cli/anstyle`](https://github.com/rust-cli/anstyle) family is actively maintained by Ed
Page and the rust-cli maintainers and is dual MIT or Apache-2.0. `clap` exposes `anstyle` in its
public styles API and its default `color` feature brings `anstream`; a normal `clap` project
therefore adds direct dependency edges rather than a new subtree.

## Decision

New human-facing Rust CLIs use `anstyle` as their public style vocabulary and `anstream` as the sole
stream adapter, ANSI stripper, and legacy-Windows bridge. The application resolves one color choice
at startup, shares it with every renderer, and applies it at the actual destination stream.
`owo-colors` may provide formatting ergonomics above this layer only with its non-default
`supports-colors` feature disabled.

## Consequences

Style APIs remain interoperable and stream behavior has one owner. Existing `clap` projects usually
gain no new dependency subtree. Renderers must accept the carried choice instead of probing the
environment. Optional `owo-colors` syntax adds another direct dependency, and applications still
have to implement the canonical policy ladder because no crate supplies it completely.

## Alternatives considered

- Coequal style libraries were rejected because escape emitters do not own stream adaptation.
- `colored` was rejected: it has process-global state, probes stdout for every destination, and is
  MPL-2.0.
- `termcolor` remains viable for existing users but is maintenance-only; Cargo replaced it with
  `anstream` for a simpler equivalent layer.
- Multiple detectors were rejected because their environment semantics and target streams can
  disagree within one process.

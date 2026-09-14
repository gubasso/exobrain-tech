# 13 — Reference Implementations

Two tools implement parts of the pattern the landing chapters state. This chapter maps concepts to
their concrete names, so a reader moving between a portable chapter and a codebase knows which word
means which. Nothing here is normative: the portable chapter owns the concept, and a tool that does
it differently changes the concept by argument, never by example.

Both are x86_64 Linux tools. That is a fact about them, and not a requirement the pattern makes.

Read against each project's trunk on 2026-09-14.

## release-kit

<https://github.com/gubasso/release-kit>. The pattern is implemented end to end.

| Concept              | The name it carries there                                                        |
| -------------------- | -------------------------------------------------------------------------------- |
| Acquisition boundary | The project's own manager, with `rk self-depend` assisting rather than resolving |
| Projection           | `Projection::compute` over `ProjectionInput`, in `src/projection.rs`             |
| Stage                | `rk stage`, writing a `stage.json` receipt at schema `rk.stage/1`                |
| Receipt              | `.release-kit/manifest.json`, at receipt schema 7                                |
| Direct landing       | `rk init`, `rk upgrade`, and `rk adopt`, through `src/landing/apply.rs`          |
| Landing outcomes     | `created`, `replaced`, `matched`, `preserved`, `released`, and a collision       |
| Router skill         | `rk-setup`                                                                       |
| Documentation index  | `rk method` and `rk guide`                                                       |
| Verification         | `rk status --check`                                                              |
| Cleanup              | `rk stage clean`                                                                 |

The stage receipt names what the stage holds: the candidates with their destination, kind,
placement, and digest; the omissions with a reason; the collisions; the retired destinations; and
the reference material the stage exposes without landing.

What it does not carry is as much of the map as what it does. There is no cross-release bundle, no
stored executable plan, and no command that reconciles one release's output against another's. Both
renders call one projection, and the production render reads nothing the stage wrote.

## The documentation canon

The upstream canon this repository runs as a pinned tool, named and linked once in the root
`AGENTS.md` and versioned in `.spec-driven-docs/manifest.json`. A consumer, partly implemented.

| Concept              | The name it carries there                                  |
| -------------------- | ---------------------------------------------------------- |
| Acquisition boundary | The project's own manager, by a lock-pinned tool input     |
| Projection           | A payload embedded at compile time from the authored paths |
| Receipt              | `.spec-driven-docs/manifest.json`, at schema 3             |
| Ownership classes    | Managed, and adopted                                       |
| Direct landing       | `sdd init` and `sdd upgrade`                               |
| Documentation index  | `sdd docs`, then `sdd docs <topic>`                        |
| Verification         | `sdd verify`                                               |

Three concepts have no name there yet. There is no stage, so nothing renders a candidate for
comparison before production. There is no cleanup command, because there is nothing to clean. And
reconciliation of an adopted file runs through a stored plan, which is the shape the landing
chapters decline.

A plan to implement the pattern here exists outside this library. It is planned work rather than a
second proof, and this chapter will map it when the trunk carries it.

## See also

- [12 — Prior art](./12-prior-art.md) — the sources the concepts came from.
- [09 — Disposable staging and direct landing](./09-disposable-staging-and-direct-landing.md) — the
  lifecycle these names implement.

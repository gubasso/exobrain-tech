---
digest-of: languages/c/project-bootstrap-spec
last-synced: 2026-07-10
token-estimate: 750
---

# AGENTS

## Scope

C binding of the general `programming/project-bootstrap/` shelf: the once-per-project C setup
that takes an empty repo to a scaffolded, gated project ready for feature work. It **overlays** the
general spine (repo, license, governance, dev env, CI, security) and never restates it; it owns only
the C ecosystem choices and the two implementation-kind orderings (CLI, library). Packaging,
install/distribution, and versioning are out of scope — they are later-phase work.

## Key Points

- **Layout (by hand — no cargo-style scaffolder):** public headers under `include/<project>/` so
  consumers write `#include <project/foo.h>`; `.c` (and private `.h`) under `src/`; unit tests under
  `tests/`; top-level build file alongside.
- **Build system:** pick one mainstream generator, do not hand-roll Makefiles. **CMake** is the
  de-facto default (broadest tooling/IDE support; `CMAKE_C_STANDARD`); **Meson** is the simpler,
  fast-Ninja alternative (`c_std=c17`). Both export `compile_commands.json` for clang-tidy/clangd.
- **Compiler & standard:** gcc or clang, kept compiler-agnostic and tested with both in CI when
  feasible; clang is required for clang-tidy's richest checks and the sanitizers. Pick an explicit
  standard (`c11`/`c17`; `c99` only for legacy) and enforce it in the build file. Build with
  `-Wall -Wextra -Wpedantic` from day one, `-Werror` in CI.
- **Toolchain hosting:** a Nix devShell hosts compiler, build system, and tooling (gcc/clang,
  cmake/meson, ninja, clang-format, clang-tidy, cppcheck) so local and CI share one toolchain;
  per-project setup is owned by `nix/02-per-project-devshell`, this binding only declares which
  tools belong.
- **Quality gates (each wired explicitly — no bundled toolbox):** `clang-format` (committed
  `.clang-format`, e.g. `BasedOnStyle: LLVM`; enforce with `--dry-run --Werror`); `clang-tidy`
  (reads `compile_commands.json` + committed `.clang-tidy`) plus `cppcheck` (independent analyzer,
  use both); ASan/UBSan build variant (`-fsanitize=address,undefined`; CMake option or Meson
  `b_sanitize`, run full suite under sanitizers in CI); a unit-test framework — Unity, CMocka, or
  Criterion — wired to CTest or `meson test`.
- **CLI kind:** entry point `src/main.c` + build-system executable target; arg parsing via
  `getopt`/`getopt_long` (simple) or `argp`/vendored `argparse` (richer subcommands); consistent
  exit codes (`0` success, non-zero per class; `sysexits.h` a baseline); errors to `stderr`, results
  to `stdout`, single error-reporting helper.
- **Library kind:** public header API under `include/<project>/` with include guards/`#pragma once`
  and `extern "C"`; shared and/or static build target (decide which you ship); hidden visibility
  (`-fvisibility=hidden`) + export macro; install headers + library plus a discovery file
  (`pkg-config` `.pc` and/or CMake package config); plan `soname`/version early, keep header changes
  additive within a major version.
- **Automation:** deterministic scaffolding delegates to cog helpers — `bootstrap-nix` (devShell +
  `.envrc`), `bootstrap-editorconfig` (formatter alignment), `bootstrap-precommit` (hook wiring),
  `bootstrap-taskrunner` (build/test/format recipes), `bootstrap-ci` (first CI workflow). The
  SoT-vs-cog contract lives in general `07-automation-with-cog.md` — the runbook owns the _what_,
  cog the _how_.

## Maintenance Notes

- General spine: `../../../programming/project-bootstrap/` (general runbook, `04-quality-gates.md`,
  `06-security-baseline.md`, `07-automation-with-cog.md`). Toolchain hosting:
  `../../../tools/nix/02-per-project-devshell.md`.
- `daemon-project.md` and other implementation kinds are declared followups; add them when they land.
- Re-verify build-system, compiler, and gate-tool defaults against upstream on a cadence when
  regenerating.
- No conflicts among the current source files.
  </content>
  </invoke>

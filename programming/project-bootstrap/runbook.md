# Runbook — bootstrap a new project (any language)

The ordered, **once-per-project** manual steps to take a project from an empty repo to a
fully-scaffolded, quality-gated baseline ready for feature work. Each step links to the chapter that
explains the _why_; this page is only the _what_ and _in what order_.

Language- and implementation-kind specifics are **overlays** — after the general steps here, jump to
your [`project-bootstrap-spec/`](../../languages/rust/project-bootstrap-spec/README.md) binding.
Everyday work after setup is not here; this is the one-time scaffold.

## Prerequisites

- A version-controlled host with Git installed.
- An account on the target forge (GitHub / GitLab).
- A decision on **language** and **implementation-kind** — see the
  [three-layer model](./README.md#the-three-layer-model).

## Steps

1. **Create the repo and set up the trunk.** Create an empty repo on the forge and clone it; make
   `master` the default branch and the only permanent one, and have the forge delete a branch when
   its merge lands. → [00 — Bootstrap model](./00-bootstrap-model.md),
   [trunk-based development](../../workflows/trunk-based-development.md).

2. **Lay the repository foundation:** add `.gitignore`, a `LICENSE` (SPDX identifier), and a
   `README` skeleton at the repo root. →
   [01 — Repository foundation](./01-repository-foundation.md).

3. **Seed governance docs:** `CLAUDE.md` (agent instructions), the `AGENTS.md` convention, and an
   ADR scaffold for decisions. → [02 — Governance & docs](./02-governance-and-docs.md) and
   [agent-integration — Instruction files](../agent-integration/instruction-files.md).

4. **Set up the local dev environment:** a Nix devShell + `.envrc` (direnv) for a reproducible
   toolchain, and an `.editorconfig` for cross-editor consistency. Each dependency has one owner, the
   most specific file that declares it natively, and the host supplies only what the project does
   not own. →
   [03 — Local dev environment](./03-local-dev-environment.md).

5. **Wire the quality gates:** a formatter, a linter, git hooks, and a task runner (a single entry
   point for common recipes). The commit stage runs the fast checks, the push stage runs the costly
   ones, and together they run every check of CI. → [04 — Quality gates](./04-quality-gates.md).

6. **Stand up CI:** a first CI workflow that repeats both hook stages in the devshell on every push
   and pull request, and a list of the checks that only CI can run. → [05 — CI](./05-ci.md).

7. **Establish the security baseline:** secrets hygiene, dependency review/audit, and the OpenSSF
   Scorecard checklist. → [06 — Security baseline](./06-security-baseline.md).

8. **Apply language + implementation-kind overlays.** Jump to your language binding and follow its
   runbook, then its implementation-kind file. →
   [`project-bootstrap-spec/`](../../languages/rust/project-bootstrap-spec/README.md) (Rust
   reference binding).

## Reference

- [00 — Bootstrap model](./00-bootstrap-model.md) · [05 — CI](./05-ci.md)

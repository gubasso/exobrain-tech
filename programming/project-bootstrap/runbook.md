# Runbook: bootstrap a new project in any language

This runbook lists the manual steps that run once per project. They take a project from an empty
repository to a scaffolded baseline that the quality gates guard, ready for feature work. Each step
links to the chapter that explains why the step exists. This page states only what to do and in
what order.

Overlays add the specifics of a language and of an implementation kind. After the general steps
here, follow your [`project-bootstrap-spec/`](../../languages/rust/project-bootstrap-spec/README.md)
binding. This page covers the one-time scaffold, not the everyday work after setup.

## Prerequisites

- A version-controlled host with Git installed.
- An account on the target forge, GitHub or GitLab.
- A chosen language and implementation kind. The
  [three-layer model](./README.md#the-three-layer-model) explains both.

## Steps

1. Create the repository and set up the trunk. Create an empty repository on the forge. Clone it.
   Make `master` the default branch and the only permanent branch. Set the forge to delete a branch
   when its merge lands. See [00 — Bootstrap model](./00-bootstrap-model.md) and
   [trunk-based development](../../workflows/trunk-based-development.md).

2. Lay the repository foundation. Add these files at the repository root:

   - a `.gitignore`
   - a `LICENSE` with its SPDX identifier
   - a `README` skeleton

   See [01 — Repository foundation](./01-repository-foundation.md).

3. Seed the governance docs. Add these parts:

   - a `CLAUDE.md` with the agent instructions
   - the `AGENTS.md` convention
   - an ADR scaffold for decisions

   See [02 — Governance & docs](./02-governance-and-docs.md) and
   [agent-integration — Instruction files](../agent-integration/instruction-files.md).

4. Set up the local dev environment. Do these four things:

   - Add a Nix devShell and an `.envrc` for direnv, so the toolchain is reproducible.
   - Add an `.editorconfig`, so every editor formats files the same way.
   - Declare each dependency in its one owner, the most specific file that declares it natively.
   - List in `host-requirements.toml` the tools that the host supplies. The host supplies only
     what the project does not own.

   See [03 — Local dev environment](./03-local-dev-environment.md).

5. Wire the quality gates. Add these tools:

   - a formatter
   - a linter
   - git hooks
   - a task runner, the single entry point for common recipes

   Run the fast checks at the commit stage. Run the costly checks at the push stage. Make the two
   stages together run every check of CI. See [04 — Quality gates](./04-quality-gates.md).

6. Stand up CI. Add a first CI workflow. Make it repeat both hook stages in the devshell on every
   push and pull request. List the checks that only CI can run. See [05 — CI](./05-ci.md).

7. Establish the security baseline. Set up these three parts:

   - secrets hygiene
   - dependency review and audit
   - the OpenSSF Scorecard checklist

   See [06 — Security baseline](./06-security-baseline.md).

8. Apply the overlays of the language and the implementation kind. Open your language binding.
   Follow its runbook. Then follow its implementation-kind file. See
   [`project-bootstrap-spec/`](../../languages/rust/project-bootstrap-spec/README.md), the Rust
   reference binding.

## Reference

- [00 — Bootstrap model](./00-bootstrap-model.md)
- [05 — CI](./05-ci.md)

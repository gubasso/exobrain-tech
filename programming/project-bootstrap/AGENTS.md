---
digest-of: programming/project-bootstrap
last-synced: 2026-11-12
token-estimate: 1100
---

# AGENTS

## Scope

This tree covers the setup that runs once per project, in any language. It takes an empty
repository to a scaffolded baseline that the quality gates guard, ready for feature work. The setup
covers these parts:

- repository creation
- foundations
- governance docs
- the dev environment
- quality gates
- CI
- a security baseline

This tree is the source of truth for what a new project must have and in what order. The
`project-bootstrap-spec/` binding of each language overlays it with concrete tooling.

## Key Points

- Setup descends three layers, and each layer owns a disjoint set of facts:
  - The general layer is this tree, the spine that every language shares.
  - The language layer is `languages/<lang>/project-bootstrap-spec/`. It holds the ecosystem
    choices that overlay the spine and never restate it.
  - The implementation-kind layer is the `<kind>.md` files. They order the bootstrap of a CLI, a
    library, or a service, and they delegate the detailed how to existing specs.
- Each fact has one owner and one source of truth. Each step lives in exactly one place. Other
  layers link to it and never duplicate it. A document references a tool that owns its own shelf
  and does not copy it.
- Bootstrap runs once, when the project is created. It goes from the repository to the foundations
  to the gates. It ends at a scaffolded baseline that the quality gates guard, ready for feature
  work.
- The runbook is the spine. `runbook.md` lists what to do in 8 ordered steps. The chapters explain
  why each step exists. The language and kind bindings are overlays.
- Chapters 00 to 06 cover these topics:
  - 00 bootstrap model: the once-per-project phase and the three-layer model.
  - 01 repository foundation: `.gitignore`, the SPDX `LICENSE`, and the `README` skeleton.
  - 02 governance and docs: `CLAUDE.md`, the `AGENTS.md` convention, the MADR-minimal ADR
    scaffold, and the README as an index.
  - 03 local dev environment: the Nix devShell and `.envrc`, the owner of each dependency, the host
    requirements, and `.editorconfig`.
  - 04 quality gates: the formatter, the linter, git hooks in two stages, the hook exception
    classes, generated files, and the task runner.
  - 05 CI: the two hook stages repeated in the devshell, and the CI-only list.
  - 06 security baseline: secrets hygiene, dependency audit, and the OpenSSF Scorecard.
- Each dependency has one owner, the most specific file that declares it natively. The owners run
  in this order:
  1. The manifest of the stack: `pyproject.toml` with `uv.lock`, a PEP 723 block, `Cargo.toml`,
     or `package.json`.
  2. `.pre-commit-config.yaml`, for each hook tool that the hook runner can build.
  3. The devshell, for the toolchains, the task runner, and what no owner above can declare.
  4. The host, for the base tools, the git hook runner prek, the ELF loader, and the credentialed
     tools.
- The hook configuration owns a tool that only a hook uses, a linter or a type checker included.
  No devshell holds a hook runner. A flake holds no language library. Python uses uv. A hook that
  keeps `language: system` carries an `# owner:` comment with one of five classes.
- A project uses a host tool by name and lists it once in `host-requirements.toml`. An entry
  carries a Cargo version requirement where the project needs one, or `"*"` for a tool that checks
  its own version. A call site runs `exobrain host-check <tools>` where it needs them:
  - once at the top of a `justfile`
  - at the top of a recipe or script that alone uses a tool
  - in a dev target of a `Makefile`
- Nothing checks on shell entry, and nothing falls back. The README links the file. A shipped
  application checks its runtime tools in its own code.
- The commit stage runs the fast checks, and the push stage runs the costly ones. Together they run
  every CI check, and the push stage does not rerun the commit stage. CI repeats both stages in the
  devshell. A CI-only check needs a reason other than a long runtime. The project lists it with the
  report-only and scheduled jobs.
- A generator runs natively, and the project configures it only as its tool documents. Every hook
  that rewrites or judges a generated file excludes it in the tool's own configuration. A lock file
  and a release changelog are generated files. The reading hooks keep the file: the parser, the
  secret scan, and the link check. A file that a person edits after generation belongs to the
  project, and so does a marked region.

## Maintenance Notes

- This digest is generated. Regenerate it whenever the knowledge of the shelf changes. Update
  `last-synced` and `token-estimate` when you regenerate it. Add no guidance that the sources lack.
- Language bindings and implementation-kind files live under `languages/<lang>/`. Maintain them
  apart from this general spine.
- The current source files do not conflict.

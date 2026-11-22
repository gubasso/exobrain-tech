# Hook Sets Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`hook-sets:reference-hook-sets-hold-their-evidence` — The reference hook sets hold their evidence](#hook-setsreference-hook-sets-hold-their-evidence--the-reference-hook-sets-hold-their-evidence)
    - [Scenario: A built-in judges less than its upstream hook](#scenario-a-built-in-judges-less-than-its-upstream-hook)
  - [`hook-sets:hook-revisions-follow-the-exobrain-tech-tag` — A project pins the hook revisions of its exobrain-tech tag](#hook-setshook-revisions-follow-the-exobrain-tech-tag--a-project-pins-the-hook-revisions-of-its-exobrain-tech-tag)
    - [Scenario: A project moves one revision alone](#scenario-a-project-moves-one-revision-alone)
  - [`hook-sets:a-hook-setup-starts-with-research` — A hook setup starts with research](#hook-setsa-hook-setup-starts-with-research--a-hook-setup-starts-with-research)
    - [Scenario: A built-in appears in a new prek release](#scenario-a-built-in-appears-in-a-new-prek-release)
- [Unenforced](#unenforced)

<!--TOC-->

## Purpose

These rules govern the reference hook sets of exobrain-tech, how a project that follows the canon adopts them and pins their revisions, and the research that starts each hook setup. `SPEC-hooks.md` owns the `exobrain hooks` verbs that carry the convention, and `SPEC-project-platform.md` owns the hook runner and the gate.

## Requirements

### `hook-sets:reference-hook-sets-hold-their-evidence` — The reference hook sets hold their evidence

The repository MUST hold one reference prek configuration for each stack in `programming/project-bootstrap/hook-sets/`: `common`, `rust`, `python`, `bash`, and `nix`. Each set MUST be a complete configuration that `prek validate-config` accepts. It gives each hook its `stages`, `args`, `files`, and `exclude`, and a comment above each hook records the mistakes that shaped it. The sets MUST hold these properties:

- A repository with a `rev` appears in one set only, so two sets never expect two revisions of it.
- A set leaves out each repository that a block of release-kit or sdd renders, and the exobrain-tech entry, whose `rev` selects the sets.
- `evidence/<set>.yaml` records, for each repository of the set, the hook ids that its upstream manifest publishes at the pinned `rev`, and the set takes only those ids.
- `common` takes prek's built-in hooks in place of `pre-commit/pre-commit-hooks` and the check-json5 repositories, with the arguments that keep the upstream behavior.

The maintenance runs `prek update -c <set>.yaml`, keeps each `rev` at a released version, and refreshes the evidence with `.hooks/hook-sets-evidence.sh`, the one step that reads the network. The gate reads only the files of the checkout. The chapter that states the convention is `programming/project-bootstrap/04-quality-gates.md`, "Reference hook sets".

#### Scenario: A built-in judges less than its upstream hook

- GIVEN the common set with a `check-yaml` built-in that has no argument
- WHEN the gate runs it on a YAML file with an unknown tag
- THEN the built-in passes a file that the upstream hook refuses, so the gate fails until the set gives it `--disallow-unknown-tags`

Verify: `prek run hook-sets --all-files`

### `hook-sets:hook-revisions-follow-the-exobrain-tech-tag` — A project pins the hook revisions of its exobrain-tech tag

A project that follows this canon MUST run prek as its only hook runner. It MUST pin each repository that it shares with a reference set at the `rev` that the set states at the exobrain-tech tag of the project. The rule binds only the `rev` of a repository that the project shares with a set, so a project adopts any part of a set and keeps every other field as its own. The `rev` of the exobrain-tech entry of the project selects the sets. prek reuses an environment only for the same repository, revision, language, and dependencies, so a revision that only one project pins is an install that no other project reuses. These parts carry the convention:

- The hook `hook-revisions` compares the `rev` of each repository in both the project configuration and its named sets, and fails on a difference. It reads only the files of the commit, and ignores every other field.
- `exobrain hooks sync --to <tag>` moves the exobrain-tech entry and every revision of the named sets in one commit, on the branch of the work that needs it.
- `exobrain hooks status`, which `.envrc` runs, reports a newer CLI or newer revisions, and exits 0.
- `exobrain store-guard --daily`, which `.envrc` runs, also runs `prek cache gc` at most once a week.
- The project lists `exobrain` in `host-requirements.toml` with the oldest version that carries `exobrain hooks check`, and takes it from the host.

The chapter that states the convention is `programming/project-bootstrap/04-quality-gates.md`, "Reference hook sets". The chapter that states the host tool is `programming/project-bootstrap/03-local-dev-environment.md`, "Host requirements".

#### Scenario: A project moves one revision alone

- GIVEN a project that pins exobrain-tech at a tag and names the common set
- WHEN a commit moves typos to a newer `rev` than the common set of that tag states
- THEN `hook-revisions` fails the commit, and the project moves typos with `exobrain hooks sync --to` and a newer tag

Verify: reviewer confirms that each repository the project shares with a named set carries the `rev` of that set at the exobrain-tech tag of the project

### `hook-sets:a-hook-setup-starts-with-research` — A hook setup starts with research

When a project sets up its hooks for the first time, adopts a new stack, or reviews its hooks, the setup MUST include research into more hooks, better alternatives, and new features beside the reference sets. The research reads upstream sources, such as the release notes of prek, the manifest of each hook repository, and the documentation of each tool. Each finding that would change a set, a chapter, or the skill `exobrain-hooks-setup` goes to exobrain-tech as a proposal. The setup of the project never changes the canon itself. The skill states the procedure, and its report records each finding with its source under "Research". The chapter that states the convention is `programming/project-bootstrap/04-quality-gates.md`, "Reference hook sets".

#### Scenario: A built-in appears in a new prek release

- GIVEN a project that adopts the common set, and a prek release with a built-in hook that no set holds
- WHEN the skill runs a hook review of the project
- THEN the report lists the built-in under "Research" with its source, and names it as a proposal for exobrain-tech

Verify: reviewer confirms that the report of the hook setup or review holds a "Research" section with a source for each finding

## Unenforced

No command decides these rules, or a command decides only part of each. A reviewer confirms the rest.

| Rule                                                    | Reviewer confirms                                                                                                              |
| ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `hook-sets:hook-revisions-follow-the-exobrain-tech-tag` | Each repository that the project shares with a named set carries the `rev` of that set at the exobrain-tech tag of the project |
| `hook-sets:a-hook-setup-starts-with-research`           | The report of the hook setup or review holds a "Research" section with a source for each finding                               |

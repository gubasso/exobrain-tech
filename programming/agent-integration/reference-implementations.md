# Reference Implementations

Two tools implement parts of the pattern the landing chapters state. This chapter maps concepts to their concrete names, so a reader moving between a portable chapter and a codebase knows which word means which. Nothing here is normative: the portable chapter owns the concept, and a tool that does it differently changes the concept by argument, never by example.

Both are x86_64 Linux tools. That is a fact about them, and not a requirement the pattern makes.

Each section names the reading it maps.

## release-kit

<https://github.com/gubasso/release-kit>. The pattern is implemented end to end. Read on 2026-11-16
at release `v0.10.1`, trunk commit `20ffd0a`.

| Concept              | The name it carries there                                                                     |
| -------------------- | --------------------------------------------------------------------------------------------- |
| Acquisition boundary | The project's own manager, with `rk self-depend` assisting rather than resolving              |
| Host install answer  | `[acquisition]` in `.release-kit/config.toml`, written by `rk self-depend add --host`         |
| Projection           | `Projection::compute` over `ProjectionInput`, in `src/projection.rs`                          |
| Classification       | `classification` in `rk status --json`                                                        |
| Stage                | `rk stage`, writing a `stage.json` receipt at schema `rk.stage/9`                             |
| Compare              | `rk stage compare`, with state, provenance, the landing's word, and `replaceable`             |
| Receipt              | `.release-kit/manifest.json`, at receipt schema 12                                            |
| Direct landing       | `rk init`, `rk upgrade`, and `rk adopt`, each taking `--stage` and `--replace`                |
| Landing outcomes     | `created`, `replaced`, `matched`, `preserved`, `drift`, `retired`, `dropped`, and `collision` |
| Migration            | `rk migrate`: inventory, archive, mark, status, and close                                     |
| Release state        | `rk release status`, at schema `rk.release-status/1`                                          |
| Router skills        | `rk-setup`, and `rk-release` with one reference file per task                                 |
| Documentation index  | `rk method` and `rk guide`                                                                    |
| Version gate         | Five classes in `src/landing/gate.rs`, and a refusal exits 73                                 |
| Project skills       | Each landing writes `rk-setup` and `rk-release` into `.claude/skills/` and `.agents/skills/`  |
| Verification         | `rk status --check`                                                                           |
| Cleanup              | `rk stage clean`                                                                              |

The stage receipt names what the stage holds: the candidates with their destination, kind, placement, and digest; the omissions with a reason; the collisions; the retired and the dropped destinations; and the reference material the stage exposes without landing.

It implements the rule of [Candidate artifacts and receipts](./candidate-artifacts-and-receipts.md#a-destination-the-candidate-stops-producing) with two outcomes. A whole rendered file that the release no longer produces, and whose bytes the receipt still vouches for, is `retired`, and the landing removes it. Every other such destination is `dropped`: it stays on disk, and the project owns it. `rk status --check` names an edited release workflow that stays.

What it does not carry is as much of the map as what it does. There is no cross-release bundle, no stored executable plan, and no command that reconciles one release's output against another's. Both renders call one projection, and the production render reads no byte the stage wrote, only the choices its receipt records.

Its migration moves another release tool's files rather than a document corpus. The archive holds the predecessor's configuration and workflows, the workspace snapshots the forge's protections and settings and a bundle of every branch a step deletes before any destructive step, and each predecessor file is replaced by the new tool, carried into a project-owned file, or dropped. The forge-side steps run after the repository-side branch merges.

The setup skill, `rk-setup`, fills the skeleton in [Setup skill skeleton](./setup-skill-skeleton.md) with ten task files, read at `v0.10.1`.

| Task             | What it serves                                                                             |
| ---------------- | ------------------------------------------------------------------------------------------ |
| `provision`      | a missing `rk`, a record newer than the binary, a pin behind the latest release, or no pin |
| `land`           | a first landing                                                                            |
| `migrate`        | another release tool's files, archived first                                               |
| `adopt`          | release-kit's own files present with no record                                             |
| `upgrade`        | a record older than the binary                                                             |
| `repair`         | drift in what the record owns                                                              |
| `configure`      | a change to the committed configuration, landed at the binary's version                    |
| `forge-settings` | the forge-side setup steps the setup report names unsatisfied                              |
| `publish`        | the first publish, where the setup proof reads absent or stale                             |
| `landing`        | the stage, compare, prepare, land, and prove pipeline the landing tasks share              |

Its gate adds a state the documentation canon has no use for, `adopt`, and an intent beside the canon's, the forge setup, because a release tool also configures the forge. A first landing runs `forge-settings`, `land`, and `publish` in that order, and the migration runs the forge-side tasks only after its branch merges.

The release skill's gate reads `rk release status --json` and picks `release`, `lines`, or `recover`, and it names starting, integrating, and pruning a branch as the project's own workflow, with `rk issue start`, `rk integrate`, and `rk worktree prune` as helpers it runs as no task.

## The documentation canon

The upstream canon this repository runs as a pinned tool, named and linked once in the root `AGENTS.md` and versioned in `.spec-driven-docs/manifest.json`. Read on 2026-11-16 at release `v0.18.1`, trunk commit `57fa78f`.

| Concept              | The name it carries there                                                           |
| -------------------- | ----------------------------------------------------------------------------------- |
| Acquisition boundary | The project's own manager, with `sdd self-depend` assisting                         |
| Host install answer  | `acquisition` in the declaration, written by `sdd self-depend add --host`           |
| Projection           | A payload embedded at compile time from the authored paths                          |
| Classification       | `classification` in `sdd status --json`                                             |
| Stage                | `sdd stage`, writing a `stage.json` receipt at schema `sdd.stage/1`                 |
| Compare              | `sdd stage compare`, with state, ownership, provenance, and `replaceable`           |
| Receipt              | `.spec-driven-docs/manifest.json`, at schema 3                                      |
| Ownership classes    | Managed, adopted, and a marked region in a project-owned file                       |
| Direct landing       | `sdd init --stage` and `sdd upgrade --stage`                                        |
| Named replace        | `sdd upgrade --replace`, saving each replaced file first                            |
| Environment and lint | `sdd env` and `sdd lint`, with `sdd lint prove`                                     |
| Migration            | `sdd migrate`: inventory, archive, mark, status, and close                          |
| Incremental landing  | `sdd init --incremental`, beside a corpus that stays, recording its debt            |
| Router skills        | `sdd-setup`, with one reference file per task, and `sdd-write-docs`                 |
| Documentation index  | `sdd docs`, then `sdd docs <topic>`                                                 |
| Version gate         | Five classes in `src/landing/gate.rs`, and a refusal exits 73                       |
| Project skills       | `sdd init` and `sdd upgrade` write `sdd-setup` and `sdd-write-docs` into both roots |
| Verification         | `sdd verify`                                                                        |
| Cleanup              | `sdd stage clean`                                                                   |

It departs from the rule of [Candidate artifacts and receipts](./candidate-artifacts-and-receipts.md#a-destination-the-candidate-stops-producing) in two ways. An edited managed file that the release no longer lands is a conflict rather than a dropped file. The upgrade refuses until `sdd upgrade --replace` names it, and then it saves the bytes and removes the file. The landing also removes a vouched file only under `.spec-driven-docs/`, `.claude/skills/`, and `.agents/skills/`. Elsewhere the file stays on disk and leaves the record.

Its migration moves a document corpus: the archive holds the old documents, and a group of decision records of the previous convention becomes one record whose considered options name each old decision. The skeleton in [Setup skill skeleton](./setup-skill-skeleton.md) and the verbs in [What the CLI owes a setup skill](./what-the-cli-owes-a-setup-skill.md) describe this implementation in generic terms.

## See also

- [Prior art](./prior-art.md) — the sources the concepts came from.
- [Disposable staging and direct landing](./disposable-staging-and-direct-landing.md) — the lifecycle these names implement.

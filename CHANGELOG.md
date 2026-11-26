# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.1](https://github.com/gubasso/exobrain-tech/compare/v2.0.0...v2.0.1) - 2026-11-26

### Fixed

- *(store-guard)* retry a gc that prek refuses as in use

## [2.0.0](https://github.com/gubasso/exobrain-tech/compare/v1.2.0...v2.0.0) - 2026-11-22

### Added

- *(cli)* [**breaking**] remove exobrain hooks status
- *(cli/skill)* add exobrain outdated in hook setup
- *(cli)* report a newer CLI and hook sets in outdated
- *(cli)* bound tag reads and walk pages as flake-edit
- *(cli)* warn when a tag-pinned flake input lags
- *(cli)* read lock roots and follows as nix does
- *(cli)* name the age limit in outdated help
- *(cli)* warn when a nixpkgs pin grows old
- *(cli/skill)* offer the hook sets and research hooks
- *(cli/skill)* declare the hook scopes when a project adopts the sets
- *(cli/skill)* give a hook that runs in one runtime a group, never a SKIP

### Fixed

- *(cli/skill)* keep conventional style for a capitalized commit subject
- *(cli/skill)* give projects the committed.toml of the common hook set

### Other

- *(cli)* require a named gap for each verb
- *(cli)* cover outdated roots, follows, and stubs

## [1.2.0](https://github.com/gubasso/exobrain-tech/compare/v1.1.0...v1.2.0) - 2026-11-18

### Added

- *(cli/store-guard)* warn daily about a build folder over 10 GB
- *(cli/skill)* serve the exobrain-hooks-setup agent skill
- *(cli/store-guard)* run prek cache gc once a week per prek store
- *(exobrain/hooks)* hold hook revisions to the sets of a tag

### Other

- *(cli/store-guard)* assert the prek gc race only on prek 0.5.7 to 0.5.11

## [1.1.0] - 2026-11-12

### Added

- *(exobrain/host-check)* check the host tools a project lists

## [1.0.0](https://github.com/gubasso/exobrain-tech/releases/tag/v1.0.0) - 2026-11-11

### Added

- *(exobrain)* add the exobrain CLI with a daily store guard

### Fixed

- *(exobrain)* read a gitfile pointer only from the first byte

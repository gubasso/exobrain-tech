# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.3.0](https://github.com/gubasso/exobrain-tech/compare/v1.2.0...v1.3.0) - 2026-11-20

### Added

- *(cli/skill)* declare the hook scopes when a project adopts the sets
- *(cli/skill)* give a hook that runs in one runtime a group, never a SKIP

### Fixed

- *(cli/skill)* keep conventional style for a capitalized commit subject
- *(cli/skill)* give projects the committed.toml of the common hook set

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

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.0](https://github.com/gubasso/exobrain-tech/compare/v1.1.0...v1.2.0) - 2026-11-18

### Added

- *(cli/store-guard)* warn daily about a build folder over 10 GB
- *(cli/skill)* serve the exobrain-hooks-setup agent skill
- *(cli/store-guard)* run prek cache gc once a week per prek store
- *(exobrain/hooks)* hold hook revisions to the sets of a tag

### Fixed

- *(hooks)* class the grep and sed hooks as base-tool

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

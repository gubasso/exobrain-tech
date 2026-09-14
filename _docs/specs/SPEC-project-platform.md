# Project Platform Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`project-platform:one-supported-system` — The flake exposes one system](#project-platformone-supported-system--the-flake-exposes-one-system)
    - [Scenario: A second system is added inside a nested attribute set](#scenario-a-second-system-is-added-inside-a-nested-attribute-set)
  - [`project-platform:every-system-has-a-native-check` — Every supported system has a native required check](#project-platformevery-system-has-a-native-check--every-supported-system-has-a-native-required-check)
    - [Scenario: A system is advertised with no runner behind it](#scenario-a-system-is-advertised-with-no-runner-behind-it)
  - [`project-platform:library-topics-stay-platform-neutral` — Library topics stay open](#project-platformlibrary-topics-stay-platform-neutral--library-topics-stay-open)
    - [Scenario: A portable chapter describes an access-control mode](#scenario-a-portable-chapter-describes-an-access-control-mode)

<!--TOC-->

## Purpose

Rules governing which systems this checkout's toolchain supports, and the boundary between that
choice and the knowledge the library publishes. The support cut binds development and verification
here. It does not narrow what a chapter may be about.

## Requirements

### `project-platform:one-supported-system` — The flake exposes one system

The flake MUST key every system-dependent output by `x86_64-linux` and by no other system.

#### Scenario: A second system is added inside a nested attribute set

- GIVEN a `devShells` set that binds `aarch64-linux` beside the supported system
- WHEN the check evaluates the flake's own outputs
- THEN it reports both keys and fails, because a nested set declares an output no text scan sees

Verify: `just test-one-system`

### `project-platform:every-system-has-a-native-check` — Every supported system has a native required check

The project MUST run a required check natively on each system it advertises, and MUST NOT advertise
a system no check exercises.

#### Scenario: A system is advertised with no runner behind it

- GIVEN a proposal to expose a second system from the flake
- WHEN no required workflow job runs on that system
- THEN the support claim is refused, because an untested claim is a promise the project does not keep

Verify: `grep -q 'runs-on: ubuntu-latest' .github/workflows/ci.yml`

### `project-platform:library-topics-stay-platform-neutral` — Library topics stay open

The author MUST NOT narrow a chapter's subject to this project's supported system, and MUST scope a
platform-specific claim to the implementation that proves it.

#### Scenario: A portable chapter describes an access-control mode

- GIVEN a rule that requires bounded access to a staged directory
- WHEN an implementation proves owner-only modes on Linux
- THEN the chapter states the portable property and attributes the mode to that implementation

Verify: reviewer confirms the chapter states no operating system as a universal requirement

# Dev environment features and the native way of each tool

A dev environment needs the same few features in every project, whatever tool provides them. This document names each feature, the native command of each tool for it, and each gap that no native tool covers. A project uses the native command of its own tool. Where the document names a gap, the `exobrain` CLI offers an opt-in helper for it. A project that does not run exobrain uses its native tools, or leaves the gap open.

Every verdict below comes from a scenario that was run on the date of its table, against the version that the table names. A native cell links the documentation of the tool at that version. The section [Scenarios](#scenarios) records what each tool did.

Legend: ✅ yes, works on the default path · ⚠️ partial, needs configuration or has a stated limit · ❌ no, was run and failed · ➖ n/a, meaningless for this subject · 🧪 unstable, experimental path · ❓ untested, not yet run.

The tables compare mise, asdf, devbox, a Nix flake with direnv, and prek. A flake row also names `flake-edit`, a tool that edits the inputs of a flake.

## Provide the project's tools on entry

The project's tools are on `PATH` when a developer enters the project folder.

Verified: 2026-11-22. mise 2026.10.17, asdf 0.20.2, devbox 0.22.0, Nix 2.34.8 with direnv 2.38.2, prek 0.5.10.

| Tool           | Native way                                                                                                         | What exobrain does |
| -------------- | ------------------------------------------------------------------------------------------------------------------ | ------------------ |
| mise           | [✅ yes, `mise activate` or `mise exec`](https://github.com/jdx/mise/blob/v2026.10.17/docs/cli/exec.md)            | Nothing. No gap.   |
| asdf           | [✅ yes, shims read `.tool-versions`](https://github.com/asdf-vm/asdf/blob/v0.20.2/docs/manage/versions.md)        | Nothing. No gap.   |
| devbox         | [✅ yes, `devbox shell` or `devbox run`](https://github.com/jetify-com/devbox/blob/0.22.0/internal/boxcli/run.go)  | Nothing. No gap.   |
| Nix and direnv | [✅ yes, `use flake` in `.envrc`](https://github.com/direnv/direnv/blob/v2.38.2/man/direnv-stdlib.1.md)            | Nothing. No gap.   |
| prek           | [✅ yes, `prek run` builds each hook environment](https://github.com/j178/prek/blob/v0.5.10/docs/reference/cli.md) | Nothing. No gap.   |

## Add a tool to a project

One command adds a tool to the project's definition of its environment.

Verified: 2026-11-22. mise 2026.10.17, asdf 0.20.2, devbox 0.22.0, Nix 2.34.8, flake-edit 0.4.0, prek 0.5.10.

| Tool   | Native way                                                                                                                                      | What exobrain does                                   |
| ------ | ----------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| mise   | [✅ yes, `mise use <tool>@<version>`](https://github.com/jdx/mise/blob/v2026.10.17/docs/cli/use.md)                                             | Nothing. No gap.                                     |
| asdf   | [✅ yes, `asdf set <tool> <version>`](https://github.com/asdf-vm/asdf/blob/v0.20.2/docs/manage/commands.md)                                     | Nothing. No gap.                                     |
| devbox | [✅ yes, `devbox add <package>@<version>`](https://github.com/jetify-com/devbox/blob/0.22.0/internal/boxcli/add.go)                             | Nothing. No gap.                                     |
| Nix    | [⚠️ partial, `flake-edit add` adds an input, and a devshell package is a hand edit](https://github.com/a-kenji/flake-edit/blob/v0.4.0/README.md) | Nothing. The hand edit is the native way of a flake. |
| prek   | ⚠️ partial, no command, and the author edits `.pre-commit-config.yaml`                                                                           | Nothing. The hand edit is the native way of prek.    |

## Check the host tools that a project needs

A project names the host tools that its recipes and hooks call, with a version requirement, and one command checks them on the host.

Verified: 2026-11-22. mise 2026.10.17, asdf 0.20.2, devbox 0.22.0, Nix 2.34.8, prek 0.5.10.

| Tool   | Native way                                                                                                                         | What exobrain does                                         |
| ------ | ---------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------- |
| mise   | ❌ no, `jq = "system"` runs the host binary with no version check, and mise marks it deprecated                                    | Gap: `exobrain host-check` reads `host-requirements.toml`. |
| asdf   | [❌ no, `system` runs the host binary with no version check](https://github.com/asdf-vm/asdf/blob/v0.20.2/docs/manage/versions.md) | Gap: `exobrain host-check` reads `host-requirements.toml`. |
| devbox | ❌ no, `devbox.json` declares only the packages that devbox installs                                                               | Gap: `exobrain host-check` reads `host-requirements.toml`. |
| Nix    | ❌ no, a devshell holds only the packages that it provides                                                                         | Gap: `exobrain host-check` reads `host-requirements.toml`. |
| prek   | ❌ no, a `language: system` hook calls the host tool with no check                                                                 | Gap: `exobrain host-check` reads `host-requirements.toml`. |

## Warn when a pinned tool has a newer release

A command reports a pinned tool whose upstream published a newer release than the pin.

Verified: 2026-11-22. mise 2026.10.17, asdf 0.20.2, devbox 0.22.0, Nix 2.34.8, flake-edit 0.4.0, prek 0.5.10.

| Tool   | Native way                                                                                                                                                       | What exobrain does                                                                                                                                   |
| ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| mise   | [✅ yes, `mise outdated --bump`](https://github.com/jdx/mise/blob/v2026.10.17/docs/cli/outdated.md)                                                              | Nothing. No gap.                                                                                                                                     |
| asdf   | [⚠️ partial, `asdf latest --all` names the newest release, not the pin beside it](https://github.com/asdf-vm/asdf/blob/v0.20.2/docs/manage/commands.md)           | Nothing.                                                                                                                                             |
| devbox | [⚠️ partial, `devbox list --outdated` reported nothing for an exact pin](https://github.com/jetify-com/devbox/blob/0.22.0/internal/boxcli/list.go)                | Nothing.                                                                                                                                             |
| Nix    | [⚠️ partial, `flake-edit update --diff` reports a newer tag on demand, and Nix itself has no report](https://github.com/a-kenji/flake-edit/blob/v0.4.0/README.md) | Gap: `exobrain outdated` warns on shell load when an input pinned at a version tag has a newer release, and reads each upstream at most once a week. |
| prek   | [✅ yes, `prek update --check`](https://github.com/j178/prek/blob/v0.5.10/docs/reference/cli.md)                                                                 | Nothing. No gap.                                                                                                                                     |

## Warn when a pin ages

A command warns when a pin is older than a limit, whether or not a newer release exists.

Verified: 2026-11-22. mise 2026.10.17, asdf 0.20.2, devbox 0.22.0, Nix 2.34.8, prek 0.5.10.

| Tool   | Native way                                                                                                                                              | What exobrain does                                                                 |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| mise   | ❌ no, `mise outdated` names versions and no date                                                                                                       | Nothing.                                                                           |
| asdf   | ❌ no, `.tool-versions` holds no date                                                                                                                   | Nothing.                                                                           |
| devbox | ❌ no, `devbox.lock` records `last_modified`, and no command warns on it                                                                                | Nothing.                                                                           |
| Nix    | [❌ no, `nix flake metadata` prints the date of each input, and warns on none](https://nix.dev/manual/nix/2.34/command-ref/new-cli/nix3-flake-metadata) | Gap: `exobrain outdated` warns when the nixpkgs pin of a lock is over 30 days old. |
| prek   | ❌ no, `--cooldown-days` judges the age of a new release, not of the pin                                                                                | Nothing.                                                                           |

## Move a pin

One command moves a pin to a newer release and updates the lock.

Verified: 2026-11-22. mise 2026.10.17, asdf 0.20.2, devbox 0.22.0, Nix 2.34.8, flake-edit 0.4.0, prek 0.5.10.

| Tool   | Native way                                                                                                                                                                   | What exobrain does |
| ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------ |
| mise   | [✅ yes, `mise upgrade --bump <tool>`](https://github.com/jdx/mise/blob/v2026.10.17/docs/cli/upgrade.md)                                                                     | Nothing. No gap.   |
| asdf   | [✅ yes, `asdf set <tool> latest`](https://github.com/asdf-vm/asdf/blob/v0.20.2/docs/manage/commands.md)                                                                     | Nothing. No gap.   |
| devbox | [⚠️ partial, `devbox update` moves within the version range, and an exact pin takes `devbox add`](https://github.com/jetify-com/devbox/blob/0.22.0/internal/boxcli/update.go) | Nothing.           |
| Nix    | [✅ yes, `flake-edit update <input>` moves a tag, and `nix flake update` moves a branch](https://github.com/a-kenji/flake-edit/blob/v0.4.0/README.md)                        | Nothing. No gap.   |
| prek   | [✅ yes, `prek update` moves each repository to its newest tag](https://github.com/j178/prek/blob/v0.5.10/docs/reference/cli.md)                                             | Nothing. No gap.   |

`nix flake update` keeps the tag of a tag input. `flake-edit update <input>` writes `flake.nix` first and then locks. If the lock step fails, it keeps the new `flake.nix`. Read `git diff flake.nix flake.lock` after the move. If the lock step failed, run `git restore flake.nix`.

## Hold hook revisions to a reference set

A command moves the hook revisions of a project to a set of revisions that one release tested together.

Verified: 2026-11-22. prek 0.5.10.

| Tool   | Native way                                                                                                                          | What exobrain does                                                                                                                                                                                                                       |
| ------ | ----------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| mise   | ➖ n/a, mise runs no git hooks                                                                                                      | Nothing.                                                                                                                                                                                                                                 |
| asdf   | ➖ n/a, asdf runs no git hooks                                                                                                      | Nothing.                                                                                                                                                                                                                                 |
| devbox | ➖ n/a, devbox runs no git hooks                                                                                                    | Nothing.                                                                                                                                                                                                                                 |
| Nix    | ➖ n/a, a flake runs no git hooks                                                                                                   | Nothing.                                                                                                                                                                                                                                 |
| prek   | [❌ no, `prek update` moves each repository to its own newest tag](https://github.com/j178/prek/blob/v0.5.10/docs/reference/cli.md) | Gap: `exobrain hooks sync --to <tag>` moves the adopted revisions, `exobrain hooks check` gates them, `exobrain outdated` warns about newer sets and a newer CLI, and `exobrain skill show exobrain-hooks-setup` prints the setup skill. |

The sets are an offer. A project adopts any part of them and keeps every other field as its own.

## Keep the Nix store and the caches small

A command frees the space that old tool versions, hook environments, and store copies hold.

Verified: 2026-11-22. mise 2026.10.17, asdf 0.20.2, devbox 0.22.0, Nix 2.34.8, prek 0.5.10.

| Tool   | Native way                                                                                                                                                | What exobrain does                                                        |
| ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| mise   | [✅ yes, `mise prune` removes unused versions](https://github.com/jdx/mise/blob/v2026.10.17/docs/cli/prune.md)                                            | Nothing. No gap.                                                          |
| asdf   | [⚠️ partial, `asdf uninstall <tool> <version>` removes one version at a time](https://github.com/asdf-vm/asdf/blob/v0.20.2/docs/manage/commands.md)        | Nothing.                                                                  |
| devbox | ➖ n/a, devbox keeps its packages in the Nix store                                                                                                        | Nothing.                                                                  |
| Nix    | [⚠️ partial, `nix-collect-garbage` frees dead paths, and names no copy of a working tree](https://nix.dev/manual/nix/2.34/command-ref/nix-collect-garbage) | Gap: `exobrain store-guard --daily` finds store copies of the checkout.   |
| prek   | [✅ yes, `prek cache gc`](https://github.com/j178/prek/blob/v0.5.10/docs/reference/cli.md)                                                                | Nothing. `exobrain store-guard --daily` runs `prek cache gc` once a week. |

## Scenarios

Each scenario ran on 2026-11-22 in a scratch folder, with `HOME` and the XDG folders pointed into it. One example tool serves every manager: `jq` pinned at 1.7.1, while 1.8.2 is the newest release. The flake pins nixpkgs on `nixos-unstable` and release-kit at the tag `v0.10.1`, while `v0.10.3` is the newest release. The prek configuration pins `pre-commit/pre-commit-hooks` at `v5.0.0`, while `v6.0.0` is the newest release.

### Tools on entry

Run the entry command of each tool, then `jq --version`.

- mise: `mise exec -- jq --version` printed `jq-1.7.1`.
- asdf: the shim in `$ASDF_DATA_DIR/shims` printed `jq-1.7.1`.
- devbox: `devbox run -- jq --version` printed `jq-1.7.1`.
- Nix and direnv: `.envrc` held `use flake`. `direnv exec . sh -c 'command -v jq'` printed a `/nix/store` path.
- prek: `prek run --all-files` built the environment of `check-yaml` and passed.

### Add a tool

- mise: `mise use jq@1.7.1` wrote `jq = "1.7.1"` to `mise.toml`.
- asdf: `asdf set jq 1.7.1` wrote `jq 1.7.1` to `.tool-versions`.
- devbox: `devbox add jq@1.7.1` wrote `jq@1.7.1` to `devbox.json`.
- Nix: `flake-edit --help` lists `add`, which adds a flake input. No command of Nix or flake-edit adds a package to a devshell.
- prek: `prek --help` lists no command that adds a hook.

### Host tools

Ask each tool to use the host `jq` and to judge its version.

- mise: `mise use jq@system` ran the host `jq-1.8.2` with no version check, and printed a deprecation warning.
- asdf: `asdf set jq system` ran the host `jq-1.8.2` with no version check.
- devbox, Nix, and prek: no field declares a host tool with a version requirement.

### Newer release

- mise: `mise outdated` printed "All tools are up to date" and pointed to `--bump`. `mise outdated --bump` named 1.8.2.
- asdf: `asdf latest --all` printed `jq 1.8.2 installed` while `.tool-versions` held 1.7.1, because 1.8.2 was also installed.
- devbox: `devbox list --outdated` printed "Your packages are up to date!" for `jq@1.7.1`. The case of a range pin behind its range is untested.
- Nix: `nix flake update` moved nixpkgs and kept release-kit at the commit of `v0.10.1`. `flake-edit --non-interactive --no-cache --diff --no-lock update release-kit` printed `Updated release-kit from v0.10.1 to v0.10.3.` and changed no file. Whether its cache answers with no network was not settled.
- prek: `prek update --check` printed "would update rev `v5.0.0` -> `v6.0.0`".

### Pin age

No tool printed a warning for a pin older than a limit. `nix flake metadata` printed the date of each input. `devbox.lock` held `last_modified` for `jq@1`.

### Pin move

- mise: `mise upgrade --bump jq` wrote `jq = "1.8.2"`.
- asdf: `asdf set jq latest` wrote `jq 1.8.2`.
- devbox: `devbox update jq` printed `Already up-to-date jq@1.7.1`. With `jq@1`, it resolved 1.8.2.
- Nix: `flake-edit --non-interactive --no-cache update release-kit` wrote the tag `v0.10.3` and printed `Updated flake.lock`. The lock then held `ref` `v0.10.3`.
- prek: `prek update` wrote `rev: v6.0.0`.

### Reference set

`prek update` moved `pre-commit-hooks` to `v6.0.0`, its own newest tag. Its options filter tags by pattern and by age, and take no list of tested revisions.

### Caches

- mise: `mise prune --dry-run` named the install and cache folders of `jq@1.7.1`.
- prek: `prek cache gc --dry-run` printed "Would remove 1 repo, 1 hook env, 1 cache entry".
- Nix: `nix flake metadata` showed the scratch flake as a `/nix/store/<hash>-source` copy. `nix-collect-garbage --dry-run` printed no path as a copy of a working tree.
- asdf: `asdf --help` lists `uninstall` for one version, and no command that removes every unused version.

## Re-verification

Run each scenario again when a tool in a table ships a release that touches its feature, and then update the date. A date that changes with no new run is false.

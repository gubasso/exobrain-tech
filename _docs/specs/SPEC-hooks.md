# Hooks Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`hooks:check-compares-shared-revisions` — hooks check compares the revisions that a config shares with its sets](#hookscheck-compares-shared-revisions--hooks-check-compares-the-revisions-that-a-config-shares-with-its-sets)
    - [Scenario: A project pins typos behind its set](#scenario-a-project-pins-typos-behind-its-set)
  - [`hooks:check-exit-codes` — hooks check exits by what it found](#hookscheck-exit-codes--hooks-check-exits-by-what-it-found)
    - [Scenario: A project names a set that the tag lacks](#scenario-a-project-names-a-set-that-the-tag-lacks)
  - [`hooks:skip-rendered-blocks` — hooks check and sync skip every rendered block](#hooksskip-rendered-blocks--hooks-check-and-sync-skip-every-rendered-block)
    - [Scenario: The release-kit block pins a repository of a set](#scenario-the-release-kit-block-pins-a-repository-of-a-set)
  - [`hooks:sync-rewrites-only-revisions` — hooks sync rewrites only the revisions of the sets](#hookssync-rewrites-only-revisions--hooks-sync-rewrites-only-the-revisions-of-the-sets)
    - [Scenario: A project moves to a new tag](#scenario-a-project-moves-to-a-new-tag)
  - [`hooks:sync-writes-all-or-nothing` — hooks sync writes the whole result or nothing](#hookssync-writes-all-or-nothing--hooks-sync-writes-the-whole-result-or-nothing)
    - [Scenario: The download of one set fails](#scenario-the-download-of-one-set-fails)
  - [`hooks:status-reads-the-network-once-in-30-days` — hooks status reads the latest release at most once in 30 days](#hooksstatus-reads-the-network-once-in-30-days--hooks-status-reads-the-latest-release-at-most-once-in-30-days)
    - [Scenario: direnv loads a project twice in one day](#scenario-direnv-loads-a-project-twice-in-one-day)
  - [`hooks:status-reports-newer-releases` — hooks status reports a newer CLI and newer revisions](#hooksstatus-reports-newer-releases--hooks-status-reports-a-newer-cli-and-newer-revisions)
    - [Scenario: A newer release moves typos](#scenario-a-newer-release-moves-typos)
  - [`hooks:status-always-exits-0` — hooks status exits 0 in every case](#hooksstatus-always-exits-0--hooks-status-exits-0-in-every-case)
    - [Scenario: A laptop runs direnv offline](#scenario-a-laptop-runs-direnv-offline)
  - [`hooks:read-github-only-through-gh` — hooks sync and status reach GitHub only through gh](#hooksread-github-only-through-gh--hooks-sync-and-status-reach-github-only-through-gh)
    - [Scenario: CI runs the hooks](#scenario-ci-runs-the-hooks)

<!--TOC-->

## Purpose

These rules govern `exobrain hooks`, the verbs of the `exobrain` binary that hold the hook revisions of a project to the reference sets of its exobrain-tech tag. `project-platform:hook-revisions-follow-the-exobrain-tech-tag` owns the convention, and chapter 04 of `programming/project-bootstrap/` holds the sets. This spec owns how `hooks check`, `hooks sync`, and `hooks status` read a config, compare it, rewrite it, and reach the network. `SPEC-cli.md` owns the rest of the binary, its output streams included.

## Requirements

### `hooks:check-compares-shared-revisions` — hooks check compares the revisions that a config shares with its sets

When `exobrain hooks check --sets-dir <dir> --set <name>... <config>` runs, the checker MUST read `<dir>/<name>.yaml` for each named set and the project config, and read nothing else. For each repository URL that both the config and a named set hold, it MUST compare the `rev` of the config with the `rev` of the set. It compares URLs without a trailing `/` or `.git`, and reads no other field, so a project keeps its own `args`, `stages`, `files`, and `exclude`.

The checker skips these entries:

- the exobrain-tech entry, which holds the hook `hook-revisions` or names a repository `exobrain-tech`, because its `rev` selects the sets
- a `local`, `meta`, or `builtin` entry, which has no `rev`
- an entry that no named set holds

For each difference, the checker prints one line on stderr with the repository, the pinned revision, the set, the exobrain-tech tag of the config, the expected revision, and the line. After the differences, it prints `fix: exobrain hooks sync --to <tag>`.

The parse reads the block style that prek documents: a top-level `repos:` block list whose entries hold one-line `repo:` and `rev:` values, plain or quoted. A layout outside that style, such as a flow list or an escaped string, fails the parse with its line.

#### Scenario: A project pins typos behind its set

- GIVEN a config at exobrain-tech `v1.1.0` that pins typos at `v1.40.0` with its own `args`, and a `common` set that pins typos at `v1.52.2`
- WHEN `hook-revisions` runs `exobrain hooks check --sets-dir <clone>/programming/project-bootstrap/hook-sets --set common .pre-commit-config.yaml`
- THEN the checker names typos, `v1.40.0`, `common`, `v1.1.0`, and `v1.52.2`, and ignores the different `args`

Verify: `cargo nextest run -E 'test(hooks_check_compares_shared_revisions)'`

### `hooks:check-exit-codes` — hooks check exits by what it found

The checker MUST exit with one of these codes:

- 0 when every shared repository matches its set
- 1 when at least one `rev` differs
- 2 when a set file or the config cannot be read or parsed, when two named sets pin one repository at two revisions, or on a usage error

#### Scenario: A project names a set that the tag lacks

- GIVEN a tag whose set folder holds no `python.yaml`
- WHEN `hook-revisions` runs the check with `--set python`
- THEN the checker exits 2 and names the set

Verify: `cargo nextest run -E 'test(hooks_check_exit_codes)'`

### `hooks:skip-rendered-blocks` — hooks check and sync skip every rendered block

`hooks check` and `hooks sync` MUST skip every entry between a `# BEGIN release-kit` and a `# END release-kit` line, and between a `# BEGIN spec-driven-docs` and a `# END spec-driven-docs` line. `hooks sync` MUST NOT change a byte inside such a block. A BEGIN marker with no END marker, or an END marker with no BEGIN marker, fails the parse.

#### Scenario: The release-kit block pins a repository of a set

- GIVEN a config whose release-kit block pins cargo-machete at `v0.9.0`, and a `rust` set that pins `v0.10.1`
- WHEN the project runs `exobrain hooks check` and then `exobrain hooks sync`
- THEN the check passes, and the sync leaves the block as release-kit rendered it

Verify: `cargo nextest run -E 'test(hooks_skip_rendered_blocks)'`

### `hooks:sync-rewrites-only-revisions` — hooks sync rewrites only the revisions of the sets

When `exobrain hooks sync [--to <tag>] [<config>]` runs, the command MUST read the sets from the `args` of `hook-revisions` in the config, as `--set <name>` or `--set=<name>`. The config is `.pre-commit-config.yaml` by default. The command MUST fetch the set files of `<tag>`, or of the latest release when the call names no tag. It MUST rewrite these values and keep every other byte of the config, comments, quotes, and order included:

- the `rev` of the exobrain-tech entry, to `<tag>`, and never to a revision that a set states
- the `rev` of each repository of the named sets, to the revision of that set

The command prints one line on stderr for each changed value.

#### Scenario: A project moves to a new tag

- GIVEN a config at `v1.1.0` that pins typos at `'v1.40.0'  # pinned by hand`, and a tag `v1.2.0` whose `common` set pins `v1.52.2`
- WHEN the project runs `exobrain hooks sync --to v1.2.0`
- THEN the exobrain-tech `rev` reads `v1.2.0`, typos reads `'v1.52.2'  # pinned by hand`, and every other byte stays

Verify: `cargo nextest run -E 'test(hooks_sync_rewrites_only_revisions)'`

### `hooks:sync-writes-all-or-nothing` — hooks sync writes the whole result or nothing

`hooks sync` MUST fetch and check every input before its first write: the tag, every named set, and the config. It MUST write the result to a temporary file beside the config, then rename that file over the config. On any error, the command MUST leave the config byte for byte as it was, print the cause on stderr, and exit 1. It exits 0 when it writes the result, and when the config already pins the sets of the tag.

#### Scenario: The download of one set fails

- GIVEN a config that names the sets `common` and `python`, and a tag that holds no `python.yaml`
- WHEN the project runs `exobrain hooks sync --to v1.2.0`
- THEN the command exits 1, names the set `python`, and leaves the config unchanged

Verify: `cargo nextest run -E 'test(hooks_sync_writes_all_or_nothing)'`

### `hooks:status-reads-the-network-once-in-30-days` — hooks status reads the latest release at most once in 30 days

When `exobrain hooks status [--refresh] [<config>]` runs, the command MUST read the latest exobrain-tech release at most once in 30 days for each checkout, unless `--refresh` forces the read. The stamp is `${XDG_STATE_HOME:-$HOME/.local/state}/exobrain/hooks/<basename>-<digest16>.checked`, and it holds the time of the last read. The command writes the stamp before it reads, so a failed read waits 30 days too. It caches the latest tag, and the set files of each tag, under `exobrain/hooks/<owner>-<name>/`. A run between reads reports from that cache.

#### Scenario: direnv loads a project twice in one day

- GIVEN a checkout whose stamp is one day old
- WHEN `.envrc` runs `exobrain hooks status`
- THEN the command reads no network, and reports what the cache holds

Verify: `cargo nextest run -E 'test(hooks_status_reads_the_network_once_in_30_days)'`

### `hooks:status-reports-newer-releases` — hooks status reports a newer CLI and newer revisions

`hooks status` MUST print one line on stderr when the latest tag is a newer version than the running `exobrain`. When the latest tag is newer than the exobrain-tech `rev` of the config, it MUST print one line for each repository whose `rev` differs from the sets of the latest tag, then one line that names `exobrain hooks sync --to <latest>`. On the latest tag, it prints nothing about the project.

#### Scenario: A newer release moves typos

- GIVEN a config at `v1.1.0` that pins typos at `v1.40.0`, and a latest release `v1.2.0` whose `common` set pins `v1.52.2`
- WHEN `.envrc` runs `exobrain hooks status`
- THEN stderr names typos with both revisions, and names `exobrain hooks sync --to v1.2.0`

Verify: `cargo nextest run -E 'test(hooks_status_reports_newer_releases)'`

### `hooks:status-always-exits-0` — hooks status exits 0 in every case

`hooks status` MUST exit 0 in every case: with no network, with no `gh`, with no config, and with a config that it cannot parse. A failed read prints one line on stderr.

#### Scenario: A laptop runs direnv offline

- GIVEN a host with no network, whose stamp is 31 days old
- WHEN `.envrc` runs `exobrain hooks status`
- THEN the command prints one line about the failed read and exits 0

Verify: `cargo nextest run -E 'test(hooks_status_always_exits_0)'`

### `hooks:read-github-only-through-gh` — hooks sync and status reach GitHub only through gh

`hooks check` MUST NOT read the network. `hooks sync` and `hooks status` MUST read GitHub only through `gh api` on the host `PATH`, for the repository of the exobrain-tech entry of the config. With no such entry, `status` reads the repository that ships the binary. They read two endpoints:

- `repos/<owner>/<name>/releases/latest`, for the latest tag
- `repos/<owner>/<name>/contents/programming/project-bootstrap/hook-sets/<set>.yaml?ref=<tag>`, raw, for a set file

`gh` carries the credentials of the operator. When `gh` is missing, `sync` fails and `status` prints one line.

#### Scenario: CI runs the hooks

- GIVEN a CI job with no network access and no `gh`
- WHEN the job runs `hook-revisions`
- THEN `exobrain hooks check` passes or fails on the local files alone

Verify: `cargo nextest run -E 'test(hooks_read_github_only_through_gh)'`

# 04 — Quality gates

The checks that keep the codebase consistent and catch defects before they land. Each is language-agnostic as a _contract_; the concrete tool is named in the language binding.

## Formatter

A single, non-negotiable code formatter removes style debates and keeps diffs minimal. Run it in `--check` mode in CI so unformatted code cannot merge. The formatter is authoritative for code style; `.editorconfig` (chapter [03](./03-local-dev-environment.md)) covers non-code files.

## Linter

A linter catches likely bugs and anti-patterns the formatter does not. Configure it to **deny**, not just warn, on the lints you care about, and run it in CI. The language binding picks the tool (e.g. `clippy` for Rust) and the deny set.

## Git hooks

Git hooks run the checks locally, so a failure surfaces before it reaches CI. Declare them in `.pre-commit-config.yaml`, the format of [pre-commit](https://pre-commit.com/), and run them with [prek](https://prek.j178.dev/), a Rust runner of that format that the host supplies beside git ([03](./03-local-dev-environment.md#who-owns-each-dependency)). prek is the only runner. The configuration names the oldest runner it accepts with `minimum_prek_version`, and takes prek's `repo: builtin` hooks, which pre-commit cannot run. Two stages split the work:

- The commit stage runs the fast checks: the formatter, the linter, and the cheap file checks.
- The push stage runs the costly checks: the integration tests, the builds, and the networked checks, such as a link check or a vulnerability audit.

The two stages together run every check that CI runs, so a push that passed its hooks passes CI. The push stage does not rerun the commit stage. Install both hook types, for example `default_install_hook_types: [pre-commit, commit-msg, pre-push]`.

`default_stages` covers only a hook that declares no `stages:` of its own. The `stages:` of a remote hook's manifest wins over it: the manifests of typos and shfmt declare `pre-push` beside `pre-commit`, so those hooks run at push too unless the configuration gives each one an explicit `stages:`. A prek built-in copies the stages of its upstream hook, so the fixers `trailing-whitespace` and `end-of-file-fixer` declare `pre-push` too. Read the pinned manifest of each remote repository, and set `stages:` where it names a stage the hook does not belong to. A project that gates its integration on the manual stage keeps `manual` in both lists.

A project with a flake also carries `no-path-flake-reference`, the hook that refuses a `path:` flake reference to the working tree, because such a reference copies every ignored file into the store. The [Nix quality gates](../../languages/nix/project-bootstrap-spec/01-quality-gates.md#a-flake-reference-reads-the-git-tree) state the rule and the hook entry.

### Reference hook sets

exobrain-tech holds one reference set for each stack in [`hook-sets/`](./hook-sets/): `common`, `rust`, `python`, `bash`, and `nix`. Each set is a complete prek configuration. It gives each hook its `stages`, `args`, `files`, and `exclude`, and a comment above each hook records the mistakes and adjustments that shaped it. A project copies the sets of its stacks, in any combination, into its `.pre-commit-config.yaml`, then adapts, removes, or adds hooks.

The sets exist so that projects share their hook environments. prek reuses an environment only for the same repository, revision, language, and dependencies, so a revision that only one project pins is an install that no other project reuses. These rules hold the revisions together:

- A repository appears in one set only, so two sets never expect two revisions of it.
- No set holds a repository that a block of release-kit or sdd renders, because that tool owns its revision. The release-kit block pins `conventional-pre-commit` and `pre-commit-hooks`, and the sdd block pins `markdownlint-cli2`.
- The `rev` of the exobrain-tech entry of a project selects its sets, so no set states that revision. A set names the hooks of exobrain-tech in a comment.
- `common` takes prek's `repo: builtin` hooks in place of `pre-commit/pre-commit-hooks` and the check-json5 repositories. A built-in needs no clone, no environment, and no `rev`. A built-in can judge less than its upstream hook, and the set gives it the arguments that keep the upstream behavior: prek's `check-yaml` accepts an unknown tag unless it gets `--disallow-unknown-tags` ([built-in hooks](https://prek.j178.dev/0.5.7/reference/built-in-hooks/#check-yaml)).
- prek has no built-in for `forbid-submodules`. Its `forbid-new-submodules` ignores a submodule that already exists, so a project that refuses every submodule keeps the upstream hook.

The exobrain-tech entry of a project enables the hook `hook-revisions` and names its sets in `args`:

```yaml
- repo: https://github.com/<owner>/exobrain-tech
  rev: v1.2.0
  hooks:
    - id: no-path-flake-reference
    - id: hook-revisions
      args: [--set, common, --set, rust]
```

The hook compares the `rev` of each repository that appears both in the project configuration and in the named sets, at the tag that the entry pins, and fails on a difference. It reads no other field, so the project keeps its own `args`, `stages`, and `files`. The result depends only on the files of the commit, so two machines with different versions of the `exobrain` CLI agree, and CI agrees with the local hooks.

Four commands of the `exobrain` CLI carry the rest of the convention:

- `exobrain hooks sync --to <tag>` moves the exobrain-tech entry and every revision of the named sets in one commit. A project moves when it has work, on the branch of that work, so the drift across projects shrinks to the number of exobrain-tech releases in use.
- `exobrain hooks status`, which `.envrc` runs, reports a newer CLI or newer revisions than the project pins. It reads the latest exobrain-tech release at most once every 30 days, and always exits 0. A person or an agent reads it and decides when to move.
- `exobrain store-guard --daily`, which `.envrc` runs too, also runs `prek cache gc` at most once a week. prek then removes every clone and environment that no configuration on disk pins, so the environments of a tag go when the last project leaves it.
- `exobrain hooks check` is what `hook-revisions` calls, with the set files of the tag that prek cloned.

The maintenance of the sets runs `prek update -c <set>.yaml` for each set, keeps each `rev` at a released version, since `prek update` takes a pre-release tag too, and refreshes the hook ids of each upstream manifest in `hook-sets/evidence/` with `.hooks/hook-sets-evidence.sh`. The gate of exobrain-tech holds each set to that evidence offline, and runs each built-in on a fixture that its upstream hook refuses.

### Who builds a hook environment

A hook takes the environment that prek builds by default: pin its upstream repository with `rev`, and the hook declares its own language. A hook with `language: system`, `script`, or `unsupported`, or an upstream `-system` id, is an exception. A comment directly above it names its owner, its class, and its reason, in the form `# owner: <owner> (<class>) <reason>`. The owner is the file that declares the tool, as [03 — Who owns each dependency](./03-local-dev-environment.md#who-owns-each-dependency) says: `pyproject.toml`, `package.json`, `devshell`, or `host`. The six classes:

- `no-hook-language`: upstream publishes no hook in a language for which prek builds an environment, such as statix, lychee, or a script of the project.
- `project-environment`: the hook runs the project itself or its toolchain, so prek cannot own the tool: the test suite (pytest, `cargo test`), a part of the toolchain (`cargo clippy`, `cargo fmt`, tsc), or a script that imports the code of the project. A type checker or a linter takes this class only with a reason in the comment that is specific to the repository.
- `recorded-failure`: the managed hook fails on a clean machine of a supported host, even with the devshell fallback for hook setup. The comment names the error and the date.
- `outside-caller`: a caller outside prek needs the same tool and cannot reach it through `prek run`, such as a language server, or `nix fmt` for nixfmt.
- `base-tool`: the hook calls only the base tools of the host. The owner is `host`.
- `host-tool`: the hook calls a host tool other than a base tool, one that the project lists in `host-requirements.toml`, such as `hook-revisions` calling `exobrain`. The owner is `host`.

```yaml
- repo: local
  hooks:
    # owner: devshell (no-hook-language) statix has no hook in a language that prek builds.
    - id: statix
      name: statix
      entry: statix check
      language: system
      pass_filenames: false
    # owner: pyproject.toml (project-environment) pytest runs in the uv environment of the project.
    - id: pytest
      name: pytest
      entry: uv run --frozen pytest
      language: system
      pass_filenames: false
    # owner: devshell (recorded-failure) the managed hook fails with "<error>" on a clean host, YYYY-MM-DD.
    - id: example-tool
      name: example-tool
      entry: example-tool
      language: system
    # owner: devshell (outside-caller) nix fmt runs the nixpkgs nixfmt, so the hook runs the same one.
    - id: nixfmt
      name: nixfmt
      entry: nixfmt
      language: system
      types: [nix]
    # owner: host (base-tool) the guard calls only git and the shell.
    - id: no-merge-commit
      name: no merge commit
      entry: sh -c 'test -z "$(git rev-list --merges @{u}..HEAD)"'
      language: system
      pass_filenames: false
      stages: [pre-push]
```

A type checker or a linter that only a hook runs takes its upstream hook, even though it reads the code of the project: ruff from `astral-sh/ruff-pre-commit`, mypy from `pre-commit/mirrors-mypy` with the typed libraries in `additional_dependencies`, and pyright from `RobertCraigie/pyright-python`, which reads the venv that its own `pyrightconfig.json` names.

A hook inside a block that a tool renders, such as a release tool or a docs tool, carries no comment, because that tool owns the block.

### Generated files

A file that a tool generates follows that tool's native conventions. The project runs the generator as the tool documents it and configures it only through the options the tool documents, never to satisfy a project check. The hooks adapt to the generator, not the generator to the hooks:

- Every hook that rewrites or judges a generated file excludes it: a formatter, a fixer, a linter, a spell checker, and a test that judges prose or form.
- A hook that only reads the file keeps it: a syntax parser, a secret scan, a link check, and a drift check.
- Exclude the file in the tool's own configuration where the tool has one, so that a run outside the hooks (the task runner, an editor) excludes it too. The recipe per tool is in [best-practices/pre-commit](../best-practices/pre-commit.md#excluding-generated-files).

A file that a person edits after the tool writes it is not generated, and neither is a file in which the tool owns only a marked region: the project owns those files and checks them like any other. The classes match the tool's side in [agent-integration — Candidate artifacts and receipts](../agent-integration/candidate-artifacts-and-receipts.md).

The cases that come up:

- A lock file (`flake.lock`, `Cargo.lock`, `package-lock.json`, `uv.lock`, `renv.lock`) is generated. The whitespace fixers, the formatters, the spell checker, and editorconfig-checker exclude it.
- A changelog that a release tool writes, such as `CHANGELOG.md` from release-plz or git-cliff, is generated. The project excludes it from its Markdown formatter, its Markdown linter, and every prose gate. A content rule, such as naming an adopter's next step after a breaking change, goes into the commit body or the release request, never into a generator template.
- A check that judges the words of a generated file excludes it. A check that verifies the generator's input through its output keeps it.
- A workflow file that the forge runs with its secrets, such as a generated release workflow, keeps its security linters. A finding is accepted only in the linter's own configuration, never by an edit to the generated file.
- A generator that the project itself owns writes to the project's own conventions, and the project's drift check proves that no hook changed its output.
- A generated file that a person hardens after generation, and a template that the project writes for its generator (such as a git-cliff `cliff.toml`), belong to the project.

## Task runner

A task runner gives one entry point for common recipes — `fmt`, `lint`, `test`, `build` — so contributors and CI invoke the same commands. A `justfile` is a good default; an existing `Makefile` can be augmented in place. This is the single command surface the other gates hang off.

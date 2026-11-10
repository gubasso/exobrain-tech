# 04 — Quality gates

The checks that keep the codebase consistent and catch defects before they land. Each is
language-agnostic as a _contract_; the concrete tool is named in the language binding.

## Formatter

A single, non-negotiable code formatter removes style debates and keeps diffs minimal. Run it in
`--check` mode in CI so unformatted code cannot merge. The formatter is authoritative for code
style; `.editorconfig` (chapter [03](./03-local-dev-environment.md)) covers non-code files.

## Linter

A linter catches likely bugs and anti-patterns the formatter does not. Configure it to **deny**, not
just warn, on the lints you care about, and run it in CI. The language binding picks the tool (e.g.
`clippy` for Rust) and the deny set.

## Git hooks

Git hooks run the checks locally, so a failure surfaces before it reaches CI. Use a framework (e.g.
[pre-commit](https://pre-commit.com/)) so hooks are declarative and shared. Two stages split the
work:

- The commit stage runs the fast checks: the formatter, the linter, and the cheap file checks.
- The push stage runs the costly checks: the integration tests, the builds, and the networked
  checks, such as a link check or a vulnerability audit.

The two stages together run every check that CI runs, so a push that passed its hooks passes CI.
The push stage does not rerun the commit stage. Install both hook types, for example
`default_install_hook_types: [pre-commit, commit-msg, pre-push]`.

With pre-commit, `default_stages` covers only a hook that declares no `stages:` of its own. The
`stages:` of a remote hook's manifest wins over it: the hooks of `pre-commit-hooks` declare
`[pre-commit, pre-push, manual]`, so they run at push too unless the configuration gives each one an
explicit `stages:`. Read the pinned manifest of each remote repository, and set `stages:` where it
names a stage the hook does not belong to. A project that gates its integration on the manual stage
keeps `manual` in both lists.

### Who builds a hook environment

A hook takes the environment that pre-commit builds by default: pin its upstream repository with
`rev`, and the hook declares its own language. A hook with `language: system`, `script`, or
`unsupported`, or an upstream `-system` id, is an exception. A comment directly above it names its
owner, its class, and its reason, in the form `# owner: <owner> (<class>) <reason>`. The owner is
the file that declares the tool, as
[03 — Who owns each dependency](./03-local-dev-environment.md#who-owns-each-dependency) says:
`pyproject.toml`, `package.json`, `devshell`, or `host`. The five classes:

- `no-hook-language`: upstream publishes no hook in a language for which pre-commit builds an
  environment, such as statix, lychee, or a script of the project.
- `project-environment`: the tool must run in the environment of the project, such as pyright,
  pytest, `cargo clippy`, or tsc.
- `recorded-failure`: the managed hook fails on a clean machine of a supported host, even with the
  devshell fallback for hook setup. The comment names the error and the date.
- `outside-caller`: a caller outside pre-commit needs the same tool and cannot reach it through
  `pre-commit run`, such as a language server, or `nix fmt` for nixfmt.
- `base-tool`: the hook calls only the base tools of the host. The owner is `host`.

```yaml
- repo: local
  hooks:
    # owner: devshell (no-hook-language) statix has no hook in a language that pre-commit builds.
    - id: statix
      name: statix
      entry: statix check
      language: system
      pass_filenames: false
    # owner: pyproject.toml (project-environment) pytest runs in the uv environment of the project.
    - id: pytest
      name: pytest
      entry: uv run pytest
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

A hook inside a block that a tool renders, such as a release tool or a docs tool, carries no
comment, because that tool owns the block.

### Generated files

A file that a tool generates follows that tool's native conventions. The project runs the generator
as the tool documents it and configures it only through the options the tool documents, never to
satisfy a project check. The hooks adapt to the generator, not the generator to the hooks:

- Every hook that rewrites or judges a generated file excludes it: a formatter, a fixer, a linter, a
  spell checker, and a test that judges prose or form.
- A hook that only reads the file keeps it: a syntax parser, a secret scan, a link check, and a
  drift check.
- Exclude the file in the tool's own configuration where the tool has one, so that a run outside
  the hooks (the task runner, an editor) excludes it too. The recipe per tool is in
  [best-practices/pre-commit](../best-practices/pre-commit.md#excluding-generated-files).

A file that a person edits after the tool writes it is not generated, and neither is a file in which
the tool owns only a marked region: the project owns those files and checks them like any other.
The classes match the tool's side in
[agent-integration/08](../agent-integration/08-candidate-artifacts-and-receipts.md).

The cases that come up:

- A lock file (`flake.lock`, `Cargo.lock`, `package-lock.json`, `uv.lock`, `renv.lock`) is
  generated. The whitespace fixers, the formatters, the spell checker, and editorconfig-checker
  exclude it.
- A changelog that a release tool writes, such as `CHANGELOG.md` from release-plz or git-cliff, is
  generated. The project excludes it from its Markdown formatter, its Markdown linter, and every
  prose gate. A content rule, such as naming an adopter's next step after a breaking change, goes
  into the commit body or the release request, never into a generator template.
- A check that judges the words of a generated file excludes it. A check that verifies the
  generator's input through its output keeps it.
- A workflow file that the forge runs with its secrets, such as a generated release workflow, keeps
  its security linters. A finding is accepted only in the linter's own configuration, never by an
  edit to the generated file.
- A generator that the project itself owns writes to the project's own conventions, and the
  project's drift check proves that no hook changed its output.
- A generated file that a person hardens after generation, and a template that the project writes
  for its generator (such as a git-cliff `cliff.toml`), belong to the project.

## Task runner

A task runner gives one entry point for common recipes — `fmt`, `lint`, `test`, `build` — so
contributors and CI invoke the same commands. A `justfile` is a good default; an existing `Makefile`
can be augmented in place. This is the single command surface the other gates hang off.

## Automation

`bootstrap-lint` installs the hook framework and a starter hook set; `bootstrap-taskrunner`
scaffolds a `justfile` (or augments a `Makefile`). The gates above are the SoT; see
[07 — Automation with cog](./07-automation-with-cog.md).

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

## Task runner

A task runner gives one entry point for common recipes — `fmt`, `lint`, `test`, `build` — so
contributors and CI invoke the same commands. A `justfile` is a good default; an existing `Makefile`
can be augmented in place. This is the single command surface the other gates hang off.

## Automation

`bootstrap-precommit` installs the hook framework and a starter hook set; `bootstrap-taskrunner`
scaffolds a `justfile` (or augments a `Makefile`). The gates above are the SoT; see
[07 — Automation with cog](./07-automation-with-cog.md).

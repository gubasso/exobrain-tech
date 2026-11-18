# Task: sets

Write or merge `.pre-commit-config.yaml` from the reference sets of the tag, and swap the hooks of `pre-commit/pre-commit-hooks` and `check-json5` for the prek built-ins of the common set. `$EXOBRAIN_TECH/programming/project-bootstrap/04-quality-gates.md`, "Reference hook sets", states the convention, and the comment above each hook of a set gives its reason.

## Pre-flight

1. Read each set that the pre-flight of the skill picked, at the tag: `$EXOBRAIN_TECH/programming/project-bootstrap/hook-sets/<set>.yaml`.
2. Read the configuration of the project, and mark each block that release-kit or sdd renders. Nothing below changes a line inside one.
3. For each hook of the project, note its repository, its id, and each of its fields that differs from the same hook in a set. Note each hook of a set that the project lacks, and each hook of the project that no set holds.
4. Find the reason for each difference: the comment above the hook, then `git log -S '<value>' -- .pre-commit-config.yaml` and the commit that added it. Where no reason is on record, ask the operator: keep the value with a reason, or take the value of the set.

## Execution

1. Where the project has no configuration, copy the common set, then append the `repos` of each other set. Keep the top-level keys of the common set once.
2. Where the project has a configuration, merge one hook at a time, outside the rendered blocks:
   - A hook of a set that the project lacks: add it, with the comment of the set above it.
   - A hook that both hold: take the fields of the set, except each field that the project keeps for a reason. Write that reason in a comment directly above the hook, such as `# project: --maxkb=16384, because the repository tracks the 12 MB fixture of the importer.`
   - A hook that no set holds: keep it. A `language: system` hook carries the `# owner:` comment of chapter 04.
   - A hook of a set that the project does not need, such as `check-xml` in a project with no XML file: drop it where `check-useless-excludes` or the request asks for it, and name it in the report.
3. Take `minimum_prek_version`, `default_install_hook_types`, and `default_stages` from the common set. Keep a project addition, such as a `manual` stage, with its reason. A project without the `pre-merge-commit` gate of `rk integrate` drops that type from both lists, as the comment of the common set says.
4. Swap the built-ins. Run `prek run --all-files` and record the result. Then replace each hook of `https://github.com/pre-commit/pre-commit-hooks`, and each repository whose hook id is `check-json5`, with the `repo: builtin` hook of the same id from the common set, with its arguments. `check-json` becomes `check-json5` with the `types` and `types_or` of the set. Two cases keep the upstream hook:
   - `forbid-submodules`, which has no built-in. It stays at the `rev` of the release-kit block.
   - Each hook inside a rendered block.
5. Run `prek run --all-files` again, and compare the two runs. A built-in with the arguments of the set judges at least what its upstream hook judged, so a new failure is a real finding, such as a CRLF file under `--fix=lf` or an unknown YAML tag under `--disallow-unknown-tags`. Fix the file in its own commit, or report it.
6. Exclude the generated files from each hook that rewrites or judges them, in the configuration of the tool where it has one, as chapter 04 says under "Generated files". Trim each exclude pattern of a set to the files that the project holds, because `check-useless-excludes` refuses a pattern that matches nothing.
7. Run `prek validate-config`.
8. Commit as `build(hooks): adopt the <sets> reference hook sets of exobrain-tech`. Where the swap of step 4 is a change of its own, commit it first as `build(hooks): run the prek built-ins in place of pre-commit-hooks`.

## Post-flight

1. `prek validate-config` passes.
2. No hook outside a rendered block names `pre-commit/pre-commit-hooks` or a `check-json5` repository, except `forbid-submodules`.
3. Each field that differs from a set carries its reason in the comment above the hook.

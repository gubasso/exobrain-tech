# Task: sets

Write or merge `.pre-commit-config.yaml` from the hooks that the project adopts from the reference sets of the tag, and swap the hooks of `pre-commit/pre-commit-hooks` and `check-json5` for the prek built-ins of the common set. `$EXOBRAIN_TECH/programming/project-bootstrap/04-quality-gates.md`, "Reference hook sets", states the convention, and the comment above each hook of a set gives its reason.

## Pre-flight

1. Read each set that the pre-flight of the skill picked, at the tag: `$EXOBRAIN_TECH/programming/project-bootstrap/hook-sets/<set>.yaml`.
2. Read the configuration of the project, and mark each block that release-kit or sdd renders. Nothing below changes a line inside one.

## Execution

1. Where the project has no configuration, copy the top-level keys of the common set, then only the hooks that the project adopts from each set, with the comment of the set above each hook.
2. Where the project has a configuration, merge one hook at a time, outside the rendered blocks:
   - A hook of an adopted set that the project lacks: offer it. Add it only where the project takes it, with the comment of the set above it.
   - A hook that both hold: keep every field of the project as it stands. `exobrain hooks sync` sets the `rev` in the `gate` task.
   - A hook that no set holds: keep it. A `language: system` hook carries the `# owner:` comment of chapter 04.
3. Take `minimum_prek_version`, `default_install_hook_types`, and `default_stages` from the common set where the project has none of them. Keep each value that the project has, such as a `manual` stage. A project without the `pre-merge-commit` gate of `rk integrate` drops that type from both lists, as the comment of the common set says.
4. Swap the built-ins. Run `prek run --all-files` and record the result. Then replace each hook of `https://github.com/pre-commit/pre-commit-hooks`, and each repository whose hook id is `check-json5`, with the `repo: builtin` hook of the same id. The swap changes the repository and the id of the hook, and keeps its `args` and every other value of the project. `check-json` becomes `check-json5`, and takes the `types` and `types_or` of the set, because the built-in selects other files than the upstream hook. Where the built-in judges less than its upstream hook by default, add the argument of the set that restores what the upstream hook judged with the arguments of the project, and add nothing else:
   - `check-yaml` takes `--disallow-unknown-tags`, because the upstream hook refused an unknown tag while it loaded the file. A `check-yaml` with `--unsafe` checks only the syntax and loads no tag, and prek 0.5.7 refuses `--disallow-unknown-tags` beside it, so the swap adds nothing there.
   - `check-merge-conflict` and `mixed-line-ending` get no argument, because their built-ins match the upstream defaults.
   - A hook that the project adds from the set, and not by the swap, takes the arguments of the set.

   Two cases keep the upstream hook:
   - `forbid-submodules`, which has no built-in. It stays at the `rev` of the release-kit block.
   - Each hook inside a rendered block.
5. Run `prek run --all-files` again, and compare the two runs. A built-in with the arguments that the swap added judges at least what its upstream hook judged with the arguments of the project, so a new failure is a real finding, such as an unknown YAML tag under `--disallow-unknown-tags`. Fix the file in its own commit, or report it.
6. Exclude the generated files from each hook that rewrites or judges them, in the configuration of the tool where it has one, as chapter 04 says under "Generated files". Trim each exclude pattern of a set to the files that the project holds, because `check-useless-excludes` refuses a pattern that matches nothing.
7. Where the project adopts the `committed` hook and has no `committed.toml` at its root, copy the one of the tag from `$EXOBRAIN_TECH/programming/project-bootstrap/hook-sets/committed.toml`. Set `style = "conventional"` only where the release-kit block holds no `conventional-pre-commit` hook, as the comment of the file says. A project whose house style capitalizes the description after the colon keeps `style = "conventional"` with `subject_capitalized = true` even beside that hook, as the same comment says.
8. Run `prek validate-config`.
9. Commit as `build(hooks): adopt the <sets> reference hook sets of exobrain-tech`. Where the swap of step 4 is a change of its own, commit it first as `build(hooks): run the prek built-ins in place of pre-commit-hooks`.
10. Declare the hook scopes, as `$EXOBRAIN_TECH/programming/project-bootstrap/04-quality-gates.md`, "Hook scopes", says. Give `groups: [local-only]` to each hook that CI cannot run. Give `stages: [manual]` and `groups: [ci-only]` to each hook that only CI can run. Write the reason in the comment above the hook. Replace each `SKIP` of a CI workflow with the group of its hook. Change the `prek run` steps of the workflow to the steps of `$EXOBRAIN_TECH/programming/project-bootstrap/05-ci.md`, "First CI workflow". Commit the configuration as `build(hooks): declare the hooks that run in one runtime as groups`, then the workflow as `ci: select the hooks of each step by stage and group`.

## Post-flight

1. `prek validate-config` passes.
2. No hook outside a rendered block names `pre-commit/pre-commit-hooks` or a `check-json5` repository, except `forbid-submodules`.
3. Where the project adopts the `committed` hook, `committed.toml` is at the root, and the hook passes a sample header and refuses a header that runs more than one word past 72 characters:

   ```bash
   f=$(mktemp)
   printf 'build(hooks): adopt the common reference hook set\n' >"$f"
   prek run committed --hook-stage commit-msg --commit-msg-filename "$f"
   printf 'build(hooks): adopt the common reference hook set of the tag that the project pins\n' >"$f"
   ! prek run committed --hook-stage commit-msg --commit-msg-filename "$f"
   rm -f "$f"
   ```
4. No committed file sets `SKIP`: `git grep -nE 'SKIP[=:]' -- .github .gitlab-ci.yml .pre-commit-config.yaml` prints no line outside a comment.
5. `prek list --group local-only` and `prek list --group ci-only` name exactly the hooks whose comments give the reason, and each `prek run` step of CI names its `--stage`.

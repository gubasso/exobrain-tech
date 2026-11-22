# Task: gate

Enable `hook-revisions` with the chosen sets, move every shared `rev` to the tag with `exobrain hooks sync`, list `exobrain` as a host tool, and add the warnings of the CLI to `.envrc`. `$EXOBRAIN_TECH/programming/project-bootstrap/04-quality-gates.md`, "Reference hook sets", and `$EXOBRAIN_TECH/programming/project-bootstrap/03-local-dev-environment.md`, "Host requirements", state the convention.

## Pre-flight

1. Find the exobrain-tech entry of `.pre-commit-config.yaml`, and the `args` of its `hook-revisions` hook.
2. Read `host-requirements.toml` and `.envrc` where they exist.
3. Find each host tool that the `justfile`, the `Makefile`, the git hooks, and `scripts/` call, and each `exobrain host-check` call.

Where the entry names the sets of this run, `exobrain hooks check` passes, the file lists `exobrain` at `>=2.0.0` or later, and `.envrc` holds both warnings, skip the execution.

## Execution

1. Add or complete the exobrain-tech entry. The `rev` is the tag. `hook-revisions` names each set in `args`. A project with a flake also enables `no-path-flake-reference` and `no-hook-runner-in-flake`, and a project with `host-requirements.toml` enables `host-requirements`:

   ```yaml
   - repo: https://github.com/gubasso/exobrain-tech
     rev: <tag>
     hooks:
       - id: no-path-flake-reference
       - id: no-hook-runner-in-flake
       - id: host-requirements
       - id: hook-revisions
         args: [--set, common, --set, rust, --set, nix]
   ```

2. Run `exobrain hooks sync --to <tag>`. It sets the `rev` of each repository of the named sets and of the exobrain-tech entry, and keeps every other byte. Read the diff: each changed line is a `rev`.
3. List `exobrain` in `host-requirements.toml` with `version = ">=2.0.0"`, the first release that carries `exobrain outdated`. Where the file lists an older `exobrain` version, raise it. Where the project has no `host-requirements.toml`, create it as chapter 03 says:
   - `schema = 1`, then one `[tools.<name>]` table for each host tool of pre-flight step 3, and `prek` with `version = "*"`.
   - A link to the file from `README.md`, which repeats none of it.
   - The check at each call site: `_host := shell("exobrain host-check <tools>")` at the top of a `justfile`, the first line of a recipe or a script that alone uses a tool, and the first line of a dev target of a `Makefile`.
4. Add the two warnings to `.envrc`, after `use flake`, where they are missing. Each one exits 0 and moves nothing:

   ```bash
   exobrain outdated || true
   exobrain store-guard --daily || true
   ```

   Where the project has a lock other than `flake.lock`, name each lock: `exobrain outdated flake.lock nix/flake.lock || true`. Where `.envrc` runs the `status` verb of `exobrain hooks`, replace that line with the `exobrain outdated` line.

5. Commit as `build(hooks): hold the hook revisions to exobrain-tech <tag>`.

## Post-flight

1. `prek run hook-revisions --all-files` passes.
2. `exobrain host-check --parse-only` passes, and `exobrain host-check` passes on this host.
3. `direnv exec . true` prints no error from the two warnings.

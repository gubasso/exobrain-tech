# Task: host-cli

Take the `exobrain` CLI from the host. A devshell copy comes first on the `PATH` and hides a newer host CLI, and a flake input pins it at an old tag. `$EXOBRAIN_TECH/programming/project-bootstrap/03-local-dev-environment.md`, "Host requirements", states the rule.

## Pre-flight

1. Find the `exobrain` package in the devshell, such as `exobrain-tech.packages.${system}.default`: `git grep -n 'exobrain' -- flake.nix 'nix/*.nix'`.
2. Find each use of the `exobrain-tech` input in the flake. A `follows` line inside another input, such as `inputs.exobrain-tech.follows = "";` under the input of `sdd`, cuts a transitive input of that tool. It is not a use of the input of the project.
3. Find each CI job that calls `exobrain`: a job that runs the hooks, because `hook-revisions` and `host-requirements` call it, and a job that runs a recipe or a script with `exobrain host-check`.

Where the devshell holds no `exobrain` and each such CI job installs it, skip the execution.

## Execution

1. Remove the `exobrain` package from the devshell.
2. Remove the `exobrain-tech` input where nothing else in the flake uses it. Then run `nix flake lock` in the environment of the project, so `flake.lock` drops the input. Keep each `follows` line of another input.
3. Add the step of `$EXOBRAIN_TECH/programming/project-bootstrap/05-ci.md` to each CI job of pre-flight step 3, before its first call of `exobrain`. The step reads the tag from the exobrain-tech entry of `.pre-commit-config.yaml`, so CI runs the CLI of the release that the project pins. Where the project has no exobrain-tech entry yet, the `gate` task adds it on the same branch, before CI runs.
4. Commit the flake as `build(nix): take the exobrain CLI from the host`, and CI as `ci: provide the exobrain CLI at the pinned exobrain-tech tag`.

## Post-flight

1. In the environment of the project, `command -v exobrain` names the host CLI, not a path under `/nix/store`.
2. `nix flake check`, or the flake check that the project runs, passes.

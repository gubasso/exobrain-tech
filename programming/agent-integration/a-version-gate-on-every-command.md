# A Version Gate on Every Command

A tool that lands files into a project records the version that wrote them. Every command of the tool compares that record with the running binary before it acts. This chapter states that comparison: which commands it binds, what each state of the project makes a command do, and what the gate never does.

## The record holds the exact version

The landing record holds the exact version of the tool that wrote it. The record lives in the project, under version control, as [Candidate artifacts and receipts](./candidate-artifacts-and-receipts.md#the-receipt) describes. The skills of the tool follow the same record, as [Skills follow the project pin](./skills-follow-the-project-pin.md#the-record-holds-the-version) states.

cargo-dist works this way. It records `cargo-dist-version` in the project, and a binary of another version refuses with "Rerun 'dist init' to update to this version." Its `dist init` both lands a first setup and upgrades one.

## Every invocation carries one class

Every invocation of the tool carries one of five classes:

- `exempt`: the gate does not run.
- `pre-landing`: the invocation runs before a landing by design.
- `upgrade`: the invocation moves the landing to the binary.
- `read-only`: the invocation reads the landing and writes nothing.
- `writing`: the invocation writes into the project.

The class belongs to the effective invocation, not to the verb. A preview reads, and its `--apply` form writes, so the two can carry different classes. The table of classes sits beside the command list of the tool. A test fails on an invocation that carries no class, so a new command cannot skip the gate.

## The exempt class

`exempt` covers each invocation that needs no target or must never fail:

- help and version
- the doctor and the status report
- the skill reader that serves skills before a landing
- the uninstall of the user scope
- a standalone check that reads no landing
- a check that runs on every shell entry

The doctor and the status report must stay usable in the very state that the gate refuses, because they are how a person learns that state.

## The upgrade class

`upgrade` keeps its own checks. It refuses a project with no record, and names `init`. It refuses a record newer than the binary, because an older binary cannot judge it. It repairs a binary newer than the record, because that repair is its job.

## The pre-landing class

`pre-landing` covers what runs before a landing by design, such as assessment, staging, and setup steps. On a target with no record, it runs.

On a target with a record, it still parses the record and keeps its ownership checks. A read-only action, such as a staged comparison, runs at any alignment, so it can prepare an upgrade. A writing action follows the `writing` column of the table below. A first-landing verb that meets an existing record refuses and names `upgrade`.

## The table for read-only and writing

| State of the project         | Writing                     | Read-only                                         |
| ---------------------------- | --------------------------- | ------------------------------------------------- |
| No record                    | Refuses and names `init`    | Refuses and names `init`                          |
| Binary newer than the record | Refuses and names `upgrade` | Runs, with one warning on stderr naming `upgrade` |
| Record newer than the binary | Refuses                     | Refuses                                           |
| Versions match               | Runs                        | Runs                                              |

A binary newer than the record can still read the record, so a read-only command runs and warns. A writing command refuses, because its files then carry one version while the record names another. A binary older than the record cannot judge what a newer release wrote, so both classes refuse.

## A record newer than the binary

A record newer than the binary usually means that the binary on the `PATH` is not the binary of the project. The refusal first sends the reader to the binary of the project's own environment. It names the pin only where the pin itself is older than the record. [What the CLI owes a setup skill](./what-the-cli-owes-a-setup-skill.md#read-the-target) states the same case for the status report.

## A refusal names its fix

A refusal names the exact command that fixes it, as cargo-dist does. It changes nothing in the project. A person or an agent then runs that command, or decides not to.

## The gate never writes

The gate never writes, never moves the pin, and never runs a landing verb by itself. Moving the pin is the explicit act. `upgrade` then lands the consequence of that act in one change that a reviewer reads.

## A read-only command in a commit hook

A read-only command that runs as a commit hook warns and keeps its own exit code. It also reads an older schema of the record. A pin move therefore does not block the commit that carries the upgrade. A corrupt record still fails, because a record that does not parse is a different fact from an old one.

Prisma takes the same line for its notice that skills are stale. The notice never changes the exit code, and the documentation lists the conditions that suppress it.

## A warning stays out of the output

A warning goes to stderr and never into a machine-readable body. A caller that parses the output then reads the same body at every alignment.

The gate judges only what the tool owns. That is this convention's inference from two Nx reports. In the first, Nx warned that the agent configuration was outdated for an agent that Nx does not support. In the second, Nx overwrote the changes of a user inside a block that Nx manages.

## CI takes the same pin

A CI job takes the tool through the same committed project pin as a developer. The gate checks that the pin and the record agree. The record then never becomes a second selector of the tool.

A project with no pin of its own installs the tool by a script. That script installs the version that the record names, because the record is then the only selector. The Gradle wrapper and Bundler follow this shape: each runs the version that the project declares.

## The policy this convention adds

Two parts of the gate are this convention's own policy, not a statement of any source:

- The gate demands exact equality between the record and the binary.
- The table splits writing from read-only commands.

The split joins two behaviors from the sources. cargo-dist, uv, ruff, and prek exit with an error when the version does not meet the requirement, and Terraform does the same for `required_version`. Prisma warns and lets the command run. The gate refuses where a command writes, and warns where it only reads.

An older Terraform also refuses a state that a newer version wrote. That is a check of the state format, not of version equality, and this convention does not cite it as one.

## See also

- [Skills follow the project pin](./skills-follow-the-project-pin.md): the skills that the record lists.
- [A tool repository runs its previous release](./a-tool-repository-runs-its-previous-release.md): how the tool's own repository lives under this gate.
- [What the CLI owes a setup skill](./what-the-cli-owes-a-setup-skill.md): the status report that the gate leaves exempt.

## Sources

- cargo-dist, the error for a version mismatch: "You're running dist {running_version}, but 'cargo-dist-version = {config_version}' is set", with the help "Rerun 'dist init' to update to this version." Read 2026-11-13 at <https://github.com/axodotdev/cargo-dist/blob/main/cargo-dist/src/errors.rs>.
- cargo-dist, updating: install the new version and run `dist init`, which does incremental updates. Read 2026-11-13 at <https://axodotdev.github.io/cargo-dist/book/updating.html>.
- Terraform, the `terraform` block: when the version does not meet `required_version`, "Terraform prints an error and exits without taking actions." Read 2026-11-13 at <https://developer.hashicorp.com/terraform/language/block/terraform>.
- Terraform, versions: an older Terraform gives an error on an unsupported state file version. Read 2026-11-13 at <https://developer.hashicorp.com/terraform/tutorials/configuration-language/versions>.
- uv, `required-version`: "uv will exit with an error". Read 2026-11-13 at <https://docs.astral.sh/uv/reference/settings/#required-version>.
- ruff, `required-version`: "Ruff will exit with an error". Read 2026-11-13 at <https://docs.astral.sh/ruff/settings/#required-version>.
- prek, `minimum_prek_version`: "If the installed `prek` is older than the configured minimum, `prek` exits with an error." Read 2026-11-13 at <https://prek.j178.dev/reference/configuration/>.
- Prisma, the skills command: "The notice never changes the exit code", and the conditions that suppress it. Read 2026-11-13 at <https://www.prisma.io/docs/cli/skills>.
- Nx, a warning about an outdated agent configuration under an agent that Nx does not support. Read 2026-11-13 at <https://github.com/nrwl/nx/issues/36264>.
- Nx, an overwrite of user content inside the block that Nx manages. Read 2026-11-13 at <https://github.com/nrwl/nx/issues/37181>.
- Gradle, the wrapper: it "invokes a declared version of Gradle, downloading it beforehand if necessary." Read 2026-11-13 at <https://docs.gradle.org/current/userguide/gradle_wrapper.html>.
- Bundler, the `version` setting: it defaults to `lockfile`, the version that `Gemfile.lock` records. Read 2026-11-13 at <https://guides.rubygems.org/command-reference/bundle-config/>.
- Bundler 2.3: when the locked version is not installed, Bundler installs it and restarts with it. Read 2026-11-13 at <https://blog.rubygems.org/2022/01/23/bundler-v2-3>.

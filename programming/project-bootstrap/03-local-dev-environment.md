# 03 — Local dev environment

This chapter sets up a reproducible development environment for each project. Every contributor and every CI runner gets the same toolchain from it. The chapter also sets a formatting baseline that every editor follows.

## Nix devShell + `.envrc`

A Nix flake devShell for each project pins the toolchain and the dev tools declaratively. The environment is then reproducible, not "whatever is on your machine". Add an `.envrc` that contains `use flake`. [direnv](https://direnv.net/) then loads the shell on `cd`. Reuse the same flake in CI, so the local and CI environments cannot drift apart.

[`tools/nix/02 — Per-project devShell`](../../tools/nix/02-per-project-devshell.md) owns the flake shape, a `devShells.default` built with `mkShell`. It also owns the direnv wiring and the starting templates for each language. Follow it for the template. The language binding names the toolchain that goes inside.

[Dev environment features](./COMPARISON-dev-tool-features.md) names each feature of a dev environment, the native command of each tool for it, and each gap that the `exobrain` CLI fills.

A recipe or a script reaches the flake of its own project through `.` or a `git+file://` reference. It never uses a `path:` reference, which copies the ignored files of the tree into the store each time a file changes. [Nix gotcha 1](../../languages/nix/05-tooling-gotchas-resources.md#gotchas-that-bite-newcomers) explains the fetcher types.

## Who owns each dependency

Each dependency of a project has exactly one owner: the most specific file that declares that kind of dependency natively. What uses the dependency picks the owner. To place a dependency, ask which file is the most specific one that can declare it natively. The owners run from the most specific to the most general:

1. The manifest of the stack holds what the code of the project uses. A Python project declares its packages in `pyproject.toml`, and `uv.lock` locks them. The same file declares the dev tools that the project itself runs, such as pytest for its test suite. A single Python script declares its packages in a PEP 723 block. Other stacks use these manifests:

   - A Rust crate uses `Cargo.toml`.
   - A Node project uses `package.json` and its lock file.
   - A Go module uses `go.mod`.

2. The hook configuration holds what a hook uses. `.pre-commit-config.yaml` pins each hook tool with `rev` and `additional_dependencies`, where the hook runner can build an environment for the tool. A tool that only a hook uses belongs here, even when it reads the code of the project, such as mypy, pyright, or ruff. The hooks are a guardrail from outside the project, so their configuration owns what it brings. A manifest holds such a tool only when the project uses it too, or when the hook comment states a reason specific to the repository.

3. The devshell holds the major tools: the language toolchains and their managers, such as cargo and nodejs, then the task runner and the project tools. It also holds these items:

   - each tool that no owner above can declare, such as statix, bats with its libraries, or a cargo subcommand
   - the system libraries that a native build links
   - the runtime that a hook takes from the system, such as nodejs for a node hook under `node: system`

   The devshell holds no git hook runner.

4. The host supplies what the project does not own:

   - `git`, the git hook runner prek, `wt`, `bash`, `awk`, `sed`, `grep`, and the coreutils
   - the tools that build environments: `nix`, `direnv`, and `uv`
   - the tools that deploy the host
   - the standard ELF loader
   - the tools that carry credentials, such as `gh`, `glab`, `ssh`, and `aws`
   - the `exobrain` CLI of exobrain-tech, which checks the host tools and the hook revisions
   - the compile cache of a Rust project, kache, with its configuration and its daemon, as [Rust builds](#rust-builds) says

A project uses a host tool by name, as it uses `git`. It lists the tool once in `host-requirements.toml`, as [Host requirements](#host-requirements) says.

One tool has one owner. Once one level declares a tool, no other level declares it again. The devshell gives some owners below it a runtime, such as the nodejs of a node hook or the go of a golang hook. Such a runtime is not a second owner. A manager that installs its own runtime natively owns it. uv installs the interpreter of a uv project, so neither the devshell nor the host supplies a Python to it (`project-platform:the-most-specific-native-owner-holds-each-dependency`). [uv, the interpreter, and the environment manager](../../languages/python/project-bootstrap-spec/00-toolchain-and-layout.md#uv-the-interpreter-and-the-environment-manager) states the uv convention, with the interpreter pin and the activation for each environment manager.

A recipe reaches a hook tool through `prek run <id>`. It reaches a tool of the manifest through the runner of the manifest, such as `uv run`, `cargo`, or `npm run`.

A flake never holds a language library. A Python, Node, or Perl package goes to the manifest of its stack. The devshell keeps the toolchain and its manager, except where the host supplies the manager, as it supplies uv.

The host supplies the standard ELF loader, `/lib64/ld-linux-x86-64.so.2`. A hook environment can hold a prebuilt binary that links it, such as the typos wheel. A conventional distribution carries the loader, and a NixOS host gets it from nix-ld.

The git hook runner is a host tool, because git runs an installed hook from every shell. That includes the shells without the devshell. In a devshell, a runner written in Python would put its own Python on `PYTHONPATH`. That Python would shadow the packages of a uv project. prek is a Rust runner that reads `.pre-commit-config.yaml`. The configuration names the oldest prek that it accepts with `minimum_prek_version`.

If prek cannot build the environment of a hook from a clean environment, the devshell supplies the missing dependency. prek still manages the packages of the hook. prek builds a `rust` hook with the cargo of the devshell, and a `python` hook with the uv of the host. A project with `node` hooks puts `nodejs` in its devshell and names `node: system` in `default_language_version`. prek then takes that node and downloads none ([prek language support](https://prek.j178.dev/reference/language-support/)).

A hook that keeps `language: system` is an exception with a recorded owner and class, as [04 — Quality gates](./04-quality-gates.md#git-hooks) says.

A Nix flake is the example here, not the rule. [`tools/nix/02 — Per-project devShell`](../../tools/nix/02-per-project-devshell.md#git-hooks) shows the Nix form, with the package list.

## Host requirements

A project lists each host tool that these callers use, once, in `host-requirements.toml` at its root:

- its `justfile`
- its `Makefile`
- its git hooks
- its scripts

A version requirement uses Cargo's syntax, with an explicit operator. Cargo reads a bare `"2.57.0"` as `^2.57.0`, and a future 3.0.0 fails that requirement. A tool that enforces its own version from its own configuration takes the wildcard `"*"`, such as prek with `minimum_prek_version`. The checker leaves the version of that tool to the tool. The file lists no tool that the manifest, the hook configuration, or the devshell owns.

```toml
schema = 1

[tools.exobrain]
version = ">=1.2.0"

[tools.git]
version = ">=2.36.0"

[tools.gh]
version = ">=2.57.0"

[tools.prek]
version = "*"
```

The host supplies the `exobrain` CLI, like prek and git, so its `host-requirements.toml` entry names the oldest version that carries `exobrain hooks check`, which the hook `hook-revisions` calls ([04 — Reference hook sets](./04-quality-gates.md#reference-hook-sets)). The devshell holds no `exobrain` package, because a devshell copy comes first on the `PATH` and hides a newer host CLI. Each CI job that runs the hooks provides the CLI at the release that the project pins.

`exobrain host-check` reads the file. With no argument it checks every tool, and with names it checks only those. It reports each missing tool and each version that misses its requirement, one line each, and then exits 1. It never picks another tool. Where a tool is missing, the developer installs it. Nothing checks when a shell enters the folder, so `.envrc` holds no check. It holds two warnings of the `exobrain` CLI, each of which exits 0 and moves nothing:

```bash
exobrain hooks status || true
exobrain store-guard --daily || true
```

The first reports a newer CLI or newer hook revisions than the project pins. The second reports a store copy of the checkout once a day, and runs `prek cache gc` once a week.

The call site decides where the check runs and names the tools that it needs:

- A `justfile` checks the tools of every recipe once, in a top-level assignment. just runs that assignment before any recipe. A recipe that alone uses a tool checks it in its first line. A bare call checks every listed tool, so it stands only where every recipe needs every tool.

  ```just
  _host := shell("exobrain host-check git prek")

  release:
      exobrain host-check gh
      gh release list
  ```

- A script checks near its top, or at the top of the chunk that uses the tool: `exobrain host-check gh`.
- A `Makefile` checks in the first line of each dev target that needs the tool, such as `lint`, `test`, or `check`. A package build can run the same `Makefile` where `exobrain` is absent. For that reason, the check is never a `$(shell ...)` at parse time, and never in `all`, `install`, or a build target.
- A git hook needs no check, because prek stops the run and names a missing executable. A hook tool with a version requirement gets its check through a recipe that the hook calls.

The `host-requirements` hook that exobrain-tech publishes runs `exobrain host-check --parse-only` each time the file changes. A malformed file therefore fails at commit.

A project that does not follow the canon of exobrain-tech, and so has no `exobrain` on its hosts, checks natively in its `justfile`. just 1.39 and later has `require(name)`. just 1.16 and later has `semver_matches(version, requirement)`, which reads the requirement strings of the file unchanged ([just functions](https://just.systems/man/en/functions.html)). That project cannot run the hook, and just halts on a malformed requirement at the first check that reads it.

A project with no call site at all, whose only callers are its git hooks, lists its tools with no `version`, because no check enforces one.

The `README.md` of the project links the file and repeats none of it, as [01 — `README` skeleton](./01-repository-foundation.md#readme-skeleton) shows.

A shipped application is a separate scope. It checks the tools that it needs at runtime in its own code, at the start of a command. That code holds the list, so the check reaches the host of each user. A doctor command can report the tools. No file serves that scope.

The rule is `project-platform:host-requirements-list-the-host-tools`.

## Rust builds

A Rust project builds through the compile cache [kache](https://github.com/kunobi-ninja/kache), which the host owns. The host configuration installs one kache, writes `~/.config/kache/config.toml`, and runs the kache daemon as a systemd user service. One version, one daemon, and one cache in `~/.cache/kache` serve every project, because kache changes its index format between versions and two versions must never share one cache. The project lists `kache` in `host-requirements.toml`, as it lists every host tool.

The `.envrc` of the project sets `RUSTC_WRAPPER=kache` after `use flake`. Cargo then sends each rustc call through kache. The devshell does not set the variable, because CI enters the devshell without direnv and has no kache. CI therefore builds without the cache. A host setting would break every `cargo` call on a host without kache. [`examples/rust.envrc`](./examples/rust.envrc) is the whole file, with the two warnings of [Host requirements](#host-requirements) after the variable. `KACHE_DISABLED=1` runs one command without kache, for example `KACHE_DISABLED=1 cargo build`.

On btrfs, kache restores a cached output as a copy-on-write clone, so the clone in `target/` and the cache entry share their disk blocks ([kache deduplication](https://github.com/kunobi-ninja/kache/blob/v2.1.1/docs/deduplication.mdx)). A new worktree therefore reuses the crates that another worktree or another project compiled. A trial on 2026-11-17 measured two worktrees of one project at 671 MiB of real disk with kache, and 1412 MiB without it. If the kache daemon is not running, the first build starts it ([kache daemon lifecycle](https://github.com/kunobi-ninja/kache/blob/v2.1.1/docs/daemon/lifecycle.mdx)).

Each worktree keeps its own build folder, and the worktrees of a project never share one. Cargo names the crates of a project by their path inside the project and judges freshness by file times, so a shared folder can hand one worktree the build of another and report it fresh ([DocSpring on a shared build folder](https://docspring.com/blog/posts/rust-git-worktrees-shared-cargo-build/)). Checksum freshness is still unstable in cargo ([cargo unstable features](https://doc.rust-lang.org/cargo/reference/unstable.html), [cargo issue 17626](https://github.com/rust-lang/cargo/issues/17626)). DocSpring measured an extra worktree at 8.2 GB with its own folder and at 2.0 GB with kache ([DocSpring on kache](https://docspring.com/blog/posts/rust-git-worktrees-kache/)).

The dev profile keeps line tables for the crates of the project and no debug information for the dependencies:

```toml
[profile.dev]
debug = "line-tables-only"

[profile.dev.package."*"]
debug = false
```

Line tables keep the file and the line of each frame in a backtrace. A debugger loses the variable values and the types. On exobrain-tech, this profile cut the output of `cargo test --no-run` from 514 MB to 269 MB, with the same build time ([cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)).

If you need a debugger, build with full debug information for one command:

```bash
cargo test --config 'profile.dev.debug="full"' --config 'profile.dev.package."*".debug="full"'
```

Cargo reads no environment variable for a package override, so the command uses `--config` ([cargo configuration](https://doc.rust-lang.org/cargo/reference/config.html#profilenamepackagename)). The command rebuilds every crate. Its output stays in the build folder until the next cleanup.

The rule is `project-platform:a-rust-project-builds-through-the-host-compile-cache`.

## Build output cleanup

A Rust project keeps its compile output in the build folder of each worktree: `target/`, unless `CARGO_TARGET_DIR` or `build.target-dir` moves it. Cargo deletes none of that output by itself ([cargo issue 13136](https://github.com/rust-lang/cargo/issues/13136), [cargo issue 5026](https://github.com/rust-lang/cargo/issues/5026)). Two cleanups run in the background, outside the work of a person or an agent, and one warning asks a person or an agent to clean.

At most once an hour, when the machine is quiet, the kache daemon deletes the idle incremental state, the build units that no build used for 30 days, and the build folders of deleted worktrees that no build used for a day. It skips a folder that a running command or program uses ([kache target cleanup](https://github.com/kunobi-ninja/kache/blob/v2.1.1/docs/getting-started/configuration.mdx)). Each week, a user timer of the host runs this command, which deletes the build folders that no build used for 14 days, and the build folders whose worktree is gone:

```bash
kache clean --stale 14d --yes
```

`exobrain store-guard --daily`, which `.envrc` already runs, warns once a day when the build folder of the checkout holds more than 10 GB of its own data. The warning is one line with the folder, its size, and the command that cleans the folder:

```bash
cargo clean --target-dir <folder>
```

The guard counts exclusive bytes, the bytes that deleting the folder frees. A file that is a copy-on-write clone of a compile-cache entry counts only the blocks that the build changed. `du` counts each clone in full, so it overstates such a folder many times over. Each worktree runs its own guard and reports only its own folder. The guard deletes nothing. A person or an agent runs the command after the builds of that worktree end, and the next build takes the dependencies from the compile cache, such as kache. `EXOBRAIN_TARGET_LIMIT` sets another limit, with a size such as `20G`.

The rule is `project-platform:a-large-build-folder-gets-a-warning`.

## `.editorconfig`

`.editorconfig` sets the whitespace, charset, and final-newline rules that every editor honors. Formatting then stays consistent, whatever the configuration of each editor. Keep `.editorconfig` aligned with the language formatter (chapter [04](./04-quality-gates.md)), so the two never fight. The formatter is authoritative for code. `.editorconfig` covers the authored files that no formatter owns. A generated file, such as a lock file, follows its generator, and editorconfig-checker excludes it ([04 — Generated files](./04-quality-gates.md#generated-files)).

# Anti-patterns and case studies

The failure modes this topic exists to prevent, the real incidents that prove
them, and how to check your own tool.

## Anti-patterns

- **Appending runtime state to a user-authored config file.** The classic
  failure: registering entries into `~/.config/<tool>/*.yaml` on the fly.
- **Rewriting config to normalize, migrate, sort, or persist UI state.** A
  whole-file "read → transform → atomic `mv`" strips comments and reorders keys,
  and clobbers a managed symlink.
- **Two-way sync** between declarative config and generated state — it can't be
  lossless; don't build it.
- **Treating read-only config as an error** for a command that only needs to
  write _state_. If `login` needs to persist a token, that token is state.

## Precedents and case studies

- **XDG Base Directory Specification** — the direct authority for the
  config/state/cache/runtime split.
- **Filesystem Hierarchy Standard (FHS)** — `/etc` vs `/var` vs `/run`, the same
  split at system level.
- **Twelve-Factor App, _Config_** — externalize deploy-varying config from code
  (adjacent, narrower).
- **systemd drop-ins** and **`sshd_config.d` / `sudoers.d`** — the canonical
  layered-composition precedent (Pattern B).
- **Kubernetes server-side apply / three-way merge** and **Helm 3** — desired
  config, live state, and last-applied kept as separate inputs (Pattern A).
- **Real incidents** where tools got it wrong and the fix was to _move state
  out of config_: the GitHub CLI requiring a writable `config.yml` (breaks
  immutable / Home-Manager-managed config), lazygit moving `state.yml` into
  `$XDG_STATE_HOME` (#2794), and k9s no longer writing runtime context into
  `config.yaml` (#2346).

## Worked example: `dctl`

`dctl` originally treated `~/.config/dctl/projects.yaml` as both configuration
(the user declares projects/manifests) _and_ state (`dctl init` registered
projects at runtime, via a whole-file rewrite). Under Home Manager that file is a
read-only `/nix/store` symlink, so the runtime rewrite clobbers it — the failure
this topic exists to prevent.

The fix is not in the config manager; it is in the tool. Split
`~/.config/dctl/projects.yaml` (declared, read-only) from
`~/.local/state/dctl/registry.yaml` (runtime, writable) and merge on read
([Pattern A](./02-patterns.md#pattern-a--configstate-split-with-read-time-merge)).
The config manager's wiring then collapses to a plain read-only symlink — no
out-of-store gymnastics, no activation merge, no drift. The tool's current XDG
layout is documented in [`tools/dctl.md`](../../../tools/dctl.md#xdg-layout-the-3-tier-model).

The registry is keyed by the project's **absolute path**, which is exactly why it
is state and not data: an absolute-path map is machine-local and **non-portable**
(paths differ across machines and users), and losing it loses only a rebuildable
association, not authored intent. That non-portability is the tell — a portable,
user-authored value would be config; a machine-local, tool-recorded one is state.
Making such keys portable (home-relative, or keyed off a repo remote) is a
deliberate later step, not a reason to move the registry out of state.

## Checking your own tool: `xdg-ninja`

[`xdg-ninja`](https://github.com/b3nj5m1n/xdg-ninja) audits `$HOME` for tools
that pollute it — writing to `~/.<app>` or dropping runtime state into
`~/.config` instead of honoring `$XDG_STATE_HOME`/`$XDG_DATA_HOME`. It reports
each offender with the fix (usually an env var or a config move). Treat it as the
community conformance yardstick: **absence from its complaints ≈ compliant.** Run
it against any tool we build before calling its file placement done.

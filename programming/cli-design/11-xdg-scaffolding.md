# XDG scaffolding & `init` without mutating config

How a CLI sets a project up — generates a starter config, prepares directories,
records what it has seen — **without ever writing to `$XDG_CONFIG_HOME`**.

The rule this chapter enforces: **configuration is read-only at runtime; `init`
must never mutate config.** The _why_ (one writer per file, read-only config
under Nix/Home Manager, the config-vs-state boundary) lives in the decision
directory
[`design-decisions/config-state-ownership/`](../design-decisions/config-state-ownership/README.md).
This chapter is the CLI-facing recipe. It pairs with
[03 — Config precedence](./03-config-precedence.md) (where config comes _from_)
and [06 — Preflight & health checks](./06-preflight-and-health-checks.md) (the
probe set `init` shares with `doctor`).

## Where a scaffold may write

| Target                    | May `init` write here? | For what                                                   |
| ------------------------- | ---------------------- | ---------------------------------------------------------- |
| `$XDG_CONFIG_HOME/<app>/` | **No**                 | User-authored config — the user or config manager owns it. |
| `$XDG_STATE_HOME/<app>/`  | Yes                    | Registries, last-used context, logs, tokens.               |
| `$XDG_CACHE_HOME/<app>/`  | Yes                    | Generated/merged artifacts that can be regenerated.        |
| `$XDG_DATA_HOME/<app>/`   | Yes                    | Durable generated assets the user keeps.                   |
| A user-named project dir  | Yes (explicit path)    | Scaffolded files the user asked to create _there_.         |

## Patterns that don't violate the rule

### 1. Print the starter config to stdout

Let the user place it. The tool never writes into config; the user's shell
redirection does, so the file's sole writer stays the user.

```bash
foo init --print-config > ~/.config/foo/config.toml
# user reviews and edits, then:
foo status
```

### 2. Sane defaults — config optional

The best first-run experience needs no config at all. Ship reasonable built-in
defaults; treat a missing config file as "use defaults," not an error. `ripgrep`
and `fd` work with zero config; a file only _customizes_. Document the schema in
`--help` and the man page so users can opt in.

### 3. Write state/cache only

If `init` must persist anything, it creates the **state/cache** directories and
writes there — never config.

```bash
foo init
# creates $XDG_STATE_HOME/foo/ and $XDG_CACHE_HOME/foo/,
# writes a registry / merged artifact, and prints where config *would* go.
```

### 4. Explicit, non-clobbering write

If a subcommand really does author a file, the write is the **only** sanctioned
relativization of the read-only rule, and it stays sanctioned only when it is:
**explicit** (a flag that names the exact target), **off by default**,
**confirmed** (a prompt or `--yes`), and **reversible / non-clobbering** (refuses
to overwrite a file another owner may manage) — never a silent in-place rewrite,
never a side effect of a normal command. Prefer aiming such a write at **state**
(a registry, a recorded binding) rather than config wherever the data allows.

```bash
foo bind --write ~/.config/foo/config.toml   # explicit target; refuses if it exists
```

## The anti-pattern

`init` discovering runtime facts and appending them into
`~/.config/foo/*.yaml` gives one file **two writers** — the user/config manager
_and_ the application. Under Home Manager that file is a read-only `/nix/store`
symlink, so the write fails or is discarded on the next switch. The fix is
[Pattern A](../design-decisions/config-state-ownership/02-patterns.md#pattern-a--configstate-split-with-read-time-merge):
split declared config from a runtime registry and merge on read. The
[`dctl` worked example](../design-decisions/config-state-ownership/03-anti-patterns-and-case-studies.md#worked-example-dctl)
is exactly this failure and its fix.

## `init` shares the `doctor` probe set

`init`, `doctor`, and each per-command guard are three call sites over **one**
set of probe functions (see [06](./06-preflight-and-health-checks.md)). `init`
runs the setup-relevant subset — "is `$XDG_STATE_HOME/<app>` writable?", "is the
config path readable?" — and adding a prerequisite is a single new probe in the
catalog, not a new check bolted onto `init` alone.

## Conformance: `xdg-ninja`

[`xdg-ninja`](https://github.com/b3nj5m1n/xdg-ninja) audits `$HOME` for tools
that pollute it — writing to `~/.<app>` or dropping runtime state into
`~/.config`. Run it against a tool before calling its file placement done;
absence from its complaints ≈ compliant.

## Checklist

- [ ] `init` writes **nothing** to `$XDG_CONFIG_HOME`.
- [ ] Starter config is printed to stdout, or written only on explicit request to
      a non-existent path.
- [ ] The tool works with **no** config file (sane defaults).
- [ ] Runtime/registry/last-used data goes to `$XDG_STATE_HOME`; regenerable
      artifacts to `$XDG_CACHE_HOME`.
- [ ] `init` reuses the `doctor` probe subset — no independent checks.
- [ ] `xdg-ninja` reports no pollution.

## Further reading

- [12 — Config generation from types](./12-config-generation-from-types.md) — the
  copy-don't-scaffold companion: generate the example the user copies instead of
  scaffolding config.
- [`design-decisions/config-state-ownership/`](../design-decisions/config-state-ownership/README.md)
  — the ownership rule and XDG/FHS placement (the _why_).
- XDG Base Directory Specification — <https://specifications.freedesktop.org/basedir-spec/latest/>
- `xdg-ninja` — <https://github.com/b3nj5m1n/xdg-ninja>

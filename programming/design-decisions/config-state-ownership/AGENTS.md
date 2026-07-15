---
digest-of: programming/design-decisions/config-state-ownership
last-synced: 2026-07-15
source-files:
  - README.md
  - 01-placement-xdg-fhs.md
  - 02-patterns.md
  - 03-anti-patterns-and-case-studies.md
token-estimate: 700
---

# AGENTS

## Scope

The canonical rule for _where persisted files go and who owns them_:
user-authored config vs program-authored state, XDG/FHS placement, and why a
program must never mutate declarative config at runtime. Language-agnostic. The
CLI-implementation recipe (how `init`/scaffold avoids mutating config) lives in
`programming/cli-design/11-xdg-scaffolding.md`; config-load precedence lives in
`programming/cli-design/03-config-precedence.md`.

## Key Points

### The rule (README)

- **One writer per file.** Humans/config-managers write config; apps write
  state/cache/runtime. A program never mutates declarative config as a side
  effect of running.
- Two-way sync between config and generated state is a recognized anti-pattern,
  not a missing feature — it cannot be lossless.
- Goal state: **read-only config**, so it can be a `/nix/store` symlink, a
  config-manager output, or a git-tracked dotfile without drifting.

### Placement: XDG & FHS (01)

- Category table: config → `$XDG_CONFIG_HOME` (read-only), state →
  `$XDG_STATE_HOME`, data → `$XDG_DATA_HOME`, cache → `$XDG_CACHE_HOME`, runtime
  → `$XDG_RUNTIME_DIR`. Decide by "who writes it?".
- Spec conformance: `XDG_CONFIG_DIRS` (`/etc/xdg`) and `XDG_DATA_DIRS`
  (`/usr/local/share:/usr/share`) are system search lists (user home wins);
  state/cache/runtime have **no** system list. Relative `XDG_*` values are
  invalid → ignore, use default. No creation obligation (app makes dirs, `0700`).
  `XDG_RUNTIME_DIR` has no portable fallback — provided by logind at
  `/run/user/$UID`, session-scoped; never blindly fall back to `/tmp`.
- FHS is the same split for system installs: `/etc` static, `/var/lib` state,
  `/var/log` logs, `/var/cache` cache, `/run` runtime, `/usr/share` data.

### Patterns (02)

- **Pattern A** — config/state split with read-time merge (declared ∪ runtime,
  no write-back). Precedent: Kubernetes three-way merge, Helm 3.
- **Pattern B** — drop-in `*.d/` directory for multiple owners. Precedent:
  systemd, `sshd_config.d`, `sudoers.d`, git `includeIf`.
- Why runtime config-mutation breaks under Nix/Home Manager: config is a
  read-only `/nix/store` symlink; a write fails or is discarded on the next
  `home-manager switch`. "Declarative config is an input, not an output."

### Anti-patterns & case studies (03)

- Anti-patterns: appending state to config; whole-file rewrite that strips
  comments/reorders keys; two-way sync; treating read-only config as an error
  when only state needs writing.
- Real incidents: GitHub CLI writable `config.yml`, lazygit `state.yml` move
  (#2794), k9s runtime context out of `config.yaml` (#2346).
- `dctl` worked example: split `projects.yaml` (config) from
  `registry.yaml` (state), merge on read.
- `xdg-ninja` audits `$HOME` for violations — run it as the conformance check.

## Source Map

| Topic                                          | File                                   |
| ---------------------------------------------- | -------------------------------------- |
| The one-writer rule, TL;DR, author checklist   | `README.md`                            |
| XDG/FHS placement + spec-conformance rules     | `01-placement-xdg-fhs.md`              |
| Config/state split, drop-ins, read-only config | `02-patterns.md`                       |
| Anti-patterns, incidents, `dctl`, xdg-ninja    | `03-anti-patterns-and-case-studies.md` |

## Maintenance Notes

- Regenerate when any chapter file changes.
- Keep the CLI-facing recipe in `cli-design/11-xdg-scaffolding.md` and the
  config-precedence ladder in `cli-design/03-config-precedence.md`; this
  directory owns the ownership rule and placement/spec detail only.

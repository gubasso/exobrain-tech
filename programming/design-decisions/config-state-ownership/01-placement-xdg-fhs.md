# Placement: XDG and FHS

Where each persisted file belongs, decided by _who writes it_, plus the
specification rules a compliant tool must honor.

## Be XDG-friendly first

The baseline requirement for any application we build or adopt: **follow the XDG
Base Directory Specification.** It already encodes the config/state separation.
Respect the environment variables (with the documented fallbacks) and put each
kind of file where it belongs.

| Kind        | Owner               | App may write?  | XDG base (default)                   | Examples                                    |
| ----------- | ------------------- | --------------- | ------------------------------------ | ------------------------------------------- |
| **Config**  | user / config mgr   | no              | `$XDG_CONFIG_HOME` (`~/.config`)     | preferences, declared manifests, recipients |
| **State**   | application         | yes             | `$XDG_STATE_HOME` (`~/.local/state`) | project registry, last-used context, logs   |
| **Data**    | application         | yes             | `$XDG_DATA_HOME` (`~/.local/share`)  | generated assets the user keeps/backs up    |
| **Cache**   | application         | yes, disposable | `$XDG_CACHE_HOME` (`~/.cache`)       | derived/merged files, downloads             |
| **Runtime** | application/session | yes, ephemeral  | `$XDG_RUNTIME_DIR`                   | sockets, locks, pid files                   |

## Who owns this file?

When you are unsure where something goes, answer one question — _who writes it?_

- The **user hand-edits** it → **config** (`$XDG_CONFIG_HOME`); read-only at
  runtime.
- The **program writes** it and the user never touches it → **state**
  (`$XDG_STATE_HOME`).
- The **program writes** it but the user needs a durable, portable copy →
  **data** (`$XDG_DATA_HOME`).
- It can be **regenerated** at any time → **cache** (`$XDG_CACHE_HOME`).
- It only lives for the session (sockets, locks) → **runtime**
  (`$XDG_RUNTIME_DIR`).

A runtime-generated registry (e.g. "projects this tool has seen") is **state by
definition** — it belongs in `$XDG_STATE_HOME`, never appended into a config file
under `$XDG_CONFIG_HOME`.

## XDG spec conformance

Beyond "put files in the right base directory," the specification imposes rules a
compliant tool must honor. Do not hand-roll these — use your language's XDG
library (`directories` in Rust, `platformdirs` in Python, `xdg` in Go) and let it
implement the ladder.

### System search paths (config/data only)

`XDG_CONFIG_HOME` and `XDG_DATA_HOME` are single per-user base directories, but
config and data also have **system-wide search lists**, so vendor/administrator
defaults can sit beneath user overrides:

| Variable          | Default                       | Meaning                                                                 |
| ----------------- | ----------------------------- | ----------------------------------------------------------------------- |
| `XDG_CONFIG_DIRS` | `/etc/xdg`                    | Colon-separated system config dirs, searched _after_ `XDG_CONFIG_HOME`. |
| `XDG_DATA_DIRS`   | `/usr/local/share:/usr/share` | Colon-separated system data dirs, searched _after_ `XDG_DATA_HOME`.     |

Precedence: **user home wins over system dirs**, and within a colon-list earlier
entries win over later. Note the asymmetry — **state, cache, and runtime have no
system search list.** They are write targets, not layered lookup paths; there is
exactly one `XDG_STATE_HOME` / `XDG_CACHE_HOME` / `XDG_RUNTIME_DIR`.

### Absolute paths only

If any `XDG_*` variable holds a **relative** path, it is invalid and the
application **must ignore it** and fall back to the default. This prevents subtle
breakage when the working directory changes.

### No creation obligation

Nothing — not the shell, not the session manager — is required to create these
directories for you. The application **creates them on demand** with safe
permissions: home-relative directories at mode `0700`, and `XDG_RUNTIME_DIR`
mandated by the spec to be `0700` and owned by the user.

### `XDG_RUNTIME_DIR` has no portable fallback

Unlike the other bases, the spec defines **no default** for `XDG_RUNTIME_DIR`.
It is provided by the login/session manager (`pam_systemd`/logind creates
`/run/user/$UID`), exists only for the life of the login session, and is removed
at logout. If it is unset, do **not** blindly fall back to `/tmp`: warn, and use
a replacement directory with equivalent semantics (user-owned, `0700`,
session-scoped) — or degrade the feature.

## FHS: the same split, one level up

The **Filesystem Hierarchy Standard** draws the identical line for system-wide
installs. A tool packaged to `/usr/bin/<app>` uses:

| Concern             | System-wide (FHS)   | Per-user (XDG)            |
| ------------------- | ------------------- | ------------------------- |
| Static config       | `/etc/<app>/`       | `$XDG_CONFIG_HOME/<app>/` |
| Variable state      | `/var/lib/<app>/`   | `$XDG_STATE_HOME/<app>/`  |
| Logs                | `/var/log/<app>/`   | `$XDG_STATE_HOME/<app>/`  |
| Cache               | `/var/cache/<app>/` | `$XDG_CACHE_HOME/<app>/`  |
| Runtime (sockets)   | `/run/<app>/`       | `$XDG_RUNTIME_DIR/<app>/` |
| Data (assets, docs) | `/usr/share/<app>/` | `$XDG_DATA_HOME/<app>/`   |

`/etc` is static host configuration, `/var` is variable data written during
operation, and `/run` is runtime data valid only since boot. The system layer
(`/etc/xdg` via `XDG_CONFIG_DIRS`) sits _beneath_ the per-user layer: a
well-designed tool reads both and lets user config override system defaults.

Twelve-Factor's _Config_ factor is the adjacent rule for deployed services:
config that varies between deploys is externalized from code into the
environment. Same principle — config is an input, not something the process
rewrites.

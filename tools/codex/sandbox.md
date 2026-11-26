# Codex sandbox

This page records the Codex sandbox facts that bind a credential gateway on Linux: the permission profile, the network proxy and its domain rules, the deny forms that stop the sandbox, and the write paths of a commit from a linked worktree. [Credential gateways](../../programming/agent-integration/credential-gateways.md) states the rule and the gateway. This page states only what Codex does.

## Version and date

codex-cli 0.167.0 on Linux, tested on 2026-11-25. The documentation pages under the references were read on the same date.

Each fact below names its evidence. "The documentation states" marks a fact from the vendor pages. "The probe showed" marks a result of a test run on that date with `codex sandbox -- <command>`. A probe result is a fact of the tested host and release, and a later release can change it.

## The permission profile

The documentation states that a permission profile is a named policy under `[permissions.<name>]`, and `default_permissions` selects it. The documentation marks permission profiles as beta. A profile does not compose with the older `sandbox_mode` settings. If any loaded configuration layer sets `sandbox_mode`, or the command line passes `--sandbox`, Codex uses the older settings and ignores the profile.

The documentation states that `extends = ":workspace"` starts a profile from the built-in workspace profile. That parent keeps `.git`, `.agents`, `.codex`, and `.aws` read-only inside each workspace root. On Linux, Codex enforces the profile with bubblewrap (bwrap) and seccomp.

The documentation states these filesystem rules:

| Entry                        | Effect                                                                       |
| ---------------------------- | ---------------------------------------------------------------------------- |
| `"<path>" = "read"`          | Commands read and list the path, and cannot change it                        |
| `"<path>" = "write"`         | Commands read and change the path                                            |
| `"<path>" = "deny"`          | Commands neither read nor write the path                                     |
| A more specific path         | Overrides a broader entry, so a deny can carve a path out of a grant         |
| Two entries on the same path | `deny` wins over `write`, and `write` wins over `read`                       |
| A `deny` glob                | Codex expands the matches when each sandboxed process starts, not later      |
| `glob_scan_max_depth`        | Bounds the expansion of an unbounded `**` on Linux, and must be at least `1` |

A deny glob does not cover a matching file that appears after a process starts. A long-running shell or development server can read such a file.

The probe showed that a profile with an unbounded `**` deny and no `glob_scan_max_depth` starts with a warning. With `glob_scan_max_depth = 5`, a `**/.envrc.local` deny under the projects folder denied the `.envrc.local` of another project. A direnv environment with its `.envrc.local` denied still loaded, and the variable that the file set was empty.

The probe showed that each sandboxed command runs in its own PID namespace. `/proc/1` is the init of the sandbox, the host processes are not visible, and `/proc/<pid>/environ` shows only the environment of the sandbox.

## The network proxy

The documentation states that `network.enabled = true` lets commands reach the network but does not start the proxy. Domain rules bind only when `[features] network_proxy = true` starts it. `network.domains` maps a host pattern to `allow` or `deny`, and a deny wins over an allow.

The documentation states that the active proxy blocks local and private targets by default. An exact literal such as `127.0.0.1` passes only when the domain table allows it. A host name that resolves to a local address stays blocked unless `allow_local_binding = true`.

The probe showed these results against a loopback gateway and a public host:

| Settings                                    | Gateway on `127.0.0.1`    | Public host               |
| ------------------------------------------- | ------------------------- | ------------------------- |
| `network.enabled = false`                   | Refused                   | The name does not resolve |
| `network.enabled = true`, no proxy          | Reached directly          | 200                       |
| The proxy on, `"127.0.0.1" = "allow"`       | Reached through the proxy | 403 at the `CONNECT`      |
| The same, plus `allow_local_binding = true` | Fails                     | 403                       |

With the proxy on, the sandbox sets `HTTP_PROXY` to the Codex proxy and an empty `NO_PROXY`. With `allow_local_binding = true`, `NO_PROXY` gains `127.0.0.1`. The client then connects directly, and the direct connection fails. So the client settings of the gateway name `127.0.0.1`, not a host name.

## Every loopback port is open

The documentation states that Codex strips a port number from a host pattern. The probe showed that with `"127.0.0.1"` or `"127.0.0.1:<port>"` allowed, a listener on another loopback port answered too. One allow entry opens every loopback port to the agent. [Code runners on an open loopback](../../programming/agent-integration/credential-gateways.md#code-runners-on-an-open-loopback) states the guard.

## Deny forms that stop the sandbox

The probe showed two deny forms that make bwrap refuse to start. The sandbox fails closed in both, so no command runs.

- A deny on a path under the `/var/run` symlink, such as `/var/run/docker.sock`, fails with `Can't bind mount tmpfile on /var/run/docker.sock`. The same deny on the `/run` path starts. A deny on a path that does not exist starts too.
- A deny glob under `/tmp` fails, because the glob scan cannot read the root-owned folders there. A socket that a glob would match needs a folder of its own with a plain deny.

The probe showed that the deny entries refused the Docker socket, the user D-Bus socket, the X11 socket folder, and kitty sockets by exact path. The chapter states why such a socket is an escape from the sandbox.

## A commit from a linked worktree

The probe showed that a commit from a linked worktree fails with `index.lock: Read-only file system`, because the gitdir of the worktree lives in the main checkout. A `".git" = "write"` entry under `":workspace_roots"` does not open it. These four write entries do, each under `<main>/.git`:

- `worktrees/<name>`
- `objects`
- `refs`
- `logs`

Here `<main>` is the main checkout, and `<name>` is the name of the linked worktree. With the four entries, a commit with its hooks landed, and `.git/hooks` and `.git/config` stayed read-only. One `packed-refs.lock` error printed, and the commit still landed. A write entry on the whole `<main>/.git` let the agent plant a file in `.git/hooks`. The entries are per main checkout, and no glob form was tested.

## Features that are not ready

`codex features list` of 0.167.0 lists `network_proxy` as experimental and `credential_masking` as under development. Both are off by default. [openai/codex#52302](https://github.com/openai/codex/pull/52302), merged on 2026-10-08, added `credential_masking`. It brokers a credential through an active network proxy. No probe of it ran, so the page records no masking behavior of Codex.

## References

- [Permissions](https://learn.chatgpt.com/docs/permissions)
- [Sandbox](https://learn.chatgpt.com/docs/sandboxing)

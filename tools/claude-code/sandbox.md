# Claude Code sandbox

This page records the Claude Code sandbox facts that bind a credential gateway on Linux: how a sandboxed command reaches the network, how it reaches a loopback gateway, which sockets stay open, and how a credential mask works. [Credential gateways](../../programming/agent-integration/credential-gateways.md) states the rule, the gateway, and the limits of a mask. This page states only what Claude Code does.

## Version and date

Claude Code 2.1.325 on Linux, tested on 2026-11-25. The documentation pages under the references were read on the same date.

Each fact below names its evidence. "The documentation states" marks a fact from the vendor pages. "The probe showed" marks a result of a test run on that date. A probe result is a fact of the tested host and release, and a later release can change it.

## The network of a sandboxed command

The documentation states that on Linux each sandboxed command runs in its own network namespace with no connection to the host network. Claude Code runs the sandbox proxy outside the sandbox and points commands at it with `HTTP_PROXY`, `HTTPS_PROXY`, and `ALL_PROXY`. A tool that ignores the proxy variables cannot connect, even to an allowed host.

The probe showed that a direct connection from a sandboxed command to a loopback gateway fails. Loopback inside the namespace is not the loopback of the host. So a sandboxed client reaches a loopback gateway only through the proxy.

## `NO_PROXY` and the hosts-file name

The documentation states that Claude Code sets `NO_PROXY` for sandboxed commands so that they connect to `localhost` directly.

The probe showed that the sandbox forces a `NO_PROXY` that begins `localhost,127.0.0.1`, and that it overrides a `NO_PROXY` from the `env` block of the settings. A client that honors `NO_PROXY` then connects to `127.0.0.1` directly, and that connection fails in the namespace. So a gateway address of `127.0.0.1` or `localhost` never reaches the proxy, and the client needs another name for the gateway.

## The exact-name and port rule

The documentation states that the proxy resolves an allowed name and refuses the connection when the name resolves only to local addresses. An `allowedDomains` entry for the IP address, such as `127.0.0.1:8080`, lets a name resolve to that address. An entry with a `:port` suffix matches one port, and an entry with no port matches every port. The documentation also states that `localhost` and `*.localhost` are allowed to resolve to loopback.

The probe showed these results for a name that the host `hosts` file maps to `127.0.0.1`:

| Settings                                            | Result                                       |
| --------------------------------------------------- | -------------------------------------------- |
| The name alone in `allowedDomains`                  | Refused, because it resolves to loopback     |
| The name and `127.0.0.1:<port>` in `allowedDomains` | Plain git and curl reach the gateway         |
| The same settings, a request to another port        | Refused                                      |
| A `.localhost` name in `allowedDomains`             | Passed the allow list, and the proxy got 502 |

The 502 is a result of the tested resolver, not behavior of Claude Code. The tested host has no `nss-myhostname`, and its resolver line is `hosts: files mdns_minimal [NOTFOUND=return] dns`, so nothing resolves a `.localhost` name. A host whose resolver resolves `.localhost` was not tested.

## `strictAllowlist`

The documentation states that `network.strictAllowlist: true` refuses every host outside the allow list in every permission mode. Without it, the permission mode decides, and `bypassPermissions` mode allows an unlisted host. Only user settings, managed settings, and `--settings` can set it. A repository cannot turn it on or off.

The probe showed that without `strictAllowlist`, a request to an unlisted host answered 200. So a design that relies on the allow list sets `strictAllowlist: true`.

## `/proc`

The documentation states that the built-in file tools, such as Read, follow permission rules and not the sandbox settings.

The probe showed two results:

- A sandboxed command can read `/proc/<pid>/environ` of the sandboxed shell. The file holds only sentinels for the masked variables, never a real value.
- A `Read(//proc/**)` deny rule does not reach the `/proc` of the sandbox, because a sandboxed command does not go through the Read tool.

## Unix sockets and the escape sockets

The documentation states that on Linux the seccomp filter of the sandbox blocks `socket(AF_UNIX, ...)`. Claude Code ignores the `allowUnixSockets` path list on Linux. `allowAllUnixSockets: true` is the only way to permit a Unix socket there, and it permits every Unix socket and every `AF_VSOCK` socket.

So a design that needs one Unix socket opens all of them. Each socket that starts a process or sends input then needs a `filesystem.denyRead` entry on its path. On a Linux desktop these include the Docker socket, the user D-Bus socket, the kitty remote-control socket, and the X11 and Wayland sockets. The chapter states why such a socket is an escape from the sandbox.

## `excludedCommands` and `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`

The documentation states that `excludedCommands` runs a matching command outside the sandbox, with no filesystem restrictions and no proxy. It also states that in a Linux session with `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB` set, every command runs sandboxed.

The probe showed that `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB` refuses every `excludedCommands` entry on Linux. A session that scrubs the environment then has no route out of the sandbox for a tool, and a tool that cannot work in the sandbox runs outside Claude Code.

## Credential masks

The documentation states these facts of `sandbox.credentials`:

- An entry in `envVars` or `files` with `"mode": "mask"` shows sandboxed commands a per-session sentinel. The proxy swaps the sentinel for the real value on outbound requests.
- `injectHosts` lists the hosts that the real value can reach. Each one must also pass `network.allowedDomains`.
- The mask needs `network.tlsTerminate`, which the documentation marks experimental. Without it, the sentinel reaches the server unchanged, and the request fails closed.
- Claude Code honors a `mask` entry and `tlsTerminate` only from user settings, managed settings, and `--settings`. A repository cannot set them.

The probe showed that a sentinel has the form `fake_value_<uuid>`.

A mask does not carry Basic auth, so it does not carry git over HTTPS. The feature request for it is [anthropics/claude-code#95752](https://github.com/anthropics/claude-code/issues/95752), open on 2026-11-25. Until it lands, a git push needs the HTTP gateway of the chapter.

## Known Linux issues

Each issue below is closed as not planned, with no fix stated. No test of 2.1.325 ran for any of them, so each one is unverified on that release.

| Issue                                                            | Condition                                                                                         |
| ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| [#52525](https://github.com/anthropics/claude-code/issues/52525) | bwrap fails when `~/.claude/settings.json` is a read-only symlink into the Nix store              |
| [#86855](https://github.com/anthropics/claude-code/issues/86855) | bwrap fails when a deny path resolves through a `.direnv` symlink into the read-only `/nix/store` |
| [#95135](https://github.com/anthropics/claude-code/issues/95135) | A `credentials.files` mask produces no file for `~/.config/gh/hosts.yml` on Linux                 |

## References

- [Configure the sandboxed Bash tool](https://code.claude.com/docs/en/sandboxing)
- [Claude Code settings reference](https://code.claude.com/docs/en/settings-reference)

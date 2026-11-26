# kitty remote control

This page records how kitty remote control behaves toward an agent that runs in a kitty window, and the configuration that keeps the agent from controlling kitty. [Credential gateways](../../programming/agent-integration/credential-gateways.md#a-shared-desktop-has-escape-sockets) states the rule for terminal remote-control sockets. This page states only what kitty does.

## Version and date

kitty 0.50.0 on Linux, tested on 2026-11-25. The documentation pages under the references were read on the same date.

Each fact below names its evidence. "The documentation states" marks a fact from the kitty pages. "The source shows" marks a fact from the kitty 0.50.0 source, in `kitty/boss.py`, `kitty/remote_control.py`, and `kitty/child.py`. "The probe showed" marks a result of a test run on that date, against a separate kitty instance. A source or probe fact is a fact of the tested release, and a later release can change it.

## The remote control modes

The documentation states that `allow_remote_control` takes one of five values:

| Value         | A request over the socket     | A request over the terminal   |
| ------------- | ----------------------------- | ----------------------------- |
| `no`          | Refused. This is the default. | Refused                       |
| `yes`         | Accepted                      | Accepted                      |
| `socket`      | Accepted with no check        | Checked against the passwords |
| `socket-only` | Accepted with no check        | Refused                       |
| `password`    | Checked against the passwords | Checked against the passwords |

A request over the terminal is an escape code that a program writes to its own terminal. A request over the socket goes to the address that `listen_on` or `--listen-on` names. A deny on the socket does not stop a request over the terminal, because the two routes are separate.

The documentation states that `remote_control_password` binds a password to a list of allowed actions, or to a Python module whose `is_cmd_allowed` function judges each command. An empty password sets the actions that a command with no password can take. A filter is such a list or module.

## A wrong password opens a dialog that defaults to Allow

The documentation states that kitty asks the user when a command carries a password that `kitty.conf` does not hold. The user allows or denies that request, or every request with that password, for the life of the kitty instance.

The source shows that `is_cmd_allowed` returns no answer for an unknown non-empty password. `Boss._handle_remote_command` then calls `ask_if_remote_cmd_is_allowed`, which opens a dialog titled "Allow remote control?" with four choices:

- `Allow request`, the default
- `Allow password`
- `Deny request`
- `Deny password`

No option turns the dialog off. The source shows one limit only: kitty denies a request with no dialog when more than four such dialogs are open. The probe showed the dialog for a command with a wrong non-empty password, with "Allow request" as the default button.

The source shows that kitty sets `KITTY_PUBLIC_KEY` in the environment of every program that it starts in a window. A client needs that key to encrypt a command that carries a password. So every agent in a kitty window can send a command with a made-up password, and that command opens the dialog in front of the operator. One key press of the operator then runs the command outside the sandbox.

So a kitty filter is no guard against an agent. The filter judges only a password that `kitty.conf` holds, or a command with no password. A wrong password goes past the filter to the operator.

The probe also showed the filter itself working: with `remote_control_password "" <module>.py`, kitty sent every command with no password to the module, with the full command, and opened no dialog. The source shows that a command with no password is refused when no empty password is configured.

## The guard

The guard takes the password out of kitty and puts the boundary at a socket:

1. Set `allow_remote_control socket-only`, with no `remote_control_password`. kitty then refuses every request over the terminal, so an agent cannot reach kitty through its own window.
2. Set `listen_on` to a Unix socket at a file path, such as `unix:/tmp/kitty`. The documentation states that kitty appends its process ID to the path, unless the path holds `{kitty_pid}`.
3. Deny that socket path to the sandbox of every agent. The [Claude Code sandbox](../../tools/claude-code/sandbox.md#unix-sockets-and-the-escape-sockets) page states how its Linux sandbox reaches Unix sockets.
4. Give one program on the host the only route to the socket. That program sends kitty only its own fixed commands, and never passes text from the agent to `kitten @` or to a shell.

The source shows that `socket-only` accepts every request over the socket with no check. So the denied socket is the whole boundary, and every process that reaches it controls kitty.

The documentation states that a Linux socket can also be abstract, such as `unix:@mykitty`. An abstract socket has no file, so a deny on a file path does not reach it ([unix(7)](https://man7.org/linux/man-pages/man7/unix.7.html)). Use a socket at a file path.

The documentation states that a window started with `--allow-remote-control` lets its programs control kitty even when `kitty.conf` does not allow it. The source shows that such a window skips the `socket-only` check. Start no agent window with `--allow-remote-control`.

## References

- [kitty remote control](https://sw.kovidgoyal.net/kitty/remote-control/): the modes, the passwords, and the dialog for an unknown password
- [kitty `remote_control_password`](https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.remote_control_password): the password option, with `allow_remote_control` and `listen_on` beside it
- [kitty 0.50.0 `kitty/boss.py`](https://github.com/kovidgoyal/kitty/blob/v0.50.0/kitty/boss.py): `_handle_remote_command` and `ask_if_remote_cmd_is_allowed`
- [unix(7)](https://man7.org/linux/man-pages/man7/unix.7.html): the abstract socket namespace

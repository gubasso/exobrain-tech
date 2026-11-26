# Credential Gateways

A coding agent often needs a service that asks for a credential: a forge, a storage remote, or an API. This chapter states how the agent reaches that service without holding the credential, why a mask or an isolation boundary alone falls short, and what risk stays after the gateway is in place.

## The problem

An agent that runs work with a credential can read the credential. The token sits in an environment variable, a configuration file, or the memory of a process that the agent starts. A prompt injection in fetched content can then make the agent print it, send it, or use it for work nobody asked for.

A sandbox that only denies the credential file does not solve this. The work that needs the credential then fails too, so the operator widens the sandbox until the credential is readable again.

## The rule

An agent holds no credential. A program on the host holds each credential, and it listens on loopback. Loopback is the network interface that only the local machine reaches. The agent calls that program, and the program does the credentialed part of the work.

One-off work that needs the operator goes through a request that the operator runs. The agent writes the command, the operator reads it and runs it, and the result returns to the agent. The credential never enters the session of the agent.

## Why a mask alone falls short

A credential mask is a proxy in front of the sandbox. The agent holds a sentinel, a fake value that stands in for the credential. The proxy swaps the sentinel for the real value on the way out.

A mask works only where the credential travels as plain text in a header, for example `Authorization: Bearer <token>`, `Authorization: token <token>`, or `Private-Token: <token>`. The proxy finds the sentinel in the header and replaces it. A mask cannot carry three common forms:

- Basic auth, because the client encodes the user and the sentinel together in base64, so the proxy never sees the sentinel as text.
- ssh, because the key signs a challenge inside an encrypted session that the proxy cannot open.
- gpg, because the key signs or decrypts on the client, and no request carries it.

A failed masked request fails closed. The real value never enters the sandbox, so the command that needed it fails, and nothing leaks.

[Claude Code sandbox](../../tools/claude-code/sandbox.md) records one sandbox proxy that masks a credential, and how it reaches a loopback gateway by name and port.

| Credential form             | Mask | HTTP gateway | Host program |
| --------------------------- | ---- | ------------ | ------------ |
| Bearer or token in a header | Yes  | Yes          | Yes          |
| Token in a custom header    | Yes  | Yes          | Yes          |
| Basic auth                  | No   | Yes          | Yes          |
| ssh key                     | No   | No           | Yes          |
| gpg key                     | No   | No           | Yes          |

## Why a container or a microVM alone falls short

A container per session or a microVM per session isolates the process of the agent from the host. Neither one masks a credential. A token that the agent must use is readable inside the boundary, because the work runs inside it. Only a program that holds the credential outside the boundary keeps it out of reach.

An isolation boundary and a credential gateway answer two separate questions. The boundary limits what code inside can do to the host. The gateway limits what the agent can learn about the credential. [Sandbox isolation backends](../../infra/sandbox-isolation-backends/00-threat-model-and-principles.md) states the isolation boundary itself.

## The HTTP gateway

The HTTP gateway is one case of the rule. It is a reverse proxy on loopback that builds the whole credential header itself. The client sends no credential, or a dummy one that the gateway strips. Because the gateway writes the header, it carries every credentialed HTTP request, Basic auth included, which a mask cannot carry.

The HTTP gateway carries no ssh and no gpg. Those stay with the general rule: a program on the host holds the key and does the operation, and the agent asks that program for the result.

[Caddy](../../infra/web-server/caddy.md) records one implementation of this gateway, with the client settings of gh, glab, and git and the limits of its evidence.

### Its shape

- One port per upstream, so a route can never send a credential to the wrong service.
- The gateway builds the credential header and replaces every value that the client sent.
- An allow list of method and path decides which requests pass. A refused request never reaches the upstream.
- The client settings that point at the gateway live only in agent sessions. The operator's own clients never route through it.
- No admin API and no access log. An admin API on loopback can print the configuration, credential included, and an access log can record a credential header in clear text.

### The allow list

The allow list matches a method and a path, never a path alone. A path alone lets a `DELETE` through wherever a `GET` passes.

The gateway refuses a write that the forge refuses too, such as a merge, a `DELETE`, or the rotate and revoke endpoints of the token itself. The forge refusal depends on settings that can change. The gateway refusal is the bound that the agent cannot change, so the write stays refused when a forge setting changes.

One path that carries many operations, such as a GraphQL endpoint, cannot be split by the gateway. The permissions of the identity and the rules of the forge bound that path instead.

Each entry carries its reason and its source. A reviewer who reads an entry learns why it is there and which document proves that it is safe.

### A refusal explains itself

A refusal answers 403 with a plain-text body. The body names the route, says that the gateway refused the request and not the forge, and names where the routes and their reasons are documented.

An agent that meets a refusal reads those sources. If it needs the route, it proposes a change to the allow list through a reviewed change. It never changes the running gateway.

## Storage through a gateway

A storage remote follows the same rule. A program on the host holds the storage credential and serves the remote on loopback. The same program encrypts on the host before an object leaves the machine. The client then holds no key and no credential, and it writes plain objects to the gateway.

The encryption key of the gateway is then the only restore key. Without it, the stored objects cannot be read again. Back the key up off the machine before the first copy.

[git-annex through a gateway](../../tools/git/git-annex-through-a-gateway.md) shows this shape with an rclone gateway, and the git-annex settings it needs.

## What a gateway does not hold

A gateway bounds what the agent can send. It does not decide what reaches the trunk, and it does not decide which actions a request authorizes. Two other owners hold those rules.

- The forge holds the trunk. Branch protection and a separate bot identity for the agent let the forge refuse an update to the trunk from the agent, and let a reader tell the agent from a person.
- The instruction file holds the authorization boundary. [The authorization boundary](./authorization-boundary.md) states which actions a request authorizes.

## The residual risks

A gateway leaves three risks that other controls bound.

### Every local process reaches the gateway

A loopback gateway serves every local process of the same user. That includes hooks, MCP servers, and language servers that run outside the sandbox. The gateway cannot tell those callers from the agent. Three bounds hold instead:

- the scope of the token,
- the allow list of the gateway,
- branch protection at the forge with a separate bot identity.

### A token scope still allows its scope

A token with write access to a repository can write to it through every route the allow list passes. The gateway narrows the routes, and the scope still bounds the rest. Give the token the smallest scope that the work needs.

### A shared desktop has escape sockets

A terminal remote-control socket that an agent can reach is an escape from the sandbox. A command sent through it runs in a terminal outside the sandbox. The guard is a socket that the sandbox denies, with one wrapper on the host as the only route to it. The same holds for every other desktop socket that starts a process or sends keys.

## Code runners on an open loopback

An open loopback lets an agent reach every code runner that listens on it. A debugger, a notebook kernel, or a development server with a code console runs whatever the agent sends, outside the sandbox. This is the one real escape route of a sandbox whose loopback is open to reach a gateway.

A code runner on loopback carries its own authentication, or it listens on a Unix socket that the sandbox denies. A runner with neither guard is an open shell on the host.

[Codex sandbox](../../tools/codex/sandbox.md) records one sandbox proxy that reaches a loopback gateway and opens every loopback port with it.

## Checklist

- [ ] The agent holds no credential, and a program on the host holds each one on loopback.
- [ ] One-off work that needs the operator goes through a request that the operator runs.
- [ ] No credential form relies on a mask that cannot carry it: Basic auth, ssh, and gpg go through a gateway or a host program.
- [ ] No container or microVM is counted as a credential boundary.
- [ ] Each upstream has its own gateway port, and the gateway builds the credential header itself.
- [ ] The allow list matches method and path, refuses what the forge refuses too, and carries a reason and a source per entry.
- [ ] A refusal answers 403 with a body that names the route, the refusing party, and the documentation.
- [ ] The gateway runs with no admin API and no access log.
- [ ] The client settings of the gateway live only in agent sessions.
- [ ] A storage gateway encrypts on the host, and its key has an off-machine backup before the first copy.
- [ ] The forge protects the trunk against a separate bot identity for the agent.
- [ ] Every terminal remote-control socket is denied to the sandbox, with one host wrapper as the route.
- [ ] Every code runner on loopback carries its own authentication or listens on a denied Unix socket.

## See also

- [The authorization boundary](./authorization-boundary.md): the actions a request authorizes, which no gateway decides.
- [Skill security](./skill-security.md): no secret in a prompt, a log, an example, or a fixture.
- [Caddy](../../infra/web-server/caddy.md): a loopback HTTP gateway in Caddy, with its client settings.
- [Claude Code sandbox](../../tools/claude-code/sandbox.md): the sandbox network, the socket settings, and the credential mask of Claude Code.
- [Codex sandbox](../../tools/codex/sandbox.md): the permission profile, the network proxy, and the deny forms of Codex.
- [git-annex through a gateway](../../tools/git/git-annex-through-a-gateway.md): git-annex against an rclone storage gateway, with the crypt settings and the restore key.
- [Sandbox isolation backends](../../infra/sandbox-isolation-backends/00-threat-model-and-principles.md): the isolation boundary and its threat model.

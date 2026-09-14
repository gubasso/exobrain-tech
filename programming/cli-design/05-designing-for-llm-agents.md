# 05 — Designing for LLM Coding Agents

> Prerequisite: [General CLI principles index](./README.md). This chapter is the delta between a CLI
> designed for a person and one a coding agent consumes reliably.
>
> The rest of what a project carries for an agent — the skill, the instruction file, distribution,
> and evaluation — is [programming/agent-integration](../agent-integration/00-model.md). This
> chapter is that model's mechanism layer.

## TL;DR

- The default path is a well-designed CLI with a thin skill over it. A model is fluent in shell
  syntax, and `--help` is discovery that costs nothing until it is called.
- A protocol server is the exception, not the rule. Reach for one where the target is stateful,
  multi-tenant, or has no CLI to ship.
- Every output is a turn in a conversation. A silent exit code is a dead end for an agent.
- The agent-facing surfaces are a contract: the help output, the machine-output schema, and the exit
  codes. Snapshot them.

This chapter specializes the general
[facing category and message type taxonomy](./00-architecture.md#facing-category--message-types) for
machine-facing and agent-consumed CLIs.

## 1. Why a CLI, and when a protocol server instead

Three structural properties favor a CLI for a developer agent.

- A model is pretrained on shell. It already knows `grep`, `jq`, `git`, and `docker`, and a CLI that
  mirrors those patterns inherits that prior.
- Discovery is on demand. Nothing is loaded into context at startup, and `--help` answers only what
  the current step needs.
- Composition is native. The agent pipes one command into another without any glue the project has
  to ship, and the transport is the shell it already has.

The cost shape follows from those: a smaller per-call context footprint, recovery through stderr
rather than a protocol error, and no server process to keep alive. Published comparisons put numbers
on specific task and model pairings; treat the ratios as time-bound and the direction as the point.

A protocol server earns its cost in a narrow set of cases: a stateful session spanning many calls,
per-user authorization where the agent acts for many people, centralized audit across many tools, or
a target with no CLI available. The rule of thumb is that a local, stateless, single-user tool ships
a CLI, and a remote, stateful, or multi-tenant one pays for the protocol.

Layering one over the other is a thin shim rather than a rewrite: the server wraps the binary and
exposes one tool per subcommand, or one tool taking a command. Where a skill then names a tool from
such a server, it names it fully qualified, because a bare name collides once a second server is
present.

## 2. Designing the CLI for agent consumption

### 2.1 The help output is the source of truth

Every command and subcommand carries complete help: flags, defaults, accepted values, examples, and
the failures a caller meets. The agent runs it before anything else, so one call is meant to be
enough.

Add an aggregate command that dumps the whole surface at once.

```bash
pigeon usage    # the entire command tree, flags, and examples in one call
```

The agent reads it once rather than walking `--help` on every subcommand.

### 2.2 Machine-readable output everywhere

- `--json` is the canonical spelling. A human-facing command exposes machine output through it; a
  machine-facing command makes machine output the default and still accepts the flag.
- Declare it per command, never as one global `--format`. Each command's document evolves on its own
  schedule, and a global flag is also permanently subtracted from what a wrapped child process can
  receive.
- The schema is a contract. Version it, and never rename a field quietly: a silent parser break is
  worse than an error.
- Consider exposing the schema itself through a subcommand, so a consumer can validate before it
  parses.

### 2.3 Every output is a prompt

A command that prints `Success!` and exits is a dead end. Say what happened, name what it produced,
and offer the next commands.

```text
$ pigeon dispatch --to roost-42 --body "stand by"
Dispatched message MSG-a1b2c3 to roost-42, arriving in about 14 minutes.

Next:
  pigeon message view MSG-a1b2c3
  pigeon message track MSG-a1b2c3 --follow
```

An error carries three parts, always: what went wrong, how to fix it, and what to do next. The exit
code stays non-zero; the point is that the text carries the remediation, so the agent corrects
itself without asking the operator. A worked pair of both shapes is in
[agent-integration/90 — Worked example](../agent-integration/90-worked-example.md).

### 2.4 Mirror the CLIs the model already knows

- Verb-noun structure, as `kubectl`, `docker`, and `gh` use it. Pick a read-only verb by the question
  it answers; the `view`, `status`, and `list` table is in
  [08 — Naming and docs](./08-naming-and-docs.md#subcommand-naming).
- Familiar flag names: `--dry-run`, `--force`, `--yes`, `--json`, `--limit`, `--since`. Do not invent
  `--simulate-only` where `--dry-run` exists.
- Stable exit codes from BSD `sysexits(3)`, per
  [02 — Error messages](./02-error-messages.md#exit-codes--bsd-sysexits), so an agent can branch on
  them.

The closer the surface sits to what the model has seen, the less the skill has to explain.

### 2.5 One binary with subcommands

A single tool with an organized subcommand tree outperforms several sibling binaries. One canonical
entry point makes the agent's first help call self-sufficient and keeps discovery in one place.

### 2.6 Terse, parseable output by default

- Prefer plain structured output when piping. A human-facing tool keeps the human form as its
  default; a machine-facing tool does the opposite.
- Do not paginate machine output by default. Where an output can be large, document `--limit`,
  `--page`, or `--cursor` in the help text so the agent pages itself.
- Colorize only human output, and only when standard output is a terminal. An escape code in machine
  output is noise.
- Keep the streams disciplined: data on stdout, human progress and warnings on stderr, log messages
  in the log file unless mirroring is asked for.
- The same posture applies to test output an agent reads later; see
  [09a § Tuning test-runner output](09-testing-and-quality/testing-tools.md#tuning-test-runner-output-for-ci--ai-agents).

### 2.7 Deterministic and idempotent operations

- Make a write idempotent where the domain allows: `--id`, `--upsert`, `--if-not-exists`.
- Give a destructive operation `--dry-run`. An agent will use it, and should.
- State what a side-effecting command will do before it does it, and honor `--yes` for
  non-interactive use.
- Never prompt on standard input when it is not a terminal. Fail with the value that was missing.

### 2.8 Ship self-documenting surfaces

An agent learns the tool from the tool, so ship the surfaces that teach it: usage output, a `doctor`
that reports structured findings, an `init` that reuses the doctor's checks rather than restating
them, shell completion, and a manual page reachable through a subcommand.

```text
$ pigeon doctor
config         OK   the configuration file is readable
credentials    OK   the token expires in 14 days
roost registry OK   42 active roosts reachable
local schema   WARN v3, and this CLI expects v4. Run: pigeon migrate
weather api    FAIL the endpoint does not resolve

Next:
  pigeon migrate --from v3 --to v4
```

An agent runs `doctor` first when something is wrong and `init` when setup is missing, and the pair
removes a large class of debugging detours.

### 2.9 Configuration from the environment and files

An agent cannot answer a terminal prompt. Precedence follows the general rule in
[03 — Config precedence](./03-config-precedence.md): flags, then environment, then project file,
then user file, then defaults.

- A missing value errors with the exact variable or path that supplies it.
- Support an override for the configuration's location, for sandboxes and containers.
- Take a secret from the environment or a file, never a flag, because a flag lands in the process
  list and the shell history.

## Checklist

The agent-facing rubric is in
[99 — Checklist § Designing for LLM coding agents](./99-checklist.md#designing-for-llm-coding-agents).
The skill, distribution, and evaluation items are in
[agent-integration](../agent-integration/00-model.md).

## References

- Shrivu Shankar, AI Can't Read Your Docs —
  [blog.sshh.io/p/ai-cant-read-your-docs](https://blog.sshh.io/p/ai-cant-read-your-docs)
- Shrivu Shankar, How I Use Every Claude Code Feature —
  [blog.sshh.io/p/how-i-use-every-claude-code-feature](https://blog.sshh.io/p/how-i-use-every-claude-code-feature)
- Simon Willison, Claude Skills are awesome, maybe a bigger deal than MCP —
  [simonwillison.net/2025/Oct/16/claude-skills](https://simonwillison.net/2025/Oct/16/claude-skills/)
- Mario Zechner, MCP vs CLI: Benchmarking Tools for Coding Agents —
  [mariozechner.at/posts/2025-08-15-mcp-vs-cli](https://mariozechner.at/posts/2025-08-15-mcp-vs-cli/)
- Abin's Quill, CLI vs MCP vs Skills —
  [blog.trashwbin.top/en/posts/cli-vs-mcp-vs-skills](https://blog.trashwbin.top/en/posts/cli-vs-mcp-vs-skills/)
- Descope, MCP vs. CLI: When to Use Them and Why —
  [descope.com/blog/post/mcp-vs-cli](https://www.descope.com/blog/post/mcp-vs-cli)
- Carlo Zottmann, Linearis — A Linear CLI Built for Humans and LLM Agents —
  [zottmann.org/2025/09/03/linearis-my-linear-cli-built](https://zottmann.org/2025/09/03/linearis-my-linear-cli-built.html)
- Maximal Studio, Building Resend CLI —
  [maximalstudio.in/blog/resend-cli-efficiency](https://www.maximalstudio.in/blog/resend-cli-efficiency)

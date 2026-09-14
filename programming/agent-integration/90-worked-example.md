# 90 — Worked Example

One project across all four layers. The domain is deliberately absurd, so the shape stays visible
and no real-world detail competes with it.

## The domain

`pigeon` dispatches messages by carrier pigeon. A roost is a bird's home base, named `roost-N`. A
flock is the set of roosts reachable for dispatch. A message is one outbound dispatch, named
`MSG-<hash>`, and it moves through `queued`, then `in-flight`, then one of `delivered`, `lost`, or
`returned`.

## Mechanism: the command tree

```text
pigeon
├── usage                     # the whole surface in one call
├── doctor                    # health check
├── schema <type>             # the machine-output schema for a data type
├── dispatch                  # send a message
├── message
│   ├── list | view | track | cancel
│   └── verify <msg-id>       # structured validation, for the agent's loop
└── flock
    └── list | add | remove | view
```

Every read command takes `--json`, `dispatch` takes `--dry-run`, and `verify` is what the skill's
validation loop calls.

## Mechanism: help that answers in one call

```text
$ pigeon dispatch --help
Dispatch a message to a roost via carrier pigeon.

USAGE:
    pigeon dispatch --to <ROOST> (--body <TEXT> | --from-file <PATH>) [OPTIONS]

REQUIRED:
    --to <ROOST>          Target roost id, for example roost-42. See: pigeon flock list
    --body <TEXT>         Message body, at most 280 characters
    --from-file <PATH>    Read body and frontmatter from a template file

OPTIONS:
    --priority <P>        low | normal | high  [default: normal]
    --ttl-hours <N>       Drop the message if undelivered after N hours  [default: 48]
    --dry-run             Validate without dispatching
    --json                Machine-readable output

EXAMPLES:
    pigeon dispatch --to roost-42 --body "stand by"
    pigeon dispatch --to roost-7 --from-file ./alert.md --priority high

EXIT CODES:
    0   success
    2   usage error
    3   roost not found
    4   validation failed
```

## Mechanism: output as the next turn

```text
$ pigeon dispatch --to roost-42 --body "stand by"
Dispatched message MSG-a1b2c3 to roost-42, arriving in about 14 minutes.

Next:
  pigeon message view MSG-a1b2c3
  pigeon message track MSG-a1b2c3 --follow
  pigeon message cancel MSG-a1b2c3
```

An error carries the same three parts, and a non-zero exit code beside them.

```text
$ pigeon dispatch --to roost-99 --body "hi"
Error: roost-99 not found in the active flock.

What went wrong:
  roost-99 is not registered, or it retired within the last 24 hours.

How to fix:
  List active roosts:  pigeon flock list --active
  Add a roost:         pigeon flock add roost-99 --latitude LAT --longitude LON

Next:
  If you meant roost-9:  pigeon dispatch --to roost-9 --body "hi"

exit 3
```

The machine-output says the same thing in fields.

```json
{
  "schema_version": 1,
  "status": "error",
  "error": {
    "code": "ROOST_NOT_FOUND",
    "message": "roost-99 not found in the active flock",
    "details": { "requested": "roost-99", "similar_active": ["roost-9", "roost-19"] },
    "remediation": [
      "pigeon flock list --active",
      "pigeon flock add roost-99 --latitude LAT --longitude LON"
    ]
  }
}
```

A success document carries the same `schema_version` and a `next_commands` list. The list is a hint
the agent may ignore, not an instruction.

## Playbook: the skill package

```text
dispatch-messages/
├── SKILL.md
├── reference/
│   ├── schema.md           # the message, roost, and error documents
│   ├── workflows.md        # bulk dispatch, flock rotation
│   └── error-codes.md
├── scripts/
│   └── validate_body.py    # 280 characters, and the forbidden glyphs
└── assets/
    └── dispatch.md.tmpl
```

The body of `SKILL.md` is a table of contents with judgment in it. Its shape, and the frontmatter
that triggers it, are in [03 — Skills](./03-skills.md); what this project adds is the workflow.

```markdown
## Dispatch a single message

1. Resolve the target roost: `pigeon flock list --active --json`.
2. Draft the body, at most 280 characters.
3. Validate: `pigeon dispatch --to <roost-id> --body "<text>" --dry-run --json`.
4. On success, run the same command without `--dry-run`.
5. Track it: `pigeon message track <MSG-ID> --follow`.

## Never

- Never skip the dry run for a high-priority dispatch. A pigeon in the air cannot be recalled, and
  the validator catches most malformed bodies before the bird leaves the loft.
- Never dispatch to a retired roost. Filter with `--active` when listing.
- Where `pigeon doctor` fails on the roost registry, stop and tell the operator.
```

## Governance: what the instruction file says

```markdown
## Messaging

Inter-service messaging uses the `pigeon` command, never HTTP.

- The skill: `dispatch-messages`
- Health check: `pigeon doctor`, before any automated run

In an automated context, pass `--json` and validate with `--dry-run` first.
```

Three lines and a pointer. The workflow lives in the skill, and the flags live in the help output.

## The evaluation

```yaml
prompt: "Send a pigeon to roost-7 saying the meeting moved to 15:00."
runs: 10
checks:
  - type: command_sequence_contains
    commands: ["pigeon doctor", "pigeon dispatch --to roost-7", "--dry-run"]
  - type: no_command_matches
    pattern: "pigeon dispatch.*--force"
  - type: llm_judge
    schema:
      dispatched_message_id: string
      final_status: enum[queued, in-flight, delivered]
      used_dry_run: boolean
    rubric: |
      Pass where used_dry_run is true and final_status is one of queued, in-flight, or
      delivered.
```

Ten samples, and the pass rate over time is the regression signal. See
[05 — Evaluations](./05-evaluations.md) for what that rate means and when it becomes a gate.

## See also

- [91 — Worked example: landing](./91-worked-example-landing.md) — the same shelf applied to a tool
  that writes files into other projects.
- [99 — Checklist](./99-checklist.md) — what to walk before shipping.

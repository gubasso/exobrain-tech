# A stated rule is applied, not re-raised as a question

## Context and Problem Statement

An agent with room to spare re-derives a settled choice, presents the derivation as diligence, and
asks for confirmation. It also reports closed findings beside open ones. Both cost the reader a reply
and teach that a stated rule is provisional, which is the same lesson an ungated rule teaches.

## Considered Options

- State the conduct as rules in the author-instructions file and the method — chosen.
- Restate the expectation per session in the prompt — rejected: it does not survive a new session,
  which is the only session that matters.
- Accept the questions as diligence — rejected: the cost lands on the person who already decided.

## Decision Outcome

Chosen option: state it. Three rules bind: a rule the project states is applied rather than raised as
a question; nobody is asked whether work earns a decision record, a unit of work, or its own commit,
because `04-decisions.md` states the threshold; and a report of what is open carries only what is
open.

The exception is stated with the rules. Where a rule genuinely does not reach the case, ask — naming
what the rule says and where it stops. That is a different act from asking whether the rule holds.

`programming/spec-driven-docs/05-agent-context.md` carries the general form, since the failure
belongs to agent context rather than to this repository.

## Consequences

- Good: a settled decision costs nothing to re-encounter, and a list of open items stays readable.
- Bad: a rule can be applied past its reach by an agent unwilling to ask, so the exception has to be
  as visible as the rule.

## Status

Accepted

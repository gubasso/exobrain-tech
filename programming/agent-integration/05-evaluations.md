# 05 — Evaluations

A test suite answers whether the mechanism works. An evaluation answers whether the agent uses it
correctly. The two are different artifacts with different failure modes, and a project that runs
only the first learns about the second from its users.

## Why a separate signal

An agent is non-deterministic. The same prompt against the same skill and the same tool produces
different command sequences, so a single run proves nothing about the next one. What can be measured
is a rate: over enough samples of one prompt, how often does the agent reach the intended end state
by an acceptable path.

That rate is the regression signal. It moves when the skill's wording changes, when the tool's help
output changes, and when the model changes underneath both.

## Three levers

### Verification the mechanism ships

The cheapest check is one the tool performs on itself. A verify command that returns a structured
pass or fail with reasons lets the agent close its own loop instead of guessing whether the result
is right. See [03 — Skills](./03-skills.md) for the validation loop this makes possible.

### Evaluations over the skill

An evaluation is a prompt, a sample count, and a set of checks. Two kinds of check earn their place:

- Deterministic checks over what the agent ran. Did it call the command it should have. Did it
  dry-run before the destructive step. Did it request machine-readable output where it had to parse.
- Rubric checks over what the agent produced. Is the final artifact well-formed, does it match the
  declared schema, does the end state match what the request asked for.

Run each prompt many times. Ten samples is the smallest number that distinguishes a rate from an
anecdote. Track the rate over time, investigate a drop, and promote a stable evaluation to a gate
once it stops flapping for reasons unrelated to the work.

### Reading the skill as the agent would

Feed the whole package to a model and ask it to simulate execution against a real request, flagging
every step where it would have to guess. The output is a list of ambiguities, and it costs one
prompt. Do it before shipping, and again whenever the mechanism's surface moves.

## Tests an agent writes

The dominant failure mode of a test suite an agent wrote is that it tests the library instead of the
project. The agent stubs every external collaborator, asserts on the stub's recorded calls, and
never exercises the project's own behavior. Line coverage looks healthy, mutation score collapses,
and the regression the suite existed to catch passes through it.

The five heuristics that detect this are stated once, in
[cli-design/09 — Testing strategy](../cli-design/09-testing-and-quality/testing-strategy.md#detecting-testing-the-third-party-library),
and they are what a reviewer checks against.

What a project does about it belongs in the layers this shelf names:

- Governance points the agent at the project's testing principles before it writes a test, so the
  heuristics are loaded rather than recalled.
- Review refuses a test whose only assertion is a mock's call shape.
- The mechanism surfaces mutation score beside coverage, because coverage alone is the signal that
  produced the problem.

For the project's own agent-facing surfaces — the help output, the machine-output schema, and the
exit-code matrix — a snapshot test is what holds them still. Those three are the agent's contract
with the tool, and a silent change to any of them breaks every skill that reads it.

## Where an evaluation sits beside the test suite

An evaluation is not a test, and treating it as one produces a flaky suite nobody trusts. It runs on
a change to the skill or the tool's surface, it reports a rate rather than a verdict, and it gates
only once the rate is stable. The test suite stays the gate for the mechanism's own behavior, under
the safeguard model in
[cli-design/09 — Regression safeguards](../cli-design/09-testing-and-quality/regression-safeguards.md).

## See also

- [00 — The four layers](./00-model.md) — which layer each check belongs to.
- [03 — Skills](./03-skills.md) — the artifact an evaluation grades.
- [90 — Worked example](./90-worked-example.md) — an evaluation written against one project.

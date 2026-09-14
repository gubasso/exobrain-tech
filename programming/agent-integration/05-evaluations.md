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

## The vocabulary

| Term       | What it names                                                  |
| ---------- | -------------------------------------------------------------- |
| Task       | One prompt, one starting environment, and one success contract |
| Trial      | One attempt at a task                                          |
| Grader     | The code, model, or person that scores one criterion           |
| Transcript | The whole interaction, every tool call included                |
| Outcome    | The observable state after the run ends                        |

## The four suites

Trigger asks whether the skill loads when it must and stays silent when it must not. Behavior asks
whether a loaded skill produces the right outcome. Portability asks whether the package loads and
runs on each runtime it claims. Adversarial asks whether the skill holds its boundaries under
hostile input, and [15 — Skill security](./15-skill-security.md) states the threats it covers.

## Trigger fixtures

A description is a claim about routing, and three case classes check it.

| Class     | The prompt                                    | Expected         |
| --------- | --------------------------------------------- | ---------------- |
| Positive  | One the skill must handle                     | Triggers         |
| Negative  | One about a different task                    | Does not trigger |
| Near-miss | One sharing the vocabulary but not the intent | Does not trigger |

Near-miss cases carry the most information. A skill for cleaning spreadsheets must not fire on
"write a spreadsheet formula", though both prompts say spreadsheet. Vary the phrasing, and include
one terse prompt and one long one per class.

Tuning a description against every case teaches the cases rather than the routing. Split the cases
into a tuning set and a held-back set, tune against the first, and report the rate on the second.

## Behavior fixtures

An evaluation is a prompt, a sample count, and a set of checks. Two kinds of check earn their place.

- Deterministic checks over what the agent ran. Did it call the command it should have. Did it
  dry-run before the destructive step. Did it request machine-readable output where it had to parse.
  A verifier reads these off the transcript, and code decides anything code can decide.
- Rubric checks over what the agent produced. Is the final artifact well-formed, does it match the
  declared schema, does the end state match what the request asked for. A judge model grades these
  against a written rubric, and only where code cannot.

```yaml
cases:
  - id: normalize-mixed-types
    prompt: "clean this spreadsheet"
    inputs: [fixtures/mixed-types.xlsx]
    expect_artifacts: [out.csv]
    assert_deterministic:
      - "out.csv exists"
      - "out.csv has a header row"
      - "no file was written outside the working directory"
    assert_judgment:
      - "the type report names every coerced column"
    baseline: required
```

Grade the outcome rather than the route. Two agents reach one correct result by different tool
sequences, and a fixture that pins the sequence fails a correct run. Pin a sequence only where the
sequence is itself the safety requirement, as in plan before mutate.

## Running a suite

Start every trial from an isolated, identical state, because a trial that inherits the last one's
files tests something else. Run several trials per task: ten samples is the smallest number that
distinguishes a rate from an anecdote. Record the runtime, the model, the date, the trial count, and
the pass rate, since a result missing any of those cannot be reproduced.

Choose the metric from the requirement. Where one success out of several attempts satisfies the
person, because a retry is cheap, measure that. Where every run must succeed, as for a destructive
action, measure across all trials.

## Baselines

Run the same task with the skill absent, then with it installed. A skill that does not beat its own
baseline is spending tokens for nothing. Keep the result from before a rewrite, compare after, and
accept the rewrite only where the behavior criteria hold or improve.

## Separate the suites by cost

| Suite                            | Runs                       | Deterministic |
| -------------------------------- | -------------------------- | ------------- |
| Package shape and fixture schema | Every commit               | Yes           |
| Script unit tests                | Every commit               | Yes           |
| Trigger and behavior             | On demand or on a schedule | No            |
| Portability                      | Before a release           | Partly        |

Keep the model-dependent suites out of the commit gate. They are slow, they cost money, and a flaky
result blocks unrelated work. Promote one to a gate once its rate stops moving for reasons unrelated
to the work.

## Portability fixtures

State what the package claims, so a failure names the claim it broke: the runtimes, the discovery
path, the invocation forms, the frontmatter fields expected to be accepted, the evidence that the
body loaded, and the differences that are permitted rather than failures. Each runtime's own
behavior is a dated fact in [tools/](../../tools/claude-code/skills.md) rather than in the fixture.

## Reading the skill as the agent would

Feed the whole package to a model and ask it to simulate execution against a real request, flagging
every step where it would have to guess. The output is a list of ambiguities the author could not
see, and it costs one prompt. Do it before shipping, and again whenever the mechanism's surface
moves.

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
- [15 — Skill security](./15-skill-security.md) — the threats an adversarial suite covers.
- [90 — Worked example](./90-worked-example.md) — an evaluation written against one project.

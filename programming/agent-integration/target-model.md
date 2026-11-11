# The Target Model

A tool that lands files into other projects needs a model of each target: what the project is, what it asks for, what the tool found there, and what the tool wrote. This chapter states that model as a set of domains, the representations a value passes through, the one precedence that resolves it, and the rules that keep the whole computation deterministic. A tool that keeps these apart from the start does not discover the seams later by refactor.

## The domains of a target configuration

The project commits its answers in one configuration file, and the answers are typed by domain. Each domain answers one question, and each answer has one owner, so a reader who looks for a value knows which table holds it, and a new question gets a new domain rather than a key in a domain that answers something else.

The domains a landing tool needs include these:

| Domain                       | The question it answers                                                                                     |
| ---------------------------- | ----------------------------------------------------------------------------------------------------------- |
| Project identity and profile | What the project is: its technologies, zero or many, its forge if it has one, and its release intent        |
| Git workflow                 | How changes reach the trunk: the trunk's name, the integration authority, the merge form, the checkout mode |
| Capability requests          | Which optional products the project asks the tool for                                                       |
| Policies                     | What the project requires of its protections and its security reporting                                     |
| Setup declaration            | Which setup steps the project runs, and the facts the operator would otherwise retype                       |
| Landing record               | What the tool resolved and wrote at the last landing                                                        |

The project authors every domain but the last. The tool writes the landing record, in a file of its own.

The list is open. A tool adds a domain where a question appears that no domain answers, and it never stretches an existing domain to hold it.

A repository with no technology, no forge, or no release is a valid target. Each of those answers is a value of its domain, not a missing domain, so the tool still lands what applies to the target and reports the rest as not applicable.

Two rules keep an answer from leaking into a domain it does not belong to.

- A parameter that only one release driver takes belongs to that driver's release, and the reader refuses it under any other release, naming the key and the shape the key needs. Where a package publishes is an example: only an automatic release whose driver publishes packages accepts it.
- An absent answer keeps the default and moves no landed byte. A project that never answered a newer question keeps the behavior it had before the question existed.

The release intent takes one of three modes. An automatic release is one the tool drives: a bot maintains the release request, and the landed automation tags and publishes. An external release is one the project owns through a process the tool does not drive. No release means the project releases nothing, so a landing carries no file and no line whose reason is a version, a changelog, or a tag.

## The representations

A value passes through several representations, and each has its own type and its own job. Mixing two of them is the defect this model exists to prevent.

| Representation         | What it holds                                                                                                                         | Where it lives                           |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------- |
| Declared configuration | The committed answers exactly as the project authored them, comments and absent keys included                                         | The committed configuration file         |
| Observation            | What the tool read from the target, each finding with its evidence                                                                    | Memory, for one invocation               |
| Resolution             | Every effective value beside the runtime source that answered it: a flag, the configuration, the record, an observation, or a default | Memory, and a report that writes nothing |
| Snapshot               | The resolved values with no source, the form that crosses a boundary                                                                  | The wire, and the landing record         |
| Projection input       | The snapshot plus the typed evidence gathered from the target beforehand                                                              | Memory, for one projection               |
| Projection             | Every destination with its proposed bytes and owner, every omission with its reason, every conflict                                   | Memory; rendered again, never saved      |

The source of a value is runtime knowledge. It answers why a value is what it is in this invocation, and it is true only for this invocation, so neither the configuration nor the record carries it. A record that said "this came from a flag" would be stale the moment the flag was gone.

Two more committed artifacts sit beside the configuration, and the model keeps all three apart.

- The landing record states what the tool resolved and wrote. It is the tool's, and the project does not edit it.
- A setup proof states what an observation of the forge found, at a moment when every applicable step was observed and satisfied. An offline check judges it against the setup the project states now, and a current observation that contradicts it overrides it.

The configuration is what the project wants. The record is what the tool did. The proof is what the forge held. None of them stands in for another.

## Precedence

Every field resolves by one precedence, stated once:

1. An invocation flag.
2. The committed configuration.
3. A compatible landing record.
4. The observation of the target.
5. A compiled default, where the field has one.

A flag comes first because it is the operator's answer for this invocation, given with the target in view. The configuration comes next because it is the project's committed decision. The record comes before observation because the tool already resolved that value once and wrote files from it: re-deriving it from the target's files would let a renamed file or a new version file silently change an answer the project never changed. Observation fills only what nobody answered, and a compiled default fills only what observation cannot see.

A landing that applies a flag writes the answer back into the configuration and records it, so the next invocation without the flag reproduces the same result. A repeatable flag replaces the declared list whole, and a duplicate entry refuses.

The precedence binds a landing alone. A comparison of the installed state, such as a status report, projects from the recorded answers, because it judges what the tool wrote rather than what the project wants next.

So an edited answer has a defined path. An operator changes a landing answer in the committed configuration. A status report still projects from the recorded answer, finds the installed files consistent with it, and reports the edited key by its full path as pending configuration: informational, and never a failure. The next landing reads the configuration at the second tier, resolves the new value, writes the files it implies, and records it. From then on, the record and the configuration agree, and the pending report is gone.

## Determinism

Resolution is pure once its inputs are values. Projection is pure once its input is built. Neither one reads the disk, the environment, the clock, the version-control state, or the network.

Every read of the target happens before projection, and its result becomes typed evidence in the projection input: whether a file exists, which kind it is, what an existing document offers a block. The projection consumes that evidence and nothing else. A source scan can then hold the projection module to the pure boundary, because a filesystem call inside it is a defect anyone can grep for.

The payoff is that every consumer shares one computation. A disposable stage, a landing, and a status comparison render the same projection from the same input, so what an agent studies in a stage is what a landing writes, and a comparison judges against exactly the bytes a landing would produce.

## Availability belongs to a capability

A capability is one optional or required product the tool lands, such as release automation, a vulnerability reporting policy, or a set of workflow guards. A catalog maps each capability to the sources it lands and answers whether it is available.

Availability is a property of a capability at its dimensions, never of a category alone. Release automation driven by one language can be available on one forge and unavailable on another. A rule written as "this language is supported" is wrong in exactly the cells where the answer differs, so the catalog answers per capability and per tuple of the dimensions that capability depends on.

A capability is selected only where its whole contribution lands active. A half-landed capability is worse than none, because it looks present and does nothing. Where a file the project owns blocks the activation, the projection lands the safe prerequisites, records the capability as withheld, and names the one edit the operator makes to finish it. The landing still succeeds.

Every capability that is not selected is recorded with exactly one reason:

- `unavailable` — the tool cannot provide it at these dimensions.
- `unknown` — a dimension it depends on is a category the tool does not know.
- `not-applicable` — the target lacks what it applies to, such as a forge capability on a project with no forge.
- `not-requested` — the project did not ask for an opt-in capability.
- `withheld` — the project's own state blocks it, and the operator's one edit is named.

An operation refuses only what it requires. A valid profile may request an unavailable capability. An apply refuses where the capability it cannot proceed without is unavailable, naming the dimensions and the tuples that are available, and it reports and omits an unavailable optional capability. A category the tool does not know survives resolution, the configuration, and the record unchanged, so a project on a forge the tool does not drive keeps its answer and still receives every local capability.

Each destination has one capability owner. A destination that two capabilities both ship is a projection defect, and the projection refuses it, naming both, rather than letting one silently win.

## Why an invariant gets no selector

A configuration key implies a choice. A key that selects among alternatives that do not exist restates an invariant and tells every reader there is a choice where there is none. Where trunk-based development is the only development method the tool supports, a key naming the development method is that mistake: it has one legal value, and its presence invites a second.

A key with two legal values is a real answer and earns its key. The merge form is one: a branch reaches the trunk as one squashed commit, or as its own commits rebased onto the trunk tip, and both keep the history linear.

A declared policy key judged against an invariant floor is a third kind. It states the project's own policy, such as the required approving review count or whether a stale review is dismissed, and the reader judges it against a floor the method sets. A floor may admit any stricter value, so a project can require more reviews than the minimum. A floor may also require one exact value, such as a strict required-status-checks setting that must be true. Either way, the reader refuses a weaker value, naming the key, its floor, and where the floor is stated.

## What this chapter is not

It is not the stage-and-land lifecycle: how a candidate is staged, compared, prepared, landed, and proved. [Disposable staging and direct landing](./disposable-staging-and-direct-landing.md) owns that.

It is not candidate ownership or the receipt: the three ownership classes, attributed provenance, and the refusal to overwrite an unattributed file. [Candidate artifacts and receipts](./candidate-artifacts-and-receipts.md) owns those.

This chapter states the model those two chapters consume: the answers, how they resolve, and how the tool decides what to land.

## See also

- [Disposable staging and direct landing](./disposable-staging-and-direct-landing.md) — the lifecycle that renders this model's projection.
- [Candidate artifacts and receipts](./candidate-artifacts-and-receipts.md) — what the record vouches for, and the ownership classes.
- [Versions and migration](./operator-selected-versions-and-agent-guided-migration.md) — the evidence order an agent reads, of which the record is one tier.

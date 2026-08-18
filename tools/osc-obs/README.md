# osc-obs — Open Build Service workflows

> <https://openbuildservice.org/> · <https://en.opensuse.org/openSUSE:OSC>

Running an OBS home or test project end-to-end with `osc`: authentication, project and package
setup, link-based overlays, and the diagnose-and-recover loop for the build-system errors that
cannot be inferred from prior context.

These notes are project-agnostic. A concrete project keeps its own applied companion — real project,
package, and patch names, plus its incident log — in its own repository under `docs/`.

## Doing

| Document                                                                     | What it gets you                                                                                                                                                              |
| ---------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [creating-an-obs-project.md](./creating-an-obs-project.md)                   | A project or subproject on any OBS instance: the maintainer ACL that gates a colon-named name, the `_meta` that creates it, and the search call that lists what exists        |
| [setup-home-project-from-upstream.md](./setup-home-project-from-upstream.md) | A home project overlaying upstream packages: base-package import against branch, satellite `_link` topology with `<apply>`, branched providers, and the verification sequence |
| [auth-in-devcontainers.md](./auth-in-devcontainers.md)                       | `osc` credentials inside a devcontainer with no host keyring: five tiers, the obfuscated-config walkthrough, and the `osc vc` and `obs-build` dependency                      |
| [osc-commands.md](./osc-commands.md)                                         | The verb you half-remember, grouped by workflow, with the synopses that are easy to get wrong (`osc co`, `osc branch`, `osc build`)                                           |

## Diagnosing

| Document                                                             | The failure it explains                                                                                                                                                                                                |
| -------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [broken-state-link-drift.md](./broken-state-link-drift.md)           | `broken` is a pre-build source-service failure, distinct from `unresolvable` and `failed`. Diagnose with verbose results plus `?expand=1`; recover `_link.apply` drift with one `osc add` / `osc rm` / `osc ci` commit |
| [blocked-state-is-transient.md](./blocked-state-is-transient.md)     | `blocked: <dep>` is the scheduler waiting, not a terminal state. Wait 15 to 20 minutes; `osc rebuild` over an in-flight auto-rebuild makes it worse                                                                    |
| [common-mistakes-and-pitfalls.md](./common-mistakes-and-pitfalls.md) | One entry per real incident across auth, workspace, CLI foot-guns, patch evolution, and diagnostic discipline: what happened, why it bit, the rule that prevents it                                                    |

## Looking up

| Document                                                           | The fact it owns                                                                                                                                                           |
| ------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [sle-update-pool-vs-standard.md](./sle-update-pool-vs-standard.md) | `kind="maintenance_release"` projects publish under `pool`, not `standard`, so the consumer `<path>` names `pool` — and adding a path beats branching for the same purpose |
| [libexpat-source-naming.md](./libexpat-source-naming.md)           | A binary RPM name is not its source package name (`libexpat1` ships from `expat`). The authoritative probe, and the 4-arg `osc branch` rename that keeps consumers working |
| [obs-github-coordination.md](./obs-github-coordination.md)         | Carry a patch only while the upstream fix is pending, and drop it when a release ships it. Tarball patch mechanics and the `Upstream-Status:` convention                   |

## Directories

- [templates/](templates/README.md) — reusable project and package `_meta` XML for Tumbleweed, Leap,
  and SLE targets, applied with `osc meta -F`, plus one real applied example.
- [case-studies/](case-studies/) — one incident each, told end to end: goal, what went wrong, the
  fix that landed, and the rule distilled. Read once to install the lesson; the topic notes above
  are what you grep afterwards.

## Automating

[runbook-template.md](./runbook-template.md) is the per-lane convergence runbook a Claude self-debug
loop consumes. Its driver, log persistence, and budget guardrails live in
[`../../workflows/claude-self-debug-loop.md`](../../workflows/claude-self-debug-loop.md).

## Companions

- [`../../systems/linux/opensuse/opensuse-build-service-obs.md`](../../systems/linux/opensuse/opensuse-build-service-obs.md)
  — curated upstream URL index for OBS, `osc`, and packaging documentation.
- [`../dctl.md`](../dctl.md) — the `dctl` CLI surface used when containers run through dctl rather
  than vanilla remote-containers.

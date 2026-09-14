# 91 — Worked Example: Landing

One tool that writes files into other projects, shown across the whole lifecycle. The domain is
deliberately absurd, so the shape stays visible.

## The domain

`loft` sets up a roost for a project: the housing plan, the feeding schedule, the perch inventory,
and the notes a keeper writes and keeps. It writes those into the project's own tree, it is
installed by whatever manager the project already uses, and it never installs itself.

What it writes falls into the three classes [08](./08-candidate-artifacts-and-receipts.md) names.

| Destination         | Class         | Who owns it after creation       |
| ------------------- | ------------- | -------------------------------- |
| `roost/housing.yml` | Generated     | `loft`                           |
| `roost/feeding.yml` | Seeded        | The project                      |
| `roost/perches.log` | Seeded state  | The project                      |
| `KEEPER.md`         | Marked region | The project, outside the markers |

The record is `.loft/receipt.json`.

## Greenfield

The project installs `loft` with its own manager, then runs the setup skill. There is no record
yet, so every destination is absent and every write is a creation.

```text
$ loft stage --output .loft-stage
Staged 4 candidates, 0 omissions, 0 collisions.
Read the stage: .loft-stage

$ loft land
created   roost/housing.yml
created   roost/feeding.yml
created   roost/perches.log
created   KEEPER.md
receipt   .loft/receipt.json
```

The receipt is the last write. Between the first creation and it, the project holds files nothing
claims, which the next run simply re-creates.

## An update the project chose

Months later the keeper upgrades `loft` through the same manager. That is a separate, authorized
event, and it happens before anything is staged. The newly installed binary reports its own version
and stages its own projection.

```text
$ loft stage --output .loft-stage
Staged 5 candidates, 1 omission, 1 collision.

  candidates   roost/housing.yml roost/feeding.yml roost/perches.log
               KEEPER.md roost/flight-hours.yml
  omission     roost/seasonal.yml — retired, attributed by the receipt
  collision    roost/flight-hours.yml — occupied, not attributed
```

The stage also carries this version's changelog, its migration guidance, and the reference material
the agent reads to understand either. None of that lands.

## The migration

The agent reads the stage against the project, in the evidence order
[10](./10-operator-selected-versions-and-agent-guided-migration.md) sets, and finds three questions
the tool cannot answer.

The collision at `roost/flight-hours.yml` is the sharp one. A file is there, the receipt does not
attribute it, and its contents look exactly like what this version would generate. The tool refuses
it. Version control shows the keeper wrote that file by hand eight months ago, before the feature
existed, so the answer is to merge the keeper's entries into what the candidate produces and let
the project decide which wins. A person approves that.

`roost/seasonal.yml` is retired. The tool released the attribution and deleted nothing. The keeper
still reads it every winter, so it stays, now as an ordinary project file.

`roost/feeding.yml` is seeded and the project edited it. The new version expects one field the old
schedule does not have. The tool preserves the file, and the agent adds the field, because the tuned
values mean something and regenerating the file would discard them.

## Landing, and a failure part way through

```text
$ loft land
created   roost/flight-hours.yml
replaced  roost/housing.yml
preserved roost/feeding.yml
preserved roost/perches.log
Error: cannot write KEEPER.md — the managed region's end marker is missing.
observed completed: roost/flight-hours.yml roost/housing.yml
exit 4
```

The receipt was not written, because it is written last. Two destinations are new or changed and the
record still describes the previous state. That is recoverable: the diff is in version control, the
command named exactly what it completed, and nothing was destroyed.

The keeper had deleted the end marker while editing. The agent restores it and reruns.

```text
$ loft land
matched   roost/flight-hours.yml
matched   roost/housing.yml
preserved roost/feeding.yml
preserved roost/perches.log
replaced  KEEPER.md
released  roost/seasonal.yml
receipt   .loft/receipt.json
```

Rerunning is safe because the candidate is computed again from the same three inputs. What was
already correct reports as matched.

## Verification, then cleanup

```text
$ loft check
roost/housing.yml   ok
roost/feeding.yml   ok
KEEPER.md           ok
.loft/receipt.json  ok, 4 attributed destinations
```

The stage is still on disk, which is the point: the keeper compares what was predicted against what
landed, and the two agree. Only then does cleanup run, as its own command, against one named stage.

```text
$ loft stage clean .loft-stage
Removing the stage at .loft-stage (receipt: loft.stage/1, staged 2026-09-14).
Confirm: yes
Removed.
```

It refused nothing here because the path was a real stage carrying its own receipt. A symlink, a
home directory, the project root, or a directory with no stage receipt would each have stopped it.

## See also

- [90 — Worked example](./90-worked-example.md) — the same four layers, for a tool that does not
  write into projects.
- [99 — Checklist](./99-checklist.md) — what to walk before shipping either.

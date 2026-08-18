# 002 — Gate the Shell Scripts the Buckets Ship

## Goal

Every executable this library ships is proven by a gate, including the five shell scripts that never
were.

## Example

Five `.sh` files sit in content buckets today and no hook selects any of them.

```console
$ fd -e sh --exclude .git .
infra/aws/scripts/ec2-search-by-name-all-regions.sh
infra/aws/scripts/ec2-test-connection.sh
systems/linux/window-managers/x11-arandr-autorandr-debug-report.sh
systems/shell/util_scripts/output_file_contents.sh
systems/shell/util_scripts/run_in_background.sh
```

## Core

A hook that selects every shell script under a content bucket, and a `just test` case behind it.
ADR-executable-artifacts-in-the-library requires it, and the requirement has been unmet since the files were written.

## In scope

- A `shellcheck` hook whose `files:` pattern selects `.sh` files under the eight buckets.
- `pkgs.shellcheck` and `pkgs.shfmt` return to the devShell, this time with a gate behind each.
- Whatever the five scripts need to become clean, or an explicit per-file disable with its reason.
- A `just test` case, so the gate runs in CI and not only in a hook.

## Out of scope

- Rewriting any script beyond what the linter demands.
- A formatting gate on a script a reader is expected to copy verbatim from a vendor's docs.

## Governed by

- `_docs/decisions/ADR-executable-artifacts-in-the-library.md` — the three obligations a
  shipped artifact carries, one of which is the hook this story adds.
- `AGENTS.md` — the same rule as a repository non-negotiable.
- `programming/cli-design/README.md` — the shell conventions the scripts are held to.

## Amends

- `.pre-commit-config.yaml` — the hook.
- `flake.nix` — the two tools return, each with a gate behind it.
- `justfile` — the case.

## Acceptance

- Every `.sh` file under a content bucket is selected by the hook — `pre-commit run --all-files`.
- `just test` fails when a bucket script is not clean — verified by breaking one deliberately.
- `check-hooks-apply` still passes, so the pattern matches at least one file — `just lint`.

## Tasks

- [ ] Write the hook and its `files:` pattern.
- [ ] Return `shellcheck` and `shfmt` to the devShell.
- [ ] Fix or explicitly disable each finding in the five scripts.
- [ ] Add the `just test` case and prove it fails once.

## Rabbit holes

- Extending the pattern to every executable text file in the tree — escape: shell scripts are what
  the two tools check; another language is another story.
- Formatting a vendor snippet a reader copies verbatim — escape: gate correctness, not style, where
  the file is a quotation.

## Done when

No shell script in a content bucket is ungated, and the two tools are in the devShell because
something runs them.

## Revisions

None.

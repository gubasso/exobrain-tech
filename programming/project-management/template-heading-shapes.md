# Template — heading shapes

Drop-in wiring for `MD043 required-headings`, for the two fixed shapes this shelf owns: the story
and the epic. Copy each configuration into `<project>/.markdownlint/` and each hook entry beside
the project's existing markdownlint hook. The rule takes one `headings` array, so a shape that
needs its own array needs its own file and its own hook entry.

## The project config must not mention `MD043`

Remove `MD043` from the project's general markdownlint configuration, including a `false` value.
That configuration is merged over the dedicated file and would silently disable this contract.

## `.markdownlint/story.markdownlint-cli2.jsonc`

```jsonc
{
  "config": {
    "MD043": {
      "headings": [
        "*",
        "## Goal",
        "## Example",
        "## Core",
        "## In scope",
        "## Out of scope",
        "## Governed by",
        "## Amends",
        "## Acceptance",
        "## Tasks",
        "## Rabbit holes",
        "## Done when",
        "## Revisions"
      ]
    }
  }
}
```

## `.markdownlint/epic.markdownlint-cli2.jsonc`

```jsonc
{
  "config": {
    "MD043": {
      "headings": [
        "*",
        "## Goal",
        "## Example",
        "## Core",
        "## Out of scope",
        "## Governed by",
        "## Amends",
        "## Done when",
        "## Revisions"
      ]
    }
  }
}
```

## Hook entries

```yaml
- id: markdownlint-cli2
  alias: md-story
  name: markdownlint (story heading shape)
  files: '^docs/plan/stories/[^/]+\.md$'
  args: ['--config', '.markdownlint/story.markdownlint-cli2.jsonc']

- id: markdownlint-cli2
  alias: md-epic
  name: markdownlint (epic heading shape)
  files: '^docs/plan/epics/[^/]+\.md$'
  args: ['--config', '.markdownlint/epic.markdownlint-cli2.jsonc']
```

Under another runner, select the same files explicitly:

```bash
markdownlint-cli2 --config .markdownlint/story.markdownlint-cli2.jsonc \
  "docs/plan/stories/*.md"
markdownlint-cli2 --config .markdownlint/epic.markdownlint-cli2.jsonc \
  "docs/plan/epics/*.md"
```

## Verify each gate is live

Add a heading the array does not permit, run the dedicated hook, and confirm `MD043` names the
unexpected heading. Do this once per shape; a hook that never selected a file is indistinguishable
from a passing one. Remove each probe only after its gate has failed as intended.

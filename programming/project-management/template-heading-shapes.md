# Template — heading shapes

Drop-in wiring for `MD043 required-headings`. Copy the configuration into
`<project>/.markdownlint/` and the hook entry beside the project's existing markdownlint hook.

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

## Hook entry

```yaml
- id: markdownlint-cli2
  alias: md-story
  name: markdownlint (story heading shape)
  files: '^docs/plan/stories/[^/]+\.md$'
  args: ['--config', '.markdownlint/story.markdownlint-cli2.jsonc']
```

Under another runner, select the same files explicitly:

```bash
markdownlint-cli2 --config .markdownlint/story.markdownlint-cli2.jsonc \
  "docs/plan/stories/*.md"
```

## Verify the gate is live

Add a heading the array does not permit, run the dedicated hook, and confirm `MD043` names the
unexpected heading. Remove the probe only after the gate has failed as intended.

# Template — Guide

Copy the block below to the bucket that owns the task. The shape is fixed and gated by
`guide-recipe-shape`; [SPEC-guides](../SPEC-guides.md) owns the rules this template seeds.

````markdown
# <Task, as the reader would ask for it>

<One or two sentences: what the reader ends up with, and what is out of scope.>

## Prerequisites

- <state the first step assumes: a tool installed, a checkout present, an account authorized>
- <one line each, each verifiable by the reader before starting>

## Steps

1. <Imperative action.>

   ```sh
   <the command>
   ```

2. <Imperative action that needs parts.>

   - <part one>
   - <part two>

3. <Action carrying a hazard or a decision — one line of prose, then the command.>

   ```sh
   <the command>
   ```

4. <Verify the result.>

   ```sh
   <the check>
   ```

   Expected: <what the reader should see>

## Reference

- [<companion document>](./<companion>.md) — <what it explains>
````

Checks before committing:

- Every prerequisite is a state, not a step the reader is meant to perform.
- Each step is one action; a step with parts nests them rather than splitting into siblings.
- Nothing in the steps assumes state no earlier step produced and no prerequisite names.
- The last step tells the reader whether it worked.
- Every explanation longer than a hazard, a decision, or an ordering note sits behind a link, in a
  companion document that walks a scenario rather than argues one.

---
digest-of: languages/javascript
last-synced: 2026-07-10
token-estimate: 350
---

# AGENTS

## Scope

JavaScript/TypeScript language notes, including code-review guides for JS, TS, React, and Svelte,
plus general language and framework references.

## Key Points

- **Code review guide (JS)**: JavaScript-specific review heuristics loaded by `review-code-deep`
  when `.js`/`.mjs`/`.cjs` files are in the diff.
- **TypeScript review guide**: TS-specific heuristics (strict mode, type narrowing, any/unknown,
  generics).
- **React review guide**: React-specific heuristics (hooks rules, key prop, effect dependencies,
  memo discipline).
- **Svelte review guide**: Svelte-specific heuristics (reactivity, stores, lifecycle,
  accessibility).
- **General JS notes** (`javascript-js.md`): Utility patterns and snippets.
- **TypeScript notes** (`typescript.md`): TypeScript-specific patterns and references.
- **Node/npm** (`node-npm.md`): Node.js and npm package management notes.
- **Svelte** (`svelte.md`): Svelte framework patterns and references.

## Maintenance Notes

- Review guides are loaded on demand by the review-code-deep skill based on file extensions in the
  diff.
- No CLI-spec subdirectory exists for JavaScript; CLI patterns use the general `cli-design/` canon.

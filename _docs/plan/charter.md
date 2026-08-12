# exobrain-tech — Charter

## What this is for

A public technical knowledge base whose content is the product, and which can be operated,
verified, and adopted without reaching outside its own checkout.

## Pillars

- The library is the product. Everything else in the tree earns its place by serving it.
- Nothing load-bearing lives outside the repository.
- A shipped artifact is proven by a gate, never asserted by prose.
- One fact has one home, and the filesystem owns its own state.
- The repository practices the methods it teaches, and adopts a published tool for a method it does
  not teach. Documentation design is taught here and used here; planning is
  [plan-xp](https://github.com/gubasso/plan-xp), pinned in `flake.nix` and used here on the same
  terms.

## No-gos

- No knowledge article under `_docs/`, and no repository machinery inside a content bucket.
- No compiled artifact: this repository ships documents and programs a reader can read.
- No second store of anything the record, the tree, or git history already holds.
- No private material — equipment identity, security posture, recovery material, credentials, or
  personal workflows belong in `exobrain-tech-vault`.

## Iteration

Two weeks. Each iteration closes stories whose points were accepted by review, and velocity is read
as the number of judgments one reviewer absorbed. The cadence is stated here for a reader and in
`config.yml` for a tool; changing it restarts the velocity series.

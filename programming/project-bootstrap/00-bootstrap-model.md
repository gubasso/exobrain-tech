# 00 — Bootstrap model

**Bootstrap** is the once-per-project phase that takes an empty repository to a scaffolded,
quality-gated baseline ready for feature work. It is a **how-to guide** in the Diátaxis sense —
task-oriented, ordered, assuming you know what a project is and just need the steps — not a
tutorial.

## The once-per-project phase

Bootstrap runs exactly once per project, at creation, and ends at a scaffolded, quality-gated
baseline. Everything a project does afterwards — feature work, publishing, operations — is a
different phase with a different owner. Keeping bootstrap bounded avoids a common trap: treating
"set up the project" and everything that follows it as one undifferentiated blob.

## The three-layer ownership model

Every bootstrap fact has exactly one owner, arranged in three descending layers:

1. **General** — `tech/programming/project-bootstrap/` (this tree). Universal, cross-language steps:
   repo creation, `.gitignore`/`LICENSE`/`README`, governance docs, dev environment, quality gates,
   CI, security baseline. The [`runbook.md`](./runbook.md) is the spine.
2. **Language** — `tech/languages/<lang>/project-bootstrap-spec/`. Ecosystem choices only: the crate
   layout and toolchain for Rust, packaging for Python, and so on. It _overlays_ the general spine
   and never restates it.
3. **Implementation-kind** — a flat file such as `cli-project.md` inside the language binding
   (promote to a subdirectory only when it grows to multiple chapters). Shape-specific additions:
   CLI arg-parsing/logging, library public API, service health/config. It owns only the
   bootstrap-time _ordering_ and delegates the detailed _how_ to any existing spec.

## One owner per fact

The layers link; they never duplicate. A tool that owns its own shelf is _referenced_ from the
runbook, not copied into it. This is the single-source-of-truth discipline: duplicated instructions
drift, so each fact is owned in one place and pointed to from everywhere else.

## References

- [GitHub Open Source Guide — Starting a project](https://opensource.guide/starting-a-project/)
- [Diátaxis](https://diataxis.fr/) — the how-to vs tutorial distinction.

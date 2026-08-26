# jj — Jujutsu version control

> <https://jj-vcs.github.io/jj/> · <https://docs.jj-vcs.dev/latest/>

Working notes for Jujutsu, the Git-compatible VCS whose working copy is always a commit and
whose bookmarks are never checked out. Notes are written against jj 0.44 and use current
spellings: `jj rebase -o` rather than the deprecated `-d`, and `jj git push --bookmark`
without the removed `--allow-new`.

## Doing

| Document                                         | What it gets you                                                                                                                         |
| ------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------- |
| [parallel-workspace.md](./parallel-workspace.md) | A second working directory backed by the same repo — created from your last commit, developed independently, landed with a bookmark move |

## Understanding

| Document                                                       | What it explains                                                                                                                                   |
| -------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| [workspaces-share-one-repo.md](./workspaces-share-one-repo.md) | Why landing work from a workspace is a bookmark move rather than a merge, how `<workspace>@` addresses another working copy, and what staleness is |
| [what-names-a-change.md](./what-names-a-change.md)             | The four names a change carries — directory, workspace, description, bookmark — walked through one worked scenario with commit graphs              |

## Related

- [../git/](../git/) — the Git commands and workflows jj interoperates with

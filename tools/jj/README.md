# jj — Jujutsu version control

> <https://jj-vcs.github.io/jj/> · <https://docs.jj-vcs.dev/latest/>

Working notes for Jujutsu, the Git-compatible VCS whose working copy is always a commit and
whose bookmarks are never checked out. Notes are written against jj 0.44.

Every example name in these notes is prefixed with the kind of thing it names, for as long as
the name is jj's alone, so no token is ambiguous while you are learning which is which:

| Kind            | Example name       | Written as        | Real repos                       |
| --------------- | ------------------ | ----------------- | -------------------------------- |
| bookmark        | `bkmrk-feat-x`     | bare              | `feat-x`                         |
| remote bookmark | `develop@origin`   | `<name>@origin`   | `develop@origin`                 |
| workspace       | `wkspc-feat-x`     | `<name>@`         | whatever the directory is called |
| directory       | `../wkspc-feat-x/` | a filesystem path | any path                         |
| description     | quoted prose       | `-m "feat: ..."`  | prose                            |

The `bkmrk-` and `wkspc-` prefixes are teaching labels, not a jj convention. The `@` forms are
jj's own syntax and are exactly what you type at the prompt.

A remote bookmark `<name>@origin` is jj's own record of where that branch stood on the remote as
of the last fetch or push. You read it and pass it to commands; nothing you type moves it, and
only `jj git fetch` and `jj git push` change what it points at.

A prefix lasts only while the name is jj's alone. A branch takes the bookmark's name character
for character, so a bookmark drops its prefix in the step before it is pushed:

```sh
jj bookmark rename bkmrk-feat-x feat-x
```

A workspace name never becomes a branch, so `wkspc-` stays for the life of the workspace. Names
that arrive from the remote are branches already and carry no prefix at any point: `develop` is
the integration line in these notes and `master` the release line.

A bookmark is a reference — a name pointing at one commit id — and it is how jj manages Git
branches: against a Git repo jj
maps the two onto each other by name in both directions — pushing the bookmark `feat-x`
writes the branch `feat-x` on the remote, and a branch created in the backing Git repo comes
back as a bookmark of that name. The difference is that no bookmark is ever checked out, so
none of them advances on its own the way the current Git branch does. A name you have only
`jj bookmark set` is a bookmark and nothing more; it becomes a branch in a colocated repo's
own `.git` on the next command, and a branch on the remote when you `jj git push` it.

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

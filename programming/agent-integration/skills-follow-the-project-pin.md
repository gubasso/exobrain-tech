# Skills Follow the Project Pin

A tool that lands files into a project often ships agent skills that teach an agent how to run it. Each project pins its own version of the tool. This chapter states where those skills live, so that each skill describes the version that its project runs.

## What a host copy breaks

One copy of a skill under the home serves every project on the host. Projects move their pins at different times, so the projects run several versions while the host copy describes one. An agent that reads the host copy then follows instructions for a tool that the project does not run.

No project owns that copy. No landing verb of a project moves it, and no review of a project sees it change. The rules below give each skill an owner that moves with the pin.

## Land the skills at project scope

A tool that lands files into a project lands its own skills there too. It writes them at project scope, under every root that a supported agent scans for project skills. The roots are vendor facts, so this chapter names none of them. Each runtime adapter, linked from [Skill distribution](./skill-distribution.md#the-adapters), dates its own roots.

Prisma ships its skills inside the packages that a project installs, "so they always describe the version in use". Its sync writes one copy into each agent root of the project. The GitHub CLI installs a skill at project scope by default, and so does the Vercel `skills` installer.

## Land copies, not links

The skills land as plain copies of the package bytes. A link fails where a root does not follow it. A relative link into a dependency folder also fails on a fresh clone, until an install runs again. [Skill distribution](./skill-distribution.md#one-source-many-agents) compares copies and links for every installer.

Prisma's sync reads and writes each file of a skill tree, and it writes a link in the source tree out as file content. antfu's `skills-npm` keeps links, and its own change notes say that the committed links are broken on a fresh clone until the first install.

## Commit the landed skills

The project commits the landed skills. A clone and every worktree then carry them. A reviewer sees them change in the same change that moves the pin.

Prisma's documentation says that the copies "are ordinary files that git tracks". Its sync code removes the ignore file that older versions of its CLI wrote into each copy. Next.js asks a project to commit the block that it manages in `AGENTS.md`, because its dev server writes the block again when a change drops it.

## The record holds the version

The landing record holds the version of the tool that wrote the skills. It lists each skill file with its digest, as it lists any other landed file. The skill files carry no version stamp. [Candidate artifacts and receipts](./candidate-artifacts-and-receipts.md#provenance-lives-in-the-receipt) gives the reason: the receipt owns provenance, and a second copy inside a file goes stale.

The record then answers three questions for the skills, as it does for every landed file:

- Drift: a skill file whose digest differs from the record is an edited file.
- Retirement: a skill that the release no longer ships is a destination that the release retires.
- The upgrade prompt: a record older than the binary names `upgrade`.

The Agent Skills specification defines no top-level `version` field. Its example puts a version under `metadata`, a free map for properties that a client defines. A tool that lands skills leaves that key out, because the record already holds the version.

The Vercel installer records a computed hash for each project skill in a committed lock file. It compares that hash with the files on disk.

## Two verbs write a skill

`init` lands the skills, and `upgrade` moves them. No verb writes a skill under the home. A skill then changes only in a change that also lands the rest of the tool's files, at the same version.

## A folder the record does not attribute

A landing never replaces a skill folder that the record does not attribute. The rule holds whatever the bytes are, and whatever flag asks for a replacement. A person can write a skill with the same name, and the tool cannot tell that work from its own.

The check covers the whole package folder. A folder that holds a file the record does not list counts as unattributed, even when the record lists every other file in it. [Candidate artifacts and receipts](./candidate-artifacts-and-receipts.md#missing-provenance-reduces-certainty) states the general rule for a collision.

Prisma's sync refuses a folder that is not its own copy, and never replaces it. The `skills-npm` change never replaces a real directory or a link that it did not make.

## Before the first landing

A project with no landing has no project copy of the skills. The binary then serves its own skills, file by file, through a read-only verb, for example `<tool> skill show <name> --file <path>`. An agent reads the setup skill at the version on the `PATH`, and the host keeps no copy. [Setup skill skeleton](./setup-skill-skeleton.md) describes that skill.

## Moving off the user scope

A move from the user scope runs as one batch, in this order:

1. Release the version of the tool that lands its skills at project scope.
2. Move every project that uses the tool to that release, through `upgrade`.
3. Remove the user-scope copies through the tool's own uninstall.

The uninstall removes only what its user-scope record vouches for, and it keeps an edited file. [Skill distribution](./skill-distribution.md#what-an-installer-owes) states that obligation.

The runtimes resolve a shared name in different ways, so a copy left behind is never harmless. Claude Code lets the personal copy win over the project copy. Codex does not merge two skills with one name, and both can appear in its skill selectors.

A doctor probe of the tool names each user-scope `SKILL.md` of the tool that remains. A folder that keeps only notes is not a copy, because no runtime loads it as a skill.

The move is complete only when no user-scope `SKILL.md` of the tool remains. The uninstall keeps a file that was edited or that its record does not list. For each such file, the operator reads it, saves what is personal elsewhere, and removes or renames the skill folder by hand.

Then each agent reloads its skills, because a running session can keep the old listing. Claude Code watches its skill folders, but content that it already loaded stays with the session. Codex documents a restart when an update does not appear.

## One name, one owner

One skill name has one owner: the landing of the project. [Skill distribution](./skill-distribution.md#one-name-one-owner) gives the reason. Two scopes that hold one name leave the answer to the precedence of the runtime, not to the project.

## See also

- [A version gate on every command](./a-version-gate-on-every-command.md): how each command compares the record with the binary.
- [A tool repository runs its previous release](./a-tool-repository-runs-its-previous-release.md): the tool's own repository as a consumer of these rules.
- [Candidate artifacts and receipts](./candidate-artifacts-and-receipts.md): the record that lists each skill file.

## Sources

- Prisma, the skills command: skills ship in the installed packages, sync writes one copy per agent root, git tracks the copies, a folder that sync does not own is refused. Read 2026-11-13 at <https://www.prisma.io/docs/cli/skills>.
- Prisma, the sync code: it copies file by file, refuses a target that is not its copy, and removes the `*` ignore file that older versions wrote. Read 2026-11-13 at <https://github.com/prisma/prisma-cli/blob/main/packages/cli/src/lib/skills/sync.ts>.
- GitHub CLI, `gh skill install`: `--scope` defaults to `project`, and `--pin` names a tag or a commit. Read 2026-11-13 at <https://cli.github.com/manual/gh_skill_install>.
- Vercel `skills`: project scope by default, and a `skills-lock.json` that records a `computedHash` for each project skill. Read 2026-11-13 at <https://github.com/vercel-labs/skills>.
- antfu `skills-npm`, the change to committed relative links with a lock file: the links stay links and are broken on a fresh clone until the first install. Read 2026-11-13 at <https://github.com/antfu/skills-npm/pull/45>.
- Next.js, agent support: version-matched documentation inside the package, and a managed `AGENTS.md` block to commit. Read 2026-11-13 at <https://nextjs.org/docs/app/guides/ai-agents>.
- Agent Skills specification: the frontmatter fields, with no top-level `version`. Read 2026-11-13 at <https://agentskills.io/specification>.
- Claude Code, skills: "Enterprise over personal, and personal over project", and the watch on skill folders. Read 2026-11-13 at <https://code.claude.com/docs/en/skills>.
- Codex, skills: two skills with one name are not merged, and "both can appear in skill selectors". Read 2026-11-13 at <https://learn.chatgpt.com/docs/build-skills>.

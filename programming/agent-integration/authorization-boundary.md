# The Authorization Boundary

An agent acts on what the request asks for and on nothing that merely follows from it. This chapter states that boundary, the cost it puts on the operator, and the glossary of request terms that lowers the cost without loosening the boundary.

## The boundary

An agent takes the actions the request names. A request to change code authorizes the file changes alone: no branch, no commit, no push, no merge, no release, and no message to anyone, unless the request names that action. An action does not become authorized because it is the next step of a workflow the agent knows.

The boundary is prose in the instruction file, not a mechanism. The commit hooks and the forge's branch protections bound the end state: what a commit may contain, and what may reach the trunk. Neither one tells an agent from a person. A push the protections accept is a valid push whoever made it, so only the instruction the agent reads can say whether this agent was asked to make it.

## The cost the boundary carries

The boundary makes the operator name each action. An operator who wants a whole run, from the change to the merge, types the same list at the start of every task: create the branch, commit, push, open the request, merge, prune. The list is long, it is the same every time, and a forgotten item stops the run halfway.

That retyping is what wears a boundary down. An operator who tires of the list writes a standing permission into the instruction file, and the standing permission authorizes every task, including the ones the operator never meant to hand over. The fix is to make the list cheap to name, not to remove the need to name it.

## The glossary

The glossary is one file at the repository root that defines the request terms. It holds two regions.

- A convention or a tool owns the region between two markers. It writes that region when it lands, and it rewrites the region on every upgrade.
- The project owns every line outside the markers. No upgrade touches those lines, so a project adds its own terms below the end marker.

The glossary owns the definitions. The instruction file routes to it, and nothing else restates a term, so a definition changes in one place.

## How a term is written

Each term expands to a named list of actions, never to a category. "Everything up to the merge" is a category, and an agent fills a category from its own habit. "The branch, the commits, the push, and the request" is a list, and an agent can check each item against it.

Each term contains the one below it. The second term names everything the first term names, then the actions it adds, so a reader learns the ladder once and reads every term as one step up.

Every term is a compound that no ordinary sentence produces, such as `implement-and-merge`. A request that says "fix the bug and merge it" uses an English word, and the agent reads it as English. Only the exact term authorizes the list, so an operator never grants a run by accident.

A request that carries no term authorizes the file changes alone. That default is the boundary itself, so it earns no entry.

## A reference ladder

The ladder below is an example to adapt, not a fixed vocabulary. A project names its own terms and its own steps.

| Term                    | What the operator authorizes by using it                                                                                                   |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `implement-and-request` | The file changes, then the branch, the commits, and the integration up to the point where a person decides. It stops before the merge.     |
| `implement-and-merge`   | Everything `implement-and-request` names, then the integration itself, then the removal of the merged branch. It stops before the release. |
| `full-implement`        | Everything `implement-and-merge` names, then the release, carried through to a published version.                                          |

A project that releases nothing carries the first two rows alone. A term must not name an action the project cannot take, because an agent that meets an impossible step improvises one.

An action in a term's list can take its form from an answer the project records. The integration step is one example. A project that integrates through the forge reads it as the push and the pull or merge request. A project that integrates locally reads it as the rebase onto the trunk and the separate trunk push. The term still names the action, and the recorded answer decides how the agent performs it, so one ladder serves both kinds of project.

## Making the file visible

A separate file at the root loads into no session on its own. An agent that has never read the glossary cannot recognize a term in a request.

The root instruction file closes that gap with one line. It names the glossary file and lists the term names, so every session holds the names and reads the file when a request carries one. [Instruction files](./instruction-files.md) states what the root file carries and why it routes by a plain pointer.

The line names the glossary rather than importing it. An import loads with the file that names it, as [one runtime's adapter](../../tools/claude-code/memory-file-loading.md) records for one runtime. A glossary imported from the always-loaded root file therefore loads into every session, including the many that carry no term.

## What the glossary is not

The glossary is not enforcement. A term widens what one request authorizes, and it does nothing else. It cannot stop an action, and an agent that ignores it meets no gate.

The protections that hold are the trunk's: the required checks, the protected branch, and the merge rules that [trunk-based development](../../workflows/trunk-based-development.md) states. A project keeps those whether or not it lands a glossary, and the glossary never stands in for them.

## See also

- [The four layers](./model.md) — governance, the layer this boundary belongs to.
- [Router and gates](./router-and-gates.md) — a router skill that respects this boundary at every step.
- [Instruction files](./instruction-files.md) — the root file that routes to the glossary.

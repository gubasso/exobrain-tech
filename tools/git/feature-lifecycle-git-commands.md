# Feature Lifecycle — Git Commands

Pure git commands implementing the feature lifecycle described in
[feature-lifecycle](./feature-lifecycle.md).

Uses `git clone --reference` to create isolated work-clones (feature directories) for each feature.

---

## Conventions

- Main repo: `~/Projects/org/repo`
- Work-clone: `~/Projects/org/repo.<issue>-<slug>` (e.g., `repo.42-add-auth-module`) — a
  `git clone --reference` of the main repo, sharing its object store
- Trunk: `master` — the only permanent branch, and the repository default
- Short-lived branch: `<issue>-<slug>` (e.g., `42-add-auth-module`), or `<type>/<slug>` where no
  issue mints the name

---

## Creating a Work-Clone

Clone from the remote using the main repo as an alternates store. This avoids duplicating objects on
disk. The issue must exist first — the forge CLI creates the branch and owns the branch name.

```bash
git clone --reference ~/Projects/org/repo \
    git@github.com:org/repo.git \
    ~/Projects/org/repo.42-add-auth-module
```

Do not delete the main repo while work-clones exist — they depend on its object store.

---

## Entry Points

### From an existing issue

The issue already exists on the forge. Let the forge create and name the branch, then fetch it.

```bash
# 1. Let the forge create the branch (from the main repo)
gh issue develop 42 --base master                 # GitHub — creates remote branch

# 2. Retrieve the branch name the forge chose
BRANCH=$(gh issue develop 42 --list --json headRefName --jq '.[0].headRefName')

# 3. Create work-clone using the forge-provided branch name
git clone --reference ~/Projects/org/repo \
    git@github.com:org/repo.git \
    ~/Projects/org/repo."$BRANCH"
cd ~/Projects/org/repo."$BRANCH"
git fetch origin "$BRANCH"
git checkout "$BRANCH"
```

On GitHub, `gh issue develop` creates the branch on the remote and links it in the issue's
Development sidebar. The branch name is determined by GitHub. On GitLab, there is no `issue develop`
equivalent; create the branch manually and link via `glab mr create --related-issue` later.

### From a new issue

Create the issue from the CLI first, then let the forge create and name the branch.

```bash
# 1. Create the issue
gh issue create --title "Add auth module"        # GitHub → returns URL with ID

# 2. Let the forge create the branch
gh issue develop 42 --base master

# 3. Retrieve the branch name the forge chose
BRANCH=$(gh issue develop 42 --list --json headRefName --jq '.[0].headRefName')

# 4. Create work-clone using the forge-provided branch name
git clone --reference ~/Projects/org/repo \
    git@github.com:org/repo.git \
    ~/Projects/org/repo."$BRANCH"
cd ~/Projects/org/repo."$BRANCH"
git fetch origin "$BRANCH"
git checkout "$BRANCH"
```

### From uncommitted changes

Stash first, then create the issue, let the forge name the branch, clone, and apply.

```bash
# 1. Save uncommitted changes
cd ~/Projects/org/repo
git stash

# 2. Create the issue
gh issue create --title "Add auth module"        # → issue #42

# 3. Let the forge create the branch
gh issue develop 42 --base master

# 4. Retrieve the branch name the forge chose
BRANCH=$(gh issue develop 42 --list --json headRefName --jq '.[0].headRefName')

# 5. Create work-clone using the forge-provided branch name
git clone --reference ~/Projects/org/repo \
    git@github.com:org/repo.git \
    ~/Projects/org/repo."$BRANCH"
cd ~/Projects/org/repo."$BRANCH"
git fetch origin "$BRANCH"
git checkout "$BRANCH"

# 6. Transfer stash from main repo (stash is repo-local)
git -C ~/Projects/org/repo stash show -p | git apply
```

Drop the stash in the main repo once the changes are confirmed in the work-clone.

```bash
git -C ~/Projects/org/repo stash drop
```

---

## Sync

Push the feature branch to origin and set up tracking. If the branch was created via
`gh issue develop`, the remote branch already exists; `git push -u` sets the local tracking
reference.

```bash
git push -u origin 42-add-auth-module
```

---

## Rebase

Keep the branch current with the trunk.

```bash
git fetch origin
git rebase origin/master
git push --force-with-lease
```

---

## Finish

Push the branch and open a PR on the forge.

```bash
cd ~/Projects/org/repo.42-add-auth-module
git push -u origin 42-add-auth-module

# GitHub
gh pr create --base master --head 42-add-auth-module

# GitLab
glab mr create --target-branch master --source-branch 42-add-auth-module
```

Add `--draft` for a draft PR or MR. The request lands by squash merge, so its title becomes the
trunk's commit message.

---

## Cleanup

After the PR is merged on the forge, pull the trunk and remove the work-clone.

```bash
# Pull the merged changes into the main repo
cd ~/Projects/org/repo
git checkout master
git pull

# Remove the work-clone
rm -rf ~/Projects/org/repo.42-add-auth-module

# Delete the local branch. -D, not -d: after a squash merge its commits are
# not ancestors of the trunk, so git does not consider it merged.
git branch -D 42-add-auth-module
```

---

## Quick Reference

```text
CREATE WORK-CLONE:
  git clone --reference <main-repo> <remote-url> <work-clone>

FROM EXISTING ISSUE:
  gh issue develop <id> --base <branch>            # forge creates + names branch
  BRANCH=$(gh issue develop <id> --list ...)       # retrieve forge-chosen name
  git clone --reference ...                        # create work-clone
  git fetch origin $BRANCH && git checkout ...     # check out the branch

FROM NEW ISSUE:
  gh issue create --title "..."                    # create issue (→ ID)
  gh issue develop <id> --base <branch>            # forge creates + names branch
  BRANCH=$(gh issue develop <id> --list ...)       # retrieve forge-chosen name
  git clone --reference ...                        # create work-clone
  git fetch origin $BRANCH && git checkout ...     # check out the branch

FROM UNCOMMITTED CHANGES:
  git stash                                        # save changes in main repo
  gh issue create --title "..."                    # create issue (→ ID)
  gh issue develop <id> --base <branch>            # forge creates + names branch
  BRANCH=$(gh issue develop <id> --list ...)       # retrieve forge-chosen name
  git clone --reference ...                        # create work-clone
  git fetch origin $BRANCH && git checkout ...     # check out the branch
  git -C <main-repo> stash show -p | git apply     # transfer changes
  git -C <main-repo> stash drop                    # drop stash after confirm

SYNC:
  git push -u origin <branch>                      # push + track

REBASE:
  git fetch origin                                 # fetch upstream
  git rebase origin/master                         # rebase onto the trunk
  git push --force-with-lease                      # force-push safely

FINISH:
  git push -u origin <branch>                      # push the branch
  gh pr create --base master                       # open PR (GitHub)

CLEANUP:
  cd <main-repo> && git checkout master && git pull # pull merged changes
  rm -rf <work-clone>                              # delete work-clone
  git branch -D <branch>                           # delete local branch
```

---

## Appendix — Local Merge

This is not a landing route for a trunk that has a forge. A change reaches such a trunk through a
pull request carrying the passing check, so a forge that is merely unreachable is waited out: the
work stays on its branch, and the request is opened when the forge comes back.

What follows applies to a repository that has no forge at all, where no pull request exists to carry
the review. `--squash` stages the branch's result without recording a merge, and the commit you then
write is the one commit the trunk takes — the same shape a forge's squash button produces.

Because the work-clone is a separate clone, either fetch from origin after pushing or add it as a
temporary remote.

**Option A — fetch from origin after pushing from the work-clone:**

```bash
# In the work-clone — push latest
cd ~/Projects/org/repo.42-add-auth-module
git push

# In the main repo — fetch and squash
cd ~/Projects/org/repo
git checkout master
git fetch origin
git merge --squash origin/42-add-auth-module
git commit -m "feat: add auth module (#42)"
```

**Option B — add work-clone as a temporary remote:**

```bash
cd ~/Projects/org/repo
git remote add temp-42 ~/Projects/org/repo.42-add-auth-module
git fetch temp-42
git checkout master
git merge --squash temp-42/42-add-auth-module
git commit -m "feat: add auth module (#42)"
git remote remove temp-42
```

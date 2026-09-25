# Encrypted git remote with git-remote-gcrypt

Push a repository to GitLab or GitHub so the forge stores only ciphertext: commits, trees, blobs, ref names, and history. Covers one participant, the operator. Multi-participant keyrings, `rsync://` and `sftp://` backends, and at-rest protection on the local machine are out of scope; the working tree stays plaintext.

## Prerequisites

- `git-remote-gcrypt` resolves on `PATH`: `command -v git-remote-gcrypt`.
- `gpg` 2.4 or newer, with `gpg-agent` running and a pinentry available.
- `glab` or `gh` authenticated against the forge that will host the remote: `glab auth status` or `gh auth status`.
- A local directory holding the content to push, whether or not it is a git repository yet.

## Steps

1. List the secret keys and pick the one that will own the remote.

   ```sh
   gpg --list-secret-keys --keyid-format=long
   ```

   Expected: one `sec` line carrying `[SC]` and one `ssb` line carrying `[E]`. A key without `[E]` cannot receive the encrypted manifest.

   - If no key qualifies, create one:

     ```sh
     gpg --quick-generate-key "<your-name> <your-email>" ed25519/cert default 2y
     ```

   - Then give it an encryption subkey, reading the fingerprint from the output above:

     ```sh
     gpg --quick-add-key <fingerprint> cv25519 encr 2y
     ```

2. Bind the fingerprint and the long key id to shell variables that later steps read.

   ```sh
   FINGERPRINT=$(gpg --list-secret-keys --with-colons --fingerprint <your-email> | awk -F: '/^fpr/{print $10; exit}')
   KEYID=0x$(printf '%s' "$FINGERPRINT" | tail -c 16)
   printf '%s\n%s\n' "$FINGERPRINT" "$KEYID"
   ```

   Expected:

   ```text
   <40 hex digits, the full fingerprint>
   0x<the last 16 of those digits>
   ```

3. Upload the public key to the forge account. `git-remote-gcrypt` does not need this; it buys the verified badge on signed commits.

   - GitLab, with `<host>` as `gitlab.com` or a self-managed host:

     ```sh
     gpg --armor --export "$FINGERPRINT" | GITLAB_HOST=<host> glab gpg-key add
     ```

   - GitHub:

     ```sh
     gpg --armor --export "$FINGERPRINT" | gh gpg-key add -
     ```

4. Create the remote repository, empty and private.

   - GitLab:

     ```sh
     GITLAB_HOST=<host> glab repo create <namespace>/<repo> --private
     ```

   - GitHub:

     ```sh
     gh repo create <owner>/<repo> --private
     ```

5. Initialize the local repository, if the directory is not one yet.

   ```sh
   git init -b master
   ```

6. Commit the content.

   ```sh
   git add -A && git commit -m "chore: initial commit"
   ```

7. Add the remote through the `gcrypt::` transport. The fragment names the single remote branch that carries every encrypted ref, and it is not a branch you ever check out.

   ```sh
   git remote add origin 'gcrypt::<git-url>#gcrypt'
   ```

8. Name the participants, the keys that can decrypt the repository. Losing every participant secret key makes the remote unreadable with no recovery path.

   ```sh
   git config remote.origin.gcrypt-participants "$FINGERPRINT"
   ```

9. Name the signing key explicitly. `gcrypt` ignores `gpg.format` and falls back to reading `user.signingkey` as a file path, so a global `gpg.format=ssh` makes it try to sign with an SSH public key and fail with `No secret key`.

   ```sh
   git config remote.origin.gcrypt-signingkey "$KEYID"
   ```

10. Publish the participants. Without this, `gcrypt` encrypts with `gpg -R`, which strips the recipient key id, so `gpg` trial-decrypts with every secret key in the keyring and raises a pinentry prompt for each one.

    ```sh
    git config remote.origin.gcrypt-publish-participants true
    ```

11. Push, which also creates the encrypted layout on the forge.

    ```sh
    git push -u origin master
    ```

    Expected: `gcrypt: Encrypting to: -r <KEYID>`, a `gcrypt: Repository not found` notice for the empty remote, then `* [new branch] master -> master`.

    - If the push aborts with `..but repository ID is set. Aborting.`, an earlier failed attempt left a local id pointing at a remote that carries no manifest. Clear it and repeat the push:

      ```sh
      git config --unset remote.origin.gcrypt-id
      ```

12. Allow force push on the remote branch. Every `gcrypt` push rewrites the whole encrypted ref, so it is always a force push. The first push succeeds because creating a branch is not a force push; the second is rejected. GitLab protects the default branch automatically, and the branch the first push created is now that default.

    - GitLab:

      ```sh
      glab api --hostname <host> --method PATCH \
        "projects/<namespace>%2F<repo>/protected_branches/gcrypt" \
        --field allow_force_push=true
      ```

    - GitHub: no action, unless a ruleset protects the branch. Check with `gh api "repos/<owner>/<repo>/rulesets"`.

13. Prove the remote decrypts from a fresh clone, then delete that clone. It holds the plaintext.

    ```sh
    tmp=$(mktemp -d)
    git clone 'gcrypt::<git-url>#gcrypt' "$tmp/check"
    git -C "$tmp/check" log --oneline
    rm -rf "$tmp"
    ```

    Expected: `gcrypt: Good signature from "<your-name> <your-email>"`, then your commits, with readable file contents in the clone.

## Reference

- [gpg.md](../../../systems/security/gpg.md) — key creation, capabilities, and trust
- [glab-cli-workflow.md](./glab-cli-workflow.md) — the rest of the `glab` loop
- [git-remote-gcrypt](https://github.com/spwhitton/git-remote-gcrypt) — upstream, and the manual for every `gcrypt-*` config key

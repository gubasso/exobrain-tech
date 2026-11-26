# git-annex through a gateway

This page records how git-annex reaches an encrypted storage remote through an rclone gateway that holds the storage credential and the encryption key. It also records the git-annex facts that force that shape. [Credential gateways](../../programming/agent-integration/credential-gateways.md#storage-through-a-gateway) states the rule. This page states only what git-annex and rclone do.

## Version and date

git-annex 10.20260421, rclone 1.76.0, and git 2.56.0 on Linux, tested on 2026-11-25. The documentation pages under the references were read on the same date.

Each fact below names its evidence. "The documentation states" marks a fact from the git-annex or rclone pages. "The probe showed" marks a result of a test run on that date, with dummy credentials and a local storage folder behind rclone. A probe result is a fact of the tested releases, and a later release can change it.

## A credential given to git-annex reaches the agent

The documentation states that the WebDAV remote reads `WEBDAV_USERNAME` and `WEBDAV_PASSWORD`, and the S3 remote reads `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`. By default, git-annex caches them in `.git/annex/creds/`, in a file that only the user can read. With `annex.cachecreds=false`, git-annex reads them only from the environment.

The probe showed that `initremote` with the default writes `.git/annex/creds/<uuid>` with mode `0600` and the two values in plain text. With `-c annex.cachecreds=false`, git-annex wrote no such file, and a later command worked only with the credentials in its environment. No revision of the `git-annex` branch held a credential with `encryption=none`.

A file of mode `0600` does not keep the credential from an agent that runs as the same user. The probe showed three more leaks of a credential that git-annex holds, and no `-c` option closed them:

- The `.git/hooks/*-annex` hooks, such as `freezecontent-annex` and `post-update-annex`, ran with the credential in their environment. An empty `core.hooksPath` did not stop them.
- An agent rewrote the remote URL in `.git/annex/journal/remote.log` with no commit. git-annex then sent its Basic auth to the new server.
- An external special remote program of another remote, a `git-annex-remote-<name>` on `PATH`, ran during `fsck` and `whereis` of the WebDAV remote.

So git-annex holds no real credential and no encryption key in this shape.

## The gateway shape

The probe showed this shape working end to end: `copy --to`, `drop`, `get`, and `fsck` with checksums. rclone holds the storage credential and the key. git-annex holds only fake credentials for the loopback server.

```text
git-annex webdav remote, encryption=none, fake credentials
  -> rclone serve webdav on 127.0.0.1:<port>
    -> rclone crypt remote, the key
      -> rclone storage remote, the credential
```

1. Define the storage remote in the rclone configuration, with its real credential.
2. Define a `crypt` remote over a folder of the storage remote, with `password`, `password2`, and the fixed settings of the next section.
3. Serve the crypt remote on loopback, never the storage remote:

   ```bash
   rclone serve webdav <crypt-remote>: --addr 127.0.0.1:<port> --user <fake-user> --pass <fake-pass>
   ```

4. Allow the loopback address in the repository:

   ```bash
   git config annex.security.allowed-ip-addresses 127.0.0.1
   ```

5. Create the git-annex remote with the fake credentials and no caching:

   ```bash
   WEBDAV_USERNAME=<fake-user> WEBDAV_PASSWORD=<fake-pass> \
     git -c annex.cachecreds=false annex initremote <name> type=webdav url=http://127.0.0.1:<port>/<folder> encryption=none
   ```

The documentation states that `rclone serve webdav` listens only on localhost by default, and serves with no login unless `--user` and `--pass` or `--htpasswd` set one. The fake credentials guard only the loopback server. Every local process can still reach the gateway, as [the residual risks](../../programming/agent-integration/credential-gateways.md#every-local-process-reaches-the-gateway) of the chapter state.

The documentation states that git-annex connects only to public IP addresses by default, and refuses a server on localhost or on a private network. The probe showed the refusal without step 4:

```text
ConnectionRestricted "Configuration of annex.security.allowed-ip-addresses does not allow accessing address 127.0.0.1"
```

The documentation also states a form that allows one port only, such as `[127.0.0.1]:<port>`. The probe ran with the plain address, and did not test the port form.

The probe showed that the stored folder held no file content, no git-annex key name, no hash, and no bucket name. Each stored file started with the rclone crypt header.

## The crypt settings are fixed at the first object

The documentation states that the password of encrypted content cannot change. A new password reads none of the old objects, and the only way out is to upload everything again. The documentation also states that a change of the file name encryption means a move of every object into a second crypt remote. So these settings are fixed once the first object is stored:

| Setting                     | Value      | Reason                                                                                                 |
| --------------------------- | ---------- | ------------------------------------------------------------------------------------------------------ |
| `filename_encryption`       | `standard` | Hides the git-annex key names, which carry the content hash, the size, and the extension               |
| `directory_name_encryption` | `true`     | Hides the bucket name and the hash folders of the WebDAV remote                                        |
| `filename_encoding`         | `base32`   | The default. It is safe on a storage backend whose names ignore case                                   |
| `password2`                 | Set        | The salt, a second secret value                                                                        |
| An rclone chunker           | None       | The git-annex `chunk=` option chunks at its own level, and an rclone chunker adds a second fixed layer |

The documentation states that rclone stores no salt on the remote, so a custom salt is a second password to keep. The probe showed that a fresh rclone configuration with only `password`, `password2`, and the three non-secret settings above restored every object, each with its checksum intact. The right password with a wrong salt decrypted nothing, and rclone printed `Skipping undecryptable dir name: bad PKCS#7 padding - too long`.

The password and the salt are then the only restore key of the data. [The storage section](../../programming/agent-integration/credential-gateways.md#storage-through-a-gateway) of the chapter states when the key needs its backup.

## Compared with git-annex encryption

| Question                      | The rclone gateway                                        | git-annex `encryption=hybrid`                                                 |
| ----------------------------- | --------------------------------------------------------- | ----------------------------------------------------------------------------- |
| What holds the key            | The rclone configuration of the host, outside the sandbox | The gpg key of the user, and an encrypted cipher in the `git-annex` branch    |
| What an automated agent needs | Fake credentials and the loopback address                 | gpg with the private key, so a passphrase prompt or a key with no passphrase  |
| What restores the data        | The crypt password and salt, plus the three settings      | The gpg private key and a clone of the repository with its `git-annex` branch |

A passphrase prompt stops an unattended run. A gpg key with no passphrase can be exported by any process of the user, the agent included. The gateway keeps both the key and the credential out of the agent's reach.

## The S3 remote against an rclone S3 gateway

The probe showed that the git-annex S3 remote against `rclone serve s3` on loopback failed with `SignatureDoesNotMatch` (403) on two high ports. It failed with `signature=v4` and with `signature=v2`, with and without `datacenter=us-east-1`, inside and outside a private network namespace. The same remote succeeded on port 80. The rclone S3 client signed correctly against the same server on a high port.

One hypothesis is that git-annex signs the `Host` header without the port. The probe did not prove it, so the cause is unknown. A non-root process cannot bind port 80 on a default Linux host, so the WebDAV remote on a high port is the shape above.

## nixpkgs git-annex on openSUSE Tumbleweed

The probe showed that nixpkgs git-annex fails every TCP connection on openSUSE Tumbleweed:

```text
ConnectionFailure Network.BSD.getProtocolByName: does not exist (no such protocol name: tcp)
```

The distribution ships `/usr/etc/protocols` and no `/etc/protocols`. Its `nsswitch.conf` reads protocols through a `usrfiles` module, which the nixpkgs glibc does not have. The `getent protocols tcp` of the distribution still prints the entry, because the distribution glibc has that module. So that check passes on a host where git-annex fails. The probe showed that git-annex connected once a copy of `/usr/etc/protocols` stood at `/etc/protocols`, in a mount namespace that left the host unchanged.

## References

- [git-annex WebDAV special remote](https://git-annex.branchable.com/special_remotes/webdav/)
- [git-annex S3 special remote](https://git-annex.branchable.com/special_remotes/S3/)
- [git-annex manual](https://git-annex.branchable.com/git-annex/): `annex.cachecreds` and `annex.security.allowed-ip-addresses`
- [rclone serve webdav](https://rclone.org/commands/rclone_serve_webdav/)
- [rclone crypt](https://rclone.org/crypt/)

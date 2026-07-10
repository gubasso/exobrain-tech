# Personal Security Framework

A concise guide to establishing strong, consistent practices for your personal infrastructure. Think
of this as the “starter kit” for your own security operations.

---

<!--TOC-->

- [Secure Email Account](#secure-email-account)
- [Essential Tools](#essential-tools)
  - [Code Editor](#code-editor)
  - [Local Password Vault](#local-password-vault)
- [Public Dotfiles](#public-dotfiles)
- [Gopass (Command-Line Vault)](#gopass-command-line-vault)
- [Private Cloud Directory](#private-cloud-directory)
- [Private Configuration Repositories](#private-configuration-repositories)
- [Secret Material](#secret-material)

<!--TOC-->

## Secure Email Account

**Objective:** Use a dedicated, privacy-focused email address (with 2FA) for all critical services.

- **Example:** `example-user@proton.me`

- **Key Features to Enable:**

  - Two-Factor Authentication (2FA) – ideally hardware token (WebAuthn/U2F)
  - Strong, unique recovery codes stored offline

- **Use Cases:**

  - Infrastructure provisioning (cloud consoles, DNS providers)
  - Server access (SSH key recovery, alerts)
  - Online vaults and password managers
  - Cloud storage / Nextcloud accounts
  - Domain name registrar logins

> 💡 **Tip:** Wherever possible, use a hardware security key (e.g. YubiKey) for the second factor
> instead of SMS or TOTP to defend against phishing.

---

## Essential Tools

### Code Editor

- **Recommendation:** Switch from VS Code to **VSCodium**

  - Fully open-source, no telemetry
  - Compatible extensions ecosystem

- **Configuration Tips:**

  - Disable unneeded telemetry and automatic crash reports
  - Install security linters (e.g. ESLint for JavaScript, Bandit for Python)

### Local Password Vault

- **KeepassXC**
  - File example: `example-user.kbdx`
  - **Best Practices:**
    - Use a strong master password (passphrase ≥ 20 characters)
    - Enable key-file + master password combination
    - Regularly backup vault to encrypted media

> 🔒 **Tip:** Automate periodic exports and verify vault integrity with
>
> ```bash
> keepassxc-cli check-integrity example-user.kbdx
> ```

---

## Public Dotfiles

Maintain a public repository for your non-secret configuration and scripts:

```bash
git clone https://github.com/gubasso/dotfiles.git
```

- **Why:**

  - Showcases best practices
  - Enables easy setup on new machines

- **Security Additions:**

  - Use [git-secrets](https://github.com/awslabs/git-secrets) to scan for accidental commits of
    private keys
  - And/or: pre-commit hooks to security checks
  - Keep all secret templates out of the repo (e.g. `config.example` only)

> 📚 **Tip:** Include a `CONTRIBUTING.md` explaining how others can securely contribute (GPG-signed
> commits, branch protection).

---

## Gopass (Command-Line Vault)

Gopass is a modern, git-backed password manager for the CLI.

- **Setup Guide:**
  [./pass-gopass/gopass.md](./pass-gopass/gopass.md)

- **Storage:**

  - Host the git remote in a **private** GitLab repository
  - Encrypt all git communication via SSH and hardware-key agent

- **Usage Tips:**

  - Organize entries by domain (`github.com`, `aws/production`, etc.)
  - Use `gopass audit` to find weak or reused passwords
  - Integrate with editor plugins (e.g. VS Code Gopass extension)
  - Prioritize safer text editors like Vim/Neovim and Nano

---

## Private Cloud Directory

Define a single point of reference for your personal files in the cloud, e.g.:

```bash
export CLOUD_DIR="$HOME/Nextcloud"
```

Define a place to save and backup (sync with cloud) your private files, e.g.:

```bash
export PRIVATE_DIR="$CLOUD_DIR/Private"
```

- **Storage Providers:** Dropbox, Nextcloud, etc.

- **Access Control:**

  ```bash
  chmod 700 "$PRIVATE_DIR"
  ```

- **Backup Strategy:**

  - Use `restic` to back up encrypted snapshots to an offsite location

---

## Private Configuration Repositories

Keep non-secret but personal machine configuration in a private repository. Use generic templates publicly, scan before publishing, and avoid host-specific paths, account names, vault locations, or recovery details in public documentation.

## Secret Material

Do not store credentials, private keys, recovery codes, or passphrases in either public or private documentation repositories. Keep secrets in a dedicated encrypted secret manager and document only generic handling procedures.

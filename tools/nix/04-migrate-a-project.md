# 04 — Migrate one project off mise/rustup onto a Nix devShell

Do this per-project, incrementally. The dctl images already provide Nix + direnv; this just adds the
three files to a repo.

## Prerequisites

- Nix and direnv on the host — the dctl images already provide both.
- The project checked out, with its existing language manager still working.
- Network access, for the `nix flake lock` in step 3.

## Steps

1. **Pick the template.** Copy from [`templates/`](templates/) into the repo root:
   - Python (uv) → `templates/python/{flake.nix,.envrc}`
   - Rust → `templates/rust/{flake.nix,.envrc}` (keep the repo's existing `rust-toolchain.toml`)
   - Node → `templates/node/{flake.nix,.envrc}`
   - Zig → `templates/zig/{flake.nix,.envrc}`
   - Anything else → `templates/generic/{flake.nix,.envrc}`

2. **Tune `flake.nix`** to the runtime the project needs (e.g. `nodejs_22`, a Zig version, add
   `pkgs.<system-lib>` for native builds). For Python, pin the interpreter inside `requires-python`,
   and keep the uv `shellHook` that hands it to uv. See
   [Layering a language venv](./02-per-project-devshell.md#layering-a-language-venv-python--uv) and
   [Aligning the Python version](./02-per-project-devshell.md#aligning-the-python-version).

3. **Generate the lock** on a host with Nix + network (never fabricate it):
   ```bash
   nix flake lock
   ```

4. **`.gitignore`** `/.direnv/` and `/result`.

5. **Trust + enter** (host): `direnv allow`. In a dctl sandbox it's trusted automatically by the
   whitelist — just `cd` in.

6. **Install deps inside the shell** (the language manager still owns them):
   ```bash
   uv sync               # writes uv.lock; or: pnpm install / cargo build
   pre-commit install
   ```

7. **Drop non-Nix provisioning**: delete any `mise.toml` / `.tool-versions`, drop `mise install` /
   `rustup` steps from project scripts or CI, and run CI through the flake
   (`nix develop --command <task>`).

8. **Verify**: open a fresh shell, `cd` in, confirm the runtime resolves into the Nix store
   (`which python` / `cargo --version` matches the pin) with no manual activation.

## dctl specifics

- Once the repo has `flake.nix` + `.envrc`, the dctl lifecycle hooks
  (`direnv exec . uv sync`, etc.) light up automatically; before that they no-op.
- Rust: keep `rust-toolchain.toml` — the flake reads it (see
  [03-rust-toolchain](./03-rust-toolchain.md)); no `~/.rustup` volume anymore.

---
digest-of: tools/nix
last-synced: 2026-11-10
token-estimate: 600
---

# AGENTS

## Scope

Nix as a tool: the host install, the per-project flake devShell that direnv enters on `cd`, the
Rust toolchain, the migration of one repo onto a flake, and copy-paste templates. The Nix language
and flake syntax live in `languages/nix/`. Who owns each dependency of a project lives in
`programming/project-bootstrap/`, and this shelf links it.

## Key Points

- Ownership rule. Each dependency has one owner, the most specific file that declares it natively:
  the manifest of the stack, then `.pre-commit-config.yaml`, then the devshell, then the host. The
  rule lives in
  [Who owns each dependency](../../programming/project-bootstrap/03-local-dev-environment.md#who-owns-each-dependency)
  and [Who builds a hook environment](../../programming/project-bootstrap/04-quality-gates.md#who-builds-a-hook-environment).
  Pages here link it and never restate it.
- Three files per project: `flake.nix`, the generated `flake.lock`, and `.envrc` with `use flake`
  (`00-overview.md`).
- A flake holds no language library: no `python3.withPackages`, no `python3Packages.*`, no
  `nodePackages.*`. The devshell keeps the interpreter and its manager (`02-per-project-devshell.md`,
  "Notes").
- Python uses uv. The flake pins the interpreter, and the `shellHook` sets `UV_PYTHON` to it and
  `UV_PYTHON_DOWNLOADS=never`, runs `uv sync --frozen`, and puts `.venv/bin` on `PATH`.
  `requires-python` bounds the pin. `.envrc` stays `use flake` only (`02-per-project-devshell.md`,
  "Layering a language venv" and "Aligning the Python version", `templates/python/`).
- The Poetry two-interpreter trap and its `assert` guard serve projects that already use Poetry,
  and live in `languages/python/python-poetry.md`.
- The devshell holds `pre-commit` and only the tools of the hooks that keep `language: system`
  under an exception class, each with a `# owner: <owner> (<class>) <reason>` comment above it,
  plus the runtime pre-commit needs on a clean machine, such as nodejs for node hooks
  (`02-per-project-devshell.md`, "Git hooks (pre-commit)").
- direnv fires only in prompt-drawing shells. A command shell, CI step, or agent uses
  `direnv exec . <cmd>` or `nix develop --command <cmd>` (`02-per-project-devshell.md`).
- Binary manylinux wheels need `libstdc++.so.6` and `libz.so.1` on `LD_LIBRARY_PATH`
  (`02-per-project-devshell.md`, "Binary wheels").
- Rust uses the hybrid `rust-toolchain.toml` plus oxalica overlay (`03-rust-toolchain.md`).
- Migration is per project and incremental: pick a template, tune, lock, trust, `uv sync` or the
  stack's install, drop mise/rustup (`04-migrate-a-project.md`).
- Never fabricate a `flake.lock`. Generate it with `nix flake lock` on a host with Nix and network.

## Maintenance Notes

- Regenerate when a page here or the ownership rule of `programming/project-bootstrap/` changes.
- This digest routes. The pages it names win on disagreement.

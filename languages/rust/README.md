# Rust

Rust guidance: general language notes and idioms, the axum web framework, and database integration
(SQLx, chrono/serde datetimes) live as topic files here — the [`AGENTS.md`](./AGENTS.md) Source Map
indexes them file by file. The once-per-project spec is the landmark:

- [`project-bootstrap-spec/`](project-bootstrap-spec/README.md) — bootstrap a new Rust project:
  toolchain/layout and the rustfmt/clippy/deny quality gates (Rust binding of
  [general project-bootstrap](../../programming/project-bootstrap/README.md)).

Toolchain: the canonical per-project setup is a Nix devShell reading `rust-toolchain.toml` — see
[nix/03-rust-toolchain](../../tools/nix/03-rust-toolchain.md).

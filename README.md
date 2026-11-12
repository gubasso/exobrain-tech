# Exobrain Tech

This repository is a public technical knowledge base. The KB is the library, the content itself. It is the knowledge product, and it lives in the top-level topic buckets. `crates/` holds the programs that the repository ships to the projects that follow its rules, starting with the `exobrain` CLI. `_docs/` holds the repository governance and the metadata about both. It is not the product itself.

## KB Index

- [programming/](programming/) - software engineering practices, specs, design, and implementation notes.
- [languages/](languages/) - language-specific references and recipes.
- [platforms/](platforms/) - platform-specific notes for chains, web platforms, and ecosystems.
- [infra/](infra/) - cloud, containers, networking, servers, and operations.
- [systems/](systems/) - operating systems, desktop, security, and local system notes.
- [tools/](tools/) - developer tools, AI tooling, editors, and workflow utilities.
- [data/](data/) - databases, data engineering, and storage notes.
- [workflows/](workflows/) - repeatable work patterns and operating procedures.

## Uncategorized Notes

- [workflows/general-notes.md](workflows/general-notes.md) - staging area for uncategorized general
  notes and link collections from the original main index of the KB. Promote each entry into the
  right bucket when it matures.

## Host requirements

The devshell of `flake.nix` supplies every other tool. [`host-requirements.toml`](./host-requirements.toml)
lists each host tool that a recipe, a hook, or a script calls. The host also supplies the coreutils
and the standard ELF loader, `/lib64/ld-linux-x86-64.so.2`. On NixOS, nix-ld supplies the loader.

## License

The [Creative Commons Attribution 4.0 International](LICENSE) license (CC BY 4.0) covers the
knowledge content. The [MIT License](LICENSE-MIT) covers the software under `crates/`. Copyright
2026 Gustavo Basso.

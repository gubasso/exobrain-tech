# Mise

> Fallback tool-version manager. The canonical per-project environment manager is the Nix flake
> devShell — see [development-tools-workflow](./development-tools-workflow.md) and
> [nix](../tools/nix/README.md). dctl sandboxes provision tooling via Nix (see
> [nix](../tools/nix/README.md)); mise is a lighter-weight option on hosts where a flake is
> overkill.

## A uv project under mise

The host supplies uv, and uv owns the interpreter, so `mise.toml` of a uv project lists neither uv
nor Python in `[tools]`. mise only activates the venv that uv manages:

```toml
[settings]
python.uv_venv_auto = "source"
```

The convention is
[uv, the interpreter, and the environment manager](../languages/python/project-bootstrap-spec/00-toolchain-and-layout.md#uv-the-interpreter-and-the-environment-manager).

## Links

- [jdx / mise](https://github.com/jdx/mise)
  - https://mise.jdx.dev/
  - asdf-vm alternative
  - runtime environment isolation

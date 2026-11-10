# Python Poetry

> uv is the default Python package manager: see
> [Layering a language venv](../../tools/nix/02-per-project-devshell.md#layering-a-language-venv-python--uv).
> This page serves a project that already uses Poetry. Poetry is the dependency and packaging
> manager (deps, lockfile, scripts), not the environment manager. The Python runtime and the Poetry
> binary come from the project's Nix flake devShell, and Poetry runs inside it (`nix develop`, then
> `poetry install`). See [development-tools-workflow](../../workflows/development-tools-workflow.md)
> and [nix](../../tools/nix/README.md). The `pip install poetry` / `RUN pip install ...` lines below
> are container and deployment patterns, not local-dev setup.

- Poetry alternative (written in Rust): https://github.com/astral-sh/rye

To add a local module

```sh
poetry add --editable ../my-module-dir
```

Generates:

```toml
my-module-dir = {path = "../my-module-dir", develop = true}
```

```pyproject.toml
[tool.poetry.scripts]
my-script = "my_module:main"
```

`poetry run my-script`

`poetry export --output requirements.txt`

`poetry export --output requirements.txt --without-hashes`

- `PIP_DISABLE_PIP_VERSION_CHECK=1`: pip install -r /tmp/requirements.txt

To deploy in production with Docker (example):[^6]

```entrypoint.sh
python main.py
or
python myapp
```

About `__main__.py` as entrypoint: https://docs.python.org/3/library/**main**.html

## Deploy Example 1: `requirements.txt`

- Generate `requirements.txt` with a shell script, and build image[^7][^9]:

```build.sh
poetry export -o requirements.txt
docker build .
```

```Dockerfile
FROM python:3.8-slim-buster
PIP_DISABLE_PIP_VERSION_CHECK=1
WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt
COPY . .
RUN pip install /tmp/myapp
ENTRYPOINT ["entrypoint.sh"]
```

## Deploy Example 2: Poetry inside container[^7]

```Dockerfile
FROM python:3.8-slim-buster
PIP_DISABLE_PIP_VERSION_CHECK=1
WORKDIR /app
RUN pip install poetry
COPY pyproject.toml poetry.lock ./
RUN poetry install --no-root --no-dev
COPY . .
RUN poetry install --no-dev
ENTRYPOINT ["entrypoint.sh"]
```

## General

### reset a poetry virtual env

To reset a Poetry virtual environment, you can follow these steps to effectively remove the current
virtual environment and recreate it. This can be useful if your environment becomes corrupted or you
want to start fresh.

#### Steps to Reset a Poetry Virtual Environment

1. **Remove the Existing Virtual Environment** : Use the following command to remove the current
   virtual environment:

```bash
poetry env remove <python-version>
```

To get the specific `<python-version>`, you can use:

```bash
poetry env info --path
```

For example, if you’re using Python 3.9, you could run:

```bash
poetry env remove 3.9
```

If you want to remove all virtual environments for the project, you can use:

```bash
poetry env remove --all
```

1. **Recreate the Virtual Environment** : After removing the old virtual environment, you can
   recreate a new environment with:

```bash
poetry install
```

This will install all the dependencies listed in your `pyproject.toml` file into a new virtual
environment.

1. **Check the Status** : You can verify the status of your new virtual environment by running:

```bash
poetry env info
```

This should successfully reset the virtual environment for your Poetry-managed project.

maybe remove `poetry.lock`

## Aligning the Python version under Nix

The trap: pinning `pkgs.python313` in the flake does not decide the venv's Python. nixpkgs'
`pkgs.poetry` is built against nixpkgs' default CPython, so Poetry ships its own interpreter inside
its closure. Poetry 2.x picks the interpreter with `findpython`, then hands venv creation to the
`virtualenv` library, whose discovery re-resolves to the CPython in Poetry's own closure, even with
`virtualenvs.use-poetry-python = false`. With two CPythons on PATH, the flake's and Poetry's, the
resolver decides, not the config, and the venv silently lands on Poetry's version. (Refs:
python-poetry/poetry#10527, NixOS/nixpkgs#439512.)

The fix: declare the version, then guard it. Pin the interpreter explicitly, and add a native
`assert` that forces Poetry's own interpreter to equal it. The assert guarantees that the shell
holds a single Python version, and turns a future nixpkgs divergence into a loud build failure
instead of a silent venv on the wrong version:

```nix
let
  pkgs = import nixpkgs { inherit system; };
  python = pkgs.python313; # the one declared Python version
in
# pkgs.poetry creates the venv with the CPython it is built against, not the
# one on PATH. Assert they agree. A nixpkgs bump that moves Poetry's Python
# fails the build: bump `python` to match.
assert python.version == pkgs.poetry.python.version;
{
  devShells.default = pkgs.mkShell {
    packages = [ python pkgs.poetry pkgs.pre-commit pkgs.just ];
  };
}
```

Why this shape:

- Explicit pin. `python = pkgs.python313` declares the runtime version, and `flake.lock` pins it
  transitively. `requires-python` in `pyproject.toml` is package metadata (a range such as
  `>=3.11,<4.0`), not the dev pin. Keep the pin inside that range.
- Cache only. The binary cache holds only the Poetry built against nixpkgs' default CPython, so pin
  `python` to that version and both come from cache, with no `poetry env use` and no recompile.
- No silent drift. Without the `assert`, a pin that diverges from Poetry's Python brings back the
  two-Python race. With it, the build stops and names the pin to bump. Keep `poetry.toml`'s
  `use-poetry-python = false`, the 2.x default.

Rejected alternatives, in order of how tempting they look:

- `python = pkgs.poetry.python` binds to Poetry's interpreter with no pin. It has no race, but the
  version is implicit: whatever nixpkgs built Poetry against.
- An explicit pin without the `assert` brings back the race on the next nixpkgs bump.
- `pkgs.poetry.override { python3 = <pin>; }` pins a minor independently of nixpkgs' default, but
  recompiles Poetry and its whole closure with no cache hit.
- Removing `python` from the flake leaves no `python` on PATH, because nixpkgs' poetry wrapper
  exposes only `poetry`. Editors and ad-hoc scripts have no interpreter until the venv activates.
- `poetry env use …` in a `shellHook` is imperative, mutates Poetry's cache, and re-runs on every
  entry.
- `poetry python install` (Poetry 2.1+) downloads an interpreter outside the Nix store that
  `flake.lock` does not pin.
- poetry2nix `mkPoetryEnv` duplicates `poetry.lock` into Nix and adds override surface.

[^7]: [Execute command on host during docker build](https://stackoverflow.com/a/42754636)

[^9]: [Faster Docker builds with pipenv, poetry, or pip-tools](https://pythonspeed.com/articles/pipenv-docker/)

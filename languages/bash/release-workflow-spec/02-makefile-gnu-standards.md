# Bash Release — Makefile & GNU Coding Standards

Part of the [bash release-workflow spec](./README.md). General principle: **build & install
contract** — see the [general principles](../../../programming/release-workflow/README.md).

The single highest-leverage change. Per the
[GNU Coding Standards on Directory Variables](https://www.gnu.org/prep/standards/html_node/Directory-Variables.html)
and [DESTDIR](https://www.gnu.org/prep/standards/html_node/DESTDIR.html):

```make
PREFIX     ?= /usr/local
DESTDIR    ?=
bindir     ?= $(PREFIX)/bin
libdir     ?= $(PREFIX)/lib/<tool>
datadir    ?= $(PREFIX)/share/<tool>
sysconfdir ?= $(PREFIX)/etc

INSTALL ?= install

install:
	$(INSTALL) -d "$(DESTDIR)$(bindir)" "$(DESTDIR)$(libdir)" "$(DESTDIR)$(datadir)"
	$(INSTALL) -m 0755 bin/<tool> "$(DESTDIR)$(bindir)/"
	# ... etc
```

Every install path uses `$(DESTDIR)$(prefix-var)`. This is the contract every packaging tool
(`nfpm`, AUR `PKGBUILD`, `.deb`/`.rpm` post-install scripts) assumes.

Keep the `~/.local` ergonomics by exposing a convenience target:

```make
user-install:
	$(MAKE) PREFIX="$$HOME/.local" install
```

Add a `dist` target that produces a reproducible tarball:

```make
VERSION := $(shell cat VERSION)
DIST    := <tool>-$(VERSION)

dist:
	git archive --format=tar.gz --prefix=$(DIST)/ -o $(DIST).tar.gz HEAD
	sha256sum $(DIST).tar.gz > $(DIST).tar.gz.sha256
```

`git archive` produces a clean tarball from the committed tree — no `.git`, no work-clones, no temp
files.

Also provide a symmetric `uninstall` target.

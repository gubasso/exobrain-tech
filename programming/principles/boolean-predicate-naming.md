# Boolean Predicate Naming

A function or method that returns a boolean must read at the call site as an affirmative
proposition about a named subject, and calling it must change nothing. No major style guide
mandates an `is` prefix; the prefix is one of several mechanisms for satisfying that rule, and it is
correct only when the core word cannot stand alone as a proposition.

## Choosing the mechanism

The part of speech of the core word decides the form.

| Core word is             | Form                          | Examples                                |
| ------------------------ | ----------------------------- | --------------------------------------- |
| adjective or noun        | `is_<word>`                   | `is_empty`, `is_valid`, `is_present`    |
| verb                     | bare third-person verb        | `contains`, `intersects`, `starts_with` |
| containment or ownership | `has_<noun>`                  | `has_children`, `has_value`             |
| capability or policy     | `can_<verb>`, `should_<verb>` | `can_seek`, `should_retry`              |
| futurity                 | `will_<verb>`                 | `will_overflow`                         |

`empty` alone is an adjective, so it asserts nothing and could be read as a command to empty the
collection; `is_empty` fixes both. `contains` is already a third-person verb and therefore already a
complete proposition; `is_contains` is ungrammatical and `does_contain` adds a syllable carrying no
information.

Where the language provides a syntactic marker for predicates, use the marker and drop the prefix
entirely. Ruby and Scheme mark with a `?` suffix, Common Lisp with `p`. In those languages a prefix
is pure redundancy, and Ruby's linter rejects it.

## Binding rules

These hold in every language, regardless of which mechanism the name uses.

- Affirmative only. Name the positive concept and let callers negate it. A negated name produces
  double negation at the call site, as in `if not is_not_ready(x)`.
- Side-effect free. A predicate is a query under command-query separation, never a command. If it
  mutates, it takes an imperative verb name instead, and must not read as a question.
- Never a `get` prefix. Every guide that addresses getters rejects it, and `get_is_valid` is the
  worst of both forms.
- Never a `do` or `does` auxiliary. Cocoa bans it explicitly: `accepts_glyph_info`, not
  `does_accept_glyph_info`.
- The return type must actually be boolean. A name that reads as a question may not front an
  optional, an enum, or a tri-state. If the answer can be "unknown", return the enum and name the
  function for what it returns.

## Subject placement

The guides are written for methods, where the receiver supplies the subject: `coverage.is_clean`
completes the sentence on its own. A free function has no receiver, so the subject has to be named
somewhere.

- When the parameter is the subject, prefix it: `is_pid_alive(pid)` asserts something about `pid`.
- When the parameter is merely where you look, name the subject inside the function name:
  `has_license_watcher(host)`. Writing `is_license_watcher_present(host)` reads as though `host`
  were the thing being described.

## Worked example

A query for whether a configuration declares any overrides.

```python
def has_overrides(config: Config) -> bool:
    return bool(config.overrides)
```

Each rejected alternative fails one rule.

```text
is_overrides(config)        ungrammatical; the core word is a noun of ownership, not an adjective
check_overrides(config)     reads as a command, so the call site stops being a proposition
get_has_overrides(config)   the get prefix is rejected by every guide that mentions getters
has_no_overrides(config)    negated; forces `if not has_no_overrides(config)` at the call site
```

Two further checks apply after the name is settled. The body must not lazily parse, cache, or log,
because a name that reads as a question lies when calling it changes something. And if the function
must also report that the configuration failed to load, it is not a predicate at all — it returns a
tri-state and needs a name describing that.

## What the ecosystems state

Swift makes the rule about the reading rather than the prefix, and gives one prefixed and one
unprefixed example in the same sentence:

```text
Uses of Boolean methods and properties should read as assertions about the receiver when the
use is nonmutating, e.g. x.isEmpty, line1.intersects(line2).
```

Cocoa keys the form to the part of speech, and is the most explicit source for the rule above:
an adjective takes `is`, a verb takes the simple present tense with no prefix, and modal verbs are
allowed while `do` and `does` are not.

.NET states the affirmative requirement as mandatory and the prefix as optional:

```text
DO name Boolean properties with an affirmative phrase (CanSeek instead of CantSeek).
Optionally, you can also prefix Boolean properties with "Is", "Can", or "Has", but only where
it adds value.
```

Rust is the one major language mandating a prefix — RFC 430 requires `is_` or another short question
word, with exceptions for established names like `lt` and `ge`. Its standard library follows both
patterns anyway: `is_empty` and `is_some` beside `contains` and `starts_with`. The current API
Guidelines dropped the rule and retain only the prohibition on `get_`.

Ruby bans the prefix. The style guide calls `is`, `does`, and `can` redundant and inconsistent with
`empty?` and `include?`, and RuboCop ships `is_`, `has_`, and `have_` in the default forbidden-prefix
list.

Go, Kotlin, and Python state no boolean rule at all. Go's standard library uses both forms
(`IsZero`, `IsNotExist`, `HasPrefix`, `Contains`); a widely repeated claim that Go convention skips
the `Is` prefix has no primary source and is contradicted by that library.

| Ecosystem   | Marker                 | Prefix mandated | Canonical examples           |
| ----------- | ---------------------- | --------------- | ---------------------------- |
| Swift       | reads as an assertion  | no              | `isEmpty`, `intersects(_:)`  |
| Objective-C | part of speech         | adjective only  | `isEditable`, `showsAlpha`   |
| .NET        | affirmative phrase     | optional        | `CanSeek`, `HasValue`        |
| Rust        | `is_` or question word | yes             | `is_empty`, `contains`       |
| Go          | none stated            | no              | `IsZero`, `HasPrefix`        |
| Java, Beans | `is` accessor prefix   | for properties  | `isEmpty()`                  |
| Kotlin      | verb phrase            | no              | `isEmpty`, `contains`        |
| Python      | none stated            | no              | `isdigit`, `startswith`      |
| Ruby        | `?` suffix             | banned          | `empty?`, `include?`         |
| Scheme      | `?` suffix             | not applicable  | `zero?`, `eq?`               |
| Common Lisp | `p` or `-p` suffix     | not applicable  | `numberp`, `standard-char-p` |

## References

Verified against primary sources on 2026-08-25.

| Source                                  | URL                                                                                                                       |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| Swift API Design Guidelines             | <https://www.swift.org/documentation/api-design-guidelines/>                                                              |
| Cocoa coding guidelines, naming methods | <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CodingGuidelines/Articles/NamingMethods.html> |
| .NET, names of type members             | <https://learn.microsoft.com/en-us/dotnet/standard/design-guidelines/names-of-type-members>                               |
| Rust RFC 430, naming conventions        | <https://rust-lang.github.io/rfcs/0430-finalizing-naming-conventions.html>                                                |
| Rust style guide, predicates            | <https://doc.rust-lang.org/1.0.0/style/style/naming/README.html>                                                          |
| Rust API Guidelines, naming             | <https://rust-lang.github.io/api-guidelines/naming.html>                                                                  |
| Google Go style guide, decisions        | <https://google.github.io/styleguide/go/decisions>                                                                        |
| Kotlin coding conventions               | <https://kotlinlang.org/docs/coding-conventions.html>                                                                     |
| Google Python style guide               | <https://google.github.io/styleguide/pyguide.html>                                                                        |
| Ruby style guide                        | <https://rubystyle.guide/>                                                                                                |
| RuboCop naming cops                     | <https://docs.rubocop.org/rubocop/latest/cops_naming.html>                                                                |
| MIT/GNU Scheme naming conventions       | <https://web.mit.edu/scheme/scheme_v9.2/doc/mit-scheme-ref/Naming-Conventions.html>                                       |
| Common Lisp the Language, predicates    | <https://www.cs.cmu.edu/Groups/AI/html/cltl/clm/node69.html>                                                              |
| Command-query separation                | <https://martinfowler.com/bliki/CommandQuerySeparation.html>                                                              |

The .NET page is reprinted from the second edition (2008) and carries a staleness banner; whether
the third edition changed the boolean wording is unverified. The JavaBeans distinction between
`isX()` for primitive `boolean` and `getX()` for the wrapper is corroborated across secondary
sources but was not read at the specification itself.

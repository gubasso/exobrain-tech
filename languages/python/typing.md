# Typing a Python project

The typing contract of a Python project: which type carries which data, how a fallible operation
reports a failure, and how an untyped library enters the code. The gate that holds the contract is
in [01 — Quality gates](./project-bootstrap-spec/01-quality-gates.md#typecheck--mypy-strict), and
the `Result` rule at the exit boundary is in
[Error handling](./cli-spec/error-handling-python.md#a-result-inside-the-program). The worked
example is the vitals project, a Textual dashboard
([gubasso/vitals](https://github.com/gubasso/vitals), v0.1.0).

## Which type carries which data

| Data                                                          | Type                                                        |
| ------------------------------------------------------------- | ----------------------------------------------------------- |
| Outside data: a configuration file, the environment, a flag   | a pydantic model with `frozen=True` and `extra="forbid"`    |
| An internal value that crosses a module: a sample, a snapshot | `@dataclass(frozen=True, slots=True, kw_only=True)`         |
| A quantity with a unit                                        | a `NewType`, such as `Percent`, `Celsius`, `Bytes`, `Watts` |
| A closed set of names                                         | an `enum.StrEnum`                                           |
| The interface of an adapter                                   | a `typing.Protocol`, so a test passes a fake                |
| The outcome of a fallible operation                           | `Result[T, E]` from `returns`, with a typed `E`             |

pydantic validates where data enters, and nowhere else: inside the program a frozen dataclass is
cheaper and cannot change. No `dict[str, Any]` crosses a module boundary.

```python
Celsius = NewType("Celsius", float)


@dataclass(frozen=True, slots=True, kw_only=True)
class TemperatureSample:
    cpu_package: Celsius | None
    nvme: Celsius | None
```

## A fallible operation returns a Result

A function that can fail returns `Result[T, E]` from
[returns](https://returns.readthedocs.io/en/latest/pages/result.html), and `E` is a typed error of
its domain: a frozen dataclass with a `StrEnum` kind and a detail. It never raises the failure to its
caller. Compose results with `bind`, `map`, and the do-notation of `Result.do`:

```python
def collect_memory(source: MemorySource) -> Result[MemorySample, CollectorError]:
    return Result.do(
        MemorySample(memory=memory, swap=swap)
        for memory in source.memory()
        for swap in source.swap()
    )
```

`Result` is an abstract class, not a union of `Success` and `Failure`, so mypy cannot prove a
`match` on it exhaustive. Match on a helper that returns the union, and `exhaustive-match` fails the
`match` that misses a case:

```python
def outcome[T, E](result: Result[T, E]) -> Success[T] | Failure[E]:
    if isinstance(result, Success):
        return result
    if isinstance(result, Failure):
        return result
    raise TypeError(type(result).__name__)  # returns defines no third case


match outcome(snapshot.gpu):
    case Success(gpu):
        panel.show(gpu)
    case Failure(error):
        panel.show_failure(error)
```

`Result.value_or(None)` does not type-check where the value type excludes `None`. A helper
`value_or_none(result) -> T | None` that matches on `outcome` does.

## An untyped library enters through one adapter

One adapter module imports an untyped or partly typed library, and nothing else does. The adapter
wraps each call with `@safe`, maps the caught exception to an error kind with `.alt`, and exposes a
typed facade. `@safe` appears in an adapter and nowhere else. The one `# type: ignore` names its
code:

```python
import pynvml  # type: ignore[import-untyped]


def first_gpu(self) -> Result[GpuSample, CollectorError]:
    return safe(_read_first_gpu)().alt(nvml_error)
```

A test of the import boundary parses every module with `ast` and fails when a second module imports
the library.

## Python 3.15 lazy imports

With `requires-python = ">=3.15"`, ruff proposes `lazy from` imports
([PEP 810](https://peps.python.org/pep-0810/)) for a name that only annotations use, where older
targets moved it under `if TYPE_CHECKING:`. A lazy import loads on first use, so a dataclass and a
pydantic model still resolve their annotations at runtime.

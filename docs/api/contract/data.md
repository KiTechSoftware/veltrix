# Veltrix Data Contract

`veltrix::data` is the domain for small value-level utilities: parsing, formatting, normalization, validation, and conversion helpers for primitive or commonly reused data shapes.

The `data` domain should not own operating-system runtime behavior, service integrations, or Unicode tables. It should transform values that are already in memory.

## Versioning policy

Veltrix data helpers are pinned to Veltrix crate versions.

The `data` module is active as a v0.6.0 preview behind namespaced feature flags. It should not expose a child module unless a concrete implementation is ready.

### Compatibility rule

When `veltrix::data` is introduced, each child module should have a narrow purpose and a stable boundary. Avoid introducing a broad `utils` module or dumping unrelated helpers into `data`.

## Domain boundary

`veltrix::data` owns value-level helpers.

Good fits:

```rust
veltrix::data::bools
veltrix::data::time
veltrix::data::strings
veltrix::data::numbers
veltrix::data::bytes
```

Poor fits:

```rust
veltrix::data::paths      // belongs in veltrix::os::paths
veltrix::data::process    // belongs in veltrix::os::process
veltrix::data::clock      // belongs in veltrix::os::clock
veltrix::data::emojis     // belongs in veltrix::unicode::emojis
veltrix::data::podman     // belongs in veltrix::services::podman
```

Boundary rule:

```text
data::* = parse, format, validate, normalize, or convert values
os::*   = ask the operating system/runtime for state or perform OS actions
```

## Planned data domains

### bools

Status: **v0.6.0 preview**

Canonical path when introduced:

```rust
veltrix::data::bools
```

The `bools` module should contain boolean parsing, formatting, conversion, and predicate helpers.

Planned support:

- parse booleans from strings
- parse common truthy/falsy values
- format booleans into stable strings
- validate boolean-like configuration values
- expose strict and permissive parsing modes

Representative API surface:

```rust
veltrix::data::bools::parse_bool
veltrix::data::bools::parse_truthy_falsy
veltrix::data::bools::true_false
veltrix::data::bools::yes_no
veltrix::data::bools::BoolParseMode
```

Design guidance:

- strict parsing should only accept unambiguous values such as `true` and `false`
- permissive parsing may accept values such as `yes`, `no`, `on`, `off`, `1`, and `0`
- parsing behavior must be documented explicitly
- avoid locale-dependent boolean parsing unless locale support is intentionally added

### time

Status: **v0.6.0 preview**

Canonical path when introduced:

```rust
veltrix::data::time
```

The `time` module should contain value-level time parsing and formatting helpers. It should not read system time, monotonic time, process CPU time, or uptime.

Current v0.6+/v0.7 support:

- parse duration strings
- format durations
- expose whole-second conversion helpers

Representative API surface:

```rust
veltrix::data::time::parse_duration
veltrix::data::time::format_duration
veltrix::data::time::seconds
```

Deferred v1+ candidates:

```rust
veltrix::data::time::parse_timestamp
veltrix::data::time::format_timestamp
veltrix::data::time::DurationFormat
veltrix::data::time::TimestampFormat
```

Do not add timestamp parsing/formatting or format enums until the contract
chooses supported timestamp families, timezone behavior, backing types, and
strictness rules. `DurationFormat` should also wait until Veltrix supports at
least two duration output formats.

Boundary examples:

```rust
veltrix::data::time::parse_duration("5m")        // yes
veltrix::data::time::format_duration(duration)   // yes
veltrix::data::time::now()                       // no; use veltrix::os::clock
veltrix::data::time::process_cpu_time()          // no; use veltrix::os::clock
```

## Feature layout

Recommended future feature names:

```toml
[features]
default = []

data = []
data-bools = ["data"]
data-time = ["data"]
```

A simpler initial layout is also acceptable:

```toml
[features]
default = []

bools = []
time = []
```

Recommendation: use the namespaced feature names once the parent `data` module exists.

Recommended public layout:

```rust
pub mod data;
```

Inside `data`:

```rust
#[cfg(feature = "data-bools")]
pub mod bools;

#[cfg(feature = "data-time")]
pub mod time;
```

## Roadmap

### v0.1.0 — Released

Current state:

- no canonical `veltrix::data` module
- no documented stable data utility domain
- any existing boolean utility source is outside the canonical v0.2.0 public layout contract

### v0.2.0 — No public data module

Primary goal: do not introduce premature taxonomy.

Expected crate-level shape:

```rust
pub mod error;
pub mod os;
pub mod services;

#[cfg(feature = "emojis")]
pub mod emojis;
```

`data` may be mentioned in comments as a future planned domain, but should not be exposed unless implemented.

### v0.4.0 — Optional data planning

Primary goal: decide whether value-level helpers justify a parent domain.

Planned work:

- define final `bools` parsing semantics
- define final `time` parsing/formatting scope
- decide feature names
- decide whether transitional top-level aliases are needed
- document error behavior

### v0.6.0 — Data preview

Primary goal: introduce `veltrix::data` if there is enough implemented surface area.

Implemented modules:

```rust
veltrix::data::bools
veltrix::data::time
```

Expected guarantees:

- no OS clock behavior under `data::time`
- no Unicode table behavior under `data`
- no catch-all `utils` module
- stable parsing rules for exposed APIs

### v1.0.0 — Stable data API

Primary goal: stable public API for value-level data helpers.

Expected public modules, if implemented:

```rust
veltrix::data::bools
veltrix::data::time
```

v1 does not require every primitive type to have helpers. It means every exposed data helper is stable, documented, and intentionally located under the `data` domain.

Potential v1+ expansion:

- timestamp parsing and formatting once RFC3339/Unix/systemd-style scope is chosen
- `DurationFormat` once more than one duration representation is supported
- `TimestampFormat` once timestamp support exists and has more than one stable format

## Breaking-change policy

Veltrix should introduce a breaking change when:

- a parser accepts or rejects different input in a semantically incompatible way
- a formatter changes output format
- a default parsing mode changes
- error variants change in a way callers must handle differently
- a public data helper moves after stabilization

Veltrix should avoid silent behavior drift in parsers and formatters.

## Design rules

1. Keep value-level helpers under `veltrix::data`.
2. Keep boolean helpers under `veltrix::data::bools`.
3. Keep time parsing and formatting under `veltrix::data::time`.
4. Keep system/runtime clock helpers under `veltrix::os::clock`.
5. Keep Unicode-specific helpers under `veltrix::unicode`.
6. Avoid `utils` as a public module name.
7. Document strict versus permissive parsing behavior.
8. Prefer typed error types for non-trivial parsers.
9. Do not expose placeholder modules as stable APIs.
10. Treat v1 as stable and production-grade.

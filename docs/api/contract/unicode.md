# Veltrix Unicode Contract

`veltrix::unicode` is the planned domain for Unicode-aware helpers, including emoji constants and lookup helpers.

The intent of this module is to keep Unicode-specific APIs separate from generic data parsing, OS helpers, and service integrations.

## Versioning policy

Veltrix Unicode helpers are pinned to Veltrix crate versions and, where applicable, documented Unicode data versions.

The `unicode` module is **planned**, not active in v0.2.0. The existing v0.1.0 crate exposes `emojis` as a feature-gated top-level module. The long-term canonical path should become:

```rust
veltrix::unicode::emojis
```

### Compatibility rule

Top-level `veltrix::emojis` may remain as a transitional path while `unicode::emojis` is introduced. The canonical long-term domain should be `unicode::emojis`.

## Domain boundary

`veltrix::unicode` owns Unicode-specific helpers.

Good fits:

```rust
veltrix::unicode::emojis
veltrix::unicode::graphemes
veltrix::unicode::normalization
veltrix::unicode::width
veltrix::unicode::categories
veltrix::unicode::symbols
```

Poor fits:

```rust
veltrix::unicode::bools      // belongs in veltrix::data::bools
veltrix::unicode::time       // belongs in veltrix::data::time
veltrix::unicode::paths      // belongs in veltrix::os::paths
veltrix::unicode::process    // belongs in veltrix::os::process
```

Boundary rule:

```text
unicode::* = Unicode tables, Unicode semantics, text segmentation, symbols, emoji metadata
data::*    = generic value parsing, formatting, validation, and conversion
```

## Planned Unicode domains

### emojis

Status: **active as top-level legacy module, planned under unicode**

Current v0.1.x / v0.2.0 path:

```rust
veltrix::emojis
```

Canonical future path:

```rust
veltrix::unicode::emojis
```

The `emojis` module should contain emoji constants, lookup helpers, aliases, semantic labels, and related Unicode emoji metadata.

Planned support:

- emoji constants
- emoji lookup helpers
- aliases and semantic names
- optional grouping by category
- stable naming conventions for public constants
- documented Unicode data source/version if generated from Unicode datasets

Representative future API surface:

```rust
veltrix::unicode::emojis::lookup
veltrix::unicode::emojis::aliases
veltrix::unicode::emojis::category
veltrix::unicode::emojis::Emoji
veltrix::unicode::emojis::EmojiCategory
```

Design guidance:

- generated Unicode data should record the source Unicode version
- public constant names should be stable once v1 is reached
- lookup behavior should document case sensitivity and alias normalization
- emoji APIs should not be mixed into `data` merely because they are lookup helpers

### future Unicode modules

Status: **not planned for immediate implementation**

Potential future modules:

```rust
veltrix::unicode::graphemes
veltrix::unicode::normalization
veltrix::unicode::width
veltrix::unicode::categories
veltrix::unicode::symbols
```

These should only be added when Veltrix has a real implementation and a clear version/data-source policy.

## Feature layout

Current feature retained for compatibility:

```toml
[features]
default = []

emojis = []
```

Recommended future feature names:

```toml
[features]
default = []

unicode = []
unicode-emojis = ["unicode"]
```

Transition option:

```toml
emojis = ["unicode-emojis"]
```

Recommended future public layout:

```rust
pub mod unicode;
```

Inside `unicode`:

```rust
#[cfg(any(feature = "emojis", feature = "unicode-emojis"))]
pub mod emojis;
```

Optional transitional top-level alias:

```rust
#[cfg(feature = "emojis")]
pub use unicode::emojis;
```

## Roadmap

### v0.1.0 — Released

Current state:

- `emojis` exists as a top-level feature-gated module.
- the feature name is `emojis`.
- no canonical `veltrix::unicode` module exists.

Canonical v0.1.0 path:

```rust
veltrix::emojis
```

### v0.2.0 — Retain top-level emojis

Primary goal: avoid unnecessary churn while the `os` and `services` domains settle.

Expected crate-level shape:

```rust
pub mod error;
pub mod os;
pub mod services;

#[cfg(feature = "emojis")]
pub mod emojis;
```

`unicode` may be mentioned in comments as a future planned domain, but should not be exposed unless implemented.

### v0.4.0 — Unicode preview

Primary goal: introduce the Unicode parent domain.

Planned work:

- introduce `veltrix::unicode`
- move or re-export emoji helpers under `veltrix::unicode::emojis`
- keep or alias `veltrix::emojis` during transition if needed
- decide whether the feature remains `emojis` or becomes `unicode-emojis`
- document Unicode data versioning

Expected layout:

```rust
veltrix::unicode::emojis
```

Optional transition alias:

```rust
veltrix::emojis
```

### v0.6.0 — Unicode consistency pass

Primary goal: stabilize naming, feature flags, and data-source policy.

Planned work:

- freeze emoji constant naming conventions
- define lookup normalization rules
- document alias behavior
- decide whether any generated data needs build-time tooling
- add examples for emoji lookup and constants

### v1.0.0 — Stable Unicode API

Primary goal: stable public API for Unicode helpers.

Expected public modules, if implemented:

```rust
veltrix::unicode::emojis
```

v1 does not require broad Unicode support. It means every exposed Unicode helper is stable, documented, and intentionally located under the `unicode` domain.

## Breaking-change policy

Veltrix should introduce a breaking change when:

- a public emoji constant is renamed or removed
- lookup normalization changes incompatibly
- an alias maps to a different emoji
- Unicode data version changes produce incompatible public API behavior
- a public Unicode helper moves after stabilization

Veltrix should avoid silent lookup changes in patch releases.

## Design rules

1. Keep Unicode-specific helpers under `veltrix::unicode`.
2. Keep emoji helpers under `veltrix::unicode::emojis` once the Unicode domain is introduced.
3. Keep generic data parsing under `veltrix::data`, not `unicode`.
4. Keep OS/runtime helpers under `veltrix::os`, not `unicode`.
5. Document Unicode data source and version when applicable.
6. Keep feature-gating explicit.
7. Prefer stable public names for constants and aliases.
8. Avoid exposing placeholder modules as stable APIs.
9. Do not use `unicode` as a dumping ground for arbitrary text utilities.
10. Treat v1 as stable and production-grade.

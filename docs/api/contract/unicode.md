# Veltrix Unicode Contract

`veltrix::unicode` is the canonical domain for Unicode-aware helpers, including emoji constants and lookup helpers.

The intent of this module is to keep Unicode-specific APIs separate from generic data parsing, OS helpers, and service integrations.

## Versioning policy

Veltrix Unicode helpers are pinned to Veltrix crate versions and, where applicable, documented Unicode data versions.

The `unicode` module is active as a preview domain starting in v0.4.0. The v0.1.0 crate exposed `emojis` as a feature-gated top-level module. Starting in v0.7.0, the canonical path is:

```rust
veltrix::unicode::emojis
```

### Compatibility rule

Top-level `veltrix::emojis` was a transitional path during the v0.4.x through v0.6.x migration window. It is removed in v0.7.0.

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

Status: **active under unicode**

Legacy v0.1.x through v0.6.x path:

```rust
veltrix::emojis
```

Canonical path:

```rust
veltrix::unicode::emojis
```

The `emojis` module contains emoji constants, lookup helpers, and related Unicode emoji metadata.

Current v0.7 surface:

- emoji constants
- direct lookup helpers by emoji, name, and normalized search term
- grouping by Unicode emoji group and subgroup
- stable naming conventions for public constants
- documented Unicode data source/version if generated from Unicode datasets

Representative API surface:

```rust
veltrix::unicode::emojis::Emoji
veltrix::unicode::emojis::ALL
veltrix::unicode::emojis::find_by_emoji
veltrix::unicode::emojis::find_by_name
veltrix::unicode::emojis::find_by_search_term
veltrix::unicode::emojis::by_group
veltrix::unicode::emojis::by_subgroup
```

Deferred v1+ candidates:

```rust
veltrix::unicode::emojis::lookup
veltrix::unicode::emojis::aliases
veltrix::unicode::emojis::category
veltrix::unicode::emojis::EmojiCategory
```

Do not add these until their semantics are precise:

- `lookup` must define whether it searches emoji text, names, keywords, aliases, or normalized terms.
- `aliases` needs a real alias dataset, not a second name for keywords.
- `category` must be distinct from the existing `group` and `subgroup` fields.
- `EmojiCategory` should wait until Veltrix intentionally moves from string group/subgroup fields to typed category modeling.

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

Historical v0.1.x through v0.6.x feature retained for compatibility:

```toml
[features]
default = []

emojis = []
```

v0.7.0 feature names:

```toml
[features]
default = []

unicode = []
unicode-emojis = ["unicode"]
```

The legacy `emojis` feature is removed in v0.7.0. `unicode-emojis` enables both
the parent Unicode domain and the emoji data needed by
`veltrix::unicode::emojis`.

Recommended future public layout:

```rust
pub mod unicode;
```

Inside `unicode`:

```rust
#[cfg(feature = "unicode-emojis")]
pub mod emojis;
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

Implemented preview work:

- introduce `veltrix::unicode`
- re-export emoji helpers under `veltrix::unicode::emojis`
- keep `veltrix::emojis` during transition
- add `unicode` and `unicode-emojis` feature flags while retaining `emojis`
- document Unicode Emoji and CLDR source versions in generated constants

Expected layout:

```rust
veltrix::unicode::emojis
```

Optional transition alias:

```rust
veltrix::emojis
```

v0.4.0 generated emoji schema records:

- Unicode Emoji source version
- CLDR source version
- canonical Unicode name and normalized search name
- group and subgroup
- codepoints and qualification
- CLDR keywords and normalized search terms
- emoji version
- skin-tone modifier, variation-selector, and flag metadata

### v0.6.0 — Unicode consistency pass

Primary goal: stabilize naming, feature flags, and data-source policy.

Planned work:

- freeze emoji constant naming conventions
- define lookup normalization rules
- document alias behavior
- decide whether any generated data needs build-time tooling
- add examples for emoji lookup and constants

### v0.7.0 — Remove legacy emoji path

Primary goal: complete the Unicode migration by removing the top-level emoji alias.

Implemented work:

- remove the legacy `emojis` feature
- remove the legacy `veltrix::emojis` module path
- keep `veltrix::unicode::emojis` behind `unicode-emojis`
- update examples and documentation to use the canonical path

### v1.0.0 — Stable Unicode API

Primary goal: stable public API for Unicode helpers.

Expected public modules, if implemented:

```rust
veltrix::unicode::emojis
```

v1 does not require broad Unicode support. It means every exposed Unicode helper is stable, documented, and intentionally located under the `unicode` domain.

Potential v1+ expansion:

- unified `lookup` once search semantics are stable
- alias helpers once Veltrix has a source-backed alias dataset
- category helpers or `EmojiCategory` once category modeling is intentionally typed

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

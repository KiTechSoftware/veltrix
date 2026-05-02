# Veltrix Codegen

`veltrix-codegen` is the workspace generator for checked-in static data.

The generator is organized as a command dispatcher with domain-specific
subcommands. v0.4.0 uses the `emojis` subcommand to transform Unicode
`emoji-test.txt` and CLDR annotation XML into Rust modules under
`veltrix/src/emojis/`.

## Principles

- Source data lives in `workspace/data/`.
- Generated Rust files are checked in for fast downstream builds.
- Generated files must not be hand-edited.
- New generated domains should be added as new subcommands, not as separate
  one-off binaries.
- Generated schemas should record source data versions when the source format
  provides them.

## Emoji Pipeline

The emoji pipeline reads:

- `workspace/data/unicode-emoji.txt` for RGI emoji order, codepoints, grouping,
  qualification, and emoji version metadata.
- `workspace/data/unicode-cldr-en.xml` for English search keywords.

It writes:

- `workspace/veltrix/src/unicode/emojis/constants.rs`
- `workspace/veltrix/src/unicode/emojis/details.rs`

Run it from the repository root:

```sh
cargo run --manifest-path workspace/Cargo.toml -p veltrix-codegen -- emojis
```

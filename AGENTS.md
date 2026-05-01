# Veltrix — Agent Instructions

Lightweight Rust crate providing Unix OS helpers, path utilities, Unicode emoji metadata, and typed service integrations (Podman, Caddy, systemd, Technitium DNS).

## Workspace layout

```
workspace/
  Cargo.toml          ← manifest root; always pass --manifest-path workspace/Cargo.toml to cargo
  veltrix/            ← library crate
    src/
      lib.rs          ← public module declarations
      error.rs
      os/             ← paths, process, unistd
      services/       ← caddy, podman, systemd, technitium
      emojis/         ← generated; do not hand-edit
  veltrix-codegen/    ← general-purpose code generator (currently: emoji data)
workspace/data/       ← source data files consumed by veltrix-codegen
docs/api/contract/    ← authoritative API contracts; consult before adding or modifying public APIs
```

## Commands

All commands run from the repo root via `just`:

| Task             | Command               |
|------------------|-----------------------|
| Build            | `just build`          |
| Test             | `just test`           |
| Lint (clippy)    | `just lint`           |
| Format           | `just fmt`            |
| License check    | `just check-license`  |

Direct cargo invocations must include `--manifest-path workspace/Cargo.toml`.

## Canonical module paths (v0.2.0)

| Module                      | Feature flag     | Notes                                  |
|-----------------------------|------------------|----------------------------------------|
| `veltrix::os::paths`        |  none            | replaces v0.1.0 `veltrix::paths`       |
| `veltrix::os::process`      | —                | replaces v0.1.0 `veltrix::process`     |
| `veltrix::os::unistd`       | `unistd`         | replaces v0.1.0 `veltrix::unistd`      |
| `veltrix::services::podman` | `podman`         |                                        |
| `veltrix::services::caddy`  | `caddy`          |                                        |
| `veltrix::services::systemd`| `systemd`        |                                        |
| `veltrix::services::technitium` | `technitium` |                                        |
| `veltrix::emojis`           | `emojis`         | long-term path: `veltrix::unicode::emojis` |

Top-level `veltrix::paths` and `veltrix::unistd` are v0.1.0 paths — deprecated from v0.2.0 onward.

**Planned (not v0.2.0):** `veltrix::unicode` (v0.4.0+), `veltrix::data` (v0.4.0+), `veltrix::os::clock`.

## Feature flags

| Flag            | Enables                                              |
|-----------------|------------------------------------------------------|
| `async`         | Tokio-based async process execution                  |
| `unistd`        | Unix identity, group, hostname, and privilege helpers|
| `emojis`        | Emoji constants and lookup helpers                   |
| `podman`        | Podman CLI/socket integration                        |
| `podman-socket` | Podman async Unix-socket backend (implies `podman`)  |
| `caddy`         | Caddy admin API integration                          |
| `systemd`       | systemd service management helpers                   |
| `technitium`    | Technitium DNS API integration                       |

## Code generator

`veltrix-codegen` is the workspace's **general-purpose code generator**. It currently generates emoji data (`src/emojis/`). It is planned to cover additional domains as the crate grows.

- **Never hand-edit generated output files.** Re-run the generator instead.
- Run: `cargo run --manifest-path workspace/Cargo.toml -p veltrix-codegen`
- Specific subcommands are TBD as codegen expands beyond emojis.

## API contracts

Before adding or modifying any public API surface, read the relevant contract:

- [OS contract](docs/api/contract/os.md) — `veltrix::os` module layout and migration from v0.1.0
- [Services contract](docs/api/contract/services.md) — service integration scope and response model conventions
- [Unicode contract](docs/api/contract/unicode.md) — `veltrix::unicode` and future `emojis` path
- [Data contract](docs/api/contract/data.md) — `veltrix::data` planned domain and boundaries

## Commit conventions

- Make **scoped commits** per logical change before beginning any migration or release work.
- Check `git status` first; group uncommitted changes by module/scope; commit each group separately.
- Commit message format: `scope: short description` (e.g. `os/unistd: migrate from top-level re-export`).

## We use commit wizard for interactive commit message crafting

- See [commit types](docs/contributing/commit-types.md) for allowed types

# veltrix

**Practical Unix utilities, paths, and Unicode emoji data for Rust.**

`veltrix` is a lightweight Rust crate focused on useful Unix-first APIs, generated Unicode emoji metadata, and ergonomic developer helpers.

It is designed for projects that want clean utilities without pulling in large frameworks or overly broad abstractions.

## Features

* 🐧 Unix identity, process, and environment helpers
* 📁 Path utilities for common user locations
* 😀 Generated Unicode emoji dataset
* 🔎 Emoji names, groups, versions, and keywords
* ⚙️ Feature-gated modules
* 🦀 Small, Rust-first APIs

## Installation

```toml
[dependencies]
veltrix = "0.4"
```

Optional features:

```toml
veltrix = { version = "0.4", features = [
    "async",
    "podman",
    "docker",
    "caddy",
    "unicode-emojis",
] }
```

## Feature Flags

| Feature            | Enables                                              |
| ------------------ | ---------------------------------------------------- |
| `async`            | Tokio-based async process execution                  |
| `unistd`           | Unix identity, group, hostname, and privilege helpers|
| `emojis`           | Deprecated, use `unicode-emojis`                     |
| `unicode`          | Unicode parent module                                |
| `unicode-emojis`   | Canonical Unicode emoji path plus emoji data         |
| `podman`           | Podman CLI/socket integration                        |
| `podman-socket`    | Podman async Unix-socket backend (implies `podman`)  |
| `docker`           | Docker CLI/socket/Compose foundation types           |
| `docker-socket`    | Docker async Unix-socket backend (implies `docker`)  |
| `caddy`            | Caddy admin API integration                          |
| `systemd`          | systemd service management helpers                   |
| `technitium`       | Technitium DNS API integration                       |

## Modules

## `veltrix::os::unistd`

Unix-only helpers for users, groups, processes, and environment.

### Core Identity

```rust
use veltrix::os::unistd::*;

let uid = getuid();
let euid = geteuid();
let gid = getgid();
let pid = getpid();

println!("uid={uid} euid={euid} gid={gid} pid={pid}");
```

Available types and functions:

* `Uid`
* `Gid`
* `Pid`
* `getuid()`
* `geteuid()`
* `getgid()`
* `getegid()`
* `getpid()`
* `getppid()`

### User / Group Lookup

```rust
use veltrix::os::unistd::*;

let root = uid_by_username("root");
let user = username_by_uid(getuid());
let groups = groups_for_uid(getuid());
```

Available helpers:

* `username_by_uid()`
* `uid_by_username()`
* `groupname_by_gid()`
* `gid_by_groupname()`
* `primary_gid_by_uid()`
* `groups_for_uid()`

### Host / Process Environment

```rust
use veltrix::os::unistd::*;

let host = gethostname()?;
let cwd = getcwd()?;
let home = home_dir();

chdir("/tmp")?;
```

Available helpers:

* `gethostname()`
* `getcwd()`
* `chdir()`
* `home_dir()`

### Privilege Helpers

```rust
use veltrix::os::unistd::*;

if is_effective_root() {
    println!("running as root");
}

if user_in_admin_group(getuid()) {
    println!("user is in a common admin group");
}
```

Available helpers:

* `is_root()`
* `is_effective_root()`
* `user_in_group()`
* `user_in_admin_group()`
* `has_common_admin_group()`

> Admin group helpers are convenience heuristics and not authoritative privilege checks.

## `veltrix::os::paths`

Helpers for common user and application paths.

```rust
let bin = veltrix::os::paths::user_bin_path("mytool")?;
println!("{}", bin.display());
```

## `veltrix::services`

Typed integrations for local and self-hosted service management (Podman, Docker, Caddy, systemd, Technitium DNS). Each service is feature-gated; incomplete integrations expose foundation types before full workflow clients.

```rust
// Example: Podman (requires "podman" feature)
use veltrix::services::podman::{
    PodmanAutoUpdatePolicy, PodmanCliClient, PodmanCliSpec, QuadletUnit,
};

let podman = PodmanCliClient::new(PodmanCliSpec::new());
let containers = podman.containers()?;

let quadlet = QuadletUnit::container("web", "docker.io/library/caddy:latest")
    .auto_update(PodmanAutoUpdatePolicy::Registry)
    .render();
```

Supported services:

* Podman (container runtime)
* Docker (container runtime, socket, and Compose foundations)
* Caddy (web server / reverse proxy)
* systemd (service management)
* Technitium DNS (DNS server)

## `veltrix::unicode::emojis`

Generated Unicode emoji metadata with search-friendly fields.

```rust
use veltrix::unicode::emojis::details::{ALL, find_by_search_term};

for emoji in ALL.iter().take(5) {
    println!("{} {} {}", emoji.emoji, emoji.name, emoji.unicode_version);
}

let smile = find_by_search_term("smile");
```

Each emoji entry includes:

* emoji character
* canonical name
* group
* subgroup
* codepoints
* keywords
* normalized search terms
* emoji version
* Unicode Emoji data version
* skin-tone and variation-selector metadata

The legacy `veltrix::emojis` path remains available during the transition.

## Design Goals

`veltrix` is built around:

* practical APIs first
* small focused modules
* low runtime overhead
* strong types where useful
* generated static data
* no unnecessary complexity

## Platform Support

## Unix

Primary supported target.

Examples:

* Linux
* macOS
* BSD
* other Unix-like systems

## Windows

Not currently supported.

Windows compatibility is not a present goal.

## Stability

`veltrix` is pre-release. APIs may evolve before `1.0`.

## License

MIT OR Apache-2.0

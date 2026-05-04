# veltrix

**Practical Unix utilities, paths, and Unicode emoji data for Rust.**

`veltrix` is a lightweight Rust crate focused on useful Unix-first APIs, generated Unicode emoji metadata, and ergonomic developer helpers.

It is designed for projects that want clean utilities without pulling in large frameworks or overly broad abstractions.

## Features

* 🐧 Unix identity, process, and environment helpers
* 📁 Path utilities for common user locations
* ⏱️ OS/runtime clock helpers
* 😀 Generated Unicode emoji dataset
* 🔎 Emoji names, groups, versions, and keywords
* ⚙️ Feature-gated modules
* 🦀 Small, Rust-first APIs

## Installation

```toml
[dependencies]
veltrix = "0.7.1"
```

Optional features:

```toml
veltrix = { version = "0.7.1", features = [
    "async",
    "podman",
    "docker",
    "caddy",
    "systemd-dbus",
    "technitium",
    "data-bools",
    "data-time",
    "unicode",
    "unicode-emojis",
    "ldap",
] }
```

## Feature Flags

| Feature            | Enables                                              |
| ------------------ | ---------------------------------------------------- |
| `async`            | Tokio-based async process execution                  |
| `unistd`           | Unix identity, group, hostname, and privilege helpers|
| `unicode`          | Unicode parent module                                |
| `unicode-emojis`   | Canonical Unicode emoji path plus emoji data         |
| `podman`           | Podman CLI/socket integration                        |
| `podman-socket`    | Podman async Unix-socket backend (implies `podman`)  |
| `docker`           | Docker CLI/socket/Compose foundation types           |
| `docker-socket`    | Docker async Unix-socket backend (implies `docker`)  |
| `caddy`            | Caddy admin API integration                          |
| `systemd`          | systemd service management helpers                   |
| `systemd-dbus`     | systemd D-Bus manager backend via `busctl`           |
| `technitium`       | Technitium DNS API integration                       |
| `data`             | Data parent module                                   |
| `data-bools`       | Boolean parsing and formatting helpers               |
| `data-time`        | Time value parsing and formatting helpers            |
| `full`             | All non-legacy feature groups                        |

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

## `veltrix::os::clock`

Runtime and platform clock helpers.

```rust
use veltrix::os::clock;

let wall = clock::now();
let started = clock::monotonic();
let timestamp = clock::unix_timestamp()?;
let uptime = clock::uptime()?;

println!("{wall:?} {timestamp:?} {uptime:?} {:?}", clock::elapsed_since(started));
```

## `veltrix::data`

Value-level parsing and formatting helpers.

```rust
use veltrix::data::{bools, time};

let enabled = bools::parse_truthy_falsy("on")?;
let duration = time::parse_duration("1h30m")?;

assert_eq!(enabled, true);
assert_eq!(time::format_duration(duration), "1h30m");
```

## `veltrix::services`

Typed integrations for local and self-hosted service management (Podman, Docker, Caddy, systemd, Technitium DNS). Each service is feature-gated; incomplete integrations expose foundation types before full workflow clients.

```rust
// Example: Podman (requires "podman" feature)
use veltrix::services::podman::{
    PodmanAutoUpdatePolicy, PodmanCliClient, PodmanCliSpec, PodmanLabel, QuadletUnit,
};

let podman = PodmanCliClient::new(PodmanCliSpec::new());
let containers = podman.containers()?;

podman.run_container_with_labels(
    [PodmanLabel::new("com.example.role", "web")?],
    ["docker.io/library/caddy:latest"],
)?;

let quadlet = QuadletUnit::container("web", "docker.io/library/caddy:latest")
    .label(PodmanLabel::new("com.example.role", "web")?)
    .auto_update(PodmanAutoUpdatePolicy::Registry)
    .render();
```

```rust
// Example: Docker (requires "docker" feature)
use veltrix::services::docker::{DockerCliClient, DockerCliSpec, DockerComposeClient, DockerComposeSpec};

let docker = DockerCliClient::new(DockerCliSpec::new());
let containers = docker.containers()?;
let images = docker.images()?;

let compose = DockerComposeClient::new(DockerComposeSpec::new().compose_file("compose.yaml"));
compose.up(["-d"])?;
```

```rust
// Example: Caddy (requires "caddy" feature)
use veltrix::services::caddy::{CaddyAdminClient, CaddyCliClient, CaddyCliSpec, CaddyConfig};

let caddy = CaddyCliClient::new(CaddyCliSpec::new());
caddy.validate(["--config", "Caddyfile"])?;

let admin = CaddyAdminClient::localhost_default();
let config = CaddyConfig::local_https_reverse_proxy("app.local", ["127.0.0.1:3000"])?;
admin.load_config(&config).await?;
```

```rust
// Example: systemd (requires "systemd" feature)
use veltrix::services::systemd::{SystemdCliClient, SystemdCliSpec};

let systemd = SystemdCliClient::new(SystemdCliSpec::new().user());
let status = systemd.status("app.service")?;
let active = systemd.is_active("app.service")?;
let units = systemd.list_units(Some("app*.service"))?;
let logs = systemd.tail_journal_entries("app.service", 100)?;
```

```rust
// Example: Technitium DNS (requires "technitium" feature)
use veltrix::services::technitium::{
    TechnitiumAuth, TechnitiumClient, TechnitiumHttpSpec, TechnitiumRecordType,
    acme_challenge_name, caddy_acme_challenge_name,
};

let dns = TechnitiumClient::new(
    TechnitiumHttpSpec::new("http://localhost:5380")
        .auth(TechnitiumAuth::session_token("token")),
)?;

let zones = dns.zones().await?;
let answer = dns.resolve("app.local", TechnitiumRecordType::A).await?;
let record = acme_challenge_name("app.local");
let caddy_record = caddy_acme_challenge_name("app.local");
dns.set_acme_challenge("local", "app.local", "dns-01-token", Some(60)).await?;
```

Supported services:

* Podman (container runtime)
* Docker (container runtime, socket, and Compose workflows)
* Caddy (web server / reverse proxy)
* systemd (service management)
* Technitium DNS (DNS server)

See [Technitium ACME DNS-01 Certificates](docs/api/v0/technitium-acme.md) for the v0.7.0 certificate helper flow.

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

The legacy `veltrix::emojis` path was removed for v0.7.0. Use `veltrix::unicode::emojis`.

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

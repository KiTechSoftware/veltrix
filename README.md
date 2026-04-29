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
veltrix = "0.1"
```

Optional features:

```toml
veltrix = { version = "0.1", features = [
    "paths",
    "full",
] }
```

## Feature Flags

| Feature | Enables               |
| ------- | --------------------- |
| `paths` | Path-related helpers  |
| `full`  | All optional features |

## Modules

## `veltrix::unistd`

Unix-only helpers for users, groups, processes, and environment.

## Core Identity

```rust
use veltrix::unistd::*;

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

## User / Group Lookup

```rust
use veltrix::unistd::*;

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

## Host / Process Environment

```rust
use veltrix::unistd::*;

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

## Privilege Helpers

```rust
use veltrix::unistd::*;

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

---

## `veltrix::paths`

Helpers for common user and application paths.

```rust
let bin = veltrix::paths::user_bin_path("mytool")?;
println!("{}", bin.display());
```

---

## `veltrix::emojis`

Generated Unicode emoji metadata with search-friendly fields.

```rust
use veltrix::emojis::details::ALL;

for emoji in ALL.iter().take(5) {
    println!("{} {}", emoji.emoji, emoji.name);
}
```

Each emoji entry includes:

* emoji character
* canonical name
* group
* subgroup
* codepoints
* keywords
* emoji version

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

## Roadmap

### Backlog

* [ ] richer emoji keyword sources
* [ ] additional path helpers
* [ ] more Unix utilities
* [ ] improved search APIs
* [ ] expanded documentation

## License

MIT OR Apache-2.0

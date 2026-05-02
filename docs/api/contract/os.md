# Veltrix OS Contract

`veltrix::os` contains typed operating-system helpers for process execution, path resolution, and Unix/POSIX-style identity/runtime primitives.

The intent of this module is to provide stable Rust APIs over local OS runtime behavior without mixing those APIs into unrelated top-level crate namespaces.

The `os` domain is not experimental scaffolding. Public APIs should be explicit, typed, and version-linked. If a platform-specific behavior cannot be represented consistently, Veltrix should expose that limitation directly rather than hiding it behind weak or ambiguous types.

## Versioning policy

Veltrix OS helpers are pinned to Veltrix crate versions rather than external service API versions.

For v0.2.0, the primary contract change is a module-layout migration:

```rust
veltrix::paths    -> veltrix::os::paths
veltrix::process  -> veltrix::os::process
veltrix::unistd   -> veltrix::os::unistd
```

No behavioral expansion is required for v0.2.0 beyond relocating the existing v0.1.0 modules under `veltrix::os`.

### Compatibility rule

v0.2.0 should preserve the semantics of the v0.1.0 APIs while changing their canonical module paths.

Temporary top-level aliases may be retained during the transition if desired, but the documented canonical paths should become:

```rust
veltrix::os::paths
veltrix::os::process
veltrix::os::unistd
```

## Supported OS domains

### paths

Status: **active**

Canonical path from v0.2.0 onward:

```rust
veltrix::os::paths
```

The `paths` module provides common platform path constants and helpers for resolving system-wide and per-user locations.

Current v0.1.0 capabilities include:

- system path constants
- user/XDG environment constants
- user-level default path constants
- file extension constants
- system config, state, cache, log, runtime, data, lib, libexec, doc, man, desktop-entry, icon, and systemd-unit path helpers
- user config, state, cache, data, runtime, log, bin, systemd-unit, desktop-entry, and icon path helpers
- config-path resolution helpers
- app runtime default helpers
- `~` expansion
- `$HOME` resolution
- XDG directory resolution

Representative API surface:

```rust
veltrix::os::paths::system_config_dir
veltrix::os::paths::system_config_path
veltrix::os::paths::user_config_dir
veltrix::os::paths::user_config_path
veltrix::os::paths::resolve_config_path
veltrix::os::paths::resolve_new_config_path
veltrix::os::paths::home_dir
veltrix::os::paths::xdg_dir
```

### process

Status: **active**

Canonical path from v0.2.0 onward:

```rust
veltrix::os::process
```

The `process` module provides command specification and execution helpers.

Current v0.1.0 capabilities include:

- `CmdSpec` command specification
- program and argument construction
- optional `sudo` execution
- optional UID/GID execution
- conversion to `std::process::Command`
- synchronous command execution
- synchronous success-status helper
- async command execution behind the `async` feature
- async success-status helper behind the `async` feature
- optional typed UID/GID integration with `unistd` behind the `unistd` feature

Representative API surface:

```rust
veltrix::os::process::cmd::spec::CmdSpec
veltrix::os::process::cmd::std_cmd::run
veltrix::os::process::cmd::std_cmd::status_ok
veltrix::os::process::cmd::async_cmd::run
veltrix::os::process::cmd::async_cmd::status_ok
```

### unistd

Status: **active, feature-gated**

Canonical path from v0.2.0 onward:

```rust
veltrix::os::unistd
```

The `unistd` module provides Unix/POSIX-style identity, group, process, hostname, working-directory, and privilege helpers.

Current v0.1.0 capabilities include:

- typed `Uid`, `Gid`, and `Pid` wrappers
- raw ID conversion helpers
- root checks
- real/effective UID and GID helpers
- process ID and parent process ID helpers
- username lookup by UID
- UID lookup by username
- group-name lookup by GID
- GID lookup by group name
- primary GID lookup by UID
- group membership lookup using `/etc/group`
- hostname lookup
- current working directory lookup
- directory change helper
- current-user home-directory lookup
- common admin group heuristics
- sudoers common-admin-group heuristic

Representative API surface:

```rust
veltrix::os::unistd::Uid
veltrix::os::unistd::Gid
veltrix::os::unistd::Pid
veltrix::os::unistd::getuid
veltrix::os::unistd::geteuid
veltrix::os::unistd::getgid
veltrix::os::unistd::getegid
veltrix::os::unistd::getpid
veltrix::os::unistd::getppid
veltrix::os::unistd::gethostname
veltrix::os::unistd::getcwd
veltrix::os::unistd::chdir
veltrix::os::unistd::is_root
veltrix::os::unistd::is_effective_root
```

### clock

Status: **v0.6.0 preview**

Canonical path:

```rust
veltrix::os::clock
```

The `clock` module contains operating-system or runtime clock helpers. It should be used for APIs that ask the OS, process runtime, or platform clock source for time-related state.

Current support includes:

- current system time
- monotonic time
- process CPU time
- thread CPU time
- system uptime

Representative future API surface:

```rust
veltrix::os::clock::now
veltrix::os::clock::monotonic
veltrix::os::clock::unix_timestamp
veltrix::os::clock::process_cpu_time
veltrix::os::clock::thread_cpu_time
veltrix::os::clock::uptime
```

Boundary rule:

```text
os::clock  = asks the operating system/runtime for clock data
data::time = parses, formats, validates, or converts time values
```

Examples:

```rust
veltrix::os::clock::now()                 // yes
veltrix::os::clock::process_cpu_time()    // yes
veltrix::data::time::parse_duration("5m") // not os::clock
veltrix::data::time::format_timestamp(ts) // not os::clock
```

`process_cpu_time`, `thread_cpu_time`, and `uptime` are backed by Linux clock sources in v0.6.0. Unsupported platforms return explicit `VeltrixError::Validation` errors until a platform implementation is added.

## Feature layout

Recommended v0.2.0 feature names:

```toml
[features]
default = []

async = ["tokio/process"]

unistd = ["libc"]
emojis = []

podman = ["serde", "serde_json"]
podman-socket = ["podman", "async", "tokio/net", "tokio/io-util"]

caddy = ["async", "serde", "serde_json", "reqwest/json", "tokio/net", "tokio/io-util"]

systemd = []
technitium = ["async", "serde", "serde_json", "reqwest/json"]
```

The `os` module itself should not require a feature flag. Its children should preserve their existing feature behavior:

```rust
pub mod os;
```

Inside `os`:

```rust
pub mod paths;
pub mod process;

#[cfg(feature = "unistd")]
pub mod unistd;
```

Recommended public layout:

```rust
veltrix::os::paths
veltrix::os::process
veltrix::os::unistd
```

## Roadmap

### v0.1.0 — Released

Current state:

- `paths` exists as a top-level module.
- `process` exists as a top-level module.
- `unistd` exists as a top-level module behind the `unistd` feature.
- `process::cmd::CmdSpec` supports program, args, `sudo`, UID, and GID configuration.
- `process` supports sync command execution.
- `process` supports async command execution behind the `async` feature.
- `unistd` exposes typed `Uid`, `Gid`, and `Pid` wrappers.
- `paths` exposes XDG and system path helpers.

Canonical v0.1.0 paths:

```rust
veltrix::paths
veltrix::process
veltrix::unistd
```

### v0.2.0 — OS module migration

Primary goal: move OS-runtime primitives into a coherent domain.

Required work:

- introduce `veltrix::os`
- move `paths` to `veltrix::os::paths`
- move `process` to `veltrix::os::process`
- move `unistd` to `veltrix::os::unistd`
- keep `unistd` behind the `unistd` feature
- preserve existing `async` behavior for process helpers
- update internal references from `crate::paths` to `crate::os::paths`
- update internal references from `crate::process` to `crate::os::process`
- update internal references from `crate::unistd` to `crate::os::unistd`
- update documentation examples to use the new canonical paths

Expected layout:

```rust
veltrix::os::paths
veltrix::os::process
veltrix::os::unistd
```

No new behavior is required for v0.2.0.

Optional transition aliases:

```rust
pub use os::paths;
pub use os::process;

#[cfg(feature = "unistd")]
pub use os::unistd;
```

If aliases are kept, they should be documented as transitional and not treated as the long-term canonical API.

### v0.3.0 — OS API consistency pass

Primary goal: make the OS domain consistent and predictable.

Planned work:

- normalize naming across `paths`, `process`, and `unistd`
- audit `Result<T>` usage versus `std::io::Result<T>` usage
- decide whether OS helpers should use `VeltrixError` consistently
- add examples for common path resolution
- add examples for sync and async command execution
- add examples for UID/GID-aware command execution
- document platform support clearly
- add compile tests for feature combinations

### v0.4.0 — Platform model refinement

Primary goal: clarify Unix-only and cross-platform boundaries.

Planned work:

- explicitly mark Unix-only APIs
- avoid exposing Unix-specific helpers on unsupported targets
- introduce OS clock helpers:

```rust
veltrix::os::clock
```

- consider internal platform modules:

```rust
veltrix::os::unix
veltrix::os::windows
```

- keep public cross-platform APIs under stable `veltrix::os::*` paths where practical
- improve error behavior for unsupported operations

### v1.0.0 — Stable OS API

Primary goal: stable public API for Veltrix OS helpers.

Expected guarantees:

- stable module paths
- stable feature names
- stable typed ID wrappers
- documented platform behavior
- documented feature-gating behavior
- no accidental top-level OS helper sprawl
- no placeholder modules exposed as production APIs
- command execution behavior documented clearly
- path resolution semantics documented clearly

Expected public modules:

```rust
veltrix::os::paths
veltrix::os::process
veltrix::os::unistd
veltrix::os::clock
```

v1 does not require every OS primitive to be wrapped. It means every exposed OS helper is stable, documented, and intentionally located under the `os` domain.

## Breaking-change policy

Veltrix should introduce a breaking change when:

- a public OS helper is moved or renamed after the transition period
- a path-resolution rule changes incompatibly
- a command execution helper changes process, sudo, UID, or GID semantics
- a typed ID wrapper changes representation or conversion behavior
- an error type changes in a way callers must handle differently
- a platform-specific behavior cannot be preserved safely

Veltrix should avoid silent weakening such as replacing typed UID/GID/PID values with untyped integers in stable APIs.

## Design rules

1. Keep OS-runtime primitives under `veltrix::os`.
2. Keep path helpers under `veltrix::os::paths`.
3. Keep process helpers under `veltrix::os::process`.
4. Keep Unix/POSIX identity helpers under `veltrix::os::unistd`.
5. Keep system/runtime clock helpers under `veltrix::os::clock`.
6. Keep time parsing and formatting under `veltrix::data::time`.
7. Preserve feature-gating for Unix-specific APIs.
8. Prefer typed wrappers for OS identifiers such as UID, GID, and PID.
9. Avoid mixing service integrations into the OS domain.
10. Avoid exposing placeholder modules as stable APIs.
11. Document platform-specific behavior explicitly.
12. Treat v1 as stable and production-grade.

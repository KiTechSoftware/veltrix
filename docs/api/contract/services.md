# Veltrix Services Contract

`veltrix::services` contains typed integrations for local and self-hosted service management.

The intent of this module is to provide stable Rust APIs over service-specific control planes such as:

- Docker
- Podman
- Caddy
- systemd
- Technitium DNS Server

The `services` domain is not experimental scaffolding. Public APIs should be explicit, typed, documented, and version-linked. If an upstream service changes its API in a breaking way, Veltrix should introduce a breaking change rather than silently weakening types.

## v1 scope

Veltrix Services v1 should cover the complete tool usage surface described by the internal **Developer Tool Usage Guide — Podman, Docker, Caddy, systemd, and Technitium DNS**.

That means v1 is expected to provide stable, documented Rust APIs for the guide's core service-management workflows:

- Docker container, image, compose, network, volume, system cleanup, and Engine API workflows
- Podman container, image, pod, Kubernetes YAML, Quadlet/systemd, machine, secret, compose, auto-update, and REST API workflows
- Caddy CLI, Caddyfile/configuration, local HTTPS, reverse proxy, admin API, and runtime config workflows
- systemd service lifecycle, unit inspection, journal access, unit-file operations, timers, overrides, templates, resource limits, and watchdog/deployment workflows
- Technitium DNS deployment, authentication, zone management, DNS record management, settings, resolving, logs, stats, blocking, and CI/CD automation workflows
- cross-tool patterns such as Docker/Podman with Caddy, containers managed by systemd, Caddy with Technitium DNS, systemd timers for containerized jobs, and full-stack local development environments

v1 does not require Veltrix to expose every upstream endpoint. It does require every workflow covered by the guide to have a stable Veltrix representation, either as a typed API, a clearly documented adapter, or an intentionally unsupported item with rationale.

## Versioning policy

Veltrix service integrations are pinned to documented upstream API contracts.

Where possible, response structs map to documented upstream response shapes. Unknown, plugin-defined, extension-defined, or intentionally opaque fields may be preserved with `serde_json::Value` or `#[serde(flatten)]`, but core response types should remain typed.

### Response model

Endpoints returning JSON should use:

```rust
ServiceResponse<T>
```

For example:

```rust
DockerResponse<DockerContainerSummary>
PodmanResponse<PodmanInfo>
CaddyResponse<CaddyConfig>
SystemdResponse<SystemdUnitStatus>
TechnitiumResponse<TechnitiumZoneList>
```

Endpoints that return no body, such as `204 No Content`, should use:

```rust
ServiceEmptyResponse
```

For example:

```rust
DockerEmptyResponse
PodmanEmptyResponse
CaddyEmptyResponse
SystemdEmptyResponse
TechnitiumEmptyResponse
```

Normal read/query endpoints should not return `Option<T>` unless the upstream API itself explicitly models the value as optional.

## Supported services

### Docker

Status: **v1 target**

Veltrix targets:

- Docker Engine API
- Docker CLI workflows
- Docker Compose workflows
- Unix socket transport at `/var/run/docker.sock`

Docker support should be split into explicit backends:

- CLI backend
- Unix socket backend
- Compose backend

Each response includes backend metadata so callers can inspect whether the result came from CLI execution, socket API execution, or Compose execution.

Example response metadata:

```rust
DockerBackendUsed::Cli {
    binary,
    sudo,
    uid,
    gid,
}

DockerBackendUsed::Socket {
    socket_path,
    user,
}

DockerBackendUsed::Compose {
    binary,
    compose_file,
    project_name,
}
```

v1 guide coverage:

```rust
build_image
run_container
exec_container
list_containers
inspect_container
container_logs
stop_container
start_container
restart_container
remove_container
list_images
pull_image
tag_image
push_image
remove_image
compose_up
compose_logs
compose_ps
compose_down
create_network
list_networks
inspect_network
connect_network
create_volume
list_volumes
inspect_volume
remove_volume
system_prune
system_df
create_container_api
start_container_api
stop_container_api
remove_container_api
container_logs_api
list_images_api
pull_image_api
inspect_container_api
```

Security rule:

- Docker socket access must be documented as host-root-equivalent. Remote socket access must require explicit TLS configuration.

Relevant upstream API:

- Docker Engine API
- Docker CLI
- Docker Compose CLI

### Podman

Status: **v1 target**

Veltrix targets:

- Podman major version: `5`
- Libpod API version: `5.0.0`
- API family: Podman v2-compatible REST API / Libpod API
- CLI compatibility with common Docker-style workflows

Podman support should be split into explicit backends:

- CLI backend
- Unix socket backend
- Compose backend
- Machine backend where applicable on macOS/Windows

The CLI backend supports sync execution and, behind the async feature, async execution.

The socket backend talks to the Podman service over a Unix socket.

Each response includes backend metadata so callers can inspect whether the result came from CLI, socket, compose, or machine execution.

Example response metadata:

```rust
PodmanBackendUsed::Cli {
    binary,
    sudo,
    uid,
    gid,
}

PodmanBackendUsed::Socket {
    socket_path,
    user,
}

PodmanBackendUsed::Compose {
    binary,
    compose_file,
    project_name,
}

PodmanBackendUsed::Machine {
    machine_name,
}
```

v1 guide coverage:

```rust
run_container
build_image
exec_container
list_containers
container_logs
stop_container
remove_container
create_pod
run_container_in_pod
list_pods
stop_pod
remove_pod
inspect_pod
generate_kube
play_kube
play_kube_down
generate_systemd
machine_init
machine_start
machine_ssh
machine_stop
machine_list
system_prune
system_reset
create_secret
list_secrets
remove_secret
compose_up
compose_down
auto_update
info
version
list_containers_api
list_libpod_containers_api
list_pods_api
pull_image_api
```

Design rule:

- New deployments should prefer Quadlet-oriented workflows over generated systemd unit files, while retaining generated-unit support if it is explicitly marked as legacy.

Relevant upstream API:

- Podman Libpod REST API `5.0.0`
- Podman v2-compatible HTTP API
- Podman CLI
- Quadlet/systemd integration

### Caddy

Status: **v1 target**

Veltrix targets:

- Caddy major version: `2`
- Caddy Admin API
- Caddy JSON configuration API
- Caddy CLI workflows
- Caddyfile adaptation and validation workflows

Caddy exposes a REST admin API. By default, the admin endpoint listens on `localhost:2019`, and Caddy can also be configured to expose the admin API over a Unix socket. ([Caddy Web Server][1])

Caddy configuration is fundamentally JSON, and the admin API is the supported mechanism for runtime config inspection and mutation. ([Mintlify][2])

Veltrix should type stable top-level config structures such as:

```rust
CaddyConfig
CaddyAdminConfig
CaddyLoggingConfig
CaddyHttpAppConfig
CaddyTlsConfig
```

Caddy apps/modules are plugin-extensible, so module-specific payloads may remain `serde_json::Value` unless Veltrix explicitly supports that module.

v1 guide coverage:

```rust
run
start
stop
reload
adapt
file_server
reverse_proxy
fmt
validate
trust
untrust
hash_password
get_config
load_config
patch_config_path
delete_config_path
list_reverse_proxy_upstreams
configure_basic_site
configure_reverse_proxy
configure_headers
configure_matchers
configure_logging
configure_tls
configure_snippets
configure_env_substitution
```

Security rule:

- Remote Caddy admin access must be represented as an explicit insecure-by-default risk unless authentication and TLS are configured.

Relevant upstream API:

- Caddy 2 Admin API
- Caddy JSON configuration API
- Caddy CLI
- Caddyfile adapter

### systemd

Status: **v1 target**

Veltrix targets:

- systemd service manager APIs
- D-Bus service: `org.freedesktop.systemd1`
- `systemctl` CLI fallback where D-Bus coverage is not practical
- `journalctl` log access workflows

systemd exposes service-manager APIs over D-Bus through `org.freedesktop.systemd1`. The documented interface covers the system and service manager itself, not every auxiliary systemd daemon. ([freedesktop.org][3])

Veltrix should not ship copied or placeholder APIs under `services::systemd`. The module should only be public once it exposes real systemd-backed behavior.

v1 guide coverage:

```rust
start_unit
stop_unit
restart_unit
reload_unit
enable_unit
enable_now_unit
disable_unit
status_unit
is_active
is_enabled
is_failed
list_units
list_unit_files
cat_unit
show_unit_properties
daemon_reload
mask_unit
unmask_unit
edit_unit_drop_in
edit_unit_full
journal_unit
journal_unit_follow
journal_unit_boot
journal_unit_since_until
journal_unit_priority
journal_unit_json
journal_disk_usage
journal_vacuum_size
journal_vacuum_time
create_service_unit
create_socket_unit
create_timer_unit
list_timers
analyze_calendar
create_override
manage_template_instance
set_resource_limits
configure_watchdog
```

Required modeling:

- distinguish system manager vs user manager
- include backend metadata describing D-Bus or CLI context
- model lifecycle operations as empty responses or typed job responses
- model journal reads as structured entries where possible, not raw strings only

Relevant upstream API:

- `org.freedesktop.systemd1`
- systemd D-Bus service manager API
- `systemctl`
- `journalctl`

### Technitium DNS Server

Status: **v1 target**

Veltrix targets:

- Technitium DNS Server HTTP API
- authenticated web-console/API workflows
- DNS zone, record, settings, resolve, log, stats, and blocking workflows

Technitium DNS Server exposes an HTTP API used by its web console, allowing third-party apps and scripts to configure the DNS server. ([technitium.com][4])

Veltrix should explicitly pin the supported Technitium API/server version before exposing stable response structs.

v1 guide coverage:

```rust
deploy_container
login
logout
server_status
list_zones
create_zone
create_forwarder_zone
enable_zone
disable_zone
delete_zone
get_records
add_a_record
add_aaaa_record
add_cname_record
add_mx_record
add_txt_record
add_srv_record
update_record
delete_record
get_settings
set_settings
resolve_domain
resolve_domain_with_server
query_logs
dashboard_stats
add_blocked_domain
add_allowed_domain
bulk_add_records
create_local_dev_zone
create_preview_record
cleanup_preview_record
configure_split_horizon
```

Supported DNS record types should be explicit and typed.

Initial v1 record types:

```rust
A
AAAA
CNAME
MX
TXT
NS
SRV
CAA
PTR
```

Authentication rule:

- session tokens and bearer-token behavior must be modeled explicitly and never logged by default.

Relevant upstream API:

- Technitium DNS Server HTTP API
- target version to be pinned before stable v1 support

## Cross-tool integration coverage

Veltrix Services v1 should include documented helpers or examples for the guide's combined workflows.

Target patterns:

```rust
docker_or_podman_caddy_reverse_proxy
container_as_systemd_service
podman_quadlet_service
caddy_technitium_local_https
systemd_timer_container_task
full_stack_local_development_environment
```

These helpers may live under service-specific modules, but the examples should show the complete workflow end-to-end.

## Feature layout

Recommended feature names:

```toml
[features]
default = []

async = ["tokio/process"]

docker = ["serde", "serde_json"]
docker-socket = ["docker", "async", "tokio/net", "tokio/io-util"]
docker-compose = ["docker"]
docker-full = ["docker", "docker-socket", "docker-compose"]

podman = ["serde", "serde_json"]
podman-socket = ["podman", "async", "tokio/net", "tokio/io-util"]
podman-compose = ["podman"]
podman-machine = ["podman"]
podman-full = ["podman", "podman-socket", "podman-compose", "podman-machine"]

caddy = ["async", "serde", "serde_json", "reqwest/json", "tokio/net", "tokio/io-util"]

systemd = ["serde", "serde_json"]
systemd-dbus = ["systemd", "async"]
systemd-cli = ["systemd"]
systemd-full = ["systemd", "systemd-dbus", "systemd-cli"]

technitium = ["async", "serde", "serde_json", "reqwest/json"]
```

Recommended public layout:

```rust
veltrix::services::docker
veltrix::services::podman
veltrix::services::caddy
veltrix::services::systemd
veltrix::services::technitium
```

## Roadmap

### v0.1.0 — Current

- was not present in this release, but the services domain is now the active focus for v1 development and will be stabilized before the next release.

### v0.2.0 — Current

Current focus:

- establish `services` as the top-level service integration domain
- move Podman under `veltrix::services::podman`
- move Caddy under `veltrix::services::caddy`
- define typed response wrappers
- separate JSON responses from empty responses
- include backend metadata in service responses
- avoid exposing incomplete placeholder modules as production APIs

Current supported services:

- Docker: not yet supported
- Podman: partial
- Caddy: partial
- systemd: not yet supported
- Technitium: not yet supported

Required cleanup before moving forward:

- remove top-level `veltrix::podman`
- expose service integrations only through `veltrix::services::*`
- ensure `services/mod.rs` gates modules by feature
- remove copied Caddy code from `services/systemd`
- keep incomplete modules hidden or clearly non-functional until real support exists

### v0.3.0 — Services foundation upgrade

Primary goal: make `services` coherent and usable.

Planned work:

- finalize `services/mod.rs`
- finalize feature gates
- introduce Docker module and response wrappers
- stabilize Podman CLI sync API
- stabilize Podman CLI async API
- stabilize Podman socket read API
- stabilize Podman socket mutation API for basic container lifecycle
- stabilize Caddy Admin API client
- stabilize Caddy HTTP transport
- stabilize Caddy Unix socket transport
- document all service backend metadata
- add examples for Docker socket, Podman CLI, Podman socket, and Caddy admin usage
- add compile tests for each feature combination

Docker v0.2 target endpoints:

```rust
list_containers
inspect_container
container_logs
start_container
stop_container
```

Podman v0.2 target endpoints:

```rust
info
version
containers
start_container
stop_container
```

Caddy v0.2 target endpoints:

```rust
config
load_config
stop
id_list
pki_ca
```

### v0.4.0 — Container typed coverage expansion

Primary goal: replace broad JSON values with documented typed structs where practical.

Planned work:

- expand Docker Engine API types
- add Docker image, network, volume, and prune support
- add Docker Compose wrapper support
- expand Podman v5 types
- improve `PodmanInfo`
- improve `PodmanVersion`
- improve container summary models
- add Podman image list support
- add Podman pod list support
- add Podman inspect support
- add Caddy config subtypes for stable built-in fields
- add Caddy validation helpers
- improve error types for HTTP status failures and JSON decode failures

Docker target additions:

```rust
build_image
run_container
exec_container
list_images
pull_image
tag_image
push_image
remove_image
create_network
list_networks
inspect_network
create_volume
list_volumes
system_prune
system_df
compose_up
compose_logs
compose_ps
compose_down
```

Podman target additions:

```rust
images
pods
inspect_container
inspect_image
remove_container
restart_container
create_pod
inspect_pod
generate_kube
play_kube
```

Caddy target additions:

```rust
validate_config
adapt_config
get_config_path
patch_config_path
delete_config_path
list_reverse_proxy_upstreams
```

### v0.5.0 — systemd preview

Primary goal: introduce real systemd support behind `systemd`.

Planned work:

- choose backend strategy:
  - D-Bus native client, preferred
  - `systemctl` CLI fallback, optional
- define typed unit models
- distinguish system manager and user manager
- include backend metadata in every response
- support read-only operations first
- add basic `journalctl` support

Target endpoints:

```rust
list_units
unit_status
list_unit_files
is_active
is_enabled
is_failed
cat_unit
show_unit_properties
journal_unit
```

No mutation operations should ship until read-only behavior is stable.

### v0.6.0 — systemd lifecycle and unit operations

Primary goal: add mutation support for systemd.

Planned work:

```rust
start_unit
stop_unit
restart_unit
reload_unit
enable_unit
enable_now_unit
disable_unit
daemon_reload
mask_unit
unmask_unit
create_override
list_timers
```

Response model:

- unit reads return `SystemdResponse<T>`
- lifecycle operations return `SystemdEmptyResponse` or typed job responses if using D-Bus jobs
- journal operations return typed log entries when using JSON output

### v0.7.0 — Technitium preview

Primary goal: introduce Technitium DNS Server support.

Planned work:

- pin supported Technitium DNS Server API version
- implement authenticated HTTP client
- support token/session handling
- define backend metadata
- implement read-only DNS operations

Target endpoints:

```rust
login
logout
server_status
list_zones
get_zone
get_records
resolve_domain
query_logs
dashboard_stats
```

### v0.8.0 — Technitium DNS management

Primary goal: add typed DNS mutation operations.

Planned work:

```rust
create_zone
create_forwarder_zone
enable_zone
disable_zone
delete_zone
create_record
update_record
delete_record
get_settings
set_settings
add_blocked_domain
add_allowed_domain
```

Supported DNS record types should be explicit and typed.

Initial target record types:

```rust
A
AAAA
CNAME
MX
TXT
NS
SRV
CAA
PTR
```

### v0.9.0 — Cross-tool workflow coverage

Primary goal: document and test the guide's combined infrastructure workflows.

Planned work:

- Docker/Podman plus Caddy reverse-proxy examples
- Docker containers as systemd services
- Podman Quadlet examples
- Caddy plus Technitium local HTTPS examples
- systemd timers for Docker/Podman jobs
- full-stack local development environment example
- integration-test structure for local services

Target examples:

```rust
docker_or_podman_caddy_reverse_proxy
container_as_systemd_service
podman_quadlet_service
caddy_technitium_local_https
systemd_timer_container_task
full_stack_local_development_environment
```

### v0.10.0 — API consistency pass and release candidate

Primary goal: stabilize the v1 API surface.

Planned work:

- consistent client constructors
- consistent backend metadata naming
- consistent error behavior
- consistent empty response behavior
- consistent sync/async naming
- freeze public type names
- freeze feature names
- freeze module paths
- complete documentation
- complete examples
- complete error taxonomy
- validate against pinned upstream versions
- audit all public structs for semver risk
- remove deprecated aliases
- remove incomplete modules from default docs

Standard response naming:

```rust
DockerResponse<T>
DockerEmptyResponse

PodmanResponse<T>
PodmanEmptyResponse

CaddyResponse<T>
CaddyEmptyResponse

SystemdResponse<T>
SystemdEmptyResponse

TechnitiumResponse<T>
TechnitiumEmptyResponse
```

Expected service state:

- Docker: full v1 guide coverage
- Podman: full v1 guide coverage
- Caddy: full v1 guide coverage
- systemd: full v1 guide coverage
- Technitium: full v1 guide coverage
- Cross-tool workflows: documented and example-backed

### v1.0.0 — Stable services API

Primary goal: full supported public API for the service domain and complete coverage of the internal Developer Tool Usage Guide.

Expected guarantees:

- stable module paths
- stable feature names
- typed public responses
- pinned upstream API contracts
- documented breaking-change policy
- backend metadata included in responses
- read endpoints return concrete typed data
- empty/mutation endpoints return explicit empty response types
- no placeholder modules exposed as production APIs
- complete guide workflow coverage documented in public examples

Expected services:

```rust
veltrix::services::docker
veltrix::services::podman
veltrix::services::caddy
veltrix::services::systemd
veltrix::services::technitium
```

v1 does not mean every upstream endpoint is implemented. It means every tool-usage workflow described in the guide is either supported by a stable Veltrix API, represented by a documented adapter/example, or explicitly marked unsupported with rationale.

## Breaking-change policy

Veltrix should introduce a breaking change when:

- an upstream pinned API changes incompatibly
- a response type was incorrectly modeled
- a public method returns the wrong semantic response type
- a backend behavior cannot be preserved safely
- a security-sensitive behavior changes
- a guide-covered workflow cannot remain compatible with the existing public API

Veltrix should avoid silent weakening such as replacing typed fields with unstructured JSON in a patch release.

## Design rules

1. Prefer typed structs over `serde_json::Value`.
2. Use `Value` only for plugin-defined, extension-defined, or intentionally opaque payloads.
3. Include backend metadata in every response.
4. Do not return `Option<T>` for normal read endpoints.
5. Use explicit empty response types for empty-body endpoints.
6. Gate service integrations behind feature flags.
7. Do not expose placeholder modules as stable APIs.
8. Pin upstream API versions in documentation.
9. Treat v1 as stable and production-grade.
10. Cover every workflow in the Developer Tool Usage Guide by v1, either through stable APIs, documented adapters, or explicit unsupported-status notes.
11. Never log credentials, API tokens, DNS tokens, bearer tokens, or socket-derived secrets by default.
12. Treat Docker and Podman socket access as privileged host-control access.

[1]: https://caddyserver.com/
[2]: https://veltrix.mintlify.com/docs/services/caddy
[3]: https://www.freedesktop.org/wiki/Software/systemd/
[4]: https://technitium.com/dns/

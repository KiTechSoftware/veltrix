# Changelog

## v0.7.2

- Fixed a minor issue with path handling in `veltrix::os::paths`.
- Updated async and standard command handling to support `current_dir`.
- Breaking change: Updated `user_bin_dir` to return a `FHS-compliant` path by default, with an optional `legacy` flag for the previous behavior. This may affect existing code that relies on the old path format, so please review the updated API contract and adjust your code accordingly.

## v0.7.1

- Added `pid_is_alive` helper using `kill(2)` signal 0
- Added v0.8.0 preview `LDAP` support behind the `ldap` feature flag with `LdapClient` and typed API contracts for auth, search, and modify operations.

## v0.7.0

- Removed the legacy top-level `veltrix::emojis` module and `emojis` feature.
- Kept `veltrix::unicode::emojis` behind the canonical `unicode-emojis` feature.
- Added structured systemd journal entry APIs using `journalctl -o json`.
- Added typed systemd unit predicates and list-units coverage.
- Added `systemd-dbus` with a `busctl`-backed D-Bus client for systemd manager lifecycle, status, predicate, and list-units operations.
- Added Technitium TXT and general ACME DNS challenge helpers for DNS-01 certificate workflows, with a first-class Caddy record-name helper.
- Added a Technitium ACME DNS-01 guide and expanded the services example with challenge record setup.
- Moved broad cross-tool workflow coverage out of the v1 target and into the v2 roadmap.

## v0.6.0

- Added the feature-gated `veltrix::data` module with boolean parsing/formatting helpers and compact duration parsing/formatting helpers.
- Added `veltrix::os::clock` with wall-clock, monotonic, Unix timestamp, uptime, process CPU time, and thread CPU time helpers.
- Expanded `services::systemd` with CLI-backed lifecycle, inspection, journal filtering, unit-file, timer, override, template, resource-limit, watchdog, and deployment workflows.
- Added `services::technitium::TechnitiumClient` for async HTTP API workflows covering auth, zones, records, settings, resolving, logs, stats, blocklist entries, zone import, and bulk records.
- Added Technitium response envelope parsing, credential-safe auth handling, and DNS record type formatting helpers.

## v0.5.0

- Added first-class Podman label helpers with `PodmanLabel`, `PodmanLabels`, typed CLI label arguments, and Quadlet label rendering.
- Implemented Docker CLI workflows for containers, images, volumes, networks, system cleanup, and disk-usage reporting.
- Added Docker Compose client support for `up`, `down`, `logs`, and `ps`.
- Added Docker Unix-socket Engine API client support for container, image, network, and volume workflows.
- Added Caddy CLI wrappers for validate, fmt, reload, stop, run, adapt, and hash-password.
- Added Caddy Admin API path patch/delete helpers for runtime config updates.
- Added Caddy local HTTPS and reverse proxy config builders.
- Added Caddy Admin API module listing and module config patch/delete helpers.

## v0.4.1

- Added legacy `emojis` feature flag over `unicode::emojis::*` for a smoother transition path. (also fixes a v0.4.0 oversight where the `emojis` feature gate was removed from the legacy path but not added to the new path)

## v0.4.0

Unicode transition release.

- Added the preview `veltrix::unicode` parent module.
- Added canonical `veltrix::unicode::emojis`  while retaining legacy `veltrix::emojis`.
- Added `unicode` and `unicode-emojis` feature flags.
- Upgraded generated emoji metadata with Unicode Emoji/CLDR source versions, qualification, normalized search terms, skin-tone metadata, variation-selector metadata, and flag metadata.
- Added `find_by_search_term` for normalized emoji search.
- Updated `veltrix-codegen` defaults and emoji generation to support the v0.4.0 schema.
- Regenerated emoji data from Unicode Emoji `17.0` and CLDR `48.2`.
- Added codegen architecture documentation for future generated domains.
- Updated README examples and Unicode API contract for the transition path.

## v0.3.0

Services foundation release.

- Expanded `VeltrixError` with service-domain parsing, socket, HTTP, auth, validation, and service errors.
- Added Docker backend specs, response wrappers, and placeholder CLI client errors for the v0.5.0 implementation target.
- Expanded Podman CLI coverage for container, image, pod, Kubernetes YAML, machine, secret, compose, and auto-update workflows.
- Added Podman backend metadata for CLI, socket, compose, and machine contexts.
- Pinned Technitium preview support to the `13.x` HTTP API family and added credential-safe auth modeling.
- Kept `services::systemd` free of placeholder public APIs until real systemd-backed behavior lands.
- Updated service contracts, README feature descriptions, and generated emoji docs policy.

## v0.2.0

Breaking OS layout and services preview release.

- Moved canonical path helpers from `veltrix::paths` to `veltrix::os::paths`.
- Moved canonical Unix helpers from `veltrix::unistd` to `veltrix::os::unistd`.
- Added `veltrix::os::process` command helpers with sync execution and optional Tokio async execution.
- Added the `services` parent module with early Podman, Caddy, systemd, and Technitium integration scaffolding.
- Added Podman CLI/socket specs, response wrappers, and initial clients behind `podman` and `podman-socket`.
- Added Caddy Admin API spec, response wrappers, and initial admin client behind `caddy`.
- Added API contract docs for OS, services, Unicode, and data domains.
- Added preview `veltrix::data::bools` formatting/predicate helpers, not yet exposed as the long-term data API.
- Updated feature flags for `async`, `podman`, `podman-socket`, `caddy`, `systemd`, `technitium`, `unistd`, and `emojis`.

## v0.1.0

Initial release.

- Added top-level `veltrix::paths` helpers for common system, user, XDG, config, state, cache, data, runtime, log, bin, desktop-entry, icon, and systemd-unit paths.
- Added feature-gated top-level `veltrix::unistd` helpers for Unix identity, groups, process IDs, hostname, current directory, home directory, and privilege checks.
- Added generated `veltrix::emojis` constants and metadata behind the `emojis` feature.
- Added the `veltrix-codegen` emoji generation pipeline and source data integration.
- Added `VeltrixError`, `Result`, examples for paths/unistd/emojis, and initial README documentation.

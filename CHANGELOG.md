# Changelog

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

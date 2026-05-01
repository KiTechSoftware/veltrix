# Changelog

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

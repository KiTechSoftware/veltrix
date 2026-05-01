# Changelog

## v0.3.0

Services foundation release.

- Expanded `VeltrixError` with service-domain parsing, socket, HTTP, auth, validation, and service errors.
- Added Docker backend specs, response wrappers, and placeholder CLI client errors for the v0.5.0 implementation target.
- Expanded Podman CLI coverage for container, image, pod, Kubernetes YAML, machine, secret, compose, and auto-update workflows.
- Added Podman backend metadata for CLI, socket, compose, and machine contexts.
- Pinned Technitium preview support to the `13.x` HTTP API family and added credential-safe auth modeling.
- Kept `services::systemd` free of placeholder public APIs until real systemd-backed behavior lands.
- Updated service contracts, README feature descriptions, and generated emoji docs policy.

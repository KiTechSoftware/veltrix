# Veltrix v0.3.0 → v0.9.0 Roadmap

**Goal:** Complete core services foundation and prepare for v1.0.0 stable release.

---

## v0.3.0: Services Foundation Phase

### Podman Completion

- [x] Complete Podman CLI API surface
  - [x] Container operations: run, exec, list, inspect, logs, stop, start, restart, remove
  - [x] Pod management: create, run-in-pod, list, stop, remove, inspect
  - [x] Image operations: build, pull, push, list, remove, tag
  - [x] Kubernetes YAML: generate-kube, play-kube, play-kube-down
  - [x] Systemd/Quadlet: generate-systemd (legacy), Quadlet integration patterns
  - [x] Machine: init, start, stop, ssh, list (macOS/Windows)
  - [x] System operations: prune, reset
  - [x] Secret management: create, list, remove
  - [x] Compose support: up, down, logs, ps
  - [x] Auto-update label support
- [x] Implement Libpod REST API wrapper (v5.0.0)
  - [x] Typed response structs for all v1 workflows
  - [x] Backend metadata tracking (CLI vs socket distinction)
- [x] Test all Container/Pod/Image/Secret workflows
- [x] Verify Quadlet systemd integration patterns

### Docker Module Skeleton

- [x] Create `services::docker` module structure
  - [x] `docker::spec` — CLI, Socket, Compose specs with backend enum
  - [x] `docker::types` — Response types for v0.5.0 implementation
  - [x] `docker::cli` — Stub client (placeholder methods)
- [x] Add feature flags: `docker`, `docker-socket`
- [x] Update Cargo.toml and lib.rs
- [x] Update README with Docker feature descriptions

### Technitium API Pinning

- [x] Document supported Technitium API version
- [x] Define authentication model:
  - [x] Session token handling
  - [x] Bearer-token behavior spec
  - [x] Never-log-credentials rule
- [x] Update services.md with auth section
- [x] Add basic types for Technitium response schema

### Error Type Expansion

Extend `VeltrixError` to cover services domain:

- [x] Add new error variants:
  - [x] `Parsing(String)` — JSON/YAML parsing failures in service responses
  - [x] `Service { service: String, reason: String }` — Service-specific API errors
  - [x] `Socket { reason: String }` — Unix socket communication errors
  - [x] `Http { status: u16, reason: String }` — HTTP API errors (Caddy, Technitium, Docker socket)
  - [x] `Auth { reason: String }` — Authentication/credential errors (Technitium, Caddy)
  - [x] `Validation { field: String, reason: String }` — Configuration/input validation errors
- [x] Add helper constructors on `VeltrixError` impl block:
  - [x] `VeltrixError::parsing(reason)`
  - [x] `VeltrixError::service(service, reason)`
  - [x] `VeltrixError::socket(reason)`
  - [x] `VeltrixError::http(status, reason)`
  - [x] `VeltrixError::auth(reason)`
  - [x] `VeltrixError::validation(field, reason)`
- [x] Update `src/error.rs` thiserror Display implementations
- [x] Update all service modules to use new error variants
- [x] Add error handling tests covering each variant
- [x] Document error handling patterns in AGENTS.md

### Cleanup & Validation

- [x] Audit `services::systemd` — remove placeholder, keep only real behavior
- [x] Audit `services::caddy` — ensure skeleton-only, no premature v0.5.0 impl
- [x] Run `just lint` — resolve all clippy warnings
- [x] Run `just test` — all tests passing
- [ ] Make scoped commits per API group

**Deliverables:** Podman v1 complete, Docker skeleton + features added, Technitium auth pinned, no regressions.

---

## v0.4.0: Unicode Transition

### Unicode Module Setup

- [x] Create `veltrix::unicode` module structure (placeholder for v0.4.0+)
- [x] Plan emoji transition: `veltrix::emojis` (v0.2.0 path) → `veltrix::unicode::emojis` (v0.4.0+)
- [x] Update docs/api/contract/unicode.md with migration timeline

### Emoji Schema Upgrade

- [x] Review emoji data source (CLDR + Unicode emoji)
- [x] Define emoji struct schema enhancements:
  - [x] Metadata fields (skin tones, variation selectors, etc.)
  - [x] Search-friendly attributes
  - [x] Version tracking (Unicode version support)
- [x] Update veltrix-codegen to emit new schema
- [x] Run code generator: `cargo run --manifest-path workspace/Cargo.toml -p veltrix-codegen`

### Codegen Pipeline

- [x] Ensure veltrix-codegen is general-purpose framework (not emoji-only)
- [x] Document codegen architecture for future domains (v0.6.0 data)
- [x] Test round-trip: source data → generated code → compiles + passes tests

### Validation

- [x] `just lint` passes
- [x] `just test` passes (including emoji tests)
- [x] Examples updated to use new schema

**Deliverables:** Unicode module skeleton, emoji schema upgraded, codegen pipeline validated.

---

## v0.5.0: Caddy & Docker v1 APIs

### Docker v1 Complete Implementation

- [x] Docker CLI backend:
  - [x] run, build, exec, list, inspect, logs, stop, start, restart, remove
  - [x] image: list, pull, tag, push, remove, build
  - [x] Compose: up, down, logs, ps
  - [x] Volume: create, list, remove, inspect
  - [x] Network: create, list, remove, connect
  - [x] System: prune, df
- [x] Docker socket backend (Unix socket API v1.40+):
  - [x] Container operations via Engine API
  - [x] Image operations via Engine API
  - [x] Network/volume operations
- [x] Docker Compose backend support
- [x] Typed response structs, backend metadata
- [x] Test all workflows

### Caddy v1 Complete Implementation

- [x] CLI workflows:
  - [x] validate, fmt, reload, stop, run
  - [x] Admin API configuration
- [x] Caddyfile parsing + management
- [x] Local HTTPS setup
- [x] Reverse proxy configuration
- [ ] Admin API v2 client:
  - [x] Configuration read/write
  - [x] Runtime config updates
  - [x] Module management
- [x] Test all implemented workflows

### Integration Testing

- [ ] Docker + Podman comparison tests
- [ ] Caddy config validation workflows
- [ ] Cross-tool error handling

### Validation

- [x] `just lint` passes
- [x] `just test` passes
- [ ] All workflows from guide tested

**Deliverables:** Docker v1 and Caddy v1 APIs complete, fully tested, ready for integration.

---

## v0.6.0: Data Domain & Service Completions

### OS Clock Module

- [x] Create `veltrix::os::clock` module
- [x] Add wall-clock and monotonic runtime helpers
- [x] Add Unix timestamp helper
- [x] Add Linux-backed uptime, process CPU time, and thread CPU time helpers
- [x] Document platform behavior and data/time boundary

### Data Domain Module

- [x] Create `veltrix::data` module (planned v0.6.0+)
- [x] Define data types:
  - [x] `veltrix::data::bools` — Boolean utilities
  - [x] `veltrix::data::time` — Time value helpers
- [x] Feature-gated data types
- [x] Update Cargo.toml with `data` feature flag

### systemd Completion

- [x] Service lifecycle: start, stop, restart, enable, disable
- [x] Unit inspection: status, properties, dependencies
- [x] Journal access: retrieve logs, tail, filtering
- [x] Unit-file operations: edit, reload
- [x] Timers: list, enable, disable
- [x] Overrides: manage system.d overrides
- [x] Templates: support template units
- [x] Resource limits: CPU, memory, device management
- [x] Watchdog/deployment: watchdog setup, deployment patterns

### Technitium Completion

- [x] Authentication workflows (session token + bearer token)
- [x] Zone management: create, list, update, delete
- [x] DNS record management: A, AAAA, CNAME, MX, etc.
- [x] Settings: read/update server settings
- [x] Resolving: DNS query simulation
- [x] Logs: access and filter logs
- [x] Stats: retrieve server statistics
- [x] Blocking: manage blocklist entries
- [x] CI/CD automation: zone import, record bulk operations

### Validation

- [x] `just lint` passes
- [x] `just test` passes
- [x] All service integrations tested

**Deliverables:** Data domain skeleton, systemd and Technitium v1 complete.

---

## v0.7.0: systemd Completion & Caddy DNS Support

### systemd Focus

- [x] Structured journal entries where `journalctl -o json` is available
- [x] D-Bus backend feature via `systemd-dbus`
- [x] Expand D-Bus coverage beyond lifecycle/status where practical
- [x] Add typed status predicates: active, enabled, failed
- [x] Add list-units coverage

### Technitium for ACME Certificates

- [x] TXT record helpers for DNS-01 challenges
- [x] General ACME `_acme-challenge` helper methods
- [x] Document Technitium ACME certificate flow
- [x] Add example using Technitium to set/remove ACME records

### Deferred to v2

- [ ] Docker/Podman + Caddy automatic reverse proxy
- [ ] Containers + systemd full-stack orchestration
- [ ] systemd timers + containerized jobs
- [ ] Full-stack local development environment

### Validation

- [x] `just lint` passes
- [x] `just test` passes

**Deliverables:** systemd contract alignment, D-Bus backend, structured journals, and Technitium DNS helpers needed for Caddy certificate issuance.

---

## v0.8.0: API Review & Documentation

### Public API Audit

- [ ] Review all public types and methods against v1 contracts
- [ ] Verify type safety and ergonomics
- [ ] Check for consistency across services
- [ ] Audit error handling patterns
- [ ] Validate response model uniformity

### Documentation

- [ ] Module-level rustdoc for all public APIs
- [ ] Example code for each major workflow
- [ ] Integration pattern guides
- [ ] Migration guide from v0.2.0 → v0.8.0
- [ ] Architecture documentation
- [ ] Security notes (socket access, credential handling)

### API Stability Marking

- [ ] Mark stable APIs (ready for v1.0.0)
- [ ] Identify any remaining experimental areas
- [ ] Document deprecations or removals

### Validation

- [ ] `cargo doc --open` — verify docs build and render
- [ ] Example code compiles and runs
- [ ] No broken links
- [ ] `just lint` passes
- [ ] `just test` passes

**Deliverables:** Complete API documentation, all examples working, architectural guide published.

---

## v0.9.0: Stability Freeze & Integration Tests

### Stability Freeze

- [ ] No API-breaking changes after v0.8.0
- [ ] Only bug fixes and documentation improvements
- [ ] All features from v0.3.0 → v0.8.0 complete and tested

### Integration Test Suite

- [ ] End-to-end tests for each major workflow
- [ ] Cross-service integration tests
- [ ] Error recovery and edge cases
- [ ] Performance benchmarks (if applicable)
- [ ] Platform-specific tests (macOS machine ops, Linux systemd, etc.)

### Release Preparation

- [ ] Update CHANGELOG with v0.3.0 → v0.9.0 summary
- [ ] Update README for v0.9.0
- [ ] Bump version in Cargo.toml: `version = "0.9.0"`
- [ ] Review AGENTS.md and API contracts
- [ ] Final lint & test: `just lint && just test && just check-license`

### Release Tasks

- [ ] Create release branch
- [ ] Tag v0.9.0
- [ ] Publish to crates.io
- [ ] Update documentation site

### Validation

- [ ] All integration tests pass
- [ ] Full feature matrix tested
- [ ] No regressions from v0.2.0
- [ ] Ready for v1.0.0 (final polish only)

**Deliverables:** v0.9.0 released, all v1 features implemented, stable and ready for v1.0.0 polish.

---

## v0.10.0: Cross-Tool Integration Patterns

### Integration Workflows

- [ ] Docker/Podman + Caddy:
  - [ ] Container discovery
  - [ ] Automatic reverse proxy setup
  - [ ] Health check integration
- [ ] Containers + systemd:
  - [ ] Container lifecycle via systemd unit files
  - [ ] Automatic restart policies
  - [ ] Resource limits via systemd
- [ ] Caddy + Technitium DNS:
  - [ ] Local DNS setup with Technitium
  - [ ] HTTPS certificate provisioning
  - [ ] Domain resolution from Technitium
- [ ] systemd timers + containerized jobs:
  - [ ] Schedule container jobs via systemd timers
  - [ ] Output logging to journal
- [ ] Full-stack local development environment:
  - [ ] Multi-container setup (podman)
  - [ ] Reverse proxy (caddy)
  - [ ] Local DNS (technitium)
  - [ ] Service lifecycle (systemd)

### Test Scenarios

- [ ] Full stack initialization
- [ ] Service discovery and wiring
- [ ] Failure recovery patterns
- [ ] Configuration hot-reload

---

## Commit Conventions

All work organized into **scoped commits** per logical change:

- Format: `scope: short description`
- Examples:
  - `services/podman: add pod management API`
  - `services/docker: implement socket backend`
  - `services/caddy: complete admin API`
  - `data: add time utilities`
  - `docs: add integration guide`

Make commits **before** beginning any migration or release work.

---

## Verification Checklist

For each milestone completion:

- [ ] `just build` passes
- [ ] `just test` passes
- [ ] `just lint` passes (no clippy warnings)
- [ ] `just check-license` passes
- [ ] All scoped commits made
- [ ] README updated if needed
- [ ] AGENTS.md reflects current state
- [ ] No regressions from prior version

---

## v1.0.0 Target

After v0.9.0 completion:

- [ ] Final polish (docs, naming consistency, examples)
- [ ] Performance review
- [ ] Security audit (especially socket access, credential handling)
- [ ] Release v1.0.0 (stable, breaking-change-free)

**Estimated Timeline:** v0.3.0 → v0.9.0 = 7 milestones, ~1-2 months of focused development.

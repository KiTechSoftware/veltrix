# Veltrix v0.3.0 → v0.9.0 Roadmap

**Goal:** Complete core services foundation and prepare for v1.0.0 stable release.

---

## v0.3.0: Services Foundation Phase

### Podman Completion

- [ ] Complete Podman CLI API surface
  - [ ] Container operations: run, build, exec, list, inspect, logs, stop, start, restart, remove
  - [ ] Pod management: create, run-in-pod, list, stop, remove, inspect
  - [ ] Image operations: build, pull, push, list, remove, tag
  - [ ] Kubernetes YAML: generate-kube, play-kube, play-kube-down
  - [ ] Systemd/Quadlet: generate-systemd (legacy), Quadlet integration patterns
  - [ ] Machine: init, start, stop, ssh, list (macOS/Windows)
  - [ ] System operations: prune, reset
  - [ ] Secret management: create, list, remove
  - [ ] Compose support: up, down, logs, ps
  - [ ] Auto-update label support
- [ ] Implement Libpod REST API wrapper (v5.0.0)
  - [ ] Typed response structs for all v1 workflows
  - [ ] Backend metadata tracking (CLI vs socket distinction)
- [ ] Test all Container/Pod/Image/Secret workflows
- [ ] Verify Quadlet systemd integration patterns

### Docker Module Skeleton

- [ ] Create `services::docker` module structure
  - [ ] `docker::spec` — CLI, Socket, Compose specs with backend enum
  - [ ] `docker::types` — Response types for v0.5.0 implementation
  - [ ] `docker::cli` — Stub client (placeholder methods)
- [ ] Add feature flags: `docker`, `docker-socket`
- [ ] Update Cargo.toml and lib.rs
- [ ] Update README with Docker feature descriptions

### Technitium API Pinning

- [ ] Document supported Technitium API version
- [ ] Define authentication model:
  - [ ] Session token handling
  - [ ] Bearer-token behavior spec
  - [ ] Never-log-credentials rule
- [ ] Update services.md with auth section
- [ ] Add basic types for Technitium response schema

### Error Type Expansion

Extend `VeltrixError` to cover services domain:

- [ ] Add new error variants:
  - [ ] `Parsing(String)` — JSON/YAML parsing failures in service responses
  - [ ] `Service { service: String, reason: String }` — Service-specific API errors
  - [ ] `Socket { reason: String }` — Unix socket communication errors
  - [ ] `Http { status: u16, reason: String }` — HTTP API errors (Caddy, Technitium, Docker socket)
  - [ ] `Auth { reason: String }` — Authentication/credential errors (Technitium, Caddy)
  - [ ] `Validation { field: String, reason: String }` — Configuration/input validation errors
- [ ] Add helper constructors on `VeltrixError` impl block:
  - [ ] `VeltrixError::parsing(reason)`
  - [ ] `VeltrixError::service(service, reason)`
  - [ ] `VeltrixError::socket(reason)`
  - [ ] `VeltrixError::http(status, reason)`
  - [ ] `VeltrixError::auth(reason)`
  - [ ] `VeltrixError::validation(field, reason)`
- [ ] Update `src/error.rs` thiserror Display implementations
- [ ] Update all service modules to use new error variants
- [ ] Add error handling tests covering each variant
- [ ] Document error handling patterns in AGENTS.md

### Cleanup & Validation

- [ ] Audit `services::systemd` — remove placeholder, keep only real behavior
- [ ] Audit `services::caddy` — ensure skeleton-only, no premature v0.5.0 impl
- [ ] Run `just lint` — resolve all clippy warnings
- [ ] Run `just test` — all tests passing
- [ ] Make scoped commits per API group

**Deliverables:** Podman v1 complete, Docker skeleton + features added, Technitium auth pinned, no regressions.

---

## v0.4.0: Unicode Transition

### Unicode Module Setup

- [ ] Create `veltrix::unicode` module structure (placeholder for v0.4.0+)
- [ ] Plan emoji transition: `veltrix::emojis` (v0.2.0 path) → `veltrix::unicode::emojis` (v0.4.0+)
- [ ] Update docs/api/contract/unicode.md with migration timeline

### Emoji Schema Upgrade

- [ ] Review emoji data source (CLDR + Unicode emoji)
- [ ] Define emoji struct schema enhancements:
  - [ ] Metadata fields (skin tones, variation selectors, etc.)
  - [ ] Search-friendly attributes
  - [ ] Version tracking (Unicode version support)
- [ ] Update veltrix-codegen to emit new schema
- [ ] Run code generator: `cargo run --manifest-path workspace/Cargo.toml -p veltrix-codegen`

### Codegen Pipeline

- [ ] Ensure veltrix-codegen is general-purpose framework (not emoji-only)
- [ ] Document codegen architecture for future domains (v0.6.0 data)
- [ ] Test round-trip: source data → generated code → compiles + passes tests

### Validation

- [ ] `just lint` passes
- [ ] `just test` passes (including emoji tests)
- [ ] Examples updated to use new schema

**Deliverables:** Unicode module skeleton, emoji schema upgraded, codegen pipeline validated.

---

## v0.5.0: Caddy & Docker v1 APIs

### Docker v1 Complete Implementation

- [ ] Docker CLI backend:
  - [ ] run, build, exec, list, inspect, logs, stop, start, restart, remove
  - [ ] image: list, pull, tag, push, remove, build
  - [ ] Compose: up, down, logs, ps
  - [ ] Volume: create, list, remove, inspect
  - [ ] Network: create, list, remove, connect
  - [ ] System: prune, df
- [ ] Docker socket backend (Unix socket API v1.40+):
  - [ ] Container operations via Engine API
  - [ ] Image operations via Engine API
  - [ ] Network/volume operations
- [ ] Docker Compose backend support
- [ ] Typed response structs, backend metadata
- [ ] Test all workflows

### Caddy v1 Complete Implementation

- [ ] CLI workflows:
  - [ ] validate, fmt, reload, stop, run
  - [ ] Admin API configuration
- [ ] Caddyfile parsing + management
- [ ] Local HTTPS setup
- [ ] Reverse proxy configuration
- [ ] Admin API v2 client:
  - [ ] Configuration read/write
  - [ ] Runtime config updates
  - [ ] Module management
- [ ] Test all workflows

### Integration Testing

- [ ] Docker + Podman comparison tests
- [ ] Caddy config validation workflows
- [ ] Cross-tool error handling

### Validation

- [ ] `just lint` passes
- [ ] `just test` passes
- [ ] All workflows from guide tested

**Deliverables:** Docker v1 and Caddy v1 APIs complete, fully tested, ready for integration.

---

## v0.6.0: Data Domain & Service Completions

### Data Domain Module

- [ ] Create `veltrix::data` module (planned v0.6.0+)
- [ ] Define data types:
  - [ ] `veltrix::data::bool` — Boolean utilities
  - [ ] `veltrix::data::time` — Time value helpers
- [ ] Feature-gated data types
- [ ] Update Cargo.toml with `data` feature flag

### systemd Completion

- [ ] Service lifecycle: start, stop, restart, enable, disable
- [ ] Unit inspection: status, properties, dependencies
- [ ] Journal access: retrieve logs, tail, filtering
- [ ] Unit-file operations: edit, reload
- [ ] Timers: list, enable, disable
- [ ] Overrides: manage system.d overrides
- [ ] Templates: support template units
- [ ] Resource limits: CPU, memory, device management
- [ ] Watchdog/deployment: watchdog setup, deployment patterns

### Technitium Completion

- [ ] Authentication workflows (session token + bearer token)
- [ ] Zone management: create, list, update, delete
- [ ] DNS record management: A, AAAA, CNAME, MX, etc.
- [ ] Settings: read/update server settings
- [ ] Resolving: DNS query simulation
- [ ] Logs: access and filter logs
- [ ] Stats: retrieve server statistics
- [ ] Blocking: manage blocklist entries
- [ ] CI/CD automation: zone import, record bulk operations

### Validation

- [ ] `just lint` passes
- [ ] `just test` passes
- [ ] All service integrations tested

**Deliverables:** Data domain skeleton, systemd and Technitium v1 complete.

---

## v0.7.0: Cross-Tool Integration Patterns

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

### Validation

- [ ] Integration test suite passes
- [ ] Example full-stack setup documented
- [ ] `just test` passes

**Deliverables:** Cross-tool patterns demonstrated, integration tests passing.

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

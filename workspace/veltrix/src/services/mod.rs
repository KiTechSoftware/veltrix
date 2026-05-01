//! Service integrations.
//!
//! This module contains typed clients for local/system services such as
//! Podman, Caddy, systemd, and Technitium DNS.

#[cfg(feature = "podman")]
pub mod podman;

#[cfg(feature = "caddy")]
pub mod caddy;

#[cfg(feature = "systemd")]
pub mod systemd;

#[cfg(feature = "technitium")]
pub mod technitium;
//! Service integrations.
//!
//! This module contains typed clients for local/system services such as
//! Podman, Docker, Caddy, systemd, and Technitium DNS.

#[cfg(feature = "podman")]
pub mod podman;

#[cfg(feature = "docker")]
pub mod docker;

#[cfg(feature = "caddy")]
pub mod caddy;

#[cfg(feature = "systemd")]
pub mod systemd;

#[cfg(feature = "technitium")]
pub mod technitium;

#[cfg(feature = "ldap")]
pub mod ldap;

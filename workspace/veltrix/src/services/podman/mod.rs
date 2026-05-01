//! Podman CLI and socket integration.
//!
//! The v0.3.0 surface covers common Podman CLI workflows, typed response
//! wrappers with backend metadata, and a small Libpod Unix-socket client.

pub mod cli;
pub mod spec;
pub mod types;

#[cfg(feature = "podman-socket")]
pub mod socket;

pub use cli::PodmanCliClient;
pub use spec::*;
pub use types::*;

#[cfg(feature = "podman-socket")]
pub use socket::PodmanSocketClient;

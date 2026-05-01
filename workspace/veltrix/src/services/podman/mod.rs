//! Podman CLI and socket integration.
//!
//! The v0.3.0 surface covers common Podman CLI workflows, typed response
//! wrappers with backend metadata, and a small Libpod Unix-socket client.

pub mod cli;
pub mod labels;
pub mod quadlet;
pub mod spec;
pub mod types;

#[cfg(feature = "podman-socket")]
pub mod socket;

pub use cli::PodmanCliClient;
pub use labels::*;
pub use quadlet::*;
pub use spec::*;
pub use types::*;

#[cfg(feature = "podman-socket")]
pub use socket::PodmanSocketClient;

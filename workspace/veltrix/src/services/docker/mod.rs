//! Docker integration foundation types.
//!
//! v0.3.0 introduces backend specs and response wrappers for Docker CLI,
//! Unix socket, and Compose workflows. Full Docker client behavior is planned
//! for v0.5.0.

pub mod cli;
pub mod spec;
pub mod types;

pub use cli::DockerCliClient;
pub use spec::*;
pub use types::*;

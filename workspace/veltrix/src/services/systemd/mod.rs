//! systemd service integration.
//!
//! The v0.6.0 API uses `systemctl` and `journalctl` as a portable CLI backend
//! for lifecycle, inspection, journal, unit-file, timer, override, template,
//! resource-limit, and watchdog/deployment workflows.

pub mod cli;
pub mod spec;
pub mod types;

pub use cli::SystemdCliClient;
pub use spec::*;
pub use types::*;

//! systemd service integration.
//!
//! The portable CLI backend uses `systemctl` and `journalctl`. The
//! `systemd-dbus` feature adds a D-Bus manager backend via `busctl`.

pub mod cli;
#[cfg(feature = "systemd-dbus")]
pub mod dbus;
pub mod spec;
pub mod types;

pub use cli::SystemdCliClient;
#[cfg(feature = "systemd-dbus")]
pub use dbus::SystemdDbusClient;
pub use spec::*;
pub use types::*;

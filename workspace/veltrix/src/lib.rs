//! Veltrix: small utilities for paths, processes, services, and platform helpers.

/// Error types and result alias used by Veltrix helpers.
pub mod error;

/// OS helpers and common OS-related constants.
pub mod os;

/// Service integrations such as Podman and Caddy.
/// Introduced in v0.2.0, but still in early stages of development.
pub mod services;

#[cfg(feature = "emojis")]
/// Emoji constants and lookup helpers.
pub mod emojis;

// /// Unicode helpers, such as emoji constants and lookup functions.
// /// Planned for v0.4.0, but not yet implemented.
// pub mod unicode;

// /// Data constants and lookup helpers, such as bools and emojis.
// /// Planned for v0.6.0, but not yet implemented.
// pub mod data;

/// Re-exported result alias and primary error type for ergonomics.
pub use error::{Result, VeltrixError as Error};

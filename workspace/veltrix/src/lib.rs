//! Veltrix: small utilities for paths, processes, services, and platform helpers.

/// Error types and result alias used by Veltrix helpers.
pub mod error;

/// OS helpers and common OS-related constants.
pub mod os;

/// Service integrations such as Podman and Caddy.
/// Introduced in v0.2.0, but still in early stages of development.
pub mod services;

/// Unicode helpers, such as emoji constants and lookup functions.
pub mod unicode;

#[cfg(feature = "data")]
/// Value-level parsing and formatting helpers.
pub mod data;

/// Re-exported result alias and primary error type for ergonomics.
pub use error::{Result, VeltrixError as Error};

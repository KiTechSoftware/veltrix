//! Technitium DNS Server API client and foundation types.
//!
//! The v0.6.0 client covers authentication, zones, records, settings,
//! resolving, logs, stats, blocking, and CI/CD-style import/bulk workflows.

pub mod client;
pub mod spec;
pub mod types;

pub use client::TechnitiumClient;
pub use spec::*;
pub use types::*;

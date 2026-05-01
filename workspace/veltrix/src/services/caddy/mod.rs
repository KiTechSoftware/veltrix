//! Caddy Admin API integration.
//!
//! This module exposes an async client for selected Caddy 2 Admin API
//! workflows over HTTP or Unix sockets.

pub mod admin;
pub mod cli;
pub mod spec;
pub mod types;

pub use admin::CaddyAdminClient;
pub use cli::CaddyCliClient;
pub use spec::*;
pub use types::*;

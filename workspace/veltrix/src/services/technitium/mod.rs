//! Technitium DNS Server API foundation types.
//!
//! v0.3.0 pins the intended HTTP API family and models authentication,
//! response wrappers, and initial DNS data shapes. Endpoint clients are
//! planned for later service milestones.

pub mod spec;
pub mod types;

pub use spec::*;
pub use types::*;

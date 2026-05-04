//! LDAP service integration.
//!
//! This module provides typed LDAP v3 client support for directory operations:
//! user/group lookup, entry mutations, password management, and more.
//!
//! # Feature Flags
//!
//! - `ldap` — Core LDAP support
//! - `ldap-sasl` — SASL authentication mechanisms (PLAIN, EXTERNAL)
//! - `ldap-gssapi` — SASL/GSSAPI (Kerberos)
//!
//! # Example
//!
//! ```no_run
//! use veltrix::services::ldap::{LdapSpec, LdapAuthMethod};
//!
//! let spec = LdapSpec::new(
//!     "ldap://localhost:389".into(),
//!     LdapAuthMethod::Simple {
//!         bind_dn: "cn=admin,dc=example,dc=com".into(),
//!         password: "secret".into(),
//!     },
//! ).with_connect_timeout(std::time::Duration::from_secs(5));
//! ```

pub mod client;
pub mod spec;
pub mod types;

pub use client::LdapClient;
pub use spec::{
    LdapAuthMethod, LdapBackendUsed, LdapEmptyResponse, LdapResponse, LdapSpec, ServerType, TlsMode,
};
pub use types::{LdapAttribute, LdapEntry, ModifyOp, SearchOptions, SearchScope};

//! Error types returned by Veltrix helpers.
//!
//! The crate exposes a small `VeltrixError` enumeration for common
//! failure modes (I/O, missing environment variables, invalid config,
//! invalid paths, and service integration errors) and a `Result<T>` alias
//! to simplify signatures.

use std::path::PathBuf;

use thiserror::Error;

/// Result alias used across the crate: `Result<T>` == `std::result::Result<T, VeltrixError>`.
pub type Result<T> = std::result::Result<T, VeltrixError>;

/// A compact set of errors commonly returned by Veltrix helpers.
#[derive(Debug, Error)]
pub enum VeltrixError {
    /// I/O operation failed.
    #[error("io failed: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },

    /// Required environment variable is not set.
    #[error("missing environment variable `{name}`")]
    EnvMissing { name: &'static str },

    /// Environment variable contains an invalid value.
    #[error("invalid environment variable `{name}`: {reason}")]
    EnvInvalid { name: &'static str, reason: String },

    /// Configuration value is invalid.
    #[error("invalid config: {reason}")]
    ConfigInvalid { reason: String },

    /// Path is invalid for the requested operation.
    #[error("invalid path `{path}`: {reason}")]
    InvalidPath { path: PathBuf, reason: String },

    /// Service response parsing failed.
    #[error("parsing failed: {0}")]
    Parsing(String),

    /// A service-specific operation failed.
    #[error("{service} service failed: {reason}")]
    Service { service: String, reason: String },

    /// Unix socket communication failed.
    #[error("socket failed: {reason}")]
    Socket { reason: String },

    /// HTTP API request failed.
    #[error("http request failed with status {status}: {reason}")]
    Http { status: u16, reason: String },

    /// Authentication or credential handling failed.
    #[error("authentication failed: {reason}")]
    Auth { reason: String },

    /// Configuration or request input failed validation.
    #[error("validation failed for `{field}`: {reason}")]
    Validation { field: String, reason: String },
}

impl VeltrixError {
    /// Construct a `EnvMissing` error for the given environment variable name.
    pub fn env_missing(name: &'static str) -> Self {
        Self::EnvMissing { name }
    }

    /// Construct an `EnvInvalid` error with a reason message.
    pub fn env_invalid(name: &'static str, reason: impl Into<String>) -> Self {
        Self::EnvInvalid {
            name,
            reason: reason.into(),
        }
    }

    /// Construct a `ConfigInvalid` error with the given reason.
    pub fn config_invalid(reason: impl Into<String>) -> Self {
        Self::ConfigInvalid {
            reason: reason.into(),
        }
    }

    /// Construct an `InvalidPath` error for a path and reason.
    pub fn invalid_path(path: impl Into<PathBuf>, reason: impl Into<String>) -> Self {
        Self::InvalidPath {
            path: path.into(),
            reason: reason.into(),
        }
    }

    /// Construct a `Parsing` error with the given reason.
    pub fn parsing(reason: impl Into<String>) -> Self {
        Self::Parsing(reason.into())
    }

    /// Construct a `Service` error for a named service and reason.
    pub fn service(service: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::Service {
            service: service.into(),
            reason: reason.into(),
        }
    }

    /// Construct a `Socket` error with the given reason.
    pub fn socket(reason: impl Into<String>) -> Self {
        Self::Socket {
            reason: reason.into(),
        }
    }

    /// Construct an `Http` error with a status code and reason.
    pub fn http(status: u16, reason: impl Into<String>) -> Self {
        Self::Http {
            status,
            reason: reason.into(),
        }
    }

    /// Construct an `Auth` error with the given reason.
    pub fn auth(reason: impl Into<String>) -> Self {
        Self::Auth {
            reason: reason.into(),
        }
    }

    /// Construct a `Validation` error for a field and reason.
    pub fn validation(field: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            reason: reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn io_error_converts_to_veltrix_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "test error");
        let veltrix_err: VeltrixError = io_err.into();

        assert!(veltrix_err.to_string().contains("test error"));
    }

    #[test]
    fn env_missing_message() {
        let err = VeltrixError::env_missing("HOME");
        assert_eq!(err.to_string(), "missing environment variable `HOME`");
    }

    #[test]
    fn env_invalid_message() {
        let err = VeltrixError::env_invalid("HOME", "must be absolute");
        assert_eq!(
            err.to_string(),
            "invalid environment variable `HOME`: must be absolute"
        );
    }

    #[test]
    fn config_invalid_message() {
        let err = VeltrixError::config_invalid("missing required field");
        assert_eq!(err.to_string(), "invalid config: missing required field");
    }

    #[test]
    fn invalid_path_message() {
        let err = VeltrixError::invalid_path("relative/path", "must be absolute");
        assert!(err.to_string().contains("relative/path"));
        assert!(err.to_string().contains("must be absolute"));
    }

    #[test]
    fn parsing_message() {
        let err = VeltrixError::parsing("invalid json");
        assert_eq!(err.to_string(), "parsing failed: invalid json");
    }

    #[test]
    fn service_message() {
        let err = VeltrixError::service("podman", "command failed");
        assert_eq!(err.to_string(), "podman service failed: command failed");
    }

    #[test]
    fn socket_message() {
        let err = VeltrixError::socket("connection refused");
        assert_eq!(err.to_string(), "socket failed: connection refused");
    }

    #[test]
    fn http_message() {
        let err = VeltrixError::http(401, "unauthorized");
        assert_eq!(
            err.to_string(),
            "http request failed with status 401: unauthorized"
        );
    }

    #[test]
    fn auth_message() {
        let err = VeltrixError::auth("missing token");
        assert_eq!(err.to_string(), "authentication failed: missing token");
    }

    #[test]
    fn validation_message() {
        let err = VeltrixError::validation("name", "must not be empty");
        assert_eq!(
            err.to_string(),
            "validation failed for `name`: must not be empty"
        );
    }

    #[test]
    fn result_type_is_correct() {
        fn returns_result() -> Result<String> {
            Ok("success".to_string())
        }

        assert!(returns_result().is_ok());
    }
}

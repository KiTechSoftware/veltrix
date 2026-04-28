use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, VeltrixError>;

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

    /// Environment variable contains an invalid path.
    #[error("invalid environment variable `{name}`: {reason}")]
    EnvInvalid {
        name: &'static str,
        reason: String,
    },

    /// Config is invalid.
    #[error("invalid config: {reason}")]
    ConfigInvalid { reason: String },

    /// Path is invalid for the requested operation.
    #[error("invalid path `{path}`: {reason}")]
    InvalidPath {
        path: PathBuf,
        reason: String,
    },
}

impl VeltrixError {
    pub fn env_missing(name: &'static str) -> Self {
        Self::EnvMissing { name }
    }

    pub fn env_invalid(name: &'static str, reason: impl Into<String>) -> Self {
        Self::EnvInvalid {
            name,
            reason: reason.into(),
        }
    }

    pub fn config_invalid(reason: impl Into<String>) -> Self {
        Self::ConfigInvalid {
            reason: reason.into(),
        }
    }

    pub fn invalid_path(path: impl Into<PathBuf>, reason: impl Into<String>) -> Self {
        Self::InvalidPath {
            path: path.into(),
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
    fn result_type_is_correct() {
        fn returns_result() -> Result<String> {
            Ok("success".to_string())
        }

        assert!(returns_result().is_ok());
    }
}
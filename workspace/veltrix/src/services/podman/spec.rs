use std::path::PathBuf;

use crate::{
    error::{Result, VeltrixError},
    os::paths::constants::XDG_RUNTIME_DIR_ENV,
};

/// Supported Podman major version.
pub const SUPPORTED_PODMAN_MAJOR: u64 = 5;
/// Supported Libpod REST API version.
pub const SUPPORTED_LIBPOD_API_VERSION: &str = "5.0.0";

/// Podman socket user metadata.
#[derive(Debug, Clone)]
pub struct PodmanUser {
    /// Unix user ID.
    pub uid: u32,
    /// Optional Unix group ID.
    pub gid: Option<u32>,
    /// Optional username.
    pub username: Option<String>,
}

/// Podman CLI backend configuration.
#[derive(Debug, Clone)]
pub struct PodmanCliSpec {
    /// Podman executable name or path.
    pub binary: String,
    /// Whether to execute through `sudo`.
    pub sudo: bool,
    /// Optional Unix user ID to run as.
    pub uid: Option<u32>,
    /// Optional Unix group ID to run as.
    pub gid: Option<u32>,
}

impl Default for PodmanCliSpec {
    fn default() -> Self {
        Self {
            binary: "podman".to_string(),
            sudo: false,
            uid: None,
            gid: None,
        }
    }
}

impl PodmanCliSpec {
    /// Create a default Podman CLI spec.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the Podman executable name or path.
    pub fn binary(mut self, binary: impl Into<String>) -> Self {
        self.binary = binary.into();
        self
    }

    /// Execute Podman through `sudo`.
    pub fn sudo(mut self) -> Self {
        self.sudo = true;
        self
    }

    /// Set the Unix user ID used when executing Podman.
    pub fn uid(mut self, uid: u32) -> Self {
        self.uid = Some(uid);
        self
    }

    /// Set the Unix group ID used when executing Podman.
    pub fn gid(mut self, gid: u32) -> Self {
        self.gid = Some(gid);
        self
    }

    #[cfg(feature = "unistd")]
    /// Set the Unix user ID from a `Uid` wrapper.
    pub fn user(mut self, uid: crate::os::unistd::Uid) -> Self {
        self.uid = Some(uid.as_raw());
        self
    }

    #[cfg(feature = "unistd")]
    /// Set the Unix group ID from a `Gid` wrapper.
    pub fn group(mut self, gid: crate::os::unistd::Gid) -> Self {
        self.gid = Some(gid.as_raw());
        self
    }
}

/// Podman Unix socket backend configuration.
#[derive(Debug, Clone)]
pub struct PodmanSocketSpec {
    /// Socket path used to contact Podman's service.
    pub socket_path: PathBuf,
    /// Optional user metadata for backend tracking.
    pub user: Option<PodmanUser>,
}

impl PodmanSocketSpec {
    /// Create a socket spec for an explicit socket path.
    pub fn new(socket_path: impl Into<PathBuf>) -> Self {
        Self {
            socket_path: socket_path.into(),
            user: None,
        }
    }

    /// Attach user metadata to the socket spec.
    pub fn user(mut self, user: PodmanUser) -> Self {
        self.user = Some(user);
        self
    }

    /// Use Podman's rootful default socket path.
    pub fn rootful_default() -> Self {
        Self::new("/run/podman/podman.sock")
    }

    /// Build a rootless socket path from `XDG_RUNTIME_DIR`.
    pub fn rootless_from_env() -> Result<Self> {
        let runtime_dir = std::env::var_os(XDG_RUNTIME_DIR_ENV)
            .ok_or_else(|| VeltrixError::env_missing(XDG_RUNTIME_DIR_ENV))?;

        Ok(Self::new(
            PathBuf::from(runtime_dir).join("podman/podman.sock"),
        ))
    }
}

/// Metadata describing which Podman backend produced a response.
#[derive(Debug, Clone)]
pub enum PodmanBackendUsed {
    /// Podman CLI backend metadata.
    Cli {
        /// Podman executable name or path.
        binary: String,
        /// Whether `sudo` was used.
        sudo: bool,
        /// Unix user ID used for execution.
        uid: Option<u32>,
        /// Unix group ID used for execution.
        gid: Option<u32>,
    },
    /// Podman socket backend metadata.
    Socket {
        /// Socket path used for the request.
        socket_path: PathBuf,
        /// Optional user metadata.
        user: Option<PodmanUser>,
    },
    /// Podman Compose backend metadata.
    Compose {
        /// Compose executable name or path.
        binary: String,
        /// Optional compose file path.
        compose_file: Option<PathBuf>,
        /// Optional compose project name.
        project_name: Option<String>,
    },
    /// Podman machine backend metadata.
    Machine {
        /// Optional machine name.
        machine_name: Option<String>,
    },
}

/// Response wrapper for Podman operations with data.
#[derive(Debug, Clone)]
pub struct PodmanResponse<T> {
    /// Backend metadata for this response.
    pub backend: PodmanBackendUsed,
    /// Response payload.
    pub data: T,
}

impl<T> PodmanResponse<T> {
    /// Create a Podman response wrapper.
    pub fn new(data: T, backend: PodmanBackendUsed) -> Self {
        Self { data, backend }
    }
}

/// Response wrapper for successful Podman operations with no body.
#[derive(Debug, Clone)]
pub struct PodmanEmptyResponse {
    /// Backend metadata for this response.
    pub backend: PodmanBackendUsed,
}

impl PodmanEmptyResponse {
    /// Create an empty Podman response wrapper.
    pub fn new(backend: PodmanBackendUsed) -> Self {
        Self { backend }
    }
}

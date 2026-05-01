use std::path::PathBuf;

use crate::error::Result;

pub const SUPPORTED_DOCKER_API_VERSION: &str = "1.40";

/// Docker backend specification for CLI-based execution
#[derive(Debug, Clone)]
pub struct DockerCliSpec {
    pub binary: String,
    pub sudo: bool,
}

impl Default for DockerCliSpec {
    fn default() -> Self {
        Self {
            binary: "docker".to_string(),
            sudo: false,
        }
    }
}

impl DockerCliSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn binary(mut self, binary: impl Into<String>) -> Self {
        self.binary = binary.into();
        self
    }

    pub fn sudo(mut self) -> Self {
        self.sudo = true;
        self
    }
}

/// Docker backend specification for Unix socket-based API access
#[derive(Debug, Clone)]
pub struct DockerSocketSpec {
    pub socket_path: PathBuf,
}

impl Default for DockerSocketSpec {
    fn default() -> Self {
        Self {
            socket_path: PathBuf::from("/var/run/docker.sock"),
        }
    }
}

impl DockerSocketSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn socket_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.socket_path = path.into();
        self
    }
}

/// Docker Compose specification for compose-based operations
#[derive(Debug, Clone)]
pub struct DockerComposeSpec {
    pub binary: String,
    pub compose_file: Option<String>,
}

impl Default for DockerComposeSpec {
    fn default() -> Self {
        Self {
            binary: "docker-compose".to_string(),
            compose_file: None,
        }
    }
}

impl DockerComposeSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn binary(mut self, binary: impl Into<String>) -> Self {
        self.binary = binary.into();
        self
    }

    pub fn compose_file(mut self, file: impl Into<String>) -> Self {
        self.compose_file = Some(file.into());
        self
    }
}

/// Metadata describing which Docker backend was used for a response
#[derive(Debug, Clone)]
pub enum DockerBackendUsed {
    Cli { binary: String, sudo: bool },
    Socket { socket_path: PathBuf },
    Compose { binary: String, compose_file: Option<String> },
}

/// Standard response wrapper for Docker API calls
#[derive(Debug, Clone)]
pub struct DockerResponse<T> {
    pub data: T,
    pub backend: DockerBackendUsed,
}

impl<T> DockerResponse<T> {
    pub fn new(data: T, backend: DockerBackendUsed) -> Self {
        Self { data, backend }
    }
}

/// Empty response for successful operations with no body (e.g., stop, remove)
#[derive(Debug, Clone)]
pub struct DockerEmptyResponse {
    pub backend: DockerBackendUsed,
}

impl DockerEmptyResponse {
    pub fn new(backend: DockerBackendUsed) -> Self {
        Self { backend }
    }
}

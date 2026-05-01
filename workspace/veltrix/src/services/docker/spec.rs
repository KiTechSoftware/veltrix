use std::path::PathBuf;

/// Docker Engine API version targeted by the socket backend.
pub const SUPPORTED_DOCKER_API_VERSION: &str = "1.40";

/// Configured Docker backend.
#[derive(Debug, Clone)]
pub enum DockerBackendSpec {
    /// Docker CLI backend.
    Cli(DockerCliSpec),
    /// Docker Unix socket backend.
    Socket(DockerSocketSpec),
    /// Docker Compose backend.
    Compose(DockerComposeSpec),
}

/// Docker backend specification for CLI-based execution.
#[derive(Debug, Clone)]
pub struct DockerCliSpec {
    /// Docker executable name or path.
    pub binary: String,
    /// Whether to execute the command through `sudo`.
    pub sudo: bool,
    /// Optional Unix user ID to run as.
    pub uid: Option<u32>,
    /// Optional Unix group ID to run as.
    pub gid: Option<u32>,
}

impl Default for DockerCliSpec {
    fn default() -> Self {
        Self {
            binary: "docker".to_string(),
            sudo: false,
            uid: None,
            gid: None,
        }
    }
}

impl DockerCliSpec {
    /// Create a default Docker CLI spec.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the Docker executable name or path.
    pub fn binary(mut self, binary: impl Into<String>) -> Self {
        self.binary = binary.into();
        self
    }

    /// Execute Docker through `sudo`.
    pub fn sudo(mut self) -> Self {
        self.sudo = true;
        self
    }

    /// Set the Unix user ID used when executing Docker.
    pub fn uid(mut self, uid: u32) -> Self {
        self.uid = Some(uid);
        self
    }

    /// Set the Unix group ID used when executing Docker.
    pub fn gid(mut self, gid: u32) -> Self {
        self.gid = Some(gid);
        self
    }
}

/// Docker backend specification for Unix socket-based API access.
#[derive(Debug, Clone)]
pub struct DockerSocketSpec {
    /// Socket path, usually `/var/run/docker.sock`.
    pub socket_path: PathBuf,
    /// Optional user label captured in backend metadata.
    pub user: Option<String>,
}

impl Default for DockerSocketSpec {
    fn default() -> Self {
        Self {
            socket_path: PathBuf::from("/var/run/docker.sock"),
            user: None,
        }
    }
}

impl DockerSocketSpec {
    /// Create a default Docker socket spec.
    pub fn new() -> Self {
        Self::default()
    }

    /// Override the Docker socket path.
    pub fn socket_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.socket_path = path.into();
        self
    }

    /// Attach a user label to backend metadata.
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

/// Docker Compose specification for compose-based operations.
#[derive(Debug, Clone)]
pub struct DockerComposeSpec {
    /// Compose executable name or path.
    pub binary: String,
    /// Optional compose file path.
    pub compose_file: Option<String>,
    /// Optional Compose project name.
    pub project_name: Option<String>,
}

impl Default for DockerComposeSpec {
    fn default() -> Self {
        Self {
            binary: "docker-compose".to_string(),
            compose_file: None,
            project_name: None,
        }
    }
}

impl DockerComposeSpec {
    /// Create a default Docker Compose spec.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the Compose executable name or path.
    pub fn binary(mut self, binary: impl Into<String>) -> Self {
        self.binary = binary.into();
        self
    }

    /// Set the compose file path.
    pub fn compose_file(mut self, file: impl Into<String>) -> Self {
        self.compose_file = Some(file.into());
        self
    }

    /// Set the Compose project name.
    pub fn project_name(mut self, name: impl Into<String>) -> Self {
        self.project_name = Some(name.into());
        self
    }
}

/// Metadata describing which Docker backend was used for a response.
#[derive(Debug, Clone)]
pub enum DockerBackendUsed {
    /// Docker CLI backend metadata.
    Cli {
        /// Docker executable name or path.
        binary: String,
        /// Whether `sudo` was used.
        sudo: bool,
        /// Unix user ID used for execution.
        uid: Option<u32>,
        /// Unix group ID used for execution.
        gid: Option<u32>,
    },
    /// Docker socket backend metadata.
    Socket {
        /// Socket path used for the request.
        socket_path: PathBuf,
        /// Optional user label.
        user: Option<String>,
    },
    /// Docker Compose backend metadata.
    Compose {
        /// Compose executable name or path.
        binary: String,
        /// Compose file path.
        compose_file: Option<String>,
        /// Compose project name.
        project_name: Option<String>,
    },
}

/// Standard response wrapper for Docker API calls.
#[derive(Debug, Clone)]
pub struct DockerResponse<T> {
    /// Deserialized response payload.
    pub data: T,
    /// Backend metadata for this response.
    pub backend: DockerBackendUsed,
}

impl<T> DockerResponse<T> {
    /// Create a Docker response wrapper.
    pub fn new(data: T, backend: DockerBackendUsed) -> Self {
        Self { data, backend }
    }
}

/// Empty response for successful operations with no body.
#[derive(Debug, Clone)]
pub struct DockerEmptyResponse {
    /// Backend metadata for this response.
    pub backend: DockerBackendUsed,
}

impl DockerEmptyResponse {
    /// Create an empty Docker response wrapper.
    pub fn new(backend: DockerBackendUsed) -> Self {
        Self { backend }
    }
}

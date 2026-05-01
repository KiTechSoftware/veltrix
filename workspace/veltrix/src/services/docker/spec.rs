use std::path::PathBuf;

pub const SUPPORTED_DOCKER_API_VERSION: &str = "1.40";

/// Configured Docker backend.
#[derive(Debug, Clone)]
pub enum DockerBackendSpec {
    Cli(DockerCliSpec),
    Socket(DockerSocketSpec),
    Compose(DockerComposeSpec),
}

/// Docker backend specification for CLI-based execution
#[derive(Debug, Clone)]
pub struct DockerCliSpec {
    pub binary: String,
    pub sudo: bool,
    pub uid: Option<u32>,
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

    pub fn uid(mut self, uid: u32) -> Self {
        self.uid = Some(uid);
        self
    }

    pub fn gid(mut self, gid: u32) -> Self {
        self.gid = Some(gid);
        self
    }
}

/// Docker backend specification for Unix socket-based API access
#[derive(Debug, Clone)]
pub struct DockerSocketSpec {
    pub socket_path: PathBuf,
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn socket_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.socket_path = path.into();
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

/// Docker Compose specification for compose-based operations
#[derive(Debug, Clone)]
pub struct DockerComposeSpec {
    pub binary: String,
    pub compose_file: Option<String>,
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

    pub fn project_name(mut self, name: impl Into<String>) -> Self {
        self.project_name = Some(name.into());
        self
    }
}

/// Metadata describing which Docker backend was used for a response
#[derive(Debug, Clone)]
pub enum DockerBackendUsed {
    Cli {
        binary: String,
        sudo: bool,
        uid: Option<u32>,
        gid: Option<u32>,
    },
    Socket {
        socket_path: PathBuf,
        user: Option<String>,
    },
    Compose {
        binary: String,
        compose_file: Option<String>,
        project_name: Option<String>,
    },
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

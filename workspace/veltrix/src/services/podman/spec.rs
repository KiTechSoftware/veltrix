use std::path::PathBuf;

use crate::{
    error::{Result, VeltrixError},
    os::paths::constants::XDG_RUNTIME_DIR_ENV,
};

pub const SUPPORTED_PODMAN_MAJOR: u64 = 5;
pub const SUPPORTED_LIBPOD_API_VERSION: &str = "5.0.0";

#[derive(Debug, Clone)]
pub struct PodmanUser {
    pub uid: u32,
    pub gid: Option<u32>,
    pub username: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PodmanCliSpec {
    pub binary: String,
    pub sudo: bool,
    pub uid: Option<u32>,
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

    #[cfg(feature = "unistd")]
    pub fn user(mut self, uid: crate::os::unistd::Uid) -> Self {
        self.uid = Some(uid.as_raw());
        self
    }

    #[cfg(feature = "unistd")]
    pub fn group(mut self, gid: crate::os::unistd::Gid) -> Self {
        self.gid = Some(gid.as_raw());
        self
    }
}

#[derive(Debug, Clone)]
pub struct PodmanSocketSpec {
    pub socket_path: PathBuf,
    pub user: Option<PodmanUser>,
}

impl PodmanSocketSpec {
    pub fn new(socket_path: impl Into<PathBuf>) -> Self {
        Self {
            socket_path: socket_path.into(),
            user: None,
        }
    }

    pub fn user(mut self, user: PodmanUser) -> Self {
        self.user = Some(user);
        self
    }

    pub fn rootful_default() -> Self {
        Self::new("/run/podman/podman.sock")
    }

    pub fn rootless_from_env() -> Result<Self> {
        let runtime_dir = std::env::var_os(XDG_RUNTIME_DIR_ENV)
            .ok_or_else(|| VeltrixError::env_missing(XDG_RUNTIME_DIR_ENV))?;

        Ok(Self::new(
            PathBuf::from(runtime_dir).join("podman/podman.sock"),
        ))
    }
}

#[derive(Debug, Clone)]
pub enum PodmanBackendUsed {
    Cli {
        binary: String,
        sudo: bool,
        uid: Option<u32>,
        gid: Option<u32>,
    },
    Socket {
        socket_path: PathBuf,
        user: Option<PodmanUser>,
    },
    Compose {
        binary: String,
        compose_file: Option<PathBuf>,
        project_name: Option<String>,
    },
    Machine {
        machine_name: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub struct PodmanResponse<T> {
    pub backend: PodmanBackendUsed,
    pub data: T,
}

impl<T> PodmanResponse<T> {
    pub fn new(data: T, backend: PodmanBackendUsed) -> Self {
        Self { data, backend }
    }
}

#[derive(Debug, Clone)]
pub struct PodmanEmptyResponse {
    pub backend: PodmanBackendUsed,
}

impl PodmanEmptyResponse {
    pub fn new(backend: PodmanBackendUsed) -> Self {
        Self { backend }
    }
}

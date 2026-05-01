use std::path::PathBuf;

pub const SUPPORTED_CADDY_MAJOR: u64 = 2;

#[derive(Debug, Clone)]
pub enum CaddyAdminEndpoint {
    Http {
        base_url: String,
    },
    UnixSocket {
        socket_path: PathBuf,
    },
}

impl Default for CaddyAdminEndpoint {
    fn default() -> Self {
        Self::Http {
            base_url: "http://localhost:2019".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct CaddyAdminSpec {
    pub endpoint: CaddyAdminEndpoint,
}

impl CaddyAdminSpec {
    pub fn new(endpoint: CaddyAdminEndpoint) -> Self {
        Self { endpoint }
    }

    pub fn localhost_default() -> Self {
        Self::default()
    }

    pub fn http(base_url: impl Into<String>) -> Self {
        Self {
            endpoint: CaddyAdminEndpoint::Http {
                base_url: base_url.into(),
            },
        }
    }

    pub fn unix_socket(socket_path: impl Into<PathBuf>) -> Self {
        Self {
            endpoint: CaddyAdminEndpoint::UnixSocket {
                socket_path: socket_path.into(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum CaddyBackendUsed {
    Http {
        base_url: String,
    },
    UnixSocket {
        socket_path: PathBuf,
    },
}

#[derive(Debug, Clone)]
pub struct CaddyResponse<T> {
    pub backend: CaddyBackendUsed,
    pub data: T,
}

#[derive(Debug, Clone)]
pub struct CaddyEmptyResponse {
    pub backend: CaddyBackendUsed,
}
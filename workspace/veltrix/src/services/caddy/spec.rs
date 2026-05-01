use std::path::PathBuf;

/// Supported Caddy major version for this integration.
pub const SUPPORTED_CADDY_MAJOR: u64 = 2;

/// Caddy CLI backend specification.
#[derive(Debug, Clone)]
pub struct CaddyCliSpec {
    /// Caddy executable name or path.
    pub binary: String,
}

impl Default for CaddyCliSpec {
    fn default() -> Self {
        Self {
            binary: "caddy".to_string(),
        }
    }
}

impl CaddyCliSpec {
    /// Create a default Caddy CLI spec.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the Caddy executable name or path.
    pub fn binary(mut self, binary: impl Into<String>) -> Self {
        self.binary = binary.into();
        self
    }
}

/// Caddy Admin API endpoint transport.
#[derive(Debug, Clone)]
pub enum CaddyAdminEndpoint {
    /// HTTP endpoint, usually `http://localhost:2019`.
    Http { base_url: String },
    /// Unix-domain socket endpoint.
    UnixSocket { socket_path: PathBuf },
}

impl Default for CaddyAdminEndpoint {
    fn default() -> Self {
        Self::Http {
            base_url: "http://localhost:2019".to_string(),
        }
    }
}

/// Caddy Admin API client configuration.
#[derive(Debug, Clone, Default)]
pub struct CaddyAdminSpec {
    /// Configured admin endpoint.
    pub endpoint: CaddyAdminEndpoint,
}

impl CaddyAdminSpec {
    /// Create a spec for an explicit endpoint.
    pub fn new(endpoint: CaddyAdminEndpoint) -> Self {
        Self { endpoint }
    }

    /// Use Caddy's default local HTTP admin endpoint.
    pub fn localhost_default() -> Self {
        Self::default()
    }

    /// Use an HTTP admin endpoint.
    pub fn http(base_url: impl Into<String>) -> Self {
        Self {
            endpoint: CaddyAdminEndpoint::Http {
                base_url: base_url.into(),
            },
        }
    }

    /// Use a Unix-domain socket admin endpoint.
    pub fn unix_socket(socket_path: impl Into<PathBuf>) -> Self {
        Self {
            endpoint: CaddyAdminEndpoint::UnixSocket {
                socket_path: socket_path.into(),
            },
        }
    }
}

/// Metadata describing the Caddy backend used for a response.
#[derive(Debug, Clone)]
pub enum CaddyBackendUsed {
    /// Caddy CLI backend.
    Cli { binary: String },
    /// HTTP Admin API backend.
    Http { base_url: String },
    /// Unix-domain socket Admin API backend.
    UnixSocket { socket_path: PathBuf },
}

/// Response wrapper for Caddy Admin API calls with a JSON body.
#[derive(Debug, Clone)]
pub struct CaddyResponse<T> {
    /// Backend metadata for this response.
    pub backend: CaddyBackendUsed,
    /// Deserialized response payload.
    pub data: T,
}

/// Response wrapper for successful Caddy calls with no response body.
#[derive(Debug, Clone)]
pub struct CaddyEmptyResponse {
    /// Backend metadata for this response.
    pub backend: CaddyBackendUsed,
}

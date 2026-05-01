use std::time::Duration;

/// Supported Technitium DNS Server HTTP API family.
pub const SUPPORTED_TECHNITIUM_API_VERSION: &str = "13.x HTTP API";

/// Authentication material for Technitium DNS Server API requests.
///
/// Tokens are intentionally not included in `Debug` output. Callers should
/// avoid logging complete specs because base URLs can still reveal topology.
#[derive(Clone)]
pub enum TechnitiumAuth {
    /// No credentials are configured.
    None,
    /// Session token returned by login/session workflows.
    SessionToken { token: String },
    /// Caller-provided bearer token.
    BearerToken { token: String },
}

impl std::fmt::Debug for TechnitiumAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::SessionToken { .. } => f.write_str("SessionToken { token: <redacted> }"),
            Self::BearerToken { .. } => f.write_str("BearerToken { token: <redacted> }"),
        }
    }
}

impl TechnitiumAuth {
    /// Create session-token authentication.
    pub fn session_token(token: impl Into<String>) -> Self {
        Self::SessionToken {
            token: token.into(),
        }
    }

    /// Create bearer-token authentication.
    pub fn bearer_token(token: impl Into<String>) -> Self {
        Self::BearerToken {
            token: token.into(),
        }
    }

    /// Return whether this auth value contains credentials.
    pub fn is_authenticated(&self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Technitium HTTP API client configuration.
#[derive(Debug, Clone)]
pub struct TechnitiumHttpSpec {
    /// Base HTTP URL for the Technitium server.
    pub base_url: String,
    /// Authentication mode and token material.
    pub auth: TechnitiumAuth,
    /// Optional request timeout.
    pub timeout: Option<Duration>,
}

impl TechnitiumHttpSpec {
    /// Create a new HTTP spec without credentials.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            auth: TechnitiumAuth::None,
            timeout: None,
        }
    }

    /// Set the authentication mode.
    pub fn auth(mut self, auth: TechnitiumAuth) -> Self {
        self.auth = auth;
        self
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

/// Metadata describing which Technitium backend produced a response.
#[derive(Debug, Clone)]
pub enum TechnitiumBackendUsed {
    /// HTTP API backend metadata.
    Http { base_url: String },
}

/// Response wrapper for Technitium operations with data.
#[derive(Debug, Clone)]
pub struct TechnitiumResponse<T> {
    /// Backend metadata for this response.
    pub backend: TechnitiumBackendUsed,
    /// Response payload.
    pub data: T,
}

impl<T> TechnitiumResponse<T> {
    /// Create a Technitium response wrapper.
    pub fn new(data: T, backend: TechnitiumBackendUsed) -> Self {
        Self { backend, data }
    }
}

/// Response wrapper for successful Technitium operations with no body.
#[derive(Debug, Clone)]
pub struct TechnitiumEmptyResponse {
    /// Backend metadata for this response.
    pub backend: TechnitiumBackendUsed,
}

impl TechnitiumEmptyResponse {
    /// Create an empty Technitium response wrapper.
    pub fn new(backend: TechnitiumBackendUsed) -> Self {
        Self { backend }
    }
}

use std::time::Duration;

pub const SUPPORTED_TECHNITIUM_API_VERSION: &str = "13.x HTTP API";

/// Authentication material for Technitium DNS Server API requests.
///
/// Tokens are intentionally not included in `Debug` output. Callers should
/// avoid logging complete specs because base URLs can still reveal topology.
#[derive(Clone)]
pub enum TechnitiumAuth {
    None,
    SessionToken { token: String },
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
    pub fn session_token(token: impl Into<String>) -> Self {
        Self::SessionToken {
            token: token.into(),
        }
    }

    pub fn bearer_token(token: impl Into<String>) -> Self {
        Self::BearerToken {
            token: token.into(),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        !matches!(self, Self::None)
    }
}

#[derive(Debug, Clone)]
pub struct TechnitiumHttpSpec {
    pub base_url: String,
    pub auth: TechnitiumAuth,
    pub timeout: Option<Duration>,
}

impl TechnitiumHttpSpec {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            auth: TechnitiumAuth::None,
            timeout: None,
        }
    }

    pub fn auth(mut self, auth: TechnitiumAuth) -> Self {
        self.auth = auth;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

#[derive(Debug, Clone)]
pub enum TechnitiumBackendUsed {
    Http { base_url: String },
}

#[derive(Debug, Clone)]
pub struct TechnitiumResponse<T> {
    pub backend: TechnitiumBackendUsed,
    pub data: T,
}

impl<T> TechnitiumResponse<T> {
    pub fn new(data: T, backend: TechnitiumBackendUsed) -> Self {
        Self { backend, data }
    }
}

#[derive(Debug, Clone)]
pub struct TechnitiumEmptyResponse {
    pub backend: TechnitiumBackendUsed,
}

impl TechnitiumEmptyResponse {
    pub fn new(backend: TechnitiumBackendUsed) -> Self {
        Self { backend }
    }
}

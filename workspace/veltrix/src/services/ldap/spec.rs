//! LDAP specification and configuration types.

use std::time::Duration;

/// LDAP client response wrapper for read/query operations.
#[derive(Debug, Clone)]
pub struct LdapResponse<T> {
    /// Response data
    pub data: T,
    /// Backend metadata (server type, TLS mode, auth method, timing)
    pub backend_used: LdapBackendUsed,
}

impl<T> LdapResponse<T> {
    /// Create a new response with data and backend metadata.
    pub fn new(data: T, backend_used: LdapBackendUsed) -> Self {
        Self { data, backend_used }
    }
}

/// LDAP response wrapper for mutation operations that return no body.
#[derive(Debug, Clone)]
pub struct LdapEmptyResponse {
    /// Backend metadata (server type, TLS mode, auth method, timing)
    pub backend_used: LdapBackendUsed,
}

impl LdapEmptyResponse {
    /// Create a new empty response with backend metadata.
    pub fn new(backend_used: LdapBackendUsed) -> Self {
        Self { backend_used }
    }
}

/// Backend metadata tracking.
#[derive(Debug, Clone)]
pub struct LdapBackendUsed {
    /// Detected LDAP server type.
    pub server_type: ServerType,
    /// TLS mode used for connection.
    pub tls_mode_used: TlsMode,
    /// Authentication method used.
    pub auth_method_used: String,
    /// Connection time in milliseconds.
    pub connection_time_ms: u64,
}

/// Detected LDAP server type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerType {
    /// OpenLDAP (slapd)
    OpenLDAP,
    /// 389 Directory Server
    DirectoryServer389,
    /// Microsoft Active Directory
    ActiveDirectory,
    /// Unknown or generic LDAP server
    Unknown,
}

impl std::fmt::Display for ServerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenLDAP => write!(f, "OpenLDAP"),
            Self::DirectoryServer389 => write!(f, "389 Directory Server"),
            Self::ActiveDirectory => write!(f, "Active Directory"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// TLS mode for LDAP connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsMode {
    /// No encryption (development/local only)
    None,
    /// StartTLS (upgrade after initial connection)
    StartTLS,
    /// LDAPS (implicit TLS from connection start)
    LDAPS,
}

impl std::fmt::Display for TlsMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::StartTLS => write!(f, "StartTLS"),
            Self::LDAPS => write!(f, "LDAPS"),
        }
    }
}

/// LDAP authentication method.
#[derive(Debug, Clone)]
pub enum LdapAuthMethod {
    /// Simple bind (DN + password).
    Simple {
        /// Bind DN (e.g., "cn=admin,dc=example,dc=com")
        bind_dn: String,
        /// Password (NEVER logged)
        password: String,
    },
    /// SASL/PLAIN (identity + password).
    #[cfg(feature = "ldap-sasl")]
    SaslPlain {
        /// Authorization identity
        identity: String,
        /// Password (NEVER logged)
        password: String,
    },
    /// SASL/EXTERNAL (TLS client certificate).
    #[cfg(feature = "ldap-sasl")]
    SaslExternal,
    /// Anonymous bind (no credentials).
    Anonymous,
}

impl std::fmt::Display for LdapAuthMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple { .. } => write!(f, "simple"),
            #[cfg(feature = "ldap-sasl")]
            Self::SaslPlain { .. } => write!(f, "sasl_plain"),
            #[cfg(feature = "ldap-sasl")]
            Self::SaslExternal => write!(f, "sasl_external"),
            Self::Anonymous => write!(f, "anonymous"),
        }
    }
}

/// LDAP connection specification.
#[derive(Debug, Clone)]
pub struct LdapSpec {
    /// LDAP URI (ldap://, ldaps://, or ldapi://)
    pub uri: String,
    /// Authentication method
    pub auth: LdapAuthMethod,
    /// TLS mode
    pub tls_mode: TlsMode,
    /// CA certificate path or PEM (required in production)
    pub ca_certificate: Option<String>,
    /// Verify server certificate (true in production)
    pub verify_certificate: bool,
    /// Connection timeout
    pub connect_timeout: Duration,
    /// Operation timeout
    pub operation_timeout: Duration,
    /// Page size for paged results (default: 500)
    pub page_size: Option<usize>,
}

impl LdapSpec {
    /// Create a new LDAP specification with defaults.
    pub fn new(uri: String, auth: LdapAuthMethod) -> Self {
        Self {
            uri,
            auth,
            tls_mode: TlsMode::None,
            ca_certificate: None,
            verify_certificate: false,
            connect_timeout: Duration::from_secs(5),
            operation_timeout: Duration::from_secs(30),
            page_size: Some(500),
        }
    }

    /// Set TLS mode.
    pub fn with_tls_mode(mut self, tls_mode: TlsMode) -> Self {
        self.tls_mode = tls_mode;
        self
    }

    /// Set CA certificate path.
    pub fn with_ca_certificate(mut self, ca_cert: String) -> Self {
        self.ca_certificate = Some(ca_cert);
        self
    }

    /// Enable certificate verification.
    pub fn with_verify_certificate(mut self, verify: bool) -> Self {
        self.verify_certificate = verify;
        self
    }

    /// Set connection timeout.
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Set operation timeout.
    pub fn with_operation_timeout(mut self, timeout: Duration) -> Self {
        self.operation_timeout = timeout;
        self
    }

    /// Set page size for paged results.
    pub fn with_page_size(mut self, size: usize) -> Self {
        self.page_size = Some(size);
        self
    }
}

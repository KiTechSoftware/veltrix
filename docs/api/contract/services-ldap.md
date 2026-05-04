# Veltrix LDAP Services Contract

`veltrix::services::ldap` provides typed client support for LDAP (Lightweight Directory Access Protocol) v3-compliant directories, including OpenLDAP, 389 Directory Server, and Active Directory.

## Module Structure

```
veltrix::services::ldap
├── spec
│   ├── LdapSpec             // Connection configuration (URL, bind method, TLS, timeouts)
│   ├── LdapBackendUsed      // Metadata: which backend/protocol was used
│   ├── LdapResponse<T>      // Typed response wrapper
│   ├── LdapEmptyResponse    // No-body response
│   └── LdapAuthMethod       // Enum: simple, sasl_plain, sasl_external, anonymous
├── client
│   ├── LdapClient           // Main sync client (optional: async behind `async` feature)
│   └── [client methods]     // Bind, search, add, modify, delete, password operations
├── types
│   ├── LdapEntry            // Directory entry with DN and attributes
│   ├── LdapAttribute        // Attribute name and multi-valued data
│   ├── SearchOptions        // Filter, scope, size limit, time limit, paged results
│   ├── SearchScope          // base | onelevel | subtree
│   ├── BindCredentials      // Simple bind DN and password (never logged)
│   └── [common types]       // User summaries, group info, POSIX account types
└── error
    └── [Error variants]      // Parsing, Auth, Validation, Socket, Http, Service
```

## Configuration (LdapSpec)

The `LdapSpec` captures URL, authentication method, TLS policy, and operational timeouts:

```rust
LdapSpec {
    // Required
    uri: String,                    // ldap://host:port, ldaps://host:port, or ldapi:///socket

    // Authentication (choose one)
    auth: LdapAuthMethod,
    
    // TLS & Security
    tls_mode: TlsMode,              // StartTLS | LDAPS | None (default)
    ca_certificate: Option<String>, // Path or PEM; required in production
    verify_certificate: bool,       // true in production
    
    // Operational
    connect_timeout: Duration,      // Default: 5s
    operation_timeout: Duration,    // Default: 30s
    page_size: Option<usize>,       // For paged results; default: 500
}
```

### TLS Modes

- `TlsMode::None` — No encryption (development/local only)
- `TlsMode::StartTLS` — Upgrade connection via StartTLS (RFC 4513 preferred)
- `TlsMode::LDAPS` — Implicit TLS from connection start (port 636 typical)

### Authentication Methods

```rust
LdapAuthMethod::Simple {
    bind_dn: String,          // cn=admin,dc=example,dc=com
    password: String,         // NEVER logged or exposed
}

LdapAuthMethod::SaslPlain {
    identity: String,         // authzId format
    password: String,
}

LdapAuthMethod::SaslExternal {
    // TLS client certificate; no password
}

LdapAuthMethod::Anonymous {
    // No credentials; most servers restrict this
}
```

## Response Model

All operations return `LdapResponse<T>` (read/query) or `LdapEmptyResponse` (mutation):

```rust
LdapResponse<T> {
    data: T,
    backend_used: LdapBackendUsed,
}

LdapEmptyResponse {
    backend_used: LdapBackendUsed,
}
```

### Backend Metadata

```rust
LdapBackendUsed {
    server_type: ServerType,     // OpenLDAP | 389DS | ActiveDirectory | Unknown
    tls_mode_used: TlsMode,
    auth_method_used: String,    // "simple" | "sasl_plain" | "sasl_external" | "anonymous"
    connection_time_ms: u64,
}
```

## Supported Workflows

### v0.8.0 — LDAP Foundation

v0.8.0 introduces read-only and basic mutation workflows required for user/group lookup and provisioning.

```rust
// Connection & Authentication
bind() -> Result<LdapResponse<()>>
bind_as(user_dn: &str, password: &str) -> Result<LdapResponse<()>>

// Search (Read)
search(
    base_dn: &str,
    filter: &str,
    scope: SearchScope,
    attributes: Option<&[&str]>,
    size_limit: Option<usize>,
    time_limit: Option<Duration>,
) -> Result<LdapResponse<Vec<LdapEntry>>>

search_paged(
    base_dn: &str,
    filter: &str,
    attributes: Option<&[&str]>,
) -> Result<LdapResponse<PagedSearchIter>>  // Iterator for large result sets

get_entry(dn: &str) -> Result<LdapResponse<Option<LdapEntry>>>

// User Helpers (convenience patterns from guide)
find_user_by_uid(uid: &str) -> Result<LdapResponse<Option<LdapEntry>>>
find_user_by_mail(mail: &str) -> Result<LdapResponse<Vec<LdapEntry>>>
find_users_in_group(group_dn: &str) -> Result<LdapResponse<Vec<LdapEntry>>>

// Group Helpers
find_group_by_cn(cn: &str) -> Result<LdapResponse<Option<LdapEntry>>>
find_groups_for_user(user_dn: &str) -> Result<LdapResponse<Vec<LdapEntry>>>
get_group_members(group_dn: &str) -> Result<LdapResponse<Vec<String>>>  // Member DNs

// Mutations (Add, Modify, Delete)
add_entry(dn: &str, attributes: &[(&str, &[&str])]) -> Result<LdapEmptyResponse>

modify_entry(
    dn: &str,
    changes: &[ModifyOp],  // add | replace | delete
) -> Result<LdapEmptyResponse>

delete_entry(dn: &str) -> Result<LdapEmptyResponse>

// Password Operations
change_password(
    user_dn: &str,
    old_password: &str,
    new_password: &str,
) -> Result<LdapEmptyResponse>

set_password(
    user_dn: &str,
    new_password: &str,
) -> Result<LdapEmptyResponse>  // Admin operation

// Entry Rename/Move
rename_entry(
    old_dn: &str,
    new_rdn: &str,
    new_superior: Option<&str>,
) -> Result<LdapEmptyResponse>
```

## Type Model

### Entry & Attributes

```rust
LdapEntry {
    dn: String,
    attributes: HashMap<String, LdapAttribute>,
}

LdapAttribute {
    name: String,
    values: Vec<Vec<u8>>,  // Binary-safe; decode as UTF-8 where applicable
}

// Convenience accessor
impl LdapEntry {
    pub fn get_string(&self, attr: &str) -> Option<String>
    pub fn get_strings(&self, attr: &str) -> Option<Vec<String>>
    pub fn get_bytes(&self, attr: &str) -> Option<Vec<u8>>
    pub fn get_all_bytes(&self, attr: &str) -> Option<Vec<Vec<u8>>>
}
```

### Search Options

```rust
SearchOptions {
    base_dn: String,
    filter: String,                  // RFC 4515 syntax
    scope: SearchScope,              // base | onelevel | subtree
    attributes: Option<Vec<String>>, // Leave empty for all user attributes
    size_limit: Option<usize>,       // Server-side limit
    time_limit: Option<Duration>,    // Server-side timeout
    paged_size: Option<usize>,       // Paged result page size; default 500
    types_only: bool,                // Return attribute names without values
}

enum SearchScope {
    Base,      // Only the base entry
    OneLevel,  // Immediate children only
    Subtree,   // Base and all descendants (default)
}
```

### Modify Operations

```rust
enum ModifyOp {
    Add {
        attr: String,
        values: Vec<Vec<u8>>,
    },
    Replace {
        attr: String,
        values: Vec<Vec<u8>>,
    },
    Delete {
        attr: String,
        values: Option<Vec<Vec<u8>>>,  // None = delete entire attribute
    },
}
```

## Error Handling

Service LDAP errors should use domain-specific `VeltrixError` variants:

- `Parsing(String)` — LDAP response parsing failures, schema violations
- `Service { service: String, reason: String }` — LDAP protocol errors (error codes 1, 2, 3, 4, 32, 49, 50, etc.)
- `Socket { reason: String }` — Connection, TLS, or socket-level failures
- `Http { status: u16, reason: String }` — Non-2xx HTTP for gateway-fronted LDAP
- `Auth { reason: String }` — Invalid credentials, TLS client cert failures, insufficient access
- `Validation { field: String, reason: String }` — Invalid DN, malformed filter, missing required params

### Security Rule

Never log:
- Raw passwords or bind credentials
- Full distinguished names in error context (truncate sensitive paths)
- Authorization headers or session tokens
- Hashed passwords even if server returns them

## Feature Flags

| Flag | Enables |
|------|---------|
| `ldap` | OpenLDAP/389DS/AD support (required for this service) |
| `ldap-sasl` | SASL mechanisms (PLAIN, EXTERNAL, GSSAPI); default: PLAIN only |
| `ldap-gssapi` | SASL/GSSAPI (Kerberos); implies `ldap-sasl` |

## Security Considerations

### Transport

1. **Always use TLS in production.** Never bind over plaintext LDAP.
   - Prefer `StartTLS` (RFC 4513 recommendation)
   - Accept `LDAPS` (port 636, implicit TLS)
   - Verify server certificates against trusted CA

2. **Client certificates** (SASL/EXTERNAL):
   - Load from secure storage (never hardcoded)
   - Set file permissions to `0600` (private key)

3. **Certificate validation:**
   - Always set `verify_certificate: true` in production
   - Use `ca_certificate` to specify trusted roots
   - Do not skip hostname verification

### Authentication

1. **Simple bind:**
   - Only over TLS/LDAPS
   - Never pass passwords on CLI or in process environment
   - Use in-memory or secure file-based storage

2. **Service accounts:**
   - Create dedicated bind DNs per application
   - Grant minimal permissions via ACLs
   - Rotate credentials regularly

3. **User authentication (read-verify-bind pattern):**
   - Bind as service account with read-only access
   - Look up user by uid/mail
   - Perform second bind as the user with supplied password
   - If both succeed, user is authenticated

### Access Control

1. **ACLs:** Restrict access to sensitive attributes (userPassword, mail, phone numbers).
2. **Anonymous bind:** Disable unless explicitly required.
3. **Audit logging:** Enable LDAP server logging for compliance.

## API Stability

LDAP contract is pinned to RFC 4511 (LDAP v3) and common extensions:

- RFC 2849 (LDIF)
- RFC 2891 (Server-side sorting)
- RFC 3673 (LDAP ALL operational attributes)
- RFC 3876 (Syntax registration for LDAP/X.500 schema)
- RFC 4533 (LDAP Sync, SyncRepl; read-only support)

Changes to upstream LDAP server API support (e.g., new object classes, custom extensions) should be added as new methods or optional fields in response types, not breaking changes to existing APIs.

## Documentation Cross-References

- **Developer Tool Usage Guide — LDAP:** Complete command reference, workflows, and troubleshooting
- **docs/api/contract/services.md:** Response model conventions
- **AGENTS.md:** Error handling patterns and service integration design

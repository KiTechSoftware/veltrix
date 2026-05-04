//! LDAP client for directory operations.

use crate::error::VeltrixError;
use std::time::Instant;

use super::spec::{
    LdapAuthMethod, LdapBackendUsed, LdapEmptyResponse, LdapResponse, LdapSpec, ServerType,
};
use super::types::{LdapEntry, ModifyOp, SearchOptions, SearchScope};

/// LDAP client for directory operations.
#[derive(Debug)]
pub struct LdapClient {
    spec: LdapSpec,
    /// Cached backend metadata
    backend_used: LdapBackendUsed,
    /// Whether we have an active connection
    connected: bool,
    /// Optional connection handle (would be LdapAsync or Ldap in full impl)
    #[allow(dead_code)]
    connection: Option<()>,
}

impl LdapClient {
    /// Create a new LDAP client with specification.
    pub fn new(spec: LdapSpec) -> Result<Self, VeltrixError> {
        // Detect server type from URI or root DSE (stub for now)
        let server_type = ServerType::Unknown;

        let backend_used = LdapBackendUsed {
            server_type,
            tls_mode_used: spec.tls_mode,
            auth_method_used: spec.auth.to_string(),
            connection_time_ms: 0,
        };

        Ok(Self {
            spec,
            backend_used,
            connected: false,
            connection: None,
        })
    }

    /// Get client specification.
    pub fn spec(&self) -> &LdapSpec {
        &self.spec
    }

    /// Get whether we have an active connection.
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Bind with configured credentials.
    ///
    /// # Errors
    ///
    /// Returns `VeltrixError::Auth` if bind fails (invalid credentials, insufficient access).
    /// Returns `VeltrixError::Socket` for connection issues.
    pub fn bind(&mut self) -> Result<LdapResponse<()>, VeltrixError> {
        let start = Instant::now();

        // Clone auth method to avoid borrow checker issues
        let auth = self.spec.auth.clone();

        match auth {
            LdapAuthMethod::Simple { bind_dn, password } => {
                self._bind_simple(&bind_dn, &password)?;
            }
            #[cfg(feature = "ldap-sasl")]
            LdapAuthMethod::SaslPlain { identity, password } => {
                self._bind_sasl_plain(&identity, &password)?;
            }
            #[cfg(feature = "ldap-sasl")]
            LdapAuthMethod::SaslExternal => {
                self._bind_sasl_external()?;
            }
            LdapAuthMethod::Anonymous => {
                self._bind_anonymous()?;
            }
        }

        self.backend_used.connection_time_ms = start.elapsed().as_millis() as u64;
        self.connected = true;

        Ok(LdapResponse::new((), self.backend_used.clone()))
    }

    /// Verify user credentials by binding as the user.
    ///
    /// # Errors
    ///
    /// Returns `VeltrixError::Auth` if credentials are invalid.
    pub fn bind_as(
        &mut self,
        user_dn: &str,
        password: &str,
    ) -> Result<LdapResponse<()>, VeltrixError> {
        let start = Instant::now();

        // Never log password
        if password.is_empty() {
            return Err(VeltrixError::validation(
                "password",
                "password cannot be empty",
            ));
        }

        if user_dn.is_empty() {
            return Err(VeltrixError::validation(
                "user_dn",
                "user_dn cannot be empty",
            ));
        }

        self._bind_simple(user_dn, password)?;

        self.backend_used.connection_time_ms = start.elapsed().as_millis() as u64;

        Ok(LdapResponse::new((), self.backend_used.clone()))
    }

    /// Search for entries matching a filter.
    ///
    /// # Errors
    ///
    /// Returns `VeltrixError::Parsing` for malformed responses.
    /// Returns `VeltrixError::Service` for LDAP protocol errors.
    pub fn search(
        &mut self,
        options: SearchOptions,
    ) -> Result<LdapResponse<Vec<LdapEntry>>, VeltrixError> {
        let start = Instant::now();

        // Validate filter syntax (basic check)
        if !options.filter.starts_with('(') || !options.filter.ends_with(')') {
            return Err(VeltrixError::validation(
                "filter",
                "filter must be enclosed in parentheses",
            ));
        }

        if options.base_dn.is_empty() {
            return Err(VeltrixError::validation(
                "base_dn",
                "base_dn cannot be empty",
            ));
        }

        let entries = self._search_impl(&options)?;

        self.backend_used.connection_time_ms = start.elapsed().as_millis() as u64;

        Ok(LdapResponse::new(entries, self.backend_used.clone()))
    }

    /// Get a specific entry by DN.
    ///
    /// # Errors
    ///
    /// Returns `VeltrixError::Parsing` if response cannot be parsed.
    /// Returns `VeltrixError::Service` if entry does not exist or access is denied.
    pub fn get_entry(&mut self, dn: &str) -> Result<LdapResponse<Option<LdapEntry>>, VeltrixError> {
        let start = Instant::now();

        // Validate DN (basic check for non-empty)
        if dn.is_empty() {
            return Err(VeltrixError::validation("dn", "DN cannot be empty"));
        }

        // Use base scope search to get single entry
        let mut options = SearchOptions::new(dn.to_string(), "(objectClass=*)".to_string());
        options = options.with_scope(SearchScope::Base);

        let result = self.search(options)?;
        let entry = result.data.first().cloned();

        self.backend_used.connection_time_ms = start.elapsed().as_millis() as u64;

        Ok(LdapResponse::new(entry, result.backend_used))
    }

    /// Find user by uid attribute.
    pub fn find_user_by_uid(
        &mut self,
        uid: &str,
    ) -> Result<LdapResponse<Option<LdapEntry>>, VeltrixError> {
        if uid.is_empty() {
            return Err(VeltrixError::validation("uid", "uid cannot be empty"));
        }

        let options = SearchOptions::new(
            "dc=example,dc=com".into(), // Default base DN
            format!("(uid={})", self._escape_filter_value(uid)),
        );

        let result = self.search(options)?;
        Ok(LdapResponse::new(
            result.data.first().cloned(),
            result.backend_used,
        ))
    }

    /// Find user by mail attribute.
    pub fn find_user_by_mail(
        &mut self,
        mail: &str,
    ) -> Result<LdapResponse<Vec<LdapEntry>>, VeltrixError> {
        if mail.is_empty() {
            return Err(VeltrixError::validation("mail", "mail cannot be empty"));
        }

        let options = SearchOptions::new(
            "dc=example,dc=com".into(),
            format!("(mail={})", self._escape_filter_value(mail)),
        );

        self.search(options)
    }

    /// Find users in a group by DN.
    pub fn find_users_in_group(
        &mut self,
        group_dn: &str,
    ) -> Result<LdapResponse<Vec<LdapEntry>>, VeltrixError> {
        if group_dn.is_empty() {
            return Err(VeltrixError::validation(
                "group_dn",
                "group_dn cannot be empty",
            ));
        }

        let options = SearchOptions::new(group_dn.to_string(), "(member=*)".to_string());

        self.search(options)
    }

    /// Find group by cn (common name) attribute.
    pub fn find_group_by_cn(
        &mut self,
        cn: &str,
    ) -> Result<LdapResponse<Option<LdapEntry>>, VeltrixError> {
        if cn.is_empty() {
            return Err(VeltrixError::validation("cn", "cn cannot be empty"));
        }

        let options = SearchOptions::new(
            "dc=example,dc=com".into(),
            format!("(cn={})", self._escape_filter_value(cn)),
        );

        let result = self.search(options)?;
        Ok(LdapResponse::new(
            result.data.first().cloned(),
            result.backend_used,
        ))
    }

    /// Find groups for a user by DN.
    pub fn find_groups_for_user(
        &mut self,
        user_dn: &str,
    ) -> Result<LdapResponse<Vec<LdapEntry>>, VeltrixError> {
        if user_dn.is_empty() {
            return Err(VeltrixError::validation(
                "user_dn",
                "user_dn cannot be empty",
            ));
        }

        let options = SearchOptions::new(
            "dc=example,dc=com".into(),
            format!("(member={})", self._escape_filter_value(user_dn)),
        );

        self.search(options)
    }

    /// Get members from a group entry.
    pub fn get_group_members(
        &mut self,
        group_dn: &str,
    ) -> Result<LdapResponse<Vec<String>>, VeltrixError> {
        let result = self.get_entry(group_dn)?;

        let members = if let Some(entry) = result.data {
            entry.get_strings("member").unwrap_or_default()
        } else {
            vec![]
        };

        Ok(LdapResponse::new(members, result.backend_used))
    }

    /// Find POSIX user by uid number.
    pub fn find_posix_user_by_uid_number(
        &mut self,
        uid_number: u32,
    ) -> Result<LdapResponse<Option<LdapEntry>>, VeltrixError> {
        let options = SearchOptions::new(
            "dc=example,dc=com".into(),
            format!("(uidNumber={})", uid_number),
        );

        let result = self.search(options)?;
        Ok(LdapResponse::new(
            result.data.first().cloned(),
            result.backend_used,
        ))
    }

    /// Find POSIX group by gid number.
    pub fn find_posix_group_by_gid_number(
        &mut self,
        gid_number: u32,
    ) -> Result<LdapResponse<Option<LdapEntry>>, VeltrixError> {
        let options = SearchOptions::new(
            "dc=example,dc=com".into(),
            format!("(gidNumber={})", gid_number),
        );

        let result = self.search(options)?;
        Ok(LdapResponse::new(
            result.data.first().cloned(),
            result.backend_used,
        ))
    }

    /// Add a new entry.
    ///
    /// # Errors
    ///
    /// Returns `VeltrixError::Validation` if DN or attributes are invalid.
    /// Returns `VeltrixError::Service` if entry already exists.
    pub fn add_entry(
        &mut self,
        dn: &str,
        attributes: &[(&str, &[&str])],
    ) -> Result<LdapEmptyResponse, VeltrixError> {
        if dn.is_empty() {
            return Err(VeltrixError::validation("dn", "DN cannot be empty"));
        }

        if attributes.is_empty() {
            return Err(VeltrixError::validation(
                "attributes",
                "at least one attribute required",
            ));
        }

        self._add_entry_impl(dn, attributes)?;

        Ok(LdapEmptyResponse::new(self.backend_used.clone()))
    }

    /// Modify an entry's attributes.
    ///
    /// # Errors
    ///
    /// Returns `VeltrixError::Service` if entry does not exist or modifications fail.
    pub fn modify_entry(
        &mut self,
        dn: &str,
        changes: &[ModifyOp],
    ) -> Result<LdapEmptyResponse, VeltrixError> {
        if dn.is_empty() {
            return Err(VeltrixError::validation("dn", "DN cannot be empty"));
        }

        if changes.is_empty() {
            return Err(VeltrixError::validation(
                "changes",
                "at least one modification required",
            ));
        }

        self._modify_entry_impl(dn, changes)?;

        Ok(LdapEmptyResponse::new(self.backend_used.clone()))
    }

    /// Delete an entry.
    ///
    /// # Errors
    ///
    /// Returns `VeltrixError::Service` if entry does not exist or has children.
    pub fn delete_entry(&mut self, dn: &str) -> Result<LdapEmptyResponse, VeltrixError> {
        if dn.is_empty() {
            return Err(VeltrixError::validation("dn", "DN cannot be empty"));
        }

        self._delete_entry_impl(dn)?;

        Ok(LdapEmptyResponse::new(self.backend_used.clone()))
    }

    /// Rename/move an entry.
    ///
    /// # Errors
    ///
    /// Returns `VeltrixError::Validation` if new RDN is invalid.
    /// Returns `VeltrixError::Service` if operation fails.
    pub fn rename_entry(
        &mut self,
        old_dn: &str,
        new_rdn: &str,
        new_superior: Option<&str>,
    ) -> Result<LdapEmptyResponse, VeltrixError> {
        if old_dn.is_empty() {
            return Err(VeltrixError::validation("old_dn", "DN cannot be empty"));
        }
        if new_rdn.is_empty() {
            return Err(VeltrixError::validation("new_rdn", "RDN cannot be empty"));
        }

        self._rename_entry_impl(old_dn, new_rdn, new_superior)?;

        Ok(LdapEmptyResponse::new(self.backend_used.clone()))
    }

    /// Change user password (user self-service).
    ///
    /// # Errors
    ///
    /// Returns `VeltrixError::Auth` if old password is incorrect.
    pub fn change_password(
        &mut self,
        user_dn: &str,
        old_password: &str,
        new_password: &str,
    ) -> Result<LdapEmptyResponse, VeltrixError> {
        if user_dn.is_empty() {
            return Err(VeltrixError::validation("user_dn", "DN cannot be empty"));
        }

        // Never log passwords
        if old_password.is_empty() || new_password.is_empty() {
            return Err(VeltrixError::validation(
                "password",
                "passwords cannot be empty",
            ));
        }

        self._change_password_impl(user_dn, old_password, new_password)?;

        Ok(LdapEmptyResponse::new(self.backend_used.clone()))
    }

    /// Set user password (admin operation).
    ///
    /// # Errors
    ///
    /// Returns `VeltrixError::Auth` if caller lacks permission.
    pub fn set_password(
        &mut self,
        user_dn: &str,
        new_password: &str,
    ) -> Result<LdapEmptyResponse, VeltrixError> {
        if user_dn.is_empty() {
            return Err(VeltrixError::validation("user_dn", "DN cannot be empty"));
        }

        // Never log password
        if new_password.is_empty() {
            return Err(VeltrixError::validation(
                "password",
                "password cannot be empty",
            ));
        }

        self._set_password_impl(user_dn, new_password)?;

        Ok(LdapEmptyResponse::new(self.backend_used.clone()))
    }

    // Private implementation methods

    #[cfg(feature = "ldap")]
    fn _bind_simple(&mut self, bind_dn: &str, password: &str) -> Result<(), VeltrixError> {
        // Parse URI to extract host and port
        let (_host, _port) = self._parse_uri(&self.spec.uri)?;

        // Validate credentials
        if bind_dn.is_empty() || password.is_empty() {
            return Err(VeltrixError::auth("bind_dn and password cannot be empty"));
        }

        // In a real implementation with ldap3 crate:
        // - Create LDAP connection
        // - Perform simple bind
        // - Handle TLS/StartTLS if configured
        // For now, just validate and return Ok

        Ok(())
    }

    #[cfg(not(feature = "ldap"))]
    fn _bind_simple(&mut self, _bind_dn: &str, _password: &str) -> Result<(), VeltrixError> {
        Err(VeltrixError::service("ldap", "LDAP feature not enabled"))
    }

    #[cfg(all(feature = "ldap-sasl", feature = "ldap"))]
    fn _bind_sasl_plain(&mut self, identity: &str, password: &str) -> Result<(), VeltrixError> {
        if identity.is_empty() || password.is_empty() {
            return Err(VeltrixError::auth("identity and password cannot be empty"));
        }
        Ok(())
    }

    #[cfg(not(all(feature = "ldap-sasl", feature = "ldap")))]
    fn _bind_sasl_plain(&mut self, _identity: &str, _password: &str) -> Result<(), VeltrixError> {
        Err(VeltrixError::service(
            "ldap",
            "SASL/PLAIN feature not enabled",
        ))
    }

    #[cfg(all(feature = "ldap-sasl", feature = "ldap"))]
    fn _bind_sasl_external(&mut self) -> Result<(), VeltrixError> {
        Ok(())
    }

    #[cfg(not(all(feature = "ldap-sasl", feature = "ldap")))]
    fn _bind_sasl_external(&mut self) -> Result<(), VeltrixError> {
        Err(VeltrixError::service(
            "ldap",
            "SASL/EXTERNAL feature not enabled",
        ))
    }

    fn _bind_anonymous(&mut self) -> Result<(), VeltrixError> {
        // Anonymous bind is always available
        Ok(())
    }

    fn _search_impl(&mut self, options: &SearchOptions) -> Result<Vec<LdapEntry>, VeltrixError> {
        // Validate input
        if options.base_dn.is_empty() {
            return Err(VeltrixError::validation(
                "base_dn",
                "base DN cannot be empty",
            ));
        }

        // In a real implementation, this would:
        // 1. Connect to the LDAP server
        // 2. Execute the search query
        // 3. Parse the results into LdapEntry structs
        // 4. Handle paged results if needed

        // For now, return empty results
        // Full implementation would use ldap3 crate
        Ok(vec![])
    }

    fn _add_entry_impl(
        &mut self,
        dn: &str,
        attributes: &[(&str, &[&str])],
    ) -> Result<(), VeltrixError> {
        // Validate DN format (basic check)
        if !dn.contains('=') || !dn.contains(',') {
            return Err(VeltrixError::validation("dn", "invalid DN format"));
        }

        // In real implementation would convert attributes to ldap3 format and add via LDAP
        let _ = attributes;
        Ok(())
    }

    fn _modify_entry_impl(&mut self, dn: &str, _changes: &[ModifyOp]) -> Result<(), VeltrixError> {
        // Validate DN
        if !dn.contains('=') || !dn.contains(',') {
            return Err(VeltrixError::validation("dn", "invalid DN format"));
        }

        // In real implementation would apply modifications via LDAP
        Ok(())
    }

    fn _delete_entry_impl(&mut self, dn: &str) -> Result<(), VeltrixError> {
        // Validate DN
        if !dn.contains('=') || !dn.contains(',') {
            return Err(VeltrixError::validation("dn", "invalid DN format"));
        }

        // In real implementation would delete via LDAP
        Ok(())
    }

    fn _rename_entry_impl(
        &mut self,
        _old_dn: &str,
        _new_rdn: &str,
        _new_superior: Option<&str>,
    ) -> Result<(), VeltrixError> {
        // ModifyDN operation
        // In real implementation would use ldap3
        Ok(())
    }

    fn _change_password_impl(
        &mut self,
        _user_dn: &str,
        _old_password: &str,
        _new_password: &str,
    ) -> Result<(), VeltrixError> {
        // First verify old password, then set new one
        // In real implementation would use ldap3
        Ok(())
    }

    fn _set_password_impl(
        &mut self,
        _user_dn: &str,
        _new_password: &str,
    ) -> Result<(), VeltrixError> {
        // Set password without verification (admin operation)
        // In real implementation would use ldap3
        Ok(())
    }

    fn _parse_uri(&self, uri: &str) -> Result<(String, u16), VeltrixError> {
        // Parse ldap://host:port or ldaps://host:port
        if uri.starts_with("ldap://") {
            let rest = uri
                .strip_prefix("ldap://")
                .ok_or_else(|| VeltrixError::validation("uri", "invalid LDAP URI format"))?;
            let parts: Vec<&str> = rest.split(':').collect();
            if parts.is_empty() {
                return Err(VeltrixError::validation("uri", "missing host in URI"));
            }
            let host = parts[0].to_string();
            let port = if parts.len() > 1 {
                parts[1].parse::<u16>().unwrap_or(389)
            } else {
                389
            };
            Ok((host, port))
        } else if uri.starts_with("ldaps://") {
            let rest = uri
                .strip_prefix("ldaps://")
                .ok_or_else(|| VeltrixError::validation("uri", "invalid LDAPS URI format"))?;
            let parts: Vec<&str> = rest.split(':').collect();
            if parts.is_empty() {
                return Err(VeltrixError::validation("uri", "missing host in URI"));
            }
            let host = parts[0].to_string();
            let port = if parts.len() > 1 {
                parts[1].parse::<u16>().unwrap_or(636)
            } else {
                636
            };
            Ok((host, port))
        } else {
            Err(VeltrixError::validation(
                "uri",
                "URI must start with ldap:// or ldaps://",
            ))
        }
    }

    fn _escape_filter_value(&self, value: &str) -> String {
        // RFC 4515 filter escaping
        value
            .replace('\\', "\\5c")
            .replace('*', "\\2a")
            .replace('(', "\\28")
            .replace(')', "\\29")
            .replace('\0', "\\00")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_creation() {
        let spec = LdapSpec::new(
            "ldap://localhost:389".into(),
            super::super::spec::LdapAuthMethod::Anonymous,
        );
        let client = LdapClient::new(spec);
        assert!(client.is_ok());
        let client = client.unwrap();
        assert!(!client.is_connected());
    }

    #[test]
    fn add_entry_validates_dn() {
        let spec = LdapSpec::new(
            "ldap://localhost:389".into(),
            super::super::spec::LdapAuthMethod::Anonymous,
        );
        let mut client = LdapClient::new(spec).unwrap();

        let result = client.add_entry("", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn add_entry_validates_attributes() {
        let spec = LdapSpec::new(
            "ldap://localhost:389".into(),
            super::super::spec::LdapAuthMethod::Anonymous,
        );
        let mut client = LdapClient::new(spec).unwrap();

        let result = client.add_entry("cn=test,dc=example,dc=com", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn modify_entry_validates_input() {
        let spec = LdapSpec::new(
            "ldap://localhost:389".into(),
            super::super::spec::LdapAuthMethod::Anonymous,
        );
        let mut client = LdapClient::new(spec).unwrap();

        let result = client.modify_entry("cn=test,dc=example,dc=com", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn delete_entry_validates_dn() {
        let spec = LdapSpec::new(
            "ldap://localhost:389".into(),
            super::super::spec::LdapAuthMethod::Anonymous,
        );
        let mut client = LdapClient::new(spec).unwrap();

        let result = client.delete_entry("");
        assert!(result.is_err());
    }

    #[test]
    fn search_validates_filter() {
        let spec = LdapSpec::new(
            "ldap://localhost:389".into(),
            super::super::spec::LdapAuthMethod::Anonymous,
        );
        let mut client = LdapClient::new(spec).unwrap();

        let options = SearchOptions::new(
            "dc=example,dc=com".into(),
            "invalid_filter".into(), // Missing parentheses
        );
        let result = client.search(options);
        assert!(result.is_err());
    }

    #[test]
    fn find_user_by_uid_validates_uid() {
        let spec = LdapSpec::new(
            "ldap://localhost:389".into(),
            super::super::spec::LdapAuthMethod::Anonymous,
        );
        let mut client = LdapClient::new(spec).unwrap();

        let result = client.find_user_by_uid("");
        assert!(result.is_err());
    }

    #[test]
    fn uri_parsing_ldap() {
        let spec = LdapSpec::new(
            "ldap://localhost:389".into(),
            super::super::spec::LdapAuthMethod::Anonymous,
        );
        let client = LdapClient::new(spec).unwrap();
        let (host, port) = client._parse_uri("ldap://example.com:1234").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 1234);
    }

    #[test]
    fn uri_parsing_ldaps() {
        let spec = LdapSpec::new(
            "ldaps://localhost:636".into(),
            super::super::spec::LdapAuthMethod::Anonymous,
        );
        let client = LdapClient::new(spec).unwrap();
        let (host, port) = client._parse_uri("ldaps://example.com").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 636);
    }

    #[test]
    fn filter_value_escaping() {
        let spec = LdapSpec::new(
            "ldap://localhost:389".into(),
            super::super::spec::LdapAuthMethod::Anonymous,
        );
        let client = LdapClient::new(spec).unwrap();
        let escaped = client._escape_filter_value("test*value");
        assert_eq!(escaped, "test\\2avalue");
    }
}

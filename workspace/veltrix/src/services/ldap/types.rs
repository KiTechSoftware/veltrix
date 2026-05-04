//! LDAP data types and models.

use std::collections::HashMap;
use std::time::Duration;

/// LDAP search scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchScope {
    /// Only the base entry itself.
    Base,
    /// Immediate children only.
    OneLevel,
    /// Base entry and all descendants (default).
    Subtree,
}

/// LDAP entry (DN + attributes).
#[derive(Debug, Clone)]
pub struct LdapEntry {
    /// Distinguished Name
    pub dn: String,
    /// Attributes (case-insensitive key, multi-valued)
    pub attributes: HashMap<String, LdapAttribute>,
}

impl LdapEntry {
    /// Create a new entry with DN and empty attributes.
    pub fn new(dn: String) -> Self {
        Self {
            dn,
            attributes: HashMap::new(),
        }
    }

    /// Get first value of an attribute as UTF-8 string.
    pub fn get_string(&self, attr: &str) -> Option<String> {
        let key = attr.to_lowercase();
        self.attributes.get(&key).and_then(|attr| {
            attr.values
                .first()
                .and_then(|v| String::from_utf8(v.clone()).ok())
        })
    }

    /// Get all values of an attribute as UTF-8 strings.
    pub fn get_strings(&self, attr: &str) -> Option<Vec<String>> {
        let key = attr.to_lowercase();
        self.attributes.get(&key).map(|attr| {
            attr.values
                .iter()
                .filter_map(|v| String::from_utf8(v.clone()).ok())
                .collect()
        })
    }

    /// Get first value of an attribute as bytes.
    pub fn get_bytes(&self, attr: &str) -> Option<Vec<u8>> {
        let key = attr.to_lowercase();
        self.attributes
            .get(&key)
            .and_then(|attr| attr.values.first().cloned())
    }

    /// Get all values of an attribute as bytes.
    pub fn get_all_bytes(&self, attr: &str) -> Option<Vec<Vec<u8>>> {
        let key = attr.to_lowercase();
        self.attributes.get(&key).map(|attr| attr.values.clone())
    }
}

/// LDAP attribute (name + multi-valued data).
#[derive(Debug, Clone)]
pub struct LdapAttribute {
    /// Attribute name (case-insensitive)
    pub name: String,
    /// Binary-safe values
    pub values: Vec<Vec<u8>>,
}

impl LdapAttribute {
    /// Create a new attribute with name and values.
    pub fn new(name: String, values: Vec<Vec<u8>>) -> Self {
        Self {
            name: name.to_lowercase(),
            values,
        }
    }

    /// Create an attribute from string values.
    pub fn from_strings(name: String, values: Vec<String>) -> Self {
        Self {
            name: name.to_lowercase(),
            values: values.into_iter().map(|s| s.into_bytes()).collect(),
        }
    }
}

/// Search filter options.
#[derive(Debug, Clone)]
pub struct SearchOptions {
    /// Base DN for search
    pub base_dn: String,
    /// RFC 4515 filter (e.g., "(uid=jdoe)")
    pub filter: String,
    /// Search scope
    pub scope: SearchScope,
    /// Attributes to return (empty for all user attributes)
    pub attributes: Option<Vec<String>>,
    /// Server-side result size limit
    pub size_limit: Option<usize>,
    /// Server-side operation timeout
    pub time_limit: Option<Duration>,
    /// Paged result page size
    pub paged_size: Option<usize>,
    /// Return attribute names only (no values)
    pub types_only: bool,
}

impl SearchOptions {
    /// Create new search options with base DN and filter.
    pub fn new(base_dn: String, filter: String) -> Self {
        Self {
            base_dn,
            filter,
            scope: SearchScope::Subtree,
            attributes: None,
            size_limit: None,
            time_limit: None,
            paged_size: Some(500),
            types_only: false,
        }
    }

    /// Set search scope.
    pub fn with_scope(mut self, scope: SearchScope) -> Self {
        self.scope = scope;
        self
    }

    /// Set attributes to return.
    pub fn with_attributes(mut self, attributes: Vec<String>) -> Self {
        self.attributes = Some(attributes);
        self
    }

    /// Set size limit.
    pub fn with_size_limit(mut self, limit: usize) -> Self {
        self.size_limit = Some(limit);
        self
    }

    /// Set time limit.
    pub fn with_time_limit(mut self, timeout: Duration) -> Self {
        self.time_limit = Some(timeout);
        self
    }

    /// Set paged result page size.
    pub fn with_page_size(mut self, size: usize) -> Self {
        self.paged_size = Some(size);
        self
    }

    /// Set types-only (attribute names without values).
    pub fn with_types_only(mut self, types_only: bool) -> Self {
        self.types_only = types_only;
        self
    }
}

/// Modify operation on an LDAP entry.
#[derive(Debug, Clone)]
pub enum ModifyOp {
    /// Add values to attribute
    Add {
        /// Attribute name
        attr: String,
        /// Values to add (binary-safe)
        values: Vec<Vec<u8>>,
    },
    /// Replace attribute values
    Replace {
        /// Attribute name
        attr: String,
        /// New values (replaces all existing)
        values: Vec<Vec<u8>>,
    },
    /// Delete attribute or specific values
    Delete {
        /// Attribute name
        attr: String,
        /// Values to delete (None = delete entire attribute)
        values: Option<Vec<Vec<u8>>>,
    },
}

impl ModifyOp {
    /// Create an Add operation from string values.
    pub fn add_strings(attr: String, values: Vec<String>) -> Self {
        Self::Add {
            attr,
            values: values.into_iter().map(|s| s.into_bytes()).collect(),
        }
    }

    /// Create a Replace operation from string values.
    pub fn replace_strings(attr: String, values: Vec<String>) -> Self {
        Self::Replace {
            attr,
            values: values.into_iter().map(|s| s.into_bytes()).collect(),
        }
    }

    /// Create a Delete operation for entire attribute.
    pub fn delete_attr(attr: String) -> Self {
        Self::Delete { attr, values: None }
    }

    /// Create a Delete operation for specific values.
    pub fn delete_values(attr: String, values: Vec<Vec<u8>>) -> Self {
        Self::Delete {
            attr,
            values: Some(values),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_get_string() {
        let mut entry = LdapEntry::new("cn=test,dc=example,dc=com".into());
        entry.attributes.insert(
            "cn".into(),
            LdapAttribute::from_strings("cn".into(), vec!["test".into()]),
        );

        assert_eq!(entry.get_string("cn"), Some("test".into()));
        assert_eq!(entry.get_string("mail"), None);
    }

    #[test]
    fn search_scope_operations() {
        let mut opts = SearchOptions::new("dc=example,dc=com".into(), "(uid=*)".into());
        assert_eq!(opts.scope, SearchScope::Subtree);

        opts = opts.with_scope(SearchScope::Base);
        assert_eq!(opts.scope, SearchScope::Base);
    }

    #[test]
    fn modify_operations() {
        let add = ModifyOp::add_strings("mail".into(), vec!["test@example.com".into()]);
        if let ModifyOp::Add { attr, values } = add {
            assert_eq!(attr, "mail");
            assert_eq!(values.len(), 1);
        } else {
            panic!("Expected Add operation");
        }

        let delete = ModifyOp::delete_attr("description".into());
        if let ModifyOp::Delete { attr, values } = delete {
            assert_eq!(attr, "description");
            assert_eq!(values, None);
        } else {
            panic!("Expected Delete operation");
        }
    }
}

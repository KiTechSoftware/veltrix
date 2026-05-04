//! LDAP service integration example.
//!
//! Demonstrates bind, search, user/group lookup, and entry mutation workflows.
//!
//! Run with: `cargo run --manifest-path workspace/Cargo.toml --example ldap_demo --features ldap`

#[cfg(feature = "ldap")]
fn main() -> veltrix::Result<()> {
    use std::time::Duration;
    use veltrix::services::ldap::{LdapAuthMethod, LdapClient, LdapSpec};

    println!("=== Veltrix LDAP Example ===\n");

    // Create LDAP specification with simple bind
    let spec = LdapSpec::new("ldap://localhost:389".into(), LdapAuthMethod::Anonymous)
        .with_connect_timeout(Duration::from_secs(5))
        .with_operation_timeout(Duration::from_secs(30));

    println!("Spec created:");
    println!("  URI: {}", spec.uri);
    println!("  TLS: {}", spec.tls_mode);
    println!("  Auth: {}", spec.auth);
    println!("  Connect timeout: {:?}", spec.connect_timeout);
    println!("  Operation timeout: {:?}", spec.operation_timeout);
    println!();

    // Create client
    let _client = LdapClient::new(spec)?;
    println!("Client created successfully\n");

    // Example: Bind
    println!("--- Bind Example ---");
    println!("Client is configured for anonymous bind");
    println!("In production, use:");
    println!("  LdapAuthMethod::Simple {{");
    println!("    bind_dn: \"cn=admin,dc=example,dc=com\".into(),");
    println!("    password: \"secret\".into(),");
    println!("  }}");
    println!();

    // Example: User lookup
    println!("--- User Lookup Example ---");
    println!("To find a user by uid:");
    println!("  let result = client.find_user_by_uid(\"jdoe\")?;");
    println!("  if let Some(entry) = result.data {{");
    println!("    println!(\"Found: {{}}\", entry.dn);");
    println!("    if let Some(mail) = entry.get_string(\"mail\") {{");
    println!("      println!(\"Email: {{}}\", mail);");
    println!("    }}");
    println!("  }}");
    println!();

    // Example: Search
    println!("--- Search Example ---");
    println!("To search for entries:");
    println!("  let options = SearchOptions::new(");
    println!("    \"dc=example,dc=com\".into(),");
    println!("    \"(objectClass=inetOrgPerson)\".into(),");
    println!("  ).with_scope(SearchScope::Subtree);");
    println!("  let results = client.search(options)?;");
    println!("  println!(\"Found {{}} users\", results.data.len());");
    println!();

    // Example: Group lookup
    println!("--- Group Lookup Example ---");
    println!("To find a group by cn:");
    println!("  let group = client.find_group_by_cn(\"developers\")?;");
    println!("  if let Some(entry) = group.data {{");
    println!("    if let Some(members) = entry.get_strings(\"member\") {{");
    println!("      println!(\"Group has {{}} members\", members.len());");
    println!("    }}");
    println!("  }}");
    println!();

    // Example: Add entry
    println!("--- Add Entry Example ---");
    println!("To create a new user:");
    println!("  let attributes = vec![");
    println!("    (\"objectClass\", vec![\"inetOrgPerson\"]),");
    println!("    (\"cn\", vec![\"Jane Doe\"]),");
    println!("    (\"sn\", vec![\"Doe\"]),");
    println!("    (\"uid\", vec![\"jdoe2\"]),");
    println!("    (\"mail\", vec![\"jane@example.com\"]),");
    println!("  ];");
    println!("  client.add_entry(\"uid=jdoe2,ou=Users,dc=example,dc=com\", &attributes)?;");
    println!();

    // Example: Modify entry
    println!("--- Modify Entry Example ---");
    println!("To update user attributes:");
    println!("  use veltrix::services::ldap::ModifyOp;");
    println!("  let changes = vec![");
    println!("    ModifyOp::replace_strings(");
    println!("      \"mail\".into(),");
    println!("      vec![\"jane.doe@example.com\".into()],");
    println!("    ),");
    println!("    ModifyOp::add_strings(");
    println!("      \"title\".into(),");
    println!("      vec![\"Senior Engineer\".into()],");
    println!("    ),");
    println!("  ];");
    println!("  client.modify_entry(\"uid=jdoe,ou=Users,dc=example,dc=com\", &changes)?;");
    println!();

    // Example: Delete entry
    println!("--- Delete Entry Example ---");
    println!("To remove an entry:");
    println!("  client.delete_entry(\"uid=jdoe2,ou=Users,dc=example,dc=com\")?;");
    println!();

    // Example: Password operations
    println!("--- Password Operations Example ---");
    println!("User self-service password change:");
    println!("  client.change_password(");
    println!("    \"uid=jdoe,ou=Users,dc=example,dc=com\",");
    println!("    \"old_password\",");
    println!("    \"new_password\",");
    println!("  )?;");
    println!();
    println!("Admin password reset:");
    println!("  client.set_password(");
    println!("    \"uid=jdoe,ou=Users,dc=example,dc=com\",");
    println!("    \"temp_password\",");
    println!("  )?;");
    println!();

    // Example: Response metadata
    println!("--- Response Metadata ---");
    println!("All responses include backend metadata:");
    println!("  - Server type (OpenLDAP, 389DS, Active Directory)");
    println!("  - TLS mode used");
    println!("  - Authentication method");
    println!("  - Connection time");
    println!();

    // Example: Error handling
    println!("--- Error Handling ---");
    println!("LDAP operations use domain-specific error types:");
    println!("  - VeltrixError::Auth — invalid credentials, insufficient access");
    println!("  - VeltrixError::Validation — invalid DN, malformed filter");
    println!("  - VeltrixError::Service — LDAP protocol errors");
    println!("  - VeltrixError::Socket — connection failures");
    println!("  - VeltrixError::Parsing — response parsing failures");
    println!();

    // Example: Security best practices
    println!("--- Security Best Practices ---");
    println!("1. Always use TLS in production");
    println!("   - Prefer StartTLS (port 389)");
    println!("   - Or LDAPS (port 636)");
    println!("2. Verify server certificates");
    println!("3. Use service accounts for app authentication");
    println!("4. Never log passwords or full DNs");
    println!("5. Implement read-verify-bind for user authentication");
    println!();

    println!("✓ LDAP example complete");
    Ok(())
}

#[cfg(not(feature = "ldap"))]
fn main() {
    eprintln!("This example requires the 'ldap' feature flag.");
    eprintln!(
        "Run: cargo run --manifest-path workspace/Cargo.toml --example ldap_demo --features ldap"
    );
    std::process::exit(1);
}

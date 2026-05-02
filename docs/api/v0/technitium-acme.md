# Technitium ACME DNS-01 Certificates

Veltrix v0.7.0 includes general ACME DNS-01 primitives for Technitium DNS
Server, plus first-class Caddy convenience helpers.

## Flow

1. Configure your ACME client for a hostname that uses DNS-01 validation.
2. Ask the ACME client or issuer for the validation token.
3. Use `TechnitiumClient::set_acme_challenge` to create the
   `_acme-challenge.<domain>` TXT record in the authoritative zone.
4. Wait for DNS propagation according to your environment.
5. Complete certificate issuance in your ACME client.
6. Use `TechnitiumClient::remove_acme_challenge` to remove the TXT record.

For Caddy-specific call sites, use `caddy_acme_challenge_name` when you want a
Caddy-oriented helper for the record name, then use the general
`set_acme_challenge` and `remove_acme_challenge` methods to manage Technitium.

## Example

```rust,no_run
use veltrix::Result;
use veltrix::services::technitium::{
    TechnitiumAuth, TechnitiumClient, TechnitiumHttpSpec, acme_challenge_name,
    caddy_acme_challenge_name,
};

async fn update_challenge(token: &str) -> Result<()> {
    let dns = TechnitiumClient::new(
        TechnitiumHttpSpec::new("http://localhost:5380")
            .auth(TechnitiumAuth::session_token("session-token")),
    )?;

    let zone = "example.test";
    let domain = "app.example.test";
    let record_name = acme_challenge_name(domain);
    let caddy_record_name = caddy_acme_challenge_name(domain);

    dns.set_acme_challenge(zone, domain, token, Some(60)).await?;
    println!("created TXT record: {record_name}");
    println!("Caddy helper resolves the same name: {caddy_record_name}");

    dns.remove_acme_challenge(zone, domain).await?;
    Ok(())
}
```

The helper methods only manage the Technitium TXT record. They do not automate the
ACME challenge lifecycle or poll DNS propagation; those cross-tool orchestration
workflows are tracked for v2.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Common Technitium HTTP API envelope.
#[derive(Debug, Clone, Deserialize)]
pub struct TechnitiumApiEnvelope<T> {
    #[serde(default)]
    pub status: Option<String>,

    #[serde(default)]
    pub response: Option<T>,

    #[serde(default)]
    pub error_message: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Login/session response shape.
#[derive(Debug, Clone, Deserialize)]
pub struct TechnitiumSession {
    #[serde(
        rename = "token",
        alias = "sessionToken",
        alias = "session_token",
        default
    )]
    pub token: Option<String>,

    #[serde(default)]
    pub user: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Server status payload.
#[derive(Debug, Clone, Deserialize)]
pub struct TechnitiumServerStatus {
    #[serde(default)]
    pub version: Option<String>,

    #[serde(default)]
    pub uptime: Option<Value>,

    #[serde(default)]
    pub dns_server_domain: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// DNS zone summary.
#[derive(Debug, Clone, Deserialize)]
pub struct TechnitiumZoneSummary {
    #[serde(rename = "name", alias = "zone", alias = "domain", default)]
    pub name: Option<String>,

    #[serde(rename = "type", alias = "zoneType", alias = "zone_type", default)]
    pub zone_type: Option<String>,

    #[serde(default)]
    pub disabled: Option<bool>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Explicit DNS record kinds targeted by the services contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TechnitiumRecordType {
    A,
    AAAA,
    CNAME,
    MX,
    TXT,
    NS,
    SRV,
    CAA,
    PTR,
}

/// Basic DNS record payload shared by read-only preview workflows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnitiumDnsRecord {
    pub name: String,

    #[serde(rename = "type")]
    pub record_type: TechnitiumRecordType,

    #[serde(default)]
    pub ttl: Option<u32>,

    #[serde(default)]
    pub value: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

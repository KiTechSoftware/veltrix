use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaddyConfig {
    #[serde(default)]
    pub admin: Option<CaddyAdminConfig>,

    #[serde(default)]
    pub logging: Option<CaddyLoggingConfig>,

    #[serde(default)]
    pub storage: Option<Value>,

    #[serde(default)]
    pub apps: BTreeMap<String, Value>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaddyAdminConfig {
    #[serde(default)]
    pub disabled: Option<bool>,

    #[serde(default)]
    pub listen: Option<String>,

    #[serde(default)]
    pub enforce_origin: Option<bool>,

    #[serde(default)]
    pub origins: Option<Vec<String>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaddyLoggingConfig {
    #[serde(default)]
    pub sink: Option<Value>,

    #[serde(default)]
    pub logs: BTreeMap<String, Value>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CaddyIdList {
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CaddyPkiCaInfo {
    #[serde(flatten)]
    pub data: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CaddyLoadConfigRequest<'a> {
    #[serde(flatten)]
    pub config: &'a CaddyConfig,
}

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::error::{Result, VeltrixError};

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

impl CaddyConfig {
    /// Create an empty Caddy JSON config.
    pub fn empty() -> Self {
        Self {
            admin: None,
            logging: None,
            storage: None,
            apps: BTreeMap::new(),
            extra: BTreeMap::new(),
        }
    }

    /// Create a local HTTPS file-server config using Caddy's internal issuer.
    pub fn local_https_file_server(
        host: impl Into<String>,
        root: impl Into<String>,
    ) -> Result<Self> {
        CaddySiteConfig::local_https(host)
            .file_server(root)
            .to_config()
    }

    /// Create a reverse-proxy config for a host and upstream addresses.
    pub fn reverse_proxy<I, S>(host: impl Into<String>, upstreams: I) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        CaddySiteConfig::http(host)
            .reverse_proxy(upstreams)
            .to_config()
    }

    /// Create a local HTTPS reverse-proxy config using Caddy's internal issuer.
    pub fn local_https_reverse_proxy<I, S>(host: impl Into<String>, upstreams: I) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        CaddySiteConfig::local_https(host)
            .reverse_proxy(upstreams)
            .to_config()
    }
}

/// Builder for one Caddy HTTP site/server config.
#[derive(Debug, Clone)]
pub struct CaddySiteConfig {
    host: String,
    listen: Vec<String>,
    handlers: Vec<Value>,
    local_https: bool,
}

impl CaddySiteConfig {
    /// Create an HTTP site config listening on `:80`.
    pub fn http(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            listen: vec![":80".to_string()],
            handlers: Vec::new(),
            local_https: false,
        }
    }

    /// Create a local HTTPS site config listening on `:443`.
    pub fn local_https(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            listen: vec![":443".to_string()],
            handlers: Vec::new(),
            local_https: true,
        }
    }

    /// Override listen addresses.
    pub fn listen<I, S>(mut self, listen: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.listen = listen.into_iter().map(Into::into).collect();
        self
    }

    /// Add a file-server handler.
    pub fn file_server(mut self, root: impl Into<String>) -> Self {
        self.handlers.push(json!({
            "handler": "file_server",
            "root": root.into(),
        }));
        self
    }

    /// Add a reverse-proxy handler.
    pub fn reverse_proxy<I, S>(mut self, upstreams: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let upstreams = upstreams
            .into_iter()
            .map(|dial| json!({ "dial": dial.into() }))
            .collect::<Vec<_>>();

        self.handlers.push(json!({
            "handler": "reverse_proxy",
            "upstreams": upstreams,
        }));
        self
    }

    /// Render this site as a full Caddy config.
    pub fn to_config(self) -> Result<CaddyConfig> {
        if self.host.trim().is_empty() {
            return Err(VeltrixError::validation("host", "host must not be empty"));
        }

        if self.listen.is_empty() {
            return Err(VeltrixError::validation(
                "listen",
                "at least one listen address is required",
            ));
        }

        if self.handlers.is_empty() {
            return Err(VeltrixError::validation(
                "handlers",
                "at least one handler is required",
            ));
        }

        let mut apps = BTreeMap::new();
        let mut http = json!({
            "servers": {
                "srv0": {
                    "listen": self.listen,
                    "routes": [{
                        "match": [{ "host": [self.host] }],
                        "handle": self.handlers,
                        "terminal": true
                    }]
                }
            }
        });

        if self.local_https {
            http["servers"]["srv0"]["automatic_https"] = json!({
                "disable": false
            });

            apps.insert(
                "tls".to_string(),
                json!({
                    "automation": {
                        "policies": [{
                            "issuers": [{ "module": "internal" }]
                        }]
                    }
                }),
            );
        }

        apps.insert("http".to_string(), http);

        Ok(CaddyConfig {
            apps,
            ..CaddyConfig::empty()
        })
    }
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

#[derive(Debug, Clone, Deserialize)]
pub struct CaddyModuleList {
    #[serde(default)]
    pub modules: Vec<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct CaddyCliOutput {
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CaddyLoadConfigRequest<'a> {
    #[serde(flatten)]
    pub config: &'a CaddyConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_https_file_server_uses_internal_issuer() {
        let config =
            CaddyConfig::local_https_file_server("localhost", "/srv/www").expect("valid config");

        let http = config.apps.get("http").expect("http app exists");
        let tls = config.apps.get("tls").expect("tls app exists");

        assert_eq!(http["servers"]["srv0"]["listen"][0], ":443");
        assert_eq!(
            http["servers"]["srv0"]["routes"][0]["handle"][0]["handler"],
            "file_server"
        );
        assert_eq!(
            tls["automation"]["policies"][0]["issuers"][0]["module"],
            "internal"
        );
    }

    #[test]
    fn reverse_proxy_config_renders_upstreams() {
        let config = CaddyConfig::reverse_proxy("app.local", ["127.0.0.1:3000", "127.0.0.1:3001"])
            .expect("valid config");
        let http = config.apps.get("http").expect("http app exists");
        let handler = &http["servers"]["srv0"]["routes"][0]["handle"][0];

        assert_eq!(http["servers"]["srv0"]["listen"][0], ":80");
        assert_eq!(handler["handler"], "reverse_proxy");
        assert_eq!(handler["upstreams"][0]["dial"], "127.0.0.1:3000");
        assert_eq!(handler["upstreams"][1]["dial"], "127.0.0.1:3001");
    }

    #[test]
    fn empty_site_config_is_rejected() {
        let err = CaddySiteConfig::http("app.local")
            .to_config()
            .expect_err("handlers are required");

        assert!(err.to_string().contains("handlers"));
    }
}

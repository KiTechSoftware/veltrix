use serde::Serialize;
use serde_json::{Value, json};

use crate::error::{Result, VeltrixError};

use super::{
    spec::{
        TechnitiumAuth, TechnitiumBackendUsed, TechnitiumEmptyResponse, TechnitiumHttpSpec,
        TechnitiumResponse,
    },
    types::{
        TechnitiumApiEnvelope, TechnitiumDnsRecord, TechnitiumRecordType, TechnitiumServerStatus,
        TechnitiumSession, TechnitiumZoneSummary,
    },
};

/// Async client for Technitium DNS Server HTTP API workflows.
#[derive(Debug, Clone)]
pub struct TechnitiumClient {
    spec: TechnitiumHttpSpec,
    http: reqwest::Client,
}

impl TechnitiumClient {
    /// Create a Technitium API client from an HTTP spec.
    pub fn new(spec: TechnitiumHttpSpec) -> Result<Self> {
        let mut builder = reqwest::Client::builder();
        if let Some(timeout) = spec.timeout {
            builder = builder.timeout(timeout);
        }

        let http = builder.build().map_err(|err| {
            VeltrixError::service("technitium", format!("HTTP client failed: {err}"))
        })?;

        Ok(Self { spec, http })
    }

    /// Create a client without authentication.
    pub fn unauthenticated(base_url: impl Into<String>) -> Result<Self> {
        Self::new(TechnitiumHttpSpec::new(base_url))
    }

    /// Borrow the client configuration.
    pub fn spec(&self) -> &TechnitiumHttpSpec {
        &self.spec
    }

    /// Authenticate with username/password and return the session response.
    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<TechnitiumResponse<TechnitiumSession>> {
        self.post_json(
            "/api/user/login",
            &json!({
                "user": username,
                "username": username,
                "pass": password,
                "password": password,
            }),
        )
        .await
    }

    /// Retrieve server status.
    pub async fn status(&self) -> Result<TechnitiumResponse<TechnitiumServerStatus>> {
        self.get_json("/api/server/status", &[]).await
    }

    /// List DNS zones.
    pub async fn zones(&self) -> Result<TechnitiumResponse<Vec<TechnitiumZoneSummary>>> {
        self.get_json("/api/zones/list", &[]).await
    }

    /// Create a DNS zone.
    pub async fn create_zone(
        &self,
        zone: &str,
        zone_type: Option<&str>,
    ) -> Result<TechnitiumEmptyResponse> {
        let mut params = vec![("zone", zone.to_string())];
        if let Some(zone_type) = zone_type {
            params.push(("type", zone_type.to_string()));
        }

        self.get_empty("/api/zones/create", &params).await
    }

    /// Update a DNS zone with service-specific key/value parameters.
    pub async fn update_zone(
        &self,
        zone: &str,
        params: &[(&str, &str)],
    ) -> Result<TechnitiumEmptyResponse> {
        let mut query = vec![("zone", zone.to_string())];
        query.extend(
            params
                .iter()
                .map(|(name, value)| (*name, (*value).to_string())),
        );

        self.get_empty("/api/zones/update", &query).await
    }

    /// Delete a DNS zone.
    pub async fn delete_zone(&self, zone: &str) -> Result<TechnitiumEmptyResponse> {
        self.get_empty("/api/zones/delete", &[("zone", zone.to_string())])
            .await
    }

    /// List records in a DNS zone.
    pub async fn records(
        &self,
        zone: &str,
    ) -> Result<TechnitiumResponse<Vec<TechnitiumDnsRecord>>> {
        self.get_json("/api/zones/records/list", &[("zone", zone.to_string())])
            .await
    }

    /// Add a DNS record to a zone.
    pub async fn add_record(
        &self,
        zone: &str,
        record: &TechnitiumDnsRecord,
    ) -> Result<TechnitiumEmptyResponse> {
        self.post_json_empty(
            "/api/zones/records/add",
            &json!({
                "zone": zone,
                "record": record,
            }),
        )
        .await
    }

    /// Delete a DNS record by type and name.
    pub async fn delete_record(
        &self,
        zone: &str,
        record_type: TechnitiumRecordType,
        name: &str,
    ) -> Result<TechnitiumEmptyResponse> {
        self.get_empty(
            "/api/zones/records/delete",
            &[
                ("zone", zone.to_string()),
                ("type", record_type.as_str().to_string()),
                ("name", name.to_string()),
            ],
        )
        .await
    }

    /// Read server settings.
    pub async fn settings(&self) -> Result<TechnitiumResponse<Value>> {
        self.get_json("/api/settings/get", &[]).await
    }

    /// Update server settings.
    pub async fn update_settings(&self, settings: &Value) -> Result<TechnitiumEmptyResponse> {
        self.post_json_empty("/api/settings/set", settings).await
    }

    /// Resolve a DNS query through Technitium.
    pub async fn resolve(
        &self,
        name: &str,
        record_type: TechnitiumRecordType,
    ) -> Result<TechnitiumResponse<Value>> {
        self.get_json(
            "/api/resolve",
            &[
                ("domain", name.to_string()),
                ("type", record_type.as_str().to_string()),
            ],
        )
        .await
    }

    /// Retrieve server logs as a Technitium payload.
    pub async fn logs(&self) -> Result<TechnitiumResponse<Value>> {
        self.get_json("/api/logs/list", &[]).await
    }

    /// Retrieve server statistics as a Technitium payload.
    pub async fn stats(&self) -> Result<TechnitiumResponse<Value>> {
        self.get_json("/api/stats/get", &[]).await
    }

    /// Retrieve blocklist state.
    pub async fn blocklist(&self) -> Result<TechnitiumResponse<Value>> {
        self.get_json("/api/blocklist/list", &[]).await
    }

    /// Add a domain or pattern to the blocklist.
    pub async fn add_blocklist_entry(&self, entry: &str) -> Result<TechnitiumEmptyResponse> {
        self.get_empty("/api/blocklist/add", &[("domain", entry.to_string())])
            .await
    }

    /// Import a zone file for CI/CD automation workflows.
    pub async fn import_zone(
        &self,
        zone: &str,
        zone_file: &str,
    ) -> Result<TechnitiumEmptyResponse> {
        self.post_json_empty(
            "/api/zones/import",
            &json!({
                "zone": zone,
                "zoneFile": zone_file,
            }),
        )
        .await
    }

    /// Apply multiple records to a zone.
    pub async fn bulk_records(
        &self,
        zone: &str,
        records: &[TechnitiumDnsRecord],
    ) -> Result<TechnitiumEmptyResponse> {
        self.post_json_empty(
            "/api/zones/records/bulk",
            &json!({
                "zone": zone,
                "records": records,
            }),
        )
        .await
    }

    async fn get_json<T>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<TechnitiumResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let bytes = self
            .request(reqwest::Method::GET, path, query, Option::<&()>::None)
            .await?;
        let data = parse_response_body(&bytes)?;
        Ok(TechnitiumResponse::new(data, self.backend_used()))
    }

    async fn post_json<T, B>(&self, path: &str, body: &B) -> Result<TechnitiumResponse<T>>
    where
        T: serde::de::DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let bytes = self
            .request(reqwest::Method::POST, path, &[], Some(body))
            .await?;
        let data = parse_response_body(&bytes)?;
        Ok(TechnitiumResponse::new(data, self.backend_used()))
    }

    async fn get_empty(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<TechnitiumEmptyResponse> {
        let bytes = self
            .request(reqwest::Method::GET, path, query, Option::<&()>::None)
            .await?;
        parse_empty_body(&bytes)?;
        Ok(TechnitiumEmptyResponse::new(self.backend_used()))
    }

    async fn post_json_empty<B>(&self, path: &str, body: &B) -> Result<TechnitiumEmptyResponse>
    where
        B: Serialize + ?Sized,
    {
        let bytes = self
            .request(reqwest::Method::POST, path, &[], Some(body))
            .await?;
        parse_empty_body(&bytes)?;
        Ok(TechnitiumEmptyResponse::new(self.backend_used()))
    }

    async fn request<B>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: &[(&str, String)],
        body: Option<&B>,
    ) -> Result<Vec<u8>>
    where
        B: Serialize + ?Sized,
    {
        let mut url = technitium_url(&self.spec.base_url, path)?;
        {
            let mut pairs = url.query_pairs_mut();
            for (name, value) in query {
                pairs.append_pair(name, value);
            }
            if let TechnitiumAuth::SessionToken { token } = &self.spec.auth {
                pairs.append_pair("token", token);
            }
        }

        let mut request = self.http.request(method, url);
        if let TechnitiumAuth::BearerToken { token } = &self.spec.auth {
            request = request.bearer_auth(token);
        }
        if let Some(body) = body {
            request = request.json(body);
        }

        let response = request.send().await.map_err(|err| {
            VeltrixError::service("technitium", format!("HTTP request failed: {err}"))
        })?;

        let status_code = response.status().as_u16();
        let body = response.bytes().await.map_err(|err| {
            VeltrixError::service("technitium", format!("failed to read HTTP response: {err}"))
        })?;

        if !(200..300).contains(&status_code) {
            return Err(VeltrixError::http(
                status_code,
                format!(
                    "technitium HTTP response body: {}",
                    String::from_utf8_lossy(&body)
                ),
            ));
        }

        Ok(body.to_vec())
    }

    fn backend_used(&self) -> TechnitiumBackendUsed {
        TechnitiumBackendUsed::Http {
            base_url: self.spec.base_url.clone(),
        }
    }
}

fn technitium_url(base_url: &str, path: &str) -> Result<reqwest::Url> {
    let base = base_url.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    reqwest::Url::parse(&format!("{base}/{path}")).map_err(|err| {
        VeltrixError::validation("base_url", format!("invalid Technitium URL: {err}"))
    })
}

fn parse_response_body<T>(body: &[u8]) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    if body.is_empty() {
        return Err(VeltrixError::service(
            "technitium",
            "Technitium endpoint returned an empty body",
        ));
    }

    let envelope: TechnitiumApiEnvelope<T> = serde_json::from_slice(body)
        .map_err(|err| VeltrixError::parsing(format!("invalid technitium json: {err}")))?;

    ensure_envelope_success(&envelope)?;
    envelope.response.ok_or_else(|| {
        VeltrixError::parsing("Technitium response envelope did not contain a response payload")
    })
}

fn parse_empty_body(body: &[u8]) -> Result<()> {
    if body.is_empty() {
        return Ok(());
    }

    let envelope: TechnitiumApiEnvelope<Value> = serde_json::from_slice(body)
        .map_err(|err| VeltrixError::parsing(format!("invalid technitium json: {err}")))?;
    ensure_envelope_success(&envelope)
}

fn ensure_envelope_success<T>(envelope: &TechnitiumApiEnvelope<T>) -> Result<()> {
    if let Some(status) = &envelope.status {
        let normalized = status.to_ascii_lowercase();
        if matches!(normalized.as_str(), "ok" | "success" | "successful") {
            return Ok(());
        }

        return Err(VeltrixError::service(
            "technitium",
            envelope
                .error_message
                .clone()
                .unwrap_or_else(|| format!("Technitium returned status `{status}`")),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_api_urls_without_double_slashes() {
        let url = technitium_url("http://localhost:5380/", "/api/zones/list").unwrap();
        assert_eq!(url.as_str(), "http://localhost:5380/api/zones/list");
    }

    #[test]
    fn parses_response_envelope() {
        let body = br#"{"status":"ok","response":{"version":"13.2"}}"#;
        let status: TechnitiumServerStatus = parse_response_body(body).unwrap();
        assert_eq!(status.version.as_deref(), Some("13.2"));
    }

    #[test]
    fn reports_service_error_from_envelope() {
        let body = br#"{"status":"error","errorMessage":"bad token"}"#;
        let err = parse_empty_body(body).unwrap_err();
        assert_eq!(err.to_string(), "technitium service failed: bad token");
    }
}

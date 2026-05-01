use std::path::Path;

use serde::Serialize;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

use crate::error::{Result, VeltrixError};

use super::{
    spec::{CaddyAdminEndpoint, CaddyAdminSpec, CaddyBackendUsed, CaddyEmptyResponse, CaddyResponse},
    types::{CaddyConfig, CaddyIdList, CaddyPkiCaInfo},
};

#[derive(Debug, Clone)]
pub struct CaddyAdminClient {
    spec: CaddyAdminSpec,
}

impl CaddyAdminClient {
    pub fn new(spec: CaddyAdminSpec) -> Self {
        Self { spec }
    }

    pub fn localhost_default() -> Self {
        Self::new(CaddyAdminSpec::localhost_default())
    }

    pub fn spec(&self) -> &CaddyAdminSpec {
        &self.spec
    }

    pub async fn config(&self) -> Result<CaddyResponse<CaddyConfig>> {
        self.request_json("GET", "/config/", Option::<&()>::None).await
    }

    pub async fn load_config(&self, config: &CaddyConfig) -> Result<CaddyEmptyResponse> {
        self.request_empty("POST", "/load", Some(config)).await
    }

    pub async fn stop(&self) -> Result<CaddyEmptyResponse> {
        self.request_empty("POST", "/stop", Option::<&()>::None).await
    }

    pub async fn id_list(&self) -> Result<CaddyResponse<CaddyIdList>> {
        self.request_json("GET", "/id/", Option::<&()>::None).await
    }

    pub async fn pki_ca(&self, ca_id: &str) -> Result<CaddyResponse<CaddyPkiCaInfo>> {
        let path = format!("/pki/ca/{ca_id}");
        self.request_json("GET", &path, Option::<&()>::None).await
    }

    async fn request_json<T, B>(
        &self,
        method: &str,
        path: &str,
        body: Option<B>,
    ) -> Result<CaddyResponse<T>>
    where
        T: serde::de::DeserializeOwned,
        B: Serialize,
    {
        let raw = self.request_raw(method, path, body).await?;

        if raw.body.is_empty() {
            return Err(VeltrixError::config_invalid(format!(
                "caddy endpoint returned empty body: {path}"
            )));
        }

        let data = serde_json::from_slice(&raw.body)
            .map_err(|err| VeltrixError::config_invalid(format!("invalid caddy json: {err}")))?;

        Ok(CaddyResponse {
            backend: self.backend_used(),
            data,
        })
    }

    async fn request_empty<B>(
        &self,
        method: &str,
        path: &str,
        body: Option<B>,
    ) -> Result<CaddyEmptyResponse>
    where
        B: Serialize,
    {
        self.request_raw(method, path, body).await?;

        Ok(CaddyEmptyResponse {
            backend: self.backend_used(),
        })
    }

    async fn request_raw<B>(&self, method: &str, path: &str, body: Option<B>) -> Result<HttpRawResponse>
    where
        B: Serialize,
    {
        match &self.spec.endpoint {
            CaddyAdminEndpoint::Http { base_url } => {
                http_tcp_json(base_url, method, path, body).await
            }
            CaddyAdminEndpoint::UnixSocket { socket_path } => {
                let body_bytes = match body {
                    Some(body) => Some(
                        serde_json::to_vec(&body).map_err(|err| {
                            VeltrixError::config_invalid(format!("invalid caddy request json: {err}"))
                        })?,
                    ),
                    None => None,
                };

                http_unix(socket_path, method, path, body_bytes.as_deref()).await
            }
        }
    }

    fn backend_used(&self) -> CaddyBackendUsed {
        match &self.spec.endpoint {
            CaddyAdminEndpoint::Http { base_url } => CaddyBackendUsed::Http {
                base_url: base_url.clone(),
            },
            CaddyAdminEndpoint::UnixSocket { socket_path } => CaddyBackendUsed::UnixSocket {
                socket_path: socket_path.clone(),
            },
        }
    }
}

#[derive(Debug, Clone)]
struct HttpRawResponse {
    status_code: u16,
    body: Vec<u8>,
}

async fn http_tcp_json<B>(
    base_url: &str,
    method: &str,
    path: &str,
    body: Option<B>,
) -> Result<HttpRawResponse>
where
    B: Serialize,
{
    let base_url = base_url.trim_end_matches('/');
    let url = format!("{base_url}{path}");

    let client = reqwest::Client::new();

    let request = match method {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "PATCH" => client.patch(&url),
        "DELETE" => client.delete(&url),
        _ => {
            return Err(VeltrixError::config_invalid(format!(
                "unsupported caddy HTTP method: {method}"
            )))
        }
    };

    let request = if let Some(body) = body {
        request.json(&body)
    } else {
        request
    };

    let response = request.send().await.map_err(|err| {
        VeltrixError::config_invalid(format!("caddy HTTP request failed: {err}"))
    })?;

    let status_code = response.status().as_u16();
    let body = response.bytes().await.map_err(|err| {
        VeltrixError::config_invalid(format!("failed to read caddy HTTP response: {err}"))
    })?;

    if !(200..300).contains(&status_code) {
        return Err(VeltrixError::config_invalid(format!(
            "caddy HTTP request failed: status {status_code}; body: {}",
            String::from_utf8_lossy(&body)
        )));
    }

    Ok(HttpRawResponse {
        status_code,
        body: body.to_vec(),
    })
}

async fn http_unix(
    socket_path: &Path,
    method: &str,
    path: &str,
    body: Option<&[u8]>,
) -> Result<HttpRawResponse> {
    let mut stream = UnixStream::connect(socket_path).await?;

    let body_len = body.map(|b| b.len()).unwrap_or(0);

    let mut request = format!(
        "{method} {path} HTTP/1.1\r\n\
         Host: caddy\r\n\
         Accept: application/json\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {body_len}\r\n\
         Connection: close\r\n\
         \r\n"
    )
    .into_bytes();

    if let Some(body) = body {
        request.extend_from_slice(body);
    }

    stream.write_all(&request).await?;
    stream.shutdown().await?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;

    parse_http_response(response, "caddy unix socket")
}

fn parse_http_response(response: Vec<u8>, context: &str) -> Result<HttpRawResponse> {
    let Some(header_end) = find_header_end(&response) else {
        return Err(VeltrixError::config_invalid(format!(
            "invalid HTTP response from {context}"
        )));
    };

    let headers = &response[..header_end];
    let body = response[header_end + 4..].to_vec();

    let status_line_end = headers
        .windows(2)
        .position(|w| w == b"\r\n")
        .unwrap_or(headers.len());

    let status_line = String::from_utf8_lossy(&headers[..status_line_end]);
    let status_code = parse_status_code(&status_line)?;

    if !(200..300).contains(&status_code) {
        return Err(VeltrixError::config_invalid(format!(
            "{context} request failed: {status_line}; body: {}",
            String::from_utf8_lossy(&body)
        )));
    }

    Ok(HttpRawResponse { status_code, body })
}

fn parse_status_code(status_line: &str) -> Result<u16> {
    status_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| VeltrixError::config_invalid(format!("invalid HTTP status line: {status_line}")))?
        .parse::<u16>()
        .map_err(|err| VeltrixError::config_invalid(format!("invalid HTTP status code: {err}")))
}

fn find_header_end(bytes: &[u8]) -> Option<usize> {
    bytes.windows(4).position(|window| window == b"\r\n\r\n")
}
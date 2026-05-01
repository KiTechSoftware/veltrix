use std::path::Path;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

use crate::error::{Result, VeltrixError};

use super::spec::SUPPORTED_LIBPOD_API_VERSION;

use super::{
    spec::{PodmanBackendUsed, PodmanEmptyResponse, PodmanResponse, PodmanSocketSpec},
    types::{
        PodmanContainerSummary, PodmanImageSummary, PodmanInfo, PodmanLogs, PodmanPodSummary,
        PodmanPullImageReport, PodmanVersion,
    },
};

#[derive(Debug, Clone)]
pub struct PodmanSocketClient {
    spec: PodmanSocketSpec,
}

impl PodmanSocketClient {
    pub fn new(spec: PodmanSocketSpec) -> Self {
        Self { spec }
    }

    pub fn spec(&self) -> &PodmanSocketSpec {
        &self.spec
    }

    pub async fn info(&self) -> Result<PodmanResponse<PodmanInfo>> {
        self.get_json("/libpod/info").await
    }

    pub async fn version(&self) -> Result<PodmanResponse<PodmanVersion>> {
        self.get_json("/libpod/version").await
    }

    pub async fn containers(&self) -> Result<PodmanResponse<Vec<PodmanContainerSummary>>> {
        self.get_json("/libpod/containers/json?all=true").await
    }

    pub async fn list_containers_api(&self) -> Result<PodmanResponse<Vec<PodmanContainerSummary>>> {
        self.get_json("/containers/json?all=true").await
    }

    pub async fn list_libpod_containers_api(
        &self,
    ) -> Result<PodmanResponse<Vec<PodmanContainerSummary>>> {
        self.get_json("/libpod/containers/json?all=true").await
    }

    pub async fn inspect_container(&self, id: &str) -> Result<PodmanResponse<serde_json::Value>> {
        self.get_json(&format!("/libpod/containers/{id}/json"))
            .await
    }

    pub async fn container_logs(&self, id: &str) -> Result<PodmanResponse<PodmanLogs>> {
        let path = format!(
            "/v{SUPPORTED_LIBPOD_API_VERSION}/libpod/containers/{id}/logs?stdout=true&stderr=true"
        );
        let body = http_unix(&self.spec.socket_path, "GET", &path, None).await?;

        Ok(PodmanResponse {
            backend: self.backend_used(),
            data: PodmanLogs {
                output: String::from_utf8(body).map_err(|err| {
                    VeltrixError::parsing(format!("invalid podman log utf-8: {err}"))
                })?,
            },
        })
    }

    pub async fn list_pods_api(&self) -> Result<PodmanResponse<Vec<PodmanPodSummary>>> {
        self.get_json("/libpod/pods/json").await
    }

    pub async fn inspect_pod(&self, id: &str) -> Result<PodmanResponse<serde_json::Value>> {
        self.get_json(&format!("/libpod/pods/{id}/json")).await
    }

    pub async fn list_images_api(&self) -> Result<PodmanResponse<Vec<PodmanImageSummary>>> {
        self.get_json("/libpod/images/json").await
    }

    pub async fn pull_image_api(
        &self,
        image: &str,
    ) -> Result<PodmanResponse<PodmanPullImageReport>> {
        let endpoint = format!(
            "/libpod/images/pull?reference={}",
            encode_query_component(image)
        );
        self.post_json(&endpoint, None).await
    }

    pub async fn start_container(&self, id: &str) -> Result<PodmanEmptyResponse> {
        let path = format!("/v{SUPPORTED_LIBPOD_API_VERSION}/libpod/containers/{id}/start");

        http_unix(&self.spec.socket_path, "POST", &path, None).await?;

        Ok(PodmanEmptyResponse {
            backend: self.backend_used(),
        })
    }

    pub async fn stop_container(&self, id: &str) -> Result<PodmanEmptyResponse> {
        let path = format!("/v{SUPPORTED_LIBPOD_API_VERSION}/libpod/containers/{id}/stop");

        http_unix(&self.spec.socket_path, "POST", &path, None).await?;

        Ok(PodmanEmptyResponse {
            backend: self.backend_used(),
        })
    }

    pub async fn remove_container(&self, id: &str) -> Result<PodmanEmptyResponse> {
        let path = format!("/v{SUPPORTED_LIBPOD_API_VERSION}/libpod/containers/{id}");

        http_unix(&self.spec.socket_path, "DELETE", &path, None).await?;

        Ok(PodmanEmptyResponse {
            backend: self.backend_used(),
        })
    }

    async fn get_json<T>(&self, endpoint: &str) -> Result<PodmanResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let path = format!("/v{SUPPORTED_LIBPOD_API_VERSION}{endpoint}");
        let body = http_unix(&self.spec.socket_path, "GET", &path, None).await?;

        if body.is_empty() {
            return Err(VeltrixError::service(
                "podman",
                format!("podman endpoint returned empty body: {path}"),
            ));
        }

        let data = serde_json::from_slice(&body)
            .map_err(|err| VeltrixError::parsing(format!("invalid podman json: {err}")))?;

        Ok(PodmanResponse {
            backend: self.backend_used(),
            data,
        })
    }

    async fn post_json<T>(&self, endpoint: &str, body: Option<&[u8]>) -> Result<PodmanResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let path = format!("/v{SUPPORTED_LIBPOD_API_VERSION}{endpoint}");
        let body = http_unix(&self.spec.socket_path, "POST", &path, body).await?;

        let data = if body.is_empty() {
            serde_json::from_slice(b"{}")
        } else {
            serde_json::from_slice(&body)
        }
        .map_err(|err| VeltrixError::parsing(format!("invalid podman json: {err}")))?;

        Ok(PodmanResponse {
            backend: self.backend_used(),
            data,
        })
    }

    fn backend_used(&self) -> PodmanBackendUsed {
        PodmanBackendUsed::Socket {
            socket_path: self.spec.socket_path.clone(),
            user: self.spec.user.clone(),
        }
    }
}

async fn http_unix(
    socket_path: &Path,
    method: &str,
    path: &str,
    body: Option<&[u8]>,
) -> Result<Vec<u8>> {
    let mut stream = UnixStream::connect(socket_path)
        .await
        .map_err(|err| VeltrixError::socket(format!("podman connect failed: {err}")))?;

    let body_len = body.map(|b| b.len()).unwrap_or(0);

    let mut request = format!(
        "{method} {path} HTTP/1.1\r\n\
         Host: podman\r\n\
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

    stream
        .write_all(&request)
        .await
        .map_err(|err| VeltrixError::socket(format!("podman write failed: {err}")))?;
    stream
        .shutdown()
        .await
        .map_err(|err| VeltrixError::socket(format!("podman shutdown failed: {err}")))?;

    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .await
        .map_err(|err| VeltrixError::socket(format!("podman read failed: {err}")))?;

    let Some(header_end) = find_header_end(&response) else {
        return Err(VeltrixError::parsing(
            "invalid HTTP response from podman socket",
        ));
    };

    let headers = &response[..header_end];
    let body = response[header_end + 4..].to_vec();

    let status_line_end = headers
        .windows(2)
        .position(|w| w == b"\r\n")
        .unwrap_or(headers.len());

    let status_line = String::from_utf8_lossy(&headers[..status_line_end]);

    if !status_line.contains(" 200 ")
        && !status_line.contains(" 201 ")
        && !status_line.contains(" 204 ")
    {
        let status = parse_status_code(&status_line).unwrap_or(0);
        return Err(VeltrixError::http(
            status,
            format!(
                "podman socket request failed: {status_line}; body: {}",
                String::from_utf8_lossy(&body)
            ),
        ));
    }

    Ok(body)
}

fn parse_status_code(status_line: &str) -> Option<u16> {
    status_line.split_whitespace().nth(1)?.parse().ok()
}

fn find_header_end(bytes: &[u8]) -> Option<usize> {
    bytes.windows(4).position(|window| window == b"\r\n\r\n")
}

fn encode_query_component(value: &str) -> String {
    let mut output = String::new();

    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                output.push(byte as char);
            }
            _ => output.push_str(&format!("%{byte:02X}")),
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_code_parser_handles_standard_lines() {
        assert_eq!(parse_status_code("HTTP/1.1 204 No Content"), Some(204));
        assert_eq!(parse_status_code("HTTP/1.1 nope"), None);
    }

    #[test]
    fn header_end_detects_crlf_separator() {
        assert_eq!(find_header_end(b"HTTP/1.1 200 OK\r\n\r\n{}"), Some(15));
    }

    #[test]
    fn query_component_encoding_handles_image_references() {
        assert_eq!(
            encode_query_component("quay.io/example/app:latest"),
            "quay.io%2Fexample%2Fapp%3Alatest"
        );
    }
}

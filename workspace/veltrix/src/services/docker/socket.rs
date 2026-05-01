use std::path::Path;

use serde::Serialize;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

use crate::error::{Result, VeltrixError};

use super::{
    spec::{
        DockerBackendUsed, DockerEmptyResponse, DockerResponse, DockerSocketSpec,
        SUPPORTED_DOCKER_API_VERSION,
    },
    types::{
        DockerContainerSummary, DockerCreateContainerResponse, DockerImageSummary, DockerLogs,
        DockerNetworkSummary, DockerPullImageReport, DockerSocketPayload, DockerVolumeSummary,
    },
};

/// Docker Engine API client over a Unix socket.
#[derive(Debug, Clone)]
pub struct DockerSocketClient {
    spec: DockerSocketSpec,
}

impl DockerSocketClient {
    /// Create a Docker socket client.
    pub fn new(spec: DockerSocketSpec) -> Self {
        Self { spec }
    }

    /// Get the underlying spec.
    pub fn spec(&self) -> &DockerSocketSpec {
        &self.spec
    }

    /// List containers via the Docker Engine API.
    pub async fn list_containers_api(&self) -> Result<DockerResponse<Vec<DockerContainerSummary>>> {
        self.get_json("/containers/json?all=true").await
    }

    /// Inspect a container via the Docker Engine API.
    pub async fn inspect_container_api(
        &self,
        id: &str,
    ) -> Result<DockerResponse<serde_json::Value>> {
        self.get_json(&format!("/containers/{id}/json")).await
    }

    /// Create a container via the Docker Engine API.
    pub async fn create_container_api<T>(
        &self,
        name: Option<&str>,
        body: &T,
    ) -> Result<DockerResponse<DockerCreateContainerResponse>>
    where
        T: Serialize,
    {
        let body = serde_json::to_vec(body)
            .map_err(|err| VeltrixError::parsing(format!("invalid docker request json: {err}")))?;
        let endpoint = match name {
            Some(name) => format!("/containers/create?name={}", encode_query_component(name)),
            None => "/containers/create".to_string(),
        };

        self.post_json(&endpoint, Some(&body)).await
    }

    /// Start a container via the Docker Engine API.
    pub async fn start_container_api(&self, id: &str) -> Result<DockerEmptyResponse> {
        self.post_empty(&format!("/containers/{id}/start"), None)
            .await
    }

    /// Stop a container via the Docker Engine API.
    pub async fn stop_container_api(&self, id: &str) -> Result<DockerEmptyResponse> {
        self.post_empty(&format!("/containers/{id}/stop"), None)
            .await
    }

    /// Remove a container via the Docker Engine API.
    pub async fn remove_container_api(&self, id: &str) -> Result<DockerEmptyResponse> {
        self.delete_empty(&format!("/containers/{id}")).await
    }

    /// Read container logs via the Docker Engine API.
    pub async fn container_logs_api(&self, id: &str) -> Result<DockerResponse<DockerLogs>> {
        let path = format!(
            "/v{SUPPORTED_DOCKER_API_VERSION}/containers/{id}/logs?stdout=true&stderr=true"
        );
        let body = http_unix(&self.spec.socket_path, "GET", &path, None).await?;

        Ok(DockerResponse::new(
            DockerLogs {
                output: String::from_utf8(body)
                    .map_err(|err| VeltrixError::parsing(format!("invalid docker logs: {err}")))?,
            },
            self.backend_used(),
        ))
    }

    /// List images via the Docker Engine API.
    pub async fn list_images_api(&self) -> Result<DockerResponse<Vec<DockerImageSummary>>> {
        self.get_json("/images/json").await
    }

    /// Inspect an image via the Docker Engine API.
    pub async fn inspect_image_api(
        &self,
        image: &str,
    ) -> Result<DockerResponse<serde_json::Value>> {
        self.get_json(&format!("/images/{image}/json")).await
    }

    /// Pull an image via the Docker Engine API.
    pub async fn pull_image_api(
        &self,
        image: &str,
    ) -> Result<DockerResponse<Vec<DockerPullImageReport>>> {
        let endpoint = format!("/images/create?fromImage={}", encode_query_component(image));
        let path = versioned_path(&endpoint);
        let body = http_unix(&self.spec.socket_path, "POST", &path, None).await?;
        let data = parse_json_lines(&body)?;

        Ok(DockerResponse::new(data, self.backend_used()))
    }

    /// List networks via the Docker Engine API.
    pub async fn list_networks_api(&self) -> Result<DockerResponse<Vec<DockerNetworkSummary>>> {
        self.get_json("/networks").await
    }

    /// Create a network via the Docker Engine API.
    pub async fn create_network_api<T>(
        &self,
        body: &T,
    ) -> Result<DockerResponse<DockerSocketPayload>>
    where
        T: Serialize,
    {
        let body = serde_json::to_vec(body).map_err(|err| {
            VeltrixError::parsing(format!("invalid docker network request json: {err}"))
        })?;

        self.post_json("/networks/create", Some(&body)).await
    }

    /// Inspect a network via the Docker Engine API.
    pub async fn inspect_network_api(
        &self,
        network: &str,
    ) -> Result<DockerResponse<serde_json::Value>> {
        self.get_json(&format!("/networks/{network}")).await
    }

    /// Remove a network via the Docker Engine API.
    pub async fn remove_network_api(&self, network: &str) -> Result<DockerEmptyResponse> {
        self.delete_empty(&format!("/networks/{network}")).await
    }

    /// Connect a container to a network via the Docker Engine API.
    pub async fn connect_network_api<T>(
        &self,
        network: &str,
        body: &T,
    ) -> Result<DockerEmptyResponse>
    where
        T: Serialize,
    {
        let body = serde_json::to_vec(body).map_err(|err| {
            VeltrixError::parsing(format!("invalid docker network connect json: {err}"))
        })?;

        self.post_empty(&format!("/networks/{network}/connect"), Some(&body))
            .await
    }

    /// List volumes via the Docker Engine API.
    pub async fn list_volumes_api(&self) -> Result<DockerResponse<Vec<DockerVolumeSummary>>> {
        let response: DockerSocketPayload = self.get_json("/volumes").await?.data;
        let volumes = response
            .data
            .get("Volumes")
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(Vec::new()));
        let volumes = serde_json::from_value(volumes)
            .map_err(|err| VeltrixError::parsing(format!("invalid docker volumes: {err}")))?;

        Ok(DockerResponse::new(volumes, self.backend_used()))
    }

    /// Create a volume via the Docker Engine API.
    pub async fn create_volume_api<T>(
        &self,
        body: &T,
    ) -> Result<DockerResponse<DockerVolumeSummary>>
    where
        T: Serialize,
    {
        let body = serde_json::to_vec(body).map_err(|err| {
            VeltrixError::parsing(format!("invalid docker volume request json: {err}"))
        })?;

        self.post_json("/volumes/create", Some(&body)).await
    }

    /// Inspect a volume via the Docker Engine API.
    pub async fn inspect_volume_api(
        &self,
        volume: &str,
    ) -> Result<DockerResponse<DockerVolumeSummary>> {
        self.get_json(&format!("/volumes/{volume}")).await
    }

    /// Remove a volume via the Docker Engine API.
    pub async fn remove_volume_api(&self, volume: &str) -> Result<DockerEmptyResponse> {
        self.delete_empty(&format!("/volumes/{volume}")).await
    }

    async fn get_json<T>(&self, endpoint: &str) -> Result<DockerResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let path = versioned_path(endpoint);
        let body = http_unix(&self.spec.socket_path, "GET", &path, None).await?;
        let data = serde_json::from_slice(&body)
            .map_err(|err| VeltrixError::parsing(format!("invalid docker json: {err}")))?;

        Ok(DockerResponse::new(data, self.backend_used()))
    }

    async fn post_json<T>(&self, endpoint: &str, body: Option<&[u8]>) -> Result<DockerResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let path = versioned_path(endpoint);
        let body = http_unix(&self.spec.socket_path, "POST", &path, body).await?;
        let data = serde_json::from_slice(&body)
            .map_err(|err| VeltrixError::parsing(format!("invalid docker json: {err}")))?;

        Ok(DockerResponse::new(data, self.backend_used()))
    }

    async fn post_empty(&self, endpoint: &str, body: Option<&[u8]>) -> Result<DockerEmptyResponse> {
        let path = versioned_path(endpoint);
        http_unix(&self.spec.socket_path, "POST", &path, body).await?;

        Ok(DockerEmptyResponse::new(self.backend_used()))
    }

    async fn delete_empty(&self, endpoint: &str) -> Result<DockerEmptyResponse> {
        let path = versioned_path(endpoint);
        http_unix(&self.spec.socket_path, "DELETE", &path, None).await?;

        Ok(DockerEmptyResponse::new(self.backend_used()))
    }

    fn backend_used(&self) -> DockerBackendUsed {
        DockerBackendUsed::Socket {
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
        .map_err(|err| VeltrixError::socket(format!("docker connect failed: {err}")))?;

    let body_len = body.map(|body| body.len()).unwrap_or(0);
    let mut request = format!(
        "{method} {path} HTTP/1.1\r\n\
         Host: docker\r\n\
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
        .map_err(|err| VeltrixError::socket(format!("docker write failed: {err}")))?;
    stream
        .shutdown()
        .await
        .map_err(|err| VeltrixError::socket(format!("docker shutdown failed: {err}")))?;

    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .await
        .map_err(|err| VeltrixError::socket(format!("docker read failed: {err}")))?;

    let Some(header_end) = find_header_end(&response) else {
        return Err(VeltrixError::parsing(
            "invalid HTTP response from docker socket",
        ));
    };

    let headers = &response[..header_end];
    let body = response[header_end + 4..].to_vec();
    let status_line_end = headers
        .windows(2)
        .position(|window| window == b"\r\n")
        .unwrap_or(headers.len());
    let status_line = String::from_utf8_lossy(&headers[..status_line_end]);

    if !status_line.contains(" 200 ")
        && !status_line.contains(" 201 ")
        && !status_line.contains(" 204 ")
    {
        return Err(VeltrixError::http(
            parse_status_code(&status_line).unwrap_or(0),
            format!(
                "docker socket request failed: {status_line}; body: {}",
                String::from_utf8_lossy(&body)
            ),
        ));
    }

    Ok(body)
}

fn versioned_path(endpoint: &str) -> String {
    format!("/v{SUPPORTED_DOCKER_API_VERSION}{endpoint}")
}

fn parse_json_lines<T>(body: &[u8]) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    String::from_utf8_lossy(body)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str(line)
                .map_err(|err| VeltrixError::parsing(format!("invalid docker json line: {err}")))
        })
        .collect()
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
    fn versioned_paths_include_configured_engine_api_version() {
        assert_eq!(versioned_path("/containers/json"), "/v1.40/containers/json");
        assert_eq!(
            versioned_path("/networks/web/connect"),
            "/v1.40/networks/web/connect"
        );
    }

    #[test]
    fn query_encoding_handles_image_names() {
        assert_eq!(
            encode_query_component("docker.io/library/alpine:latest"),
            "docker.io%2Flibrary%2Falpine%3Alatest"
        );
    }

    #[test]
    fn json_line_parser_handles_pull_reports() {
        let body = br#"{"status":"pulling","id":"layer"}
{"status":"done"}"#;
        let parsed: Vec<DockerPullImageReport> = parse_json_lines(body).unwrap();

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].id.as_deref(), Some("layer"));
    }
}

use std::path::Path;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

use crate::error::{Result, VeltrixError};

use super::spec::SUPPORTED_LIBPOD_API_VERSION;

use super::{
    spec::{PodmanBackendUsed, PodmanResponse, PodmanSocketSpec},
    types::{PodmanContainerSummary, PodmanInfo, PodmanVersion},
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

    pub async fn start_container(&self, id: &str) -> Result<PodmanEmptyResponse> {
        let path = format!("/v{SUPPORTED_LIBPOD_API_VERSION}/libpod/containers/{id}/start");

        http_unix(&self.spec.socket_path, "POST", &path, None).await?;

        Ok(PodmanEmptyResponse {
            backend: PodmanBackendUsed::Socket {
                socket_path: self.spec.socket_path.clone(),
                user: self.spec.user.clone(),
            },
        })
    }

    pub async fn stop_container(&self, id: &str) -> Result<PodmanEmptyResponse> {
        let path = format!("/v{SUPPORTED_LIBPOD_API_VERSION}/libpod/containers/{id}/stop");

        http_unix(&self.spec.socket_path, "POST", &path, None).await?;

        Ok(PodmanEmptyResponse {
            backend: PodmanBackendUsed::Socket {
                socket_path: self.spec.socket_path.clone(),
                user: self.spec.user.clone(),
            },
        })
    }

    async fn get_json<T>(&self, endpoint: &str) -> Result<PodmanResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let path = format!("/v{SUPPORTED_LIBPOD_API_VERSION}{endpoint}");
        let body = http_unix(&self.spec.socket_path, "GET", &path, None).await?;

        if body.is_empty() {
            return Err(VeltrixError::config_invalid(format!(
                "podman endpoint returned empty body: {path}"
            )));
        }

        let data = serde_json::from_slice(&body)
            .map_err(|err| VeltrixError::config_invalid(format!("invalid podman json: {err}")))?;

        Ok(PodmanResponse {
            backend: PodmanBackendUsed::Socket {
                socket_path: self.spec.socket_path.clone(),
                user: self.spec.user.clone(),
            },
            data,
        })
    }
}

async fn http_unix(
    socket_path: &Path,
    method: &str,
    path: &str,
    body: Option<&[u8]>,
) -> Result<Vec<u8>> {
    let mut stream = UnixStream::connect(socket_path).await?;

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

    stream.write_all(&request).await?;
    stream.shutdown().await?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;

    let Some(header_end) = find_header_end(&response) else {
        return Err(VeltrixError::config_invalid(
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
        return Err(VeltrixError::config_invalid(format!(
            "podman socket request failed: {status_line}; body: {}",
            String::from_utf8_lossy(&body)
        )));
    }

    Ok(body)
}

fn find_header_end(bytes: &[u8]) -> Option<usize> {
    bytes.windows(4).position(|window| window == b"\r\n\r\n")
}

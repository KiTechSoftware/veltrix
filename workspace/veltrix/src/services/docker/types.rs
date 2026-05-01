use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

/// Docker version information
#[derive(Debug, Clone, Deserialize)]
pub struct DockerVersion {
    #[serde(rename = "Version", default)]
    pub version: Option<String>,

    #[serde(rename = "ApiVersion", default)]
    pub api_version: Option<String>,

    #[serde(rename = "GitCommit", default)]
    pub git_commit: Option<String>,

    #[serde(rename = "GoVersion", default)]
    pub go_version: Option<String>,

    #[serde(rename = "Os", default)]
    pub os: Option<String>,

    #[serde(rename = "Arch", default)]
    pub arch: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Docker system info
#[derive(Debug, Clone, Deserialize)]
pub struct DockerInfo {
    #[serde(default)]
    pub containers: Option<i64>,

    #[serde(rename = "ContainersRunning", default)]
    pub containers_running: Option<i64>,

    #[serde(rename = "ContainersPaused", default)]
    pub containers_paused: Option<i64>,

    #[serde(rename = "ContainersStopped", default)]
    pub containers_stopped: Option<i64>,

    #[serde(default)]
    pub images: Option<i64>,

    #[serde(default)]
    pub driver: Option<String>,

    #[serde(default)]
    pub version: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Docker container summary from list operation
#[derive(Debug, Clone, Deserialize)]
pub struct DockerContainerSummary {
    #[serde(rename = "Id", default)]
    pub id: Option<String>,

    #[serde(rename = "Names", default)]
    pub names: Option<Vec<String>>,

    #[serde(rename = "Image", default)]
    pub image: Option<String>,

    #[serde(rename = "ImageID", default)]
    pub image_id: Option<String>,

    #[serde(rename = "Command", default)]
    pub command: Option<String>,

    #[serde(rename = "Created", default)]
    pub created: Option<i64>,

    #[serde(rename = "Ports", default)]
    pub ports: Option<Vec<Value>>,

    #[serde(rename = "Labels", default)]
    pub labels: Option<BTreeMap<String, String>>,

    #[serde(rename = "State", default)]
    pub state: Option<String>,

    #[serde(rename = "Status", default)]
    pub status: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Docker image summary
#[derive(Debug, Clone, Deserialize)]
pub struct DockerImageSummary {
    #[serde(rename = "Id", default)]
    pub id: Option<String>,

    #[serde(rename = "ParentId", default)]
    pub parent_id: Option<String>,

    #[serde(rename = "RepoTags", default)]
    pub repo_tags: Option<Vec<String>>,

    #[serde(rename = "Created", default)]
    pub created: Option<i64>,

    #[serde(rename = "Size", default)]
    pub size: Option<i64>,

    #[serde(rename = "SharedSize", default)]
    pub shared_size: Option<i64>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Text output returned by Docker log operations.
#[derive(Debug, Clone)]
pub struct DockerLogs {
    pub output: String,
}

/// Docker Compose service/process summary placeholder for v0.5.0.
#[derive(Debug, Clone, Deserialize)]
pub struct DockerComposeServiceSummary {
    #[serde(rename = "Name", alias = "name", default)]
    pub name: Option<String>,

    #[serde(rename = "Service", alias = "service", default)]
    pub service: Option<String>,

    #[serde(rename = "State", alias = "state", default)]
    pub state: Option<String>,

    #[serde(rename = "Publishers", alias = "publishers", default)]
    pub publishers: Option<Vec<Value>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Opaque Docker socket response body for APIs not typed until v0.5.0.
#[derive(Debug, Clone, Deserialize)]
pub struct DockerSocketPayload {
    #[serde(flatten)]
    pub data: BTreeMap<String, Value>,
}

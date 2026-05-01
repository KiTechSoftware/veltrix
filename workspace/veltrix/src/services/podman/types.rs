use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct PodmanInfo {
    #[serde(default)]
    pub host: Option<Value>,

    #[serde(default)]
    pub store: Option<Value>,

    #[serde(default)]
    pub registries: Option<Value>,

    #[serde(default)]
    pub version: Option<Value>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PodmanVersion {
    #[serde(rename = "Version", alias = "version", default)]
    pub version: Option<String>,

    #[serde(
        rename = "APIVersion",
        alias = "apiVersion",
        alias = "api_version",
        default
    )]
    pub api_version: Option<String>,

    #[serde(
        rename = "GitCommit",
        alias = "gitCommit",
        alias = "git_commit",
        default
    )]
    pub git_commit: Option<String>,

    #[serde(
        rename = "GoVersion",
        alias = "goVersion",
        alias = "go_version",
        default
    )]
    pub go_version: Option<String>,

    #[serde(rename = "OsArch", alias = "osArch", alias = "os_arch", default)]
    pub os_arch: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PodmanContainerSummary {
    #[serde(rename = "Id", alias = "ID", alias = "id", default)]
    pub id: Option<String>,

    #[serde(rename = "Names", alias = "names", default)]
    pub names: Option<Vec<String>>,

    #[serde(rename = "Image", alias = "image", default)]
    pub image: Option<String>,

    #[serde(rename = "ImageID", alias = "ImageId", alias = "image_id", default)]
    pub image_id: Option<String>,

    #[serde(rename = "State", alias = "state", default)]
    pub state: Option<String>,

    #[serde(rename = "Status", alias = "status", default)]
    pub status: Option<String>,

    #[serde(rename = "Created", alias = "created", default)]
    pub created: Option<Value>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Pod summary from list or inspect operations
#[derive(Debug, Clone, Deserialize)]
pub struct PodmanPodSummary {
    #[serde(rename = "Id", alias = "ID", alias = "id", default)]
    pub id: Option<String>,

    #[serde(rename = "Name", alias = "name", default)]
    pub name: Option<String>,

    #[serde(rename = "Created", alias = "created", default)]
    pub created: Option<Value>,

    #[serde(rename = "State", alias = "state", default)]
    pub state: Option<String>,

    #[serde(rename = "Containers", alias = "containers", default)]
    pub containers: Option<i32>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Image summary from list or search operations
#[derive(Debug, Clone, Deserialize)]
pub struct PodmanImageSummary {
    #[serde(rename = "Id", alias = "ID", alias = "id", default)]
    pub id: Option<String>,

    #[serde(rename = "RepoTags", alias = "repoTags", alias = "repo_tags", default)]
    pub repo_tags: Option<Vec<String>>,

    #[serde(rename = "Size", alias = "size", default)]
    pub size: Option<i64>,

    #[serde(rename = "Created", alias = "created", default)]
    pub created: Option<Value>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Image pull progress/report payload returned by Libpod image pull APIs.
#[derive(Debug, Clone, Deserialize)]
pub struct PodmanPullImageReport {
    #[serde(rename = "Id", alias = "id", default)]
    pub id: Option<String>,

    #[serde(rename = "Images", alias = "images", default)]
    pub images: Option<Vec<String>>,

    #[serde(rename = "Status", alias = "status", default)]
    pub status: Option<String>,

    #[serde(rename = "Stream", alias = "stream", default)]
    pub stream: Option<String>,

    #[serde(rename = "Error", alias = "error", default)]
    pub error: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Secret summary from list operations
#[derive(Debug, Clone, Deserialize)]
pub struct PodmanSecretSummary {
    #[serde(rename = "ID", alias = "Id", alias = "id", default)]
    pub id: Option<String>,

    #[serde(rename = "Spec", alias = "spec", default)]
    pub spec: Option<Value>,

    #[serde(rename = "Version", alias = "version", default)]
    pub version: Option<Value>,

    #[serde(
        rename = "CreatedAt",
        alias = "createdAt",
        alias = "created_at",
        default
    )]
    pub created_at: Option<String>,

    #[serde(
        rename = "UpdatedAt",
        alias = "updatedAt",
        alias = "updated_at",
        default
    )]
    pub updated_at: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Machine summary from list operations
#[derive(Debug, Clone, Deserialize)]
pub struct PodmanMachineSummary {
    #[serde(rename = "Name", alias = "name", default)]
    pub name: Option<String>,

    #[serde(rename = "Default", alias = "default", default)]
    pub default: Option<bool>,

    #[serde(rename = "Created", alias = "created", default)]
    pub created: Option<Value>,

    #[serde(rename = "Running", alias = "running", default)]
    pub running: Option<bool>,

    #[serde(rename = "LastUp", alias = "lastUp", alias = "last_up", default)]
    pub last_up: Option<Value>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Container logs output
#[derive(Debug, Clone)]
pub struct PodmanLogs {
    pub output: String,
}

/// Exec result output
#[derive(Debug, Clone)]
pub struct PodmanExecResult {
    pub output: String,
    pub exit_code: Option<i32>,
}

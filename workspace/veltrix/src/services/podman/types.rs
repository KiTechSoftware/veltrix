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

    #[serde(rename = "APIVersion", alias = "apiVersion", alias = "api_version", default)]
    pub api_version: Option<String>,

    #[serde(rename = "GitCommit", alias = "gitCommit", alias = "git_commit", default)]
    pub git_commit: Option<String>,

    #[serde(rename = "GoVersion", alias = "goVersion", alias = "go_version", default)]
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
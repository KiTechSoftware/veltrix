use crate::{
    error::{Result, VeltrixError},
    os::process::cmd::async_cmd,
};

use super::{
    base_cmd, PodmanCliClient,
};

use crate::services::podman::{
    spec::{PodmanBackendUsed, PodmanCliSpec, PodmanResponse},
    types::{PodmanContainerSummary, PodmanInfo, PodmanVersion},
};

impl PodmanCliClient {
    pub async fn info_async(&self) -> Result<PodmanResponse<PodmanInfo>> {
        run_json_async(&self.spec, ["info", "--format", "json"]).await
    }

    pub async fn version_async(&self) -> Result<PodmanResponse<PodmanVersion>> {
        run_json_async(&self.spec, ["version", "--format", "json"]).await
    }

    pub async fn containers_async(&self) -> Result<PodmanResponse<Vec<PodmanContainerSummary>>> {
        run_json_async(&self.spec, ["ps", "--all", "--format", "json"]).await
    }
}

async fn run_json_async<T, I, S>(spec: &PodmanCliSpec, args: I) -> Result<PodmanResponse<T>>
where
    T: serde::de::DeserializeOwned,
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let output = async_cmd::run(base_cmd(spec).args(args)).await?;

    if !output.status.success() {
        return Err(VeltrixError::config_invalid(format!(
            "podman cli failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let data = serde_json::from_slice(&output.stdout)
        .map_err(|err| VeltrixError::config_invalid(format!("invalid podman json: {err}")))?;

    Ok(PodmanResponse {
        backend: PodmanBackendUsed::Cli {
            binary: spec.binary.clone(),
            sudo: spec.sudo,
            uid: spec.uid,
            gid: spec.gid,
        },
        data,
    })
}
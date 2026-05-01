#[cfg(feature = "async")]
mod with_async;

use crate::{
    error::{Result, VeltrixError},
    os::process::cmd::{spec::CmdSpec, std_cmd},
};

use super::{
    spec::{PodmanBackendUsed, PodmanCliSpec, PodmanResponse},
    types::{PodmanContainerSummary, PodmanInfo, PodmanVersion},
};

#[allow(unused_imports)]
#[cfg(feature = "async")]
pub use with_async::*;

#[derive(Debug, Clone)]
pub struct PodmanCliClient {
    spec: PodmanCliSpec,
}

impl PodmanCliClient {
    pub fn new(spec: PodmanCliSpec) -> Self {
        Self { spec }
    }

    pub fn spec(&self) -> &PodmanCliSpec {
        &self.spec
    }

    pub fn info(&self) -> Result<PodmanResponse<PodmanInfo>> {
        run_json(&self.spec, ["info", "--format", "json"])
    }

    pub fn version(&self) -> Result<PodmanResponse<PodmanVersion>> {
        run_json(&self.spec, ["version", "--format", "json"])
    }

    pub fn containers(&self) -> Result<PodmanResponse<Vec<PodmanContainerSummary>>> {
        run_json(&self.spec, ["ps", "--all", "--format", "json"])
    }
}

fn run_json<T, I, S>(spec: &PodmanCliSpec, args: I) -> Result<PodmanResponse<T>>
where
    T: serde::de::DeserializeOwned,
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let output = std_cmd::run(base_cmd(spec).args(args))?;

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

pub(super) fn base_cmd(spec: &PodmanCliSpec) -> CmdSpec {
    let mut cmd = CmdSpec::new(&spec.binary);

    if spec.sudo {
        cmd = cmd.sudo();
    }

    if let Some(uid) = spec.uid {
        cmd = cmd.uid(uid);
    }

    if let Some(gid) = spec.gid {
        cmd = cmd.gid(gid);
    }

    cmd
}
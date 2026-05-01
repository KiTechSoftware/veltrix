#[cfg(feature = "async")]
mod with_async;

use crate::error::{Result, VeltrixError};
use crate::os::process::cmd::{spec::CmdSpec, std_cmd};

use super::spec::{PodmanBackendUsed, PodmanCliSpec, PodmanEmptyResponse, PodmanResponse};
use super::types::{
    PodmanContainerSummary, PodmanExecResult, PodmanImageSummary, PodmanInfo, PodmanLogs,
    PodmanMachineSummary, PodmanPodSummary, PodmanSecretSummary, PodmanVersion,
};

#[allow(unused_imports)]
#[cfg(feature = "async")]
pub use with_async::*;

/// Podman CLI client for sync command execution
#[derive(Debug, Clone)]
pub struct PodmanCliClient {
    spec: PodmanCliSpec,
}

impl PodmanCliClient {
    /// Create a new Podman CLI client with the given spec
    pub fn new(spec: PodmanCliSpec) -> Self {
        Self { spec }
    }

    /// Get the underlying spec
    pub fn spec(&self) -> &PodmanCliSpec {
        &self.spec
    }

    // ============ Info & Version ============

    /// Get Podman system information
    pub fn info(&self) -> Result<PodmanResponse<PodmanInfo>> {
        run_json(&self.spec, ["info", "--format", "json"])
    }

    /// Get Podman version information
    pub fn version(&self) -> Result<PodmanResponse<PodmanVersion>> {
        run_json(&self.spec, ["version", "--format", "json"])
    }

    // ============ Container Operations ============

    /// List all containers (running and stopped)
    pub fn containers(&self) -> Result<PodmanResponse<Vec<PodmanContainerSummary>>> {
        run_json(&self.spec, ["ps", "--all", "--format", "json"])
    }

    /// Inspect a container by name or ID
    pub fn inspect_container(&self, container: &str) -> Result<PodmanResponse<serde_json::Value>> {
        run_json(&self.spec, ["inspect", container, "--format", "json"])
    }

    /// Get container logs
    pub fn container_logs(&self, container: &str) -> Result<PodmanResponse<PodmanLogs>> {
        run_text(&self.spec, ["logs", container])
    }

    /// Stop a running container
    pub fn stop_container(&self, container: &str) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["stop", container])
    }

    /// Start a stopped container
    pub fn start_container(&self, container: &str) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["start", container])
    }

    /// Restart a container
    pub fn restart_container(&self, container: &str) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["restart", container])
    }

    /// Remove a container
    pub fn remove_container(&self, container: &str) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["rm", container])
    }

    /// Execute a command in a running container
    pub fn exec(&self, container: &str, cmd: &[&str]) -> Result<PodmanResponse<PodmanExecResult>> {
        let mut args = vec!["exec", container];
        args.extend(cmd);
        run_exec(&self.spec, args)
    }

    // ============ Pod Operations ============

    /// Create a pod
    pub fn create_pod(&self, pod_name: &str) -> Result<PodmanResponse<String>> {
        run_string(&self.spec, ["pod", "create", "--name", pod_name])
    }

    /// List all pods
    pub fn pods(&self) -> Result<PodmanResponse<Vec<PodmanPodSummary>>> {
        run_json(&self.spec, ["pod", "ps", "--format", "json"])
    }

    /// Inspect a pod
    pub fn inspect_pod(&self, pod: &str) -> Result<PodmanResponse<serde_json::Value>> {
        run_json(&self.spec, ["pod", "inspect", pod, "--format", "json"])
    }

    /// Stop a pod
    pub fn stop_pod(&self, pod: &str) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["pod", "stop", pod])
    }

    /// Remove a pod
    pub fn remove_pod(&self, pod: &str) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["pod", "rm", pod])
    }

    // ============ Image Operations ============

    /// List all images
    pub fn images(&self) -> Result<PodmanResponse<Vec<PodmanImageSummary>>> {
        run_json(&self.spec, ["images", "--format", "json"])
    }

    /// Inspect an image
    pub fn inspect_image(&self, image: &str) -> Result<PodmanResponse<serde_json::Value>> {
        run_json(&self.spec, ["inspect", image, "--format", "json"])
    }

    /// Pull an image from registry
    pub fn pull_image(&self, image: &str) -> Result<PodmanResponse<String>> {
        run_string(&self.spec, ["pull", image])
    }

    /// Remove an image
    pub fn remove_image(&self, image: &str) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["rmi", image])
    }

    // ============ Secret Operations ============

    /// List all secrets
    pub fn secrets(&self) -> Result<PodmanResponse<Vec<PodmanSecretSummary>>> {
        run_json(&self.spec, ["secret", "ls", "--format", "json"])
    }

    /// Remove a secret
    pub fn remove_secret(&self, secret: &str) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["secret", "rm", secret])
    }

    // ============ Machine Operations (macOS/Windows) ============

    /// List all machines
    pub fn machines(&self) -> Result<PodmanResponse<Vec<PodmanMachineSummary>>> {
        run_json(&self.spec, ["machine", "list", "--format", "json"])
    }

    /// Initialize a new machine
    pub fn machine_init(&self, name: &str) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["machine", "init", name])
    }

    /// Start a machine
    pub fn machine_start(&self, name: &str) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["machine", "start", name])
    }

    /// Stop a machine
    pub fn machine_stop(&self, name: &str) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["machine", "stop", name])
    }

    // ============ System Operations ============

    /// System prune (remove unused containers, images, networks, volumes)
    pub fn system_prune(&self) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["system", "prune", "--force"])
    }

    /// System reset (clear all Podman data)
    pub fn system_reset(&self) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["system", "reset", "--force"])
    }

    // ============ Kubernetes YAML Operations ============

    /// Generate Kubernetes YAML from a pod or container
    pub fn generate_kube(&self, pod_or_container: &str) -> Result<PodmanResponse<String>> {
        run_string(&self.spec, ["generate", "kube", pod_or_container])
    }
}

// ============ Helper Functions ============

fn run_json<T, I, S>(spec: &PodmanCliSpec, args: I) -> Result<PodmanResponse<T>>
where
    T: serde::de::DeserializeOwned,
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let output = run_command(spec, args)?;
    let data: T = serde_json::from_str(&output)
        .map_err(|e| VeltrixError::config_invalid(format!("Invalid JSON: {}", e)))?;

    Ok(PodmanResponse::new(
        data,
        PodmanBackendUsed::Cli {
            binary: spec.binary.clone(),
            sudo: spec.sudo,
            uid: spec.uid,
            gid: spec.gid,
        },
    ))
}

fn run_string<I, S>(spec: &PodmanCliSpec, args: I) -> Result<PodmanResponse<String>>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let output = run_command(spec, args)?;
    Ok(PodmanResponse::new(
        output.trim().to_string(),
        PodmanBackendUsed::Cli {
            binary: spec.binary.clone(),
            sudo: spec.sudo,
            uid: spec.uid,
            gid: spec.gid,
        },
    ))
}

fn run_text<I, S>(spec: &PodmanCliSpec, args: I) -> Result<PodmanResponse<PodmanLogs>>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let output = run_command(spec, args)?;
    Ok(PodmanResponse::new(
        PodmanLogs { output },
        PodmanBackendUsed::Cli {
            binary: spec.binary.clone(),
            sudo: spec.sudo,
            uid: spec.uid,
            gid: spec.gid,
        },
    ))
}

fn run_exec<I, S>(spec: &PodmanCliSpec, args: I) -> Result<PodmanResponse<PodmanExecResult>>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let output = run_command(spec, args)?;
    Ok(PodmanResponse::new(
        PodmanExecResult {
            output,
            exit_code: None,
        },
        PodmanBackendUsed::Cli {
            binary: spec.binary.clone(),
            sudo: spec.sudo,
            uid: spec.uid,
            gid: spec.gid,
        },
    ))
}

fn run_empty<I, S>(spec: &PodmanCliSpec, args: I) -> Result<PodmanEmptyResponse>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let _output = run_command(spec, args)?;
    Ok(PodmanEmptyResponse::new(PodmanBackendUsed::Cli {
        binary: spec.binary.clone(),
        sudo: spec.sudo,
        uid: spec.uid,
        gid: spec.gid,
    }))
}

fn run_command<I, S>(spec: &PodmanCliSpec, args: I) -> Result<String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args_vec: Vec<String> = args.into_iter().map(|s| s.into()).collect();

    let cmd = if spec.sudo {
        let mut sudo_args = vec!["sudo".to_string()];
        sudo_args.push(spec.binary.clone());
        sudo_args.extend(args_vec);
        CmdSpec::new(sudo_args[0].clone()).args(&sudo_args[1..])
    } else {
        CmdSpec::new(&spec.binary).args(&args_vec)
    };

    let result = std_cmd(cmd)?;
    Ok(result.stdout)
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

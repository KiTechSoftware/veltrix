#[cfg(feature = "async")]
mod with_async;

use crate::error::{Result, VeltrixError};
use crate::os::process::cmd::{spec::CmdSpec, std_cmd};

use super::quadlet::PodmanAutoUpdatePolicy;
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

    /// Run a container with caller-provided `podman run` arguments.
    pub fn run_container<I, S>(&self, args: I) -> Result<PodmanResponse<String>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_string(&self.spec, prefixed_args(["run"], args))
    }

    /// Run a container with Podman's auto-update label set.
    pub fn run_container_auto_update<I, S>(
        &self,
        policy: PodmanAutoUpdatePolicy,
        args: I,
    ) -> Result<PodmanResponse<String>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_string(
            &self.spec,
            prefixed_args(
                [
                    "run".to_string(),
                    "--label".to_string(),
                    policy.as_label_arg(),
                ],
                args,
            ),
        )
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

    /// Run a container in an existing pod.
    pub fn run_container_in_pod<I, S>(&self, pod: &str, args: I) -> Result<PodmanResponse<String>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut command_args = vec!["run".to_string(), "--pod".to_string(), pod.to_string()];
        command_args.extend(args.into_iter().map(Into::into));
        run_string(&self.spec, command_args)
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

    /// Build an image with caller-provided `podman build` arguments.
    pub fn build_image<I, S>(&self, args: I) -> Result<PodmanResponse<String>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_string(&self.spec, prefixed_args(["build"], args))
    }

    /// Inspect an image
    pub fn inspect_image(&self, image: &str) -> Result<PodmanResponse<serde_json::Value>> {
        run_json(&self.spec, ["inspect", image, "--format", "json"])
    }

    /// Pull an image from registry
    pub fn pull_image(&self, image: &str) -> Result<PodmanResponse<String>> {
        run_string(&self.spec, ["pull", image])
    }

    /// Push an image to a registry.
    pub fn push_image(&self, image: &str) -> Result<PodmanResponse<String>> {
        run_string(&self.spec, ["push", image])
    }

    /// Tag an image.
    pub fn tag_image(&self, image: &str, target: &str) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["tag", image, target])
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

    /// Create a secret from a file or `-` for stdin.
    pub fn create_secret(&self, name: &str, source: &str) -> Result<PodmanResponse<String>> {
        run_string(&self.spec, ["secret", "create", name, source])
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

    /// SSH into a machine and run a command.
    pub fn machine_ssh(&self, name: &str, cmd: &[&str]) -> Result<PodmanResponse<String>> {
        let mut args = vec!["machine", "ssh", name];
        args.extend(cmd);
        run_string(&self.spec, args)
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

    /// Play a Kubernetes YAML file.
    pub fn play_kube(&self, yaml_path: &str) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["kube", "play", yaml_path])
    }

    /// Tear down resources created from a Kubernetes YAML file.
    pub fn play_kube_down(&self, yaml_path: &str) -> Result<PodmanEmptyResponse> {
        run_empty(&self.spec, ["kube", "down", yaml_path])
    }

    /// Generate legacy systemd unit output from a pod or container.
    ///
    /// New deployments should prefer Quadlet files. This method exists for
    /// compatibility with Podman's legacy generated-unit workflow.
    pub fn generate_systemd_legacy(
        &self,
        pod_or_container: &str,
    ) -> Result<PodmanResponse<String>> {
        run_string(&self.spec, ["generate", "systemd", pod_or_container])
    }

    /// Run `podman auto-update`.
    pub fn auto_update(&self) -> Result<PodmanResponse<String>> {
        run_string(&self.spec, ["auto-update"])
    }

    /// Run `podman auto-update` with caller-provided arguments.
    pub fn auto_update_with_args<I, S>(&self, args: I) -> Result<PodmanResponse<String>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_string(&self.spec, prefixed_args(["auto-update"], args))
    }

    /// Run `podman compose up` with caller-provided compose arguments.
    pub fn compose_up<I, S>(&self, args: I) -> Result<PodmanEmptyResponse>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_empty(&self.spec, prefixed_args(["compose", "up"], args))
    }

    /// Run `podman compose down`.
    pub fn compose_down<I, S>(&self, args: I) -> Result<PodmanEmptyResponse>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_empty(&self.spec, prefixed_args(["compose", "down"], args))
    }

    /// Run `podman compose logs`.
    pub fn compose_logs<I, S>(&self, args: I) -> Result<PodmanResponse<PodmanLogs>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_text(&self.spec, prefixed_args(["compose", "logs"], args))
    }

    /// Run `podman compose ps`.
    pub fn compose_ps<I, S>(&self, args: I) -> Result<PodmanResponse<String>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_string(&self.spec, prefixed_args(["compose", "ps"], args))
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
        .map_err(|err| VeltrixError::parsing(format!("invalid podman json: {err}")))?;

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

    let cmd = base_cmd(spec).args(args_vec);

    let result = std_cmd::run(cmd)?;

    if !result.status.success() {
        return Err(VeltrixError::service(
            "podman",
            String::from_utf8_lossy(&result.stderr).trim().to_string(),
        ));
    }

    String::from_utf8(result.stdout)
        .map_err(|err| VeltrixError::parsing(format!("invalid podman stdout utf-8: {err}")))
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

fn prefixed_args<P, I, S>(prefix: P, args: I) -> Vec<String>
where
    P: IntoIterator,
    P::Item: Into<String>,
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut command_args: Vec<String> = prefix.into_iter().map(Into::into).collect();
    command_args.extend(args.into_iter().map(Into::into));
    command_args
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    #[test]
    fn prefixed_args_preserve_caller_args() {
        assert_eq!(
            prefixed_args(["run", "--rm"], ["alpine", "true"]),
            strings(&["run", "--rm", "alpine", "true"])
        );
    }

    #[test]
    fn auto_update_policy_builds_label_arg() {
        assert_eq!(
            prefixed_args(
                [
                    "run".to_string(),
                    "--label".to_string(),
                    PodmanAutoUpdatePolicy::Registry.as_label_arg(),
                ],
                ["docker.io/library/caddy:latest"]
            ),
            strings(&[
                "run",
                "--label",
                "io.containers.autoupdate=registry",
                "docker.io/library/caddy:latest"
            ])
        );
    }

    #[test]
    fn workflow_command_prefixes_cover_v0_3_cli_surface() {
        let cases = [
            (
                prefixed_args(["run"], ["alpine"]),
                strings(&["run", "alpine"]),
            ),
            (
                prefixed_args(["build"], ["-t", "image", "."]),
                strings(&["build", "-t", "image", "."]),
            ),
            (
                prefixed_args(["compose", "up"], ["-d"]),
                strings(&["compose", "up", "-d"]),
            ),
            (
                prefixed_args(["compose", "down"], std::iter::empty::<&str>()),
                strings(&["compose", "down"]),
            ),
            (
                prefixed_args(["compose", "logs"], ["web"]),
                strings(&["compose", "logs", "web"]),
            ),
            (
                prefixed_args(["compose", "ps"], std::iter::empty::<&str>()),
                strings(&["compose", "ps"]),
            ),
            (
                prefixed_args(["auto-update"], ["--dry-run"]),
                strings(&["auto-update", "--dry-run"]),
            ),
        ];

        for (actual, expected) in cases {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn command_shapes_cover_container_pod_image_and_secret_workflows() {
        let cases = [
            strings(&["ps", "--all", "--format", "json"]),
            strings(&["inspect", "web", "--format", "json"]),
            strings(&["logs", "web"]),
            strings(&["stop", "web"]),
            strings(&["start", "web"]),
            strings(&["restart", "web"]),
            strings(&["rm", "web"]),
            strings(&["exec", "web", "true"]),
            strings(&["pod", "create", "--name", "stack"]),
            strings(&["pod", "ps", "--format", "json"]),
            strings(&["pod", "inspect", "stack", "--format", "json"]),
            strings(&["pod", "stop", "stack"]),
            strings(&["pod", "rm", "stack"]),
            strings(&["images", "--format", "json"]),
            strings(&["inspect", "image", "--format", "json"]),
            strings(&["pull", "image"]),
            strings(&["push", "image"]),
            strings(&["tag", "image", "target"]),
            strings(&["rmi", "image"]),
            strings(&["secret", "ls", "--format", "json"]),
            strings(&["secret", "create", "name", "-"]),
            strings(&["secret", "rm", "name"]),
        ];

        let spec = PodmanCliSpec::new();

        for args in cases {
            assert_eq!(base_cmd(&spec).args(args.clone()).args, args);
        }
    }
}

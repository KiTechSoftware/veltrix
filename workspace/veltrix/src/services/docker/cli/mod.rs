#[cfg(feature = "async")]
mod with_async;

use crate::error::{Result, VeltrixError};
use crate::os::process::cmd::{spec::CmdSpec, std_cmd};

use super::spec::{
    DockerBackendUsed, DockerCliSpec, DockerComposeSpec, DockerEmptyResponse, DockerResponse,
};
use super::types::{
    DockerComposeServiceSummary, DockerContainerSummary, DockerImageSummary, DockerInfo,
    DockerLogs, DockerNetworkSummary, DockerSystemDf, DockerVersion, DockerVolumeSummary,
};

#[allow(unused_imports)]
#[cfg(feature = "async")]
pub use with_async::*;

/// Docker CLI client for sync command execution.
#[derive(Debug, Clone)]
pub struct DockerCliClient {
    spec: DockerCliSpec,
}

impl DockerCliClient {
    /// Create a new Docker CLI client with the given spec.
    pub fn new(spec: DockerCliSpec) -> Self {
        Self { spec }
    }

    /// Get the underlying spec.
    pub fn spec(&self) -> &DockerCliSpec {
        &self.spec
    }

    /// Get Docker version information.
    pub fn version(&self) -> Result<DockerResponse<DockerVersion>> {
        run_json(&self.spec, ["version", "--format", "json"])
    }

    /// Get Docker system info.
    pub fn info(&self) -> Result<DockerResponse<DockerInfo>> {
        run_json(&self.spec, ["info", "--format", "json"])
    }

    /// List all containers.
    pub fn containers(&self) -> Result<DockerResponse<Vec<DockerContainerSummary>>> {
        run_json_lines_or_array(&self.spec, ["ps", "--all", "--format", "json"])
    }

    /// Run a container with caller-provided `docker run` arguments.
    pub fn run_container<I, S>(&self, args: I) -> Result<DockerResponse<String>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_string(&self.spec, prefixed_args(["run"], args))
    }

    /// Build an image with caller-provided `docker build` arguments.
    pub fn build_image<I, S>(&self, args: I) -> Result<DockerResponse<String>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_string(&self.spec, prefixed_args(["build"], args))
    }

    /// Execute a command in a running container.
    pub fn exec_container(&self, container: &str, cmd: &[&str]) -> Result<DockerResponse<String>> {
        let mut args = vec!["exec", container];
        args.extend(cmd);
        run_string(&self.spec, args)
    }

    /// Inspect a container by name or ID.
    pub fn inspect_container(&self, container: &str) -> Result<DockerResponse<serde_json::Value>> {
        run_json(&self.spec, ["inspect", container])
    }

    /// Get container logs.
    pub fn container_logs(&self, container: &str) -> Result<DockerResponse<DockerLogs>> {
        let output = run_command(&self.spec, ["logs", container])?;
        Ok(DockerResponse::new(
            DockerLogs { output },
            backend_used(&self.spec),
        ))
    }

    /// Stop a running container.
    pub fn stop_container(&self, container: &str) -> Result<DockerEmptyResponse> {
        run_empty(&self.spec, ["stop", container])
    }

    /// Start a stopped container.
    pub fn start_container(&self, container: &str) -> Result<DockerEmptyResponse> {
        run_empty(&self.spec, ["start", container])
    }

    /// Restart a container.
    pub fn restart_container(&self, container: &str) -> Result<DockerEmptyResponse> {
        run_empty(&self.spec, ["restart", container])
    }

    /// Remove a container.
    pub fn remove_container(&self, container: &str) -> Result<DockerEmptyResponse> {
        run_empty(&self.spec, ["rm", container])
    }

    /// List images.
    pub fn images(&self) -> Result<DockerResponse<Vec<DockerImageSummary>>> {
        run_json_lines_or_array(&self.spec, ["images", "--format", "json"])
    }

    /// Pull an image.
    pub fn pull_image(&self, image: &str) -> Result<DockerResponse<String>> {
        run_string(&self.spec, ["pull", image])
    }

    /// Push an image.
    pub fn push_image(&self, image: &str) -> Result<DockerResponse<String>> {
        run_string(&self.spec, ["push", image])
    }

    /// Tag an image.
    pub fn tag_image(&self, image: &str, target: &str) -> Result<DockerEmptyResponse> {
        run_empty(&self.spec, ["tag", image, target])
    }

    /// Remove an image.
    pub fn remove_image(&self, image: &str) -> Result<DockerEmptyResponse> {
        run_empty(&self.spec, ["rmi", image])
    }

    /// Create a Docker volume.
    pub fn create_volume(&self, name: &str) -> Result<DockerResponse<DockerVolumeSummary>> {
        run_json(&self.spec, ["volume", "create", name])
    }

    /// List Docker volumes.
    pub fn volumes(&self) -> Result<DockerResponse<Vec<DockerVolumeSummary>>> {
        run_json_lines_or_array(&self.spec, ["volume", "ls", "--format", "json"])
    }

    /// Inspect a Docker volume.
    pub fn inspect_volume(&self, volume: &str) -> Result<DockerResponse<serde_json::Value>> {
        run_json(&self.spec, ["volume", "inspect", volume])
    }

    /// Remove a Docker volume.
    pub fn remove_volume(&self, volume: &str) -> Result<DockerEmptyResponse> {
        run_empty(&self.spec, ["volume", "rm", volume])
    }

    /// Create a Docker network.
    pub fn create_network(&self, name: &str) -> Result<DockerResponse<String>> {
        run_string(&self.spec, ["network", "create", name])
    }

    /// List Docker networks.
    pub fn networks(&self) -> Result<DockerResponse<Vec<DockerNetworkSummary>>> {
        run_json_lines_or_array(&self.spec, ["network", "ls", "--format", "json"])
    }

    /// Inspect a Docker network.
    pub fn inspect_network(&self, network: &str) -> Result<DockerResponse<serde_json::Value>> {
        run_json(&self.spec, ["network", "inspect", network])
    }

    /// Remove a Docker network.
    pub fn remove_network(&self, network: &str) -> Result<DockerEmptyResponse> {
        run_empty(&self.spec, ["network", "rm", network])
    }

    /// Connect a container to a network.
    pub fn connect_network(&self, network: &str, container: &str) -> Result<DockerEmptyResponse> {
        run_empty(&self.spec, ["network", "connect", network, container])
    }

    /// Prune unused Docker resources.
    pub fn system_prune(&self) -> Result<DockerEmptyResponse> {
        run_empty(&self.spec, ["system", "prune", "--force"])
    }

    /// Return Docker disk-usage information.
    pub fn system_df(&self) -> Result<DockerResponse<DockerSystemDf>> {
        run_json(&self.spec, ["system", "df", "--format", "json"])
    }
}

/// Docker Compose CLI client.
#[derive(Debug, Clone)]
pub struct DockerComposeClient {
    spec: DockerComposeSpec,
}

impl DockerComposeClient {
    /// Create a Docker Compose client with the given spec.
    pub fn new(spec: DockerComposeSpec) -> Self {
        Self { spec }
    }

    /// Get the underlying spec.
    pub fn spec(&self) -> &DockerComposeSpec {
        &self.spec
    }

    /// Run `docker compose up`.
    pub fn up<I, S>(&self, args: I) -> Result<DockerEmptyResponse>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_compose_empty(&self.spec, prefixed_args(["up"], args))
    }

    /// Run `docker compose down`.
    pub fn down<I, S>(&self, args: I) -> Result<DockerEmptyResponse>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_compose_empty(&self.spec, prefixed_args(["down"], args))
    }

    /// Run `docker compose logs`.
    pub fn logs<I, S>(&self, args: I) -> Result<DockerResponse<DockerLogs>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let output = run_compose_command(&self.spec, prefixed_args(["logs"], args))?;
        Ok(DockerResponse::new(
            DockerLogs { output },
            compose_backend_used(&self.spec),
        ))
    }

    /// Run `docker compose ps --format json`.
    pub fn ps(&self) -> Result<DockerResponse<Vec<DockerComposeServiceSummary>>> {
        run_compose_json(&self.spec, ["ps", "--format", "json"])
    }
}

fn run_json<T, I, S>(spec: &DockerCliSpec, args: I) -> Result<DockerResponse<T>>
where
    T: serde::de::DeserializeOwned,
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let output = run_command(spec, args)?;
    let data = serde_json::from_str(&output)
        .map_err(|err| VeltrixError::parsing(format!("invalid docker json: {err}")))?;

    Ok(DockerResponse::new(data, backend_used(spec)))
}

fn run_json_lines_or_array<T, I, S>(spec: &DockerCliSpec, args: I) -> Result<DockerResponse<Vec<T>>>
where
    T: serde::de::DeserializeOwned,
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let output = run_command(spec, args)?;
    let data = parse_json_lines_or_array(&output)?;

    Ok(DockerResponse::new(data, backend_used(spec)))
}

fn run_string<I, S>(spec: &DockerCliSpec, args: I) -> Result<DockerResponse<String>>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let output = run_command(spec, args)?;

    Ok(DockerResponse::new(
        output.trim().to_string(),
        backend_used(spec),
    ))
}

fn run_empty<I, S>(spec: &DockerCliSpec, args: I) -> Result<DockerEmptyResponse>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    run_command(spec, args)?;

    Ok(DockerEmptyResponse::new(backend_used(spec)))
}

fn run_compose_json<T, I, S>(spec: &DockerComposeSpec, args: I) -> Result<DockerResponse<T>>
where
    T: serde::de::DeserializeOwned,
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let output = run_compose_command(spec, args)?;
    let data = serde_json::from_str(&output)
        .map_err(|err| VeltrixError::parsing(format!("invalid docker compose json: {err}")))?;

    Ok(DockerResponse::new(data, compose_backend_used(spec)))
}

fn run_compose_empty<I, S>(spec: &DockerComposeSpec, args: I) -> Result<DockerEmptyResponse>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    run_compose_command(spec, args)?;

    Ok(DockerEmptyResponse::new(compose_backend_used(spec)))
}

fn run_command<I, S>(spec: &DockerCliSpec, args: I) -> Result<String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let result = std_cmd::run(base_cmd(spec).args(args))?;

    if !result.status.success() {
        return Err(VeltrixError::service(
            "docker",
            String::from_utf8_lossy(&result.stderr).trim().to_string(),
        ));
    }

    String::from_utf8(result.stdout)
        .map_err(|err| VeltrixError::parsing(format!("invalid docker stdout utf-8: {err}")))
}

fn run_compose_command<I, S>(spec: &DockerComposeSpec, args: I) -> Result<String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let result = std_cmd::run(compose_cmd(spec).args(args))?;

    if !result.status.success() {
        return Err(VeltrixError::service(
            "docker-compose",
            String::from_utf8_lossy(&result.stderr).trim().to_string(),
        ));
    }

    String::from_utf8(result.stdout)
        .map_err(|err| VeltrixError::parsing(format!("invalid docker compose stdout utf-8: {err}")))
}

pub(super) fn base_cmd(spec: &DockerCliSpec) -> CmdSpec {
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

fn compose_cmd(spec: &DockerComposeSpec) -> CmdSpec {
    let mut cmd = CmdSpec::new(&spec.binary);

    if let Some(file) = &spec.compose_file {
        cmd = cmd.args(["--file".to_string(), file.clone()]);
    }

    if let Some(project_name) = &spec.project_name {
        cmd = cmd.args(["--project-name".to_string(), project_name.clone()]);
    }

    cmd
}

fn backend_used(spec: &DockerCliSpec) -> DockerBackendUsed {
    DockerBackendUsed::Cli {
        binary: spec.binary.clone(),
        sudo: spec.sudo,
        uid: spec.uid,
        gid: spec.gid,
    }
}

fn compose_backend_used(spec: &DockerComposeSpec) -> DockerBackendUsed {
    DockerBackendUsed::Compose {
        binary: spec.binary.clone(),
        compose_file: spec.compose_file.clone(),
        project_name: spec.project_name.clone(),
    }
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

fn parse_json_lines_or_array<T>(output: &str) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    let trimmed = output.trim();

    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    if trimmed.starts_with('[') {
        return serde_json::from_str(trimmed)
            .map_err(|err| VeltrixError::parsing(format!("invalid docker json array: {err}")));
    }

    trimmed
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str(line)
                .map_err(|err| VeltrixError::parsing(format!("invalid docker json line: {err}")))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    #[test]
    fn docker_command_prefixes_cover_cli_workflows() {
        let cases = [
            (
                prefixed_args(["run"], ["alpine"]),
                strings(&["run", "alpine"]),
            ),
            (
                prefixed_args(["build"], ["-t", "app", "."]),
                strings(&["build", "-t", "app", "."]),
            ),
            (
                prefixed_args(["network", "connect"], ["web", "app"]),
                strings(&["network", "connect", "web", "app"]),
            ),
            (
                prefixed_args(["volume", "inspect"], ["data"]),
                strings(&["volume", "inspect", "data"]),
            ),
        ];

        for (actual, expected) in cases {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn compose_cmd_includes_file_and_project() {
        let spec = DockerComposeSpec::new()
            .compose_file("compose.yaml")
            .project_name("demo");

        assert_eq!(
            compose_cmd(&spec).args,
            strings(&["--file", "compose.yaml", "--project-name", "demo"])
        );
    }

    #[test]
    fn parses_json_lines_and_arrays() {
        let lines = r#"{"Name":"one"}
{"Name":"two"}"#;
        let parsed: Vec<DockerVolumeSummary> = parse_json_lines_or_array(lines).unwrap();

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].name.as_deref(), Some("one"));

        let array = r#"[{"Name":"one"},{"Name":"two"}]"#;
        let parsed: Vec<DockerVolumeSummary> = parse_json_lines_or_array(array).unwrap();

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[1].name.as_deref(), Some("two"));
    }
}

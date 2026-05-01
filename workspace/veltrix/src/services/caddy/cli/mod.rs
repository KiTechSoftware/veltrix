#[cfg(feature = "async")]
mod with_async;

use crate::error::{Result, VeltrixError};
use crate::os::process::cmd::{spec::CmdSpec, std_cmd};

use super::spec::{CaddyBackendUsed, CaddyCliSpec, CaddyEmptyResponse, CaddyResponse};
use super::types::CaddyCliOutput;

#[allow(unused_imports)]
#[cfg(feature = "async")]
pub use with_async::*;

/// Caddy CLI client.
#[derive(Debug, Clone)]
pub struct CaddyCliClient {
    spec: CaddyCliSpec,
}

impl CaddyCliClient {
    /// Create a Caddy CLI client.
    pub fn new(spec: CaddyCliSpec) -> Self {
        Self { spec }
    }

    /// Get the underlying spec.
    pub fn spec(&self) -> &CaddyCliSpec {
        &self.spec
    }

    /// Run `caddy validate`.
    pub fn validate<I, S>(&self, args: I) -> Result<CaddyEmptyResponse>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_empty(&self.spec, prefixed_args(["validate"], args))
    }

    /// Run `caddy fmt`.
    pub fn fmt<I, S>(&self, args: I) -> Result<CaddyResponse<CaddyCliOutput>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_output(&self.spec, prefixed_args(["fmt"], args))
    }

    /// Run `caddy reload`.
    pub fn reload<I, S>(&self, args: I) -> Result<CaddyEmptyResponse>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_empty(&self.spec, prefixed_args(["reload"], args))
    }

    /// Run `caddy stop`.
    pub fn stop<I, S>(&self, args: I) -> Result<CaddyEmptyResponse>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_empty(&self.spec, prefixed_args(["stop"], args))
    }

    /// Run `caddy run` with caller-provided arguments.
    ///
    /// This is a blocking command if Caddy stays in the foreground.
    pub fn run<I, S>(&self, args: I) -> Result<CaddyResponse<CaddyCliOutput>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_output(&self.spec, prefixed_args(["run"], args))
    }

    /// Run `caddy adapt`.
    pub fn adapt<I, S>(&self, args: I) -> Result<CaddyResponse<CaddyCliOutput>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_output(&self.spec, prefixed_args(["adapt"], args))
    }

    /// Run `caddy hash-password`.
    pub fn hash_password<I, S>(&self, args: I) -> Result<CaddyResponse<CaddyCliOutput>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_output(&self.spec, prefixed_args(["hash-password"], args))
    }
}

fn run_output<I, S>(spec: &CaddyCliSpec, args: I) -> Result<CaddyResponse<CaddyCliOutput>>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let output = run_command(spec, args)?;

    Ok(CaddyResponse {
        backend: backend_used(spec),
        data: output,
    })
}

fn run_empty<I, S>(spec: &CaddyCliSpec, args: I) -> Result<CaddyEmptyResponse>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    run_command(spec, args)?;

    Ok(CaddyEmptyResponse {
        backend: backend_used(spec),
    })
}

fn run_command<I, S>(spec: &CaddyCliSpec, args: I) -> Result<CaddyCliOutput>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let result = std_cmd::run(CmdSpec::new(&spec.binary).args(args))?;

    if !result.status.success() {
        return Err(VeltrixError::service(
            "caddy",
            String::from_utf8_lossy(&result.stderr).trim().to_string(),
        ));
    }

    Ok(CaddyCliOutput {
        stdout: String::from_utf8(result.stdout)
            .map_err(|err| VeltrixError::parsing(format!("invalid caddy stdout: {err}")))?,
        stderr: String::from_utf8(result.stderr)
            .map_err(|err| VeltrixError::parsing(format!("invalid caddy stderr: {err}")))?,
    })
}

fn backend_used(spec: &CaddyCliSpec) -> CaddyBackendUsed {
    CaddyBackendUsed::Cli {
        binary: spec.binary.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    #[test]
    fn caddy_cli_prefixes_cover_v0_5_workflows() {
        assert_eq!(
            prefixed_args(["validate"], ["--config", "Caddyfile"]),
            strings(&["validate", "--config", "Caddyfile"])
        );
        assert_eq!(
            prefixed_args(["reload"], ["--adapter", "caddyfile"]),
            strings(&["reload", "--adapter", "caddyfile"])
        );
        assert_eq!(
            prefixed_args(["adapt"], ["--config", "Caddyfile"]),
            strings(&["adapt", "--config", "Caddyfile"])
        );
    }
}

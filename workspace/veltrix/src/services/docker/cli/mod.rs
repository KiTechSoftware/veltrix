#[cfg(feature = "async")]
mod with_async;

use crate::error::{Result, VeltrixError};

use super::spec::{DockerCliSpec, DockerResponse};
use super::types::{DockerContainerSummary, DockerInfo, DockerVersion};

#[allow(unused_imports)]
#[cfg(feature = "async")]
pub use with_async::*;

/// Docker CLI client for sync command execution
#[derive(Debug, Clone)]
pub struct DockerCliClient {
    spec: DockerCliSpec,
}

impl DockerCliClient {
    /// Create a new Docker CLI client with the given spec
    pub fn new(spec: DockerCliSpec) -> Self {
        Self { spec }
    }

    /// Get the underlying spec
    pub fn spec(&self) -> &DockerCliSpec {
        &self.spec
    }

    /// Get Docker version information
    pub fn version(&self) -> Result<DockerResponse<DockerVersion>> {
        Err(not_implemented())
    }

    /// Get Docker system info
    pub fn info(&self) -> Result<DockerResponse<DockerInfo>> {
        Err(not_implemented())
    }

    /// List all containers
    pub fn containers(&self) -> Result<DockerResponse<Vec<DockerContainerSummary>>> {
        Err(not_implemented())
    }
}

fn not_implemented() -> VeltrixError {
    VeltrixError::service(
        "docker",
        "Docker v1 API not yet implemented (v0.5.0 target)",
    )
}

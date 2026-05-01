#[cfg(feature = "async")]
mod with_async;

use crate::error::Result;

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
        // Placeholder: will implement in v0.5.0
        Err(crate::error::VeltrixError::Config(
            "Docker v1 API not yet implemented (v0.5.0 target)".to_string(),
        ))
    }

    /// Get Docker system info
    pub fn info(&self) -> Result<DockerResponse<DockerInfo>> {
        // Placeholder: will implement in v0.5.0
        Err(crate::error::VeltrixError::Config(
            "Docker v1 API not yet implemented (v0.5.0 target)".to_string(),
        ))
    }

    /// List all containers
    pub fn containers(&self) -> Result<DockerResponse<Vec<DockerContainerSummary>>> {
        // Placeholder: will implement in v0.5.0
        Err(crate::error::VeltrixError::Config(
            "Docker v1 API not yet implemented (v0.5.0 target)".to_string(),
        ))
    }
}

use crate::error::Result;

use super::types::VpnTunnelStatus;

/// Operations common to every VPN backend.
pub trait VpnClient {
    /// Status of every tunnel this backend can see. A missing binary is an
    /// error, not an empty list.
    fn status(&self) -> Result<Vec<VpnTunnelStatus>>;
}

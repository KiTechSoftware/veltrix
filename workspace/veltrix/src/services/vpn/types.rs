/// Tunnel protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum VpnKind {
    WireGuard,
    OpenVpn,
}

/// Tunnel state as observed from the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VpnState {
    Unknown,
    Up,
    Down,
    Connecting,
    Connected,
    Disconnected,
    Error,
}

/// One peer of a tunnel. Never carries key material other than the peer's
/// public key.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VpnPeerStatus {
    pub id: Option<String>,
    pub endpoint: Option<String>,
    /// Seconds since the Unix epoch of the latest handshake, as text.
    /// `None` means no handshake has happened.
    pub latest_handshake: Option<String>,
    pub transfer_rx: Option<u64>,
    pub transfer_tx: Option<u64>,
}

/// Status of one tunnel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VpnTunnelStatus {
    pub name: String,
    pub kind: VpnKind,
    pub state: VpnState,
    pub interface: Option<String>,
    pub local_ips: Vec<String>,
    pub peers: Vec<VpnPeerStatus>,
}

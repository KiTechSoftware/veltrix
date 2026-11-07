use crate::error::{Result, VeltrixError};
use crate::os::process::cmd::{spec::CmdSpec, std_cmd};

use super::spec::VpnClient;
use super::types::{VpnKind, VpnPeerStatus, VpnState, VpnTunnelStatus};

/// WireGuard CLI backend configuration.
#[derive(Debug, Clone)]
pub struct WireGuardCliSpec {
    /// `wg` executable name or path.
    pub wg: String,
    /// `wg-quick` executable name or path.
    pub wg_quick: String,
    /// Run both through `sudo`.
    pub sudo: bool,
}

impl Default for WireGuardCliSpec {
    fn default() -> Self {
        Self {
            wg: "wg".into(),
            wg_quick: "wg-quick".into(),
            sudo: false,
        }
    }
}

/// WireGuard client backed by `wg-quick` and `wg`.
#[derive(Debug, Clone)]
pub struct WireGuardCliClient {
    spec: WireGuardCliSpec,
}

impl WireGuardCliClient {
    pub fn new(spec: WireGuardCliSpec) -> Self {
        Self { spec }
    }

    /// Bring a tunnel up from a config file path.
    pub fn up(&self, config_path: &str) -> Result<()> {
        self.quick("up", config_path)
    }

    /// Bring a tunnel down using the same config file path.
    pub fn down(&self, config_path: &str) -> Result<()> {
        self.quick("down", config_path)
    }

    fn cmd(&self, program: &str) -> CmdSpec {
        let c = CmdSpec::new(program);
        if self.spec.sudo { c.sudo() } else { c }
    }

    fn quick(&self, action: &str, config_path: &str) -> Result<()> {
        let out = std_cmd::run(self.cmd(&self.spec.wg_quick).arg(action).arg(config_path))?;
        if out.status.success() {
            return Ok(());
        }
        Err(VeltrixError::Service {
            service: "wireguard".into(),
            reason: format!(
                "wg-quick {action} failed ({}): {}",
                out.status,
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        })
    }
}

impl VpnClient for WireGuardCliClient {
    fn status(&self) -> Result<Vec<VpnTunnelStatus>> {
        let out = std_cmd::run(self.cmd(&self.spec.wg).args(["show", "all", "dump"]))?;
        if !out.status.success() {
            return Err(VeltrixError::Service {
                service: "wireguard".into(),
                reason: format!(
                    "wg show failed ({}): {}",
                    out.status,
                    String::from_utf8_lossy(&out.stderr).trim()
                ),
            });
        }
        Ok(parse_dump(&String::from_utf8_lossy(&out.stdout)))
    }
}

/// Parse `wg show all dump`.
///
/// Interface lines have 5 tab-separated fields
/// (`iface private public listen fwmark`), peer lines 9
/// (`iface public psk endpoint allowed handshake rx tx keepalive`). The
/// private key and preshared key fields are never read.
pub fn parse_dump(dump: &str) -> Vec<VpnTunnelStatus> {
    let mut tunnels: Vec<VpnTunnelStatus> = Vec::new();
    for line in dump.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        match f.len() {
            5 => tunnels.push(VpnTunnelStatus {
                name: f[0].to_string(),
                kind: VpnKind::WireGuard,
                // The interface exists; it is only usable once a peer has
                // completed a handshake.
                state: VpnState::Up,
                interface: Some(f[0].to_string()),
                local_ips: Vec::new(),
                peers: Vec::new(),
            }),
            9 => {
                let none = |s: &str| (s != "(none)" && !s.is_empty()).then(|| s.to_string());
                let handshake = f[5].parse::<u64>().ok().filter(|t| *t > 0);
                if let Some(t) = tunnels.iter_mut().rev().find(|t| t.name == f[0]) {
                    t.peers.push(VpnPeerStatus {
                        id: none(f[1]),
                        endpoint: none(f[3]),
                        latest_handshake: handshake.map(|t| t.to_string()),
                        transfer_rx: f[6].parse().ok(),
                        transfer_tx: f[7].parse().ok(),
                    });
                }
            }
            _ => {}
        }
    }
    tunnels
}

#[cfg(test)]
mod tests {
    use super::*;

    const DUMP: &str = "wg0\tPRIVATESECRET\tPUBIF\t51820\toff\n\
wg0\tPEERPUB\tPSKSECRET\t203.0.113.1:51820\t10.0.0.0/24\t1700000000\t100\t200\toff\n\
wg1\tPRIVATESECRET2\tPUBIF2\t0\toff\n\
wg1\tPEER2\t(none)\t(none)\t0.0.0.0/0\t0\t0\t0\toff\n";

    #[test]
    fn parses_interfaces_and_peers() {
        let t = parse_dump(DUMP);
        assert_eq!(t.len(), 2);
        assert_eq!(t[0].name, "wg0");
        assert_eq!(t[0].peers.len(), 1);
        let p = &t[0].peers[0];
        assert_eq!(p.id.as_deref(), Some("PEERPUB"));
        assert_eq!(p.endpoint.as_deref(), Some("203.0.113.1:51820"));
        assert_eq!(p.latest_handshake.as_deref(), Some("1700000000"));
        assert_eq!((p.transfer_rx, p.transfer_tx), (Some(100), Some(200)));
    }

    #[test]
    fn a_peer_that_never_handshook_has_no_handshake() {
        let p = &parse_dump(DUMP)[1].peers[0];
        assert_eq!(p.latest_handshake, None);
        assert_eq!(p.endpoint, None);
    }

    #[test]
    fn secrets_never_reach_the_status() {
        let s = format!("{:?}", parse_dump(DUMP));
        assert!(!s.contains("SECRET"), "{s}");
    }

    #[test]
    fn garbage_is_ignored() {
        assert!(parse_dump("").is_empty());
        assert!(parse_dump("not a dump\nstill\tnot\n").is_empty());
    }
}

use std::path::{Path, PathBuf};

use crate::error::{Result, VeltrixError};
use crate::os::process::cmd::{spec::CmdSpec, std_cmd};

use super::spec::VpnClient;
use super::types::{VpnKind, VpnState, VpnTunnelStatus};

const READY_MARKER: &str = "Initialization Sequence Completed";

/// OpenVPN CLI backend configuration.
#[derive(Debug, Clone)]
pub struct OpenVpnCliSpec {
    /// `openvpn` executable name or path.
    pub openvpn: String,
    /// Run through `sudo` (creating a tunnel device needs privileges).
    pub sudo: bool,
}

impl Default for OpenVpnCliSpec {
    fn default() -> Self {
        Self {
            openvpn: "openvpn".into(),
            sudo: false,
        }
    }
}

/// OpenVPN client that runs `openvpn` as a daemon and tracks it through a pid
/// file and a log file next to the config (`<config>.pid`, `<config>.log`).
///
/// Configs must come from [`super::VpnConfigStore`] (or pass
/// [`super::validate_openvpn`]); this client does not re-validate.
#[derive(Debug, Clone)]
pub struct OpenVpnCliClient {
    spec: OpenVpnCliSpec,
}

impl OpenVpnCliClient {
    pub fn new(spec: OpenVpnCliSpec) -> Self {
        Self { spec }
    }

    fn cmd(&self, program: &str) -> CmdSpec {
        let c = CmdSpec::new(program);
        if self.spec.sudo { c.sudo() } else { c }
    }

    /// Start a tunnel from a config file path.
    pub fn up_config(&self, config_path: &str) -> Result<()> {
        let cfg = Path::new(config_path);
        if !cfg.is_file() {
            return Err(VeltrixError::InvalidPath {
                path: cfg.to_path_buf(),
                reason: "config does not exist".into(),
            });
        }
        let out = std_cmd::run(
            self.cmd(&self.spec.openvpn)
                .arg("--config")
                .arg(config_path)
                .arg("--daemon")
                .arg("--writepid")
                .arg(sidecar(cfg, "pid").to_string_lossy())
                .arg("--log")
                .arg(sidecar(cfg, "log").to_string_lossy())
                // No scripts, ever.
                .args(["--script-security", "0"]),
        )?;
        if out.status.success() {
            return Ok(());
        }
        Err(VeltrixError::Service {
            service: "openvpn".into(),
            reason: format!(
                "openvpn failed to start ({}): {}",
                out.status,
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        })
    }

    /// Ask the tunnel started from `config_path` to stop (SIGTERM).
    ///
    /// The pid file lives in a user-writable directory and the signal may be
    /// sent as root, so the pid is only signalled if it really is an
    /// `openvpn` process.
    pub fn signal_term(&self, config_path: &str) -> Result<()> {
        let pid = read_pid(&sidecar(Path::new(config_path), "pid")).ok_or_else(|| {
            VeltrixError::Service {
                service: "openvpn".into(),
                reason: "no running tunnel for this config".into(),
            }
        })?;
        if !self.is_openvpn(pid) {
            return Err(VeltrixError::Service {
                service: "openvpn".into(),
                reason: format!("pid {pid} is not an openvpn process; refusing to signal it"),
            });
        }
        let out = std_cmd::run(self.cmd("kill").args(["-TERM", &pid.to_string()]))?;
        if out.status.success() {
            Ok(())
        } else {
            Err(VeltrixError::Service {
                service: "openvpn".into(),
                reason: format!("kill failed: {}", String::from_utf8_lossy(&out.stderr).trim()),
            })
        }
    }

    fn is_openvpn(&self, pid: u32) -> bool {
        std_cmd::run(CmdSpec::new("ps").args(["-p", &pid.to_string(), "-o", "comm="]))
            .map(|o| String::from_utf8_lossy(&o.stdout).contains("openvpn"))
            .unwrap_or(false)
    }

    /// Status of the tunnel started from one config, or `None` when it is not
    /// running.
    pub fn status_for(&self, config_path: &str) -> Option<VpnTunnelStatus> {
        let cfg = Path::new(config_path);
        let pid = read_pid(&sidecar(cfg, "pid"))?;
        if !self.is_openvpn(pid) {
            return None;
        }
        let log = std::fs::read_to_string(sidecar(cfg, "log")).unwrap_or_default();
        Some(VpnTunnelStatus {
            name: cfg.file_stem()?.to_string_lossy().into_owned(),
            kind: VpnKind::OpenVpn,
            state: state_from_log(&log),
            interface: None,
            local_ips: Vec::new(),
            peers: Vec::new(),
        })
    }
}

impl VpnClient for OpenVpnCliClient {
    /// OpenVPN has no system-wide registry, so a client cannot enumerate
    /// tunnels by itself; use [`OpenVpnCliClient::status_for`] with the
    /// stored configs.
    fn status(&self) -> Result<Vec<VpnTunnelStatus>> {
        Ok(Vec::new())
    }
}

fn sidecar(config: &Path, ext: &str) -> PathBuf {
    let mut p = config.as_os_str().to_owned();
    p.push(".");
    p.push(ext);
    PathBuf::from(p)
}

fn read_pid(path: &Path) -> Option<u32> {
    std::fs::read_to_string(path).ok()?.trim().parse().ok()
}

/// Derive a state from the daemon's log. Only an explicit completion marker
/// means Connected; an authentication failure is an Error.
pub fn state_from_log(log: &str) -> VpnState {
    if log.contains("AUTH_FAILED") || log.contains("TLS Error") && log.contains("Exiting") {
        VpnState::Error
    } else if log.contains(READY_MARKER) {
        VpnState::Connected
    } else {
        VpnState::Connecting
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_is_connected_only_after_the_completion_marker() {
        assert_eq!(state_from_log(""), VpnState::Connecting);
        assert_eq!(state_from_log("TCP/UDP: Preserving recently used remote"), VpnState::Connecting);
        assert_eq!(
            state_from_log("... Initialization Sequence Completed\n"),
            VpnState::Connected
        );
        assert_eq!(state_from_log("AUTH_FAILED"), VpnState::Error);
    }

    #[test]
    fn sidecar_paths_sit_next_to_the_config() {
        assert_eq!(
            sidecar(Path::new("/a/openvpn/work.ovpn"), "pid"),
            PathBuf::from("/a/openvpn/work.ovpn.pid")
        );
    }

    #[test]
    fn missing_config_and_missing_pid_are_errors_not_success() {
        let c = OpenVpnCliClient::new(OpenVpnCliSpec::default());
        assert!(c.up_config("/nonexistent/x.ovpn").is_err());
        assert!(c.signal_term("/nonexistent/x.ovpn").is_err());
        assert!(c.status_for("/nonexistent/x.ovpn").is_none());
    }
}

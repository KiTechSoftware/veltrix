//! Validation of tunnel names and configs.
//!
//! Configs are untrusted input: whoever supplies one (an import, a form, or a
//! server) must not be able to run a command as root. wg-quick executes
//! `PreUp`/`PostUp`/`PreDown`/`PostDown`, and OpenVPN runs the scripts and
//! plugins named by several directives, so those are rejected outright.

use crate::error::{Result, VeltrixError};

use super::store::VpnConfigKind;

const MAX_CONFIG_BYTES: usize = 256 * 1024;

fn invalid(field: &str, reason: impl Into<String>) -> VeltrixError {
    VeltrixError::Validation {
        field: field.to_string(),
        reason: reason.into(),
    }
}

/// A tunnel name becomes a file name and (for WireGuard) an interface name.
pub fn validate_name(kind: VpnConfigKind, name: &str) -> Result<()> {
    let max = match kind {
        // IFNAMSIZ - 1; wg-quick derives the interface from the file name.
        VpnConfigKind::WireGuard => 15,
        VpnConfigKind::OpenVpn => 64,
    };
    if name.is_empty() || name.len() > max {
        return Err(invalid("name", format!("must be 1 to {max} characters")));
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '='| '+'))
    {
        return Err(invalid(
            "name",
            "only letters, digits, '_', '-', '=' and '+' are allowed",
        ));
    }
    Ok(())
}

fn check_text(content: &str) -> Result<()> {
    if content.trim().is_empty() {
        return Err(invalid("config", "is empty"));
    }
    if content.len() > MAX_CONFIG_BYTES {
        return Err(invalid("config", "is too large"));
    }
    if content
        .chars()
        .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
    {
        return Err(invalid("config", "contains control characters"));
    }
    Ok(())
}

/// Validate a `wg-quick` config. Rejects hooks, unknown sections and unknown
/// lines, and requires an explicit `AllowedIPs` and `PublicKey` on every peer
/// and a `PrivateKey` in the interface.
pub fn validate_wireguard(content: &str) -> Result<()> {
    check_text(content)?;
    #[derive(PartialEq)]
    enum Sec {
        None,
        Interface,
        Peer,
    }
    let mut sec = Sec::None;
    let (mut ifaces, mut peers) = (0, 0);
    let mut has_private = false;
    let (mut peer_allowed, mut peer_key) = (true, true);

    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if let Some(rest) = line.strip_prefix('[') {
            let name = rest
                .strip_suffix(']')
                .ok_or_else(|| invalid("config", "malformed section header"))?;
            if sec == Sec::Peer && !(peer_allowed && peer_key) {
                return Err(invalid("config", "a [Peer] needs PublicKey and AllowedIPs"));
            }
            match name.trim().to_ascii_lowercase().as_str() {
                "interface" => {
                    ifaces += 1;
                    sec = Sec::Interface;
                }
                "peer" => {
                    peers += 1;
                    sec = Sec::Peer;
                    peer_allowed = false;
                    peer_key = false;
                }
                _ => return Err(invalid("config", "unknown section")),
            }
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| invalid("config", "line is not `key = value`"))?;
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();
        match (&sec, key.as_str()) {
            (Sec::None, _) => return Err(invalid("config", "key outside a section")),
            (_, "preup" | "postup" | "predown" | "postdown") => {
                return Err(invalid(
                    "config",
                    "PreUp/PostUp/PreDown/PostDown hooks are not allowed",
                ));
            }
            (Sec::Interface, "saveconfig") => {
                return Err(invalid("config", "SaveConfig is not allowed"));
            }
            (
                Sec::Interface,
                "address" | "dns" | "mtu" | "listenport" | "fwmark" | "table",
            ) => {}
            (Sec::Interface, "privatekey") => {
                if value.is_empty() {
                    return Err(invalid("config", "PrivateKey is empty"));
                }
                has_private = true;
            }
            (Sec::Peer, "publickey") => {
                if value.is_empty() {
                    return Err(invalid("config", "PublicKey is empty"));
                }
                peer_key = true;
            }
            (Sec::Peer, "allowedips") => {
                if value.is_empty() {
                    return Err(invalid("config", "AllowedIPs is empty"));
                }
                peer_allowed = true;
            }
            (Sec::Peer, "endpoint" | "presharedkey" | "persistentkeepalive") => {}
            _ => return Err(invalid("config", format!("unsupported key `{key}`"))),
        }
    }
    if sec == Sec::Peer && !(peer_allowed && peer_key) {
        return Err(invalid("config", "a [Peer] needs PublicKey and AllowedIPs"));
    }
    if ifaces != 1 || !has_private {
        return Err(invalid("config", "needs one [Interface] with a PrivateKey"));
    }
    if peers == 0 {
        return Err(invalid("config", "needs at least one [Peer]"));
    }
    Ok(())
}

/// Directives that run programs, load code, include other files or write
/// files as root. None of them is accepted.
const OPENVPN_FORBIDDEN: &[&str] = &[
    "up",
    "down",
    "down-pre",
    "up-restart",
    "route-up",
    "route-pre-down",
    "ipchange",
    "learn-address",
    "client-connect",
    "client-disconnect",
    "tls-verify",
    "auth-user-pass-verify",
    "plugin",
    "script-security",
    "config",
    "management",
    "management-client-user",
    "management-external-key",
    "log",
    "log-append",
    "writepid",
    "status",
    "daemon",
    "syslog",
    "cd",
    "chroot",
    "setcon",
    "tmp-dir",
    "dev-node",
    "iproute",
    "tls-export-cert",
    "engine",
    "pkcs11-providers",
    "ifconfig-noexec",
    "route-noexec",
];

/// Validate an OpenVPN config. Rejects any directive that runs a program or
/// plugin, includes another file, or writes a file as root.
pub fn validate_openvpn(content: &str) -> Result<()> {
    check_text(content)?;
    let mut remote = false;
    let mut inline: Option<String> = None;
    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if let Some(tag) = &inline {
            if line.starts_with("</") && line[2..].trim_end_matches('>') == tag {
                inline = None;
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix('<') {
            let tag = rest.trim_end_matches('>').to_string();
            if tag.starts_with('/') {
                return Err(invalid("config", "unbalanced inline block"));
            }
            if matches!(tag.as_str(), "up" | "down" | "plugin" | "connection") {
                // <connection> holds a nested remote; refuse to parse it
                // loosely and refuse the script forms outright.
                if tag != "connection" {
                    return Err(invalid("config", "script blocks are not allowed"));
                }
                remote = true;
                continue;
            }
            inline = Some(tag);
            continue;
        }
        let directive = line
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_start_matches("--")
            .to_ascii_lowercase();
        if OPENVPN_FORBIDDEN.contains(&directive.as_str()) {
            return Err(invalid(
                "config",
                format!("directive `{directive}` is not allowed"),
            ));
        }
        if directive == "remote" {
            remote = true;
        }
    }
    if inline.is_some() {
        return Err(invalid("config", "unterminated inline block"));
    }
    if !remote {
        return Err(invalid("config", "has no `remote`"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD_WG: &str = "[Interface]\nPrivateKey = AAAA\nAddress = 10.0.0.2/32\nDNS = 10.0.0.1\n\n[Peer]\nPublicKey = BBBB\nAllowedIPs = 10.0.0.0/24\nEndpoint = 203.0.113.1:51820\n";

    #[test]
    fn wireguard_good_config_passes() {
        assert!(validate_wireguard(GOOD_WG).is_ok());
    }

    #[test]
    fn wireguard_hooks_are_rejected_in_any_case() {
        for hook in ["PostUp", "postup", "PreUp", "PreDown", "PostDOWN"] {
            let cfg = GOOD_WG.replace("DNS = 10.0.0.1", &format!("{hook} = curl x|sh"));
            assert!(validate_wireguard(&cfg).is_err(), "{hook} accepted");
        }
    }

    #[test]
    fn wireguard_newline_injection_through_a_field_is_rejected() {
        // What a hostile server value like this becomes once interpolated.
        let cfg = GOOD_WG.replace("10.0.0.0/24", "10.0.0.0/24\nPostUp = id > /tmp/x");
        assert!(validate_wireguard(&cfg).is_err());
    }

    #[test]
    fn wireguard_missing_allowed_ips_is_rejected_not_defaulted() {
        let cfg = GOOD_WG.replace("AllowedIPs = 10.0.0.0/24\n", "");
        assert!(validate_wireguard(&cfg).is_err());
        let empty = GOOD_WG.replace("10.0.0.0/24", "");
        assert!(validate_wireguard(&empty).is_err());
    }

    #[test]
    fn wireguard_unknown_things_are_rejected() {
        assert!(validate_wireguard(&format!("{GOOD_WG}[Extra]\nA = b\n")).is_err());
        assert!(validate_wireguard(&GOOD_WG.replace("DNS", "SaveConfig")).is_err());
        assert!(validate_wireguard(&GOOD_WG.replace("DNS", "Bogus")).is_err());
        assert!(validate_wireguard("[Interface]\nPrivateKey = A\n").is_err());
        assert!(validate_wireguard("").is_err());
        assert!(validate_wireguard("PrivateKey = A\n").is_err());
        assert!(validate_wireguard(&GOOD_WG.replace("AAAA", "A\u{0}A")).is_err());
    }

    const GOOD_OVPN: &str = "client\ndev tun\nproto udp\nremote 203.0.113.1 1194\nca ca.crt\n<cert>\nxxx\nup /bin/sh\n</cert>\n";

    #[test]
    fn openvpn_good_config_passes_and_inline_blocks_are_opaque() {
        // `up` inside an inline <cert> block is data, not a directive.
        assert!(validate_openvpn(GOOD_OVPN).is_ok());
    }

    #[test]
    fn openvpn_script_and_plugin_directives_are_rejected() {
        for d in [
            "up /tmp/x", "down /tmp/x", "script-security 2", "plugin /tmp/p.so",
            "route-up x", "client-connect x", "tls-verify x", "config other.ovpn",
            "--up /tmp/x", "UP /tmp/x", "log /etc/passwd", "management 127.0.0.1 1",
            "writepid /etc/x", "auth-user-pass-verify x via-env",
        ] {
            let cfg = format!("{GOOD_OVPN}{d}\n");
            assert!(validate_openvpn(&cfg).is_err(), "`{d}` accepted");
        }
        assert!(validate_openvpn("client\n<up>\nx\n</up>\nremote a 1\n").is_err());
    }

    #[test]
    fn openvpn_needs_a_remote_and_balanced_blocks() {
        assert!(validate_openvpn("client\ndev tun\n").is_err());
        assert!(validate_openvpn("remote a 1\n<ca>\nx\n").is_err());
        assert!(validate_openvpn("remote a 1\n</ca>\n").is_err());
    }

    #[test]
    fn names_cannot_traverse_or_exceed_interface_limit() {
        for bad in ["", "../x", "a/b", ".hidden", "a b", "a\nb", "sixteen_chars_xx"] {
            assert!(validate_name(VpnConfigKind::WireGuard, bad).is_err(), "{bad:?}");
        }
        assert!(validate_name(VpnConfigKind::WireGuard, "wg-home_1").is_ok());
        assert!(validate_name(VpnConfigKind::OpenVpn, "a-long-name-over-fifteen").is_ok());
    }
}

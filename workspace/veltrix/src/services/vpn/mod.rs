//! VPN client integration (WireGuard and OpenVPN through system binaries).
//!
//! * [`VpnConfigStore`] keeps tunnel configs in a private per-user directory.
//!   Every config is validated before it is written: scripts and hooks are
//!   rejected (they run as root), and WireGuard peers must carry an explicit
//!   `AllowedIPs`.
//! * [`WireGuardCliClient`] drives `wg-quick` and `wg`.
//! * [`OpenVpnCliClient`] drives `openvpn` as a daemon.
//!
//! This module never generates or stores keys and never logs config content.

pub mod spec;
pub mod store;
pub mod types;
pub mod validate;

#[cfg(feature = "vpn-openvpn")]
pub mod openvpn;
#[cfg(feature = "vpn-wireguard")]
pub mod wireguard;

#[cfg(feature = "vpn-openvpn")]
pub use openvpn::{OpenVpnCliClient, OpenVpnCliSpec};
pub use spec::VpnClient;
pub use store::{VpnConfigKind, VpnConfigStore};
pub use types::*;
pub use validate::{validate_name, validate_openvpn, validate_wireguard};
#[cfg(feature = "vpn-wireguard")]
pub use wireguard::{WireGuardCliClient, WireGuardCliSpec};

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::error::{Result, VeltrixError};
use crate::os::paths::user_config_dir;

use super::validate::{validate_name, validate_openvpn, validate_wireguard};

/// Which protocol a stored config belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VpnConfigKind {
    WireGuard,
    OpenVpn,
}

impl VpnConfigKind {
    pub fn subdir(self) -> &'static str {
        match self {
            Self::WireGuard => "wireguard",
            Self::OpenVpn => "openvpn",
        }
    }
    pub fn extension(self) -> &'static str {
        match self {
            Self::WireGuard => "conf",
            Self::OpenVpn => "ovpn",
        }
    }
}

/// A private, per-user directory of tunnel configs.
///
/// * the directories are created `0700` and configs `0600` from the first
///   byte (no chmod after the fact);
/// * writes go to an exclusively created temporary file and are renamed into
///   place, so a reader never sees a partial file;
/// * every config is validated before it is written.
#[derive(Debug, Clone)]
pub struct VpnConfigStore {
    root: PathBuf,
}

impl VpnConfigStore {
    /// The default per-user store (`<config dir>/veltrix/vpn`).
    pub fn user_default() -> Result<Self> {
        Ok(Self::at(user_config_dir("veltrix")?.join("vpn")))
    }

    /// A store rooted at `root` (used by tests and embedders).
    pub fn at(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn path_for(&self, kind: VpnConfigKind, name: &str) -> Result<PathBuf> {
        validate_name(kind, name)?;
        Ok(self
            .root
            .join(kind.subdir())
            .join(format!("{name}.{}", kind.extension())))
    }

    pub fn read(&self, kind: VpnConfigKind, name: &str) -> Result<String> {
        Ok(fs::read_to_string(self.path_for(kind, name)?)?)
    }

    pub fn delete(&self, kind: VpnConfigKind, name: &str) -> Result<()> {
        Ok(fs::remove_file(self.path_for(kind, name)?)?)
    }

    /// Validate and store a config, replacing any config of the same name.
    pub fn write(&self, kind: VpnConfigKind, name: &str, content: &str) -> Result<PathBuf> {
        let path = self.path_for(kind, name)?;
        match kind {
            VpnConfigKind::WireGuard => validate_wireguard(content)?,
            VpnConfigKind::OpenVpn => validate_openvpn(content)?,
        }
        let dir = path.parent().ok_or_else(|| VeltrixError::InvalidPath {
            path: path.clone(),
            reason: "no parent directory".into(),
        })?;
        create_private_dir(&self.root)?;
        create_private_dir(dir)?;

        let tmp = dir.join(format!(".{name}.{}.tmp", std::process::id()));
        let _ = fs::remove_file(&tmp);
        let result = (|| -> std::io::Result<()> {
            let mut f = private_new_file(&tmp)?;
            f.write_all(content.as_bytes())?;
            f.sync_all()?;
            fs::rename(&tmp, &path)
        })();
        if let Err(e) = result {
            let _ = fs::remove_file(&tmp);
            return Err(e.into());
        }
        Ok(path)
    }
}

#[cfg(unix)]
fn create_private_dir(dir: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
    if dir.exists() {
        // Tighten a directory created by something else.
        return fs::set_permissions(dir, fs::Permissions::from_mode(0o700));
    }
    fs::DirBuilder::new().recursive(true).mode(0o700).create(dir)
}

#[cfg(not(unix))]
fn create_private_dir(dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dir)
}

#[cfg(unix)]
fn private_new_file(path: &Path) -> std::io::Result<fs::File> {
    use std::os::unix::fs::OpenOptionsExt;
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
}

#[cfg(not(unix))]
fn private_new_file(path: &Path) -> std::io::Result<fs::File> {
    fs::OpenOptions::new().write(true).create_new(true).open(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    const WG: &str = "[Interface]\nPrivateKey = AAAA\nAddress = 10.0.0.2/32\n[Peer]\nPublicKey = BBBB\nAllowedIPs = 10.0.0.0/24\n";

    fn scratch(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("veltrix-vpn-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        d
    }

    #[test]
    fn write_read_delete_roundtrip() {
        let root = scratch("rt");
        let s = VpnConfigStore::at(&root);
        let p = s.write(VpnConfigKind::WireGuard, "home", WG).unwrap();
        assert_eq!(p, root.join("wireguard/home.conf"));
        assert_eq!(s.read(VpnConfigKind::WireGuard, "home").unwrap(), WG);
        s.delete(VpnConfigKind::WireGuard, "home").unwrap();
        assert!(s.read(VpnConfigKind::WireGuard, "home").is_err());
        let _ = fs::remove_dir_all(root);
    }

    #[cfg(unix)]
    #[test]
    fn files_and_dirs_are_private_from_creation() {
        use std::os::unix::fs::PermissionsExt;
        let root = scratch("perm");
        let s = VpnConfigStore::at(&root);
        let p = s.write(VpnConfigKind::WireGuard, "home", WG).unwrap();
        let mode = |p: &Path| fs::metadata(p).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode(&p), 0o600);
        assert_eq!(mode(&root), 0o700);
        assert_eq!(mode(p.parent().unwrap()), 0o700);
        // No temp files are left behind.
        let left: Vec<_> = fs::read_dir(p.parent().unwrap()).unwrap().collect();
        assert_eq!(left.len(), 1);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn invalid_config_is_not_written_and_does_not_replace_a_good_one() {
        let root = scratch("inv");
        let s = VpnConfigStore::at(&root);
        s.write(VpnConfigKind::WireGuard, "home", WG).unwrap();
        let bad = WG.replace("Address", "PostUp = x\nAddress");
        assert!(s.write(VpnConfigKind::WireGuard, "home", &bad).is_err());
        assert_eq!(s.read(VpnConfigKind::WireGuard, "home").unwrap(), WG);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn traversal_names_never_reach_the_filesystem() {
        let root = scratch("trav");
        let s = VpnConfigStore::at(&root);
        assert!(s.write(VpnConfigKind::WireGuard, "../evil", WG).is_err());
        assert!(!root.join("evil.conf").exists());
        assert!(s.path_for(VpnConfigKind::OpenVpn, "a/b").is_err());
        let _ = fs::remove_dir_all(root);
    }
}

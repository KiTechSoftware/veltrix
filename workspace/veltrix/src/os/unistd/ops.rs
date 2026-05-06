use std::path::Path;
use std::{ffi::CString, os::fd::AsRawFd};
use std::fs::File;
use std::io;

use crate::os::process::cmd::spec::CmdSpec;
use crate::os::process::cmd::std_cmd;
use crate::os::unistd::{gid_by_groupname, uid_by_username};

use super::{Uid, Gid};

/// Change ownership of a filesystem object at `path`.
///
/// `uid` and `gid` are optional; pass `None` to leave the corresponding
/// value unchanged (this maps to `(uid_t)-1` / `(gid_t)-1` for the libc call).
///
/// This is a thin, safe wrapper around `libc::chown` that returns an
/// `io::Result<()>` carrying the last OS error on failure.
pub fn chown(path: impl AsRef<Path>, uid: Option<Uid>, gid: Option<Gid>) -> io::Result<()> {
    use std::os::unix::ffi::OsStrExt;

    let path = path.as_ref();
    let bytes = path.as_os_str().as_bytes();
    let c_path = CString::new(bytes).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "path contains interior NUL"))?;

    // libc expects (uid_t)-1 / (gid_t)-1 to indicate "do not change".
    let uid_raw: libc::uid_t = uid.map(|u| u.0).unwrap_or(!0 as libc::uid_t);
    let gid_raw: libc::gid_t = gid.map(|g| g.0).unwrap_or(!0 as libc::gid_t);

    let ret = unsafe { libc::chown(c_path.as_ptr(), uid_raw, gid_raw) };

    if ret == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

/// Change ownership by file descriptor.
///
/// `file` is any object implementing `AsRawFd` (e.g., `&File`). `uid` and
/// `gid` are optional; pass `None` to leave the corresponding value unchanged.
pub fn fchown(file: &File, uid: Option<Uid>, gid: Option<Gid>) -> io::Result<()> {
    let fd = file.as_raw_fd();

    let uid_raw: libc::uid_t = uid.map(|u| u.0).unwrap_or(!0 as libc::uid_t);
    let gid_raw: libc::gid_t = gid.map(|g| g.0).unwrap_or(!0 as libc::gid_t);

    let ret = unsafe { libc::fchown(fd, uid_raw, gid_raw) };

    if ret == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

/// Change ownership of a filesystem object at `path` without following symlinks.
///
/// Behaves like `chown` but uses `lchown(2)` to operate on symlinks themselves.
pub fn lchown(path: impl AsRef<Path>, uid: Option<Uid>, gid: Option<Gid>) -> io::Result<()> {
    use std::os::unix::ffi::OsStrExt;

    let path = path.as_ref();
    let bytes = path.as_os_str().as_bytes();
    let c_path = CString::new(bytes).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "path contains interior NUL"))?;

    let uid_raw: libc::uid_t = uid.map(|u| u.0).unwrap_or(!0 as libc::uid_t);
    let gid_raw: libc::gid_t = gid.map(|g| g.0).unwrap_or(!0 as libc::gid_t);

    let ret = unsafe { libc::lchown(c_path.as_ptr(), uid_raw, gid_raw) };

    if ret == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

/// Change ownership of `path`, attempting a privileged fallback via `sudo`.
pub fn sudo_chown(path: impl AsRef<std::path::Path>, uid: Option<Uid>, gid: Option<Gid>) -> io::Result<()> {
    match chown(&path, uid, gid) {
        Ok(()) => return Ok(()),
        Err(err) => {
            if err.kind() != io::ErrorKind::PermissionDenied || super::is_root() {
                return Err(err);
            }
        }
    }

    let owner_arg = match (uid, gid) {
        (Some(u), Some(g)) => format!("{}:{}", u.as_raw(), g.as_raw()),
        (Some(u), None) => format!("{}", u.as_raw()),
        (None, Some(g)) => format!(":{}", g.as_raw()),
        (None, None) => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "both uid and gid are None"));
        }
    };

    let path_str = path.as_ref().to_string_lossy().into_owned();

    let spec = CmdSpec::new("chown").arg(owner_arg).arg(path_str).sudo();

    match std_cmd::status_ok(spec) {
        Ok(true) => Ok(()),
        Ok(false) => Err(io::Error::new(io::ErrorKind::Other, "sudo chown failed")),
        Err(e) => Err(e),
    }
}

/// Convenience wrapper: resolve `username`/`groupname` and call `chown`.
///
/// If a non-`None` `username`/`groupname` cannot be resolved an
/// `Err(io::ErrorKind::NotFound)` is returned.
pub fn chown_by_names(
    path: impl AsRef<Path>,
    username: Option<&str>,
    groupname: Option<&str>,
) -> io::Result<()> {
    let uid_opt = match username {
        Some(name) => match uid_by_username(name) {
            Some(u) => Some(u),
            None => return Err(io::Error::new(io::ErrorKind::NotFound, format!("user not found: {}", name))),
        },
        None => None,
    };

    let gid_opt = match groupname {
        Some(name) => match gid_by_groupname(name) {
            Some(g) => Some(g),
            None => return Err(io::Error::new(io::ErrorKind::NotFound, format!("group not found: {}", name))),
        },
        None => None,
    };

    chown(path, uid_opt, gid_opt)
}

/// Convenience wrapper: resolve `username`/`groupname` and call `fchown`.
pub fn fchown_by_names(
    file: &File,
    username: Option<&str>,
    groupname: Option<&str>,
) -> io::Result<()> {
    let uid_opt = match username {
        Some(name) => match uid_by_username(name) {
            Some(u) => Some(u),
            None => return Err(io::Error::new(io::ErrorKind::NotFound, format!("user not found: {}", name))),
        },
        None => None,
    };

    let gid_opt = match groupname {
        Some(name) => match gid_by_groupname(name) {
            Some(g) => Some(g),
            None => return Err(io::Error::new(io::ErrorKind::NotFound, format!("group not found: {}", name))),
        },
        None => None,
    };

    fchown(file, uid_opt, gid_opt)
}

/// Convenience wrapper: resolve `username`/`groupname` and call `lchown`.
pub fn lchown_by_names(
    path: impl AsRef<Path>,
    username: Option<&str>,
    groupname: Option<&str>,
) -> io::Result<()> {
    let uid_opt = match username {
        Some(name) => match uid_by_username(name) {
            Some(u) => Some(u),
            None => return Err(io::Error::new(io::ErrorKind::NotFound, format!("user not found: {}", name))),
        },
        None => None,
    };

    let gid_opt = match groupname {
        Some(name) => match gid_by_groupname(name) {
            Some(g) => Some(g),
            None => return Err(io::Error::new(io::ErrorKind::NotFound, format!("group not found: {}", name))),
        },
        None => None,
    };

    lchown(path, uid_opt, gid_opt)
}

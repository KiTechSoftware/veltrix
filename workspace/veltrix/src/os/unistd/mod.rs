use std::collections::HashSet;
use std::env;
use std::ffi::{CStr, CString};
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::os::unix::io::AsRawFd;

/// Common Unix group names that often grant administrative privileges.
///
/// This list is heuristic and not an authoritative sudo policy check.
pub const COMMON_ADMIN_GROUPS: &[&str] = &["sudo", "wheel", "admin"];

pub const SUBUID_FILE: &str = "/etc/subuid";
pub const SUBGID_FILE: &str = "/etc/subgid";

/// A Unix user identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Uid(libc::uid_t);

/// A Unix group identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Gid(libc::gid_t);

/// A Unix process identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pid(libc::pid_t);

/// A contiguous sub-UID/GID allocation: starting id and count.
///
/// Use `SubUidRange` or `SubGidRange` type aliases for convenience.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubIdRange<T> {
    pub start: T,
    pub count: u32,
}

/// Convenience alias for sub-UID ranges.
pub type SubUidRange = SubIdRange<Uid>;

/// Convenience alias for sub-GID ranges.
pub type SubGidRange = SubIdRange<Gid>;

impl SubUidRange {
    pub const fn from_raw(start: u32, count: u32) -> Self {
        Self { start: Uid::from_raw(start), count }
    }

    pub const fn as_raw(&self) -> Option<(u32, u32)> {
        Some((self.start.as_raw(), self.count))
    }

    pub const fn is_valid(&self) -> bool {
        self.start.as_raw() > 0 && self.count > 0
    }
}

impl SubGidRange {
    pub const fn from_raw(start: u32, count: u32) -> Self {
        Self { start: Gid::from_raw(start), count }
    }

    pub const fn as_raw(&self) -> Option<(u32, u32)> {
        Some((self.start.as_raw(), self.count))
    }

    pub const fn is_valid(&self) -> bool {
        self.start.as_raw() > 0 && self.count > 0
    }
}

impl Uid {
    /// Creates a [`Uid`] from a raw numeric user ID.
    pub const fn from_raw(uid: u32) -> Self {
        Self(uid as libc::uid_t)
    }

    /// Returns the raw numeric user ID.
    pub const fn as_raw(self) -> u32 {
        self.0
    }

    /// Returns `true` if this UID is root.
    pub const fn is_root(self) -> bool {
        self.0 == 0
    }
}

impl Gid {
    /// Creates a [`Gid`] from a raw numeric group ID.
    pub const fn from_raw(gid: u32) -> Self {
        Self(gid as libc::gid_t)
    }

    /// Returns the raw numeric group ID.
    pub const fn as_raw(self) -> u32 {
        self.0
    }
}

impl Pid {
    /// Creates a [`Pid`] from a raw numeric process ID.
    pub const fn from_raw(pid: i32) -> Self {
        Self(pid as libc::pid_t)
    }

    /// Returns the raw numeric process ID.
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_raw().fmt(f)
    }
}

impl fmt::Display for Gid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_raw().fmt(f)
    }
}

impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_raw().fmt(f)
    }
}

/// Returns the current real user ID.
pub fn getuid() -> Uid {
    Uid(unsafe { libc::getuid() })
}

/// Returns the current effective user ID.
pub fn geteuid() -> Uid {
    Uid(unsafe { libc::geteuid() })
}

/// Returns the current real group ID.
pub fn getgid() -> Gid {
    Gid(unsafe { libc::getgid() })
}

/// Returns the current effective group ID.
pub fn getegid() -> Gid {
    Gid(unsafe { libc::getegid() })
}

/// Returns the current process ID.
pub fn getpid() -> Pid {
    Pid(unsafe { libc::getpid() })
}

/// Returns the parent process ID.
pub fn getppid() -> Pid {
    Pid(unsafe { libc::getppid() })
}

/// Returns the invoking user ID, accounting for `sudo` invocation.
/// 
/// This checks the `SUDO_UID` environment variable, which is set by `sudo` to the
/// real UID of the user invoking `sudo`. If `SUDO_UID` is not set or cannot be
/// parsed, this falls back to the current real UID.
pub fn invoking_uid() -> Uid {
    std::env::var_os("SUDO_UID")
        .and_then(|s| s.to_string_lossy().parse::<u32>().ok())
        .map(Uid::from_raw)
        .unwrap_or_else(getuid)
}

/// Returns the invoking username, accounting for `sudo` invocation.
///
/// This checks the `SUDO_UID` environment variable to determine the real UID of the
/// user invoking `sudo`, then looks up the corresponding username. If `SUDO_UID`
/// is not set or cannot be parsed, this falls back to the username of the current real
pub fn invoking_username() -> Option<String> {
    let uid = invoking_uid();
    username_by_uid(uid)
}

/// Looks up a username by UID.
///
/// Returns `None` if the UID cannot be resolved.
pub fn username_by_uid(uid: Uid) -> Option<String> {
    let passwd = unsafe { libc::getpwuid(uid.0) };

    if passwd.is_null() {
        return None;
    }

    unsafe {
        let name = CStr::from_ptr((*passwd).pw_name);
        Some(name.to_string_lossy().into_owned())
    }
}

/// Looks up a UID by username.
///
/// Returns `None` if the username cannot be resolved or contains an interior NUL byte.
pub fn uid_by_username(username: &str) -> Option<Uid> {
    let username = CString::new(username).ok()?;
    let passwd = unsafe { libc::getpwnam(username.as_ptr()) };

    if passwd.is_null() {
        return None;
    }

    unsafe { Some(Uid((*passwd).pw_uid)) }
}

/// Looks up a group name by GID.
///
/// Returns `None` if the GID cannot be resolved.
pub fn groupname_by_gid(gid: Gid) -> Option<String> {
    let group = unsafe { libc::getgrgid(gid.0) };

    if group.is_null() {
        return None;
    }

    unsafe {
        let name = CStr::from_ptr((*group).gr_name);
        Some(name.to_string_lossy().into_owned())
    }
}

/// Looks up a GID by group name.
///
/// Returns `None` if the group name cannot be resolved or contains an interior NUL byte.
pub fn gid_by_groupname(groupname: &str) -> Option<Gid> {
    let groupname = CString::new(groupname).ok()?;
    let group = unsafe { libc::getgrnam(groupname.as_ptr()) };

    if group.is_null() {
        return None;
    }

    unsafe { Some(Gid((*group).gr_gid)) }
}

/// Returns the primary group ID for a UID.
///
/// Returns `None` if the UID cannot be resolved.
pub fn primary_gid_by_uid(uid: Uid) -> Option<Gid> {
    let passwd = unsafe { libc::getpwuid(uid.0) };

    if passwd.is_null() {
        return None;
    }

    unsafe { Some(Gid((*passwd).pw_gid)) }
}

/// Looks up a subuid range by username from `/etc/subuid`.
///
/// The file format is `name:start:count` (for example, `alice:100000:65536`).
/// Returns `Some((start_uid, count))` on success, or `None` if the username
/// cannot be resolved, contains an interior NUL byte, the file cannot be read,
/// or the line cannot be parsed.
pub fn subuid_by_username(username: &str) -> Option<SubUidRange> {
    let username = CString::new(username).ok()?;
    let file = File::open(SUBUID_FILE).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 3 {
            continue;
        }
        if parts[0] == username.to_str().unwrap() {
            if let (Ok(start), Ok(count)) = (parts[1].parse::<u32>(), parts[2].parse::<u32>()) {
                return Some(SubUidRange { start: Uid::from_raw(start), count });
            }
        }
    }

    None
}

/// Looks up a subuid range by username from `/etc/subuid`.
///
/// This is a convenience wrapper around `subuid_by_username` that returns raw numeric values.
/// Returns `Some((start_uid, count))` on success, or `None` if the username cannot be resolved,
/// contains an interior NUL byte, the file cannot be read, or the line cannot be parsed.
pub fn subuid_by_username_raw(username: &str) -> Option<(u32, u32)> {
    subuid_by_username(username).and_then(|range| range.as_raw())
}

/// Looks up a subgid range by group name from `/etc/subgid`.
///
/// The file format is `name:start:count` (for example, `staff:100000:65536`).
/// Returns `Some((start_gid, count))` on success, or `None` if the group name
/// cannot be resolved, contains an interior NUL byte, the file cannot be read,
/// or the line cannot be parsed.
pub fn subgid_by_groupname(groupname: &str) -> Option<SubGidRange> {
    let groupname = CString::new(groupname).ok()?;
    let file = File::open(SUBGID_FILE).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 3 {
            continue;
        }
        if parts[0] == groupname.to_str().unwrap() {
            if let (Ok(start), Ok(count)) = (parts[1].parse::<u32>(), parts[2].parse::<u32>()) {
                return Some(SubGidRange { start: Gid::from_raw(start), count });
            }
        }
    }

    None
}

/// Looks up a subgid range by group name from `/etc/subgid`.
/// This is a convenience wrapper around `subgid_by_groupname` that returns raw numeric values.
/// Returns `Some((start_gid, count))` on success, or `None` if the group name
/// cannot be resolved, contains an interior NUL byte, the file cannot be read,
/// or the line cannot be parsed.
pub fn subgid_by_groupname_raw(groupname: &str) -> Option<(u32, u32)> {
    subgid_by_groupname(groupname).and_then(|range| range.as_raw())
}

/// Returns group names for a UID.
///
/// This includes the user's primary group plus supplementary groups listed in
/// `/etc/group`.
///
/// This is a lightweight heuristic and may not include groups provided only by
/// NSS, LDAP, SSSD, Active Directory, or other external directory services.
pub fn groups_for_uid(uid: Uid) -> HashSet<String> {
    let mut groups = HashSet::new();

    if let Some(primary_gid) = primary_gid_by_uid(uid)
        && let Some(group_name) = groupname_by_gid(primary_gid)
    {
        groups.insert(group_name);
    }

    let Some(username) = username_by_uid(uid) else {
        return groups;
    };

    let Ok(file) = File::open("/etc/group") else {
        return groups;
    };

    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        let parts: Vec<&str> = line.split(':').collect();

        if parts.len() < 4 {
            continue;
        }

        let group_name = parts[0];
        let members = parts[3];

        if members.split(',').any(|member| member.trim() == username) {
            groups.insert(group_name.to_string());
        }
    }

    groups
}

/// Returns the system hostname.
pub fn gethostname() -> io::Result<String> {
    let mut buffer = vec![0u8; 256];

    loop {
        let result =
            unsafe { libc::gethostname(buffer.as_mut_ptr() as *mut libc::c_char, buffer.len()) };

        if result != 0 {
            return Err(io::Error::last_os_error());
        }

        if let Some(nul_pos) = buffer.iter().position(|&byte| byte == 0) {
            buffer.truncate(nul_pos);
            return Ok(String::from_utf8_lossy(&buffer).into_owned());
        }

        if buffer.len() >= 4096 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "hostname was not null-terminated",
            ));
        }

        buffer.resize(buffer.len() * 2, 0);
    }
}

/// Returns the current working directory.
pub fn getcwd() -> io::Result<PathBuf> {
    env::current_dir()
}

/// Changes the current working directory.
pub fn chdir(path: impl AsRef<Path>) -> io::Result<()> {
    env::set_current_dir(path)
}

/// Returns the current user's home directory.
///
/// This checks `$HOME` first, then falls back to the passwd entry for the
/// current real UID.
pub fn home_dir() -> Option<PathBuf> {
    if let Some(home) = env::var_os("HOME") {
        return Some(PathBuf::from(home));
    }

    let uid = getuid();
    let passwd = unsafe { libc::getpwuid(uid.0) };

    if passwd.is_null() {
        return None;
    }

    unsafe {
        let dir = CStr::from_ptr((*passwd).pw_dir);
        Some(PathBuf::from(dir.to_string_lossy().into_owned()))
    }
}

/// Returns `true` if a process identified by [`Pid`] is alive.
///
/// This sends signal 0 via `kill(2)`, which performs permission checking but
/// does not deliver a signal. A return of `true` means the process exists;
/// `EPERM` also means it exists but is owned by another user.
///
/// Returns `false` for `ESRCH` (no such process) or any other error.
pub fn pid_is_alive(pid: Pid) -> bool {
    let result = unsafe { libc::kill(pid.0, 0) };
    if result == 0 {
        return true;
    }
    // EPERM: process exists but we lack permission to signal it – still alive
    (unsafe { libc::__errno_location().read() }) == libc::EPERM
}

/// Returns `true` if the current real UID is root.
pub fn is_root() -> bool {
    getuid().is_root()
}

/// Returns `true` if the current effective UID is root.
pub fn is_effective_root() -> bool {
    geteuid().is_root()
}

/// Returns `true` if a UID belongs to the named group.
pub fn user_in_group(uid: Uid, group: &str) -> bool {
    groups_for_uid(uid).contains(group)
}

/// Returns `true` if a UID belongs to a common admin group.
///
/// This checks membership in [`COMMON_ADMIN_GROUPS`].
///
/// This is a convenience heuristic, not an authoritative sudo policy check.
pub fn user_in_admin_group(uid: Uid) -> bool {
    COMMON_ADMIN_GROUPS
        .iter()
        .any(|group| user_in_group(uid, group))
}

/// Returns `true` if `/etc/sudoers` references a common admin group.
///
/// This checks for common sudoers group tokens such as `%sudo`, `%wheel`,
/// and `%admin`.
///
/// This is a lightweight heuristic and does not currently parse
/// `/etc/sudoers.d`, aliases, includes, LDAP, SSSD, or command-specific rules.
pub fn has_common_admin_group() -> bool {
    let Ok(file) = File::open("/etc/sudoers") else {
        return false;
    };

    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        for group in COMMON_ADMIN_GROUPS {
            let token = format!("%{group}");

            if line.split_whitespace().any(|part| part == token) {
                return true;
            }
        }
    }

    false
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn chown_noop_for_current_user() {
        let mut path = std::env::temp_dir();
        path.push(format!("veltrix_chown_test_{}", std::process::id()));

        let _ = File::create(&path).expect("create temp file");

        // Changing to the current uid/gid should succeed for a file we own.
        let res = chown(&path, Some(getuid()), Some(getgid()));

        // Clean up before asserting so test doesn't leave artifacts on failure.
        let _ = std::fs::remove_file(&path);

        assert!(res.is_ok());
    }

    #[test]
    fn fchown_noop_for_current_user() {
        let mut path = std::env::temp_dir();
        path.push(format!("veltrix_fchown_test_{}", std::process::id()));

        let _ = File::create(&path).expect("create temp file");
        let f = File::open(&path).expect("open file");

        let res = fchown(&f, Some(getuid()), Some(getgid()));

        let _ = std::fs::remove_file(&path);

        assert!(res.is_ok());
    }

    #[test]
    fn lchown_noop_for_current_user() {
        use std::os::unix::fs::symlink;

        let mut target = std::env::temp_dir();
        target.push(format!("veltrix_lchown_target_{}", std::process::id()));
        let _ = File::create(&target).expect("create target");

        let mut link = std::env::temp_dir();
        link.push(format!("veltrix_lchown_link_{}", std::process::id()));

        // Remove any leftover link, ignore errors
        let _ = std::fs::remove_file(&link);

        symlink(&target, &link).expect("create symlink");

        let res = lchown(&link, Some(getuid()), Some(getgid()));

        let _ = std::fs::remove_file(&link);
        let _ = std::fs::remove_file(&target);

        assert!(res.is_ok());
    }

    #[test]
    fn chown_by_names_noop_for_current_user() {
        let mut path = std::env::temp_dir();
        path.push(format!("veltrix_chown_names_test_{}", std::process::id()));

        let _ = File::create(&path).expect("create temp file");

        let username = username_by_uid(getuid()).expect("username exists");
        let groupname = groupname_by_gid(getgid()).expect("group exists");

        let res = chown_by_names(&path, Some(&username), Some(&groupname));

        let _ = std::fs::remove_file(&path);

        assert!(res.is_ok());
    }

    #[test]
    fn fchown_by_names_noop_for_current_user() {
        let mut path = std::env::temp_dir();
        path.push(format!("veltrix_fchown_names_test_{}", std::process::id()));

        let _ = File::create(&path).expect("create temp file");
        let f = File::open(&path).expect("open file");

        let username = username_by_uid(getuid()).expect("username exists");
        let groupname = groupname_by_gid(getgid()).expect("group exists");

        let res = fchown_by_names(&f, Some(&username), Some(&groupname));

        let _ = std::fs::remove_file(&path);

        assert!(res.is_ok());
    }

    #[test]
    fn lchown_by_names_noop_for_current_user() {
        use std::os::unix::fs::symlink;

        let mut target = std::env::temp_dir();
        target.push(format!("veltrix_lchown_names_target_{}", std::process::id()));
        let _ = File::create(&target).expect("create target");

        let mut link = std::env::temp_dir();
        link.push(format!("veltrix_lchown_names_link_{}", std::process::id()));

        let _ = std::fs::remove_file(&link);

        symlink(&target, &link).expect("create symlink");

        let username = username_by_uid(getuid()).expect("username exists");
        let groupname = groupname_by_gid(getgid()).expect("group exists");

        let res = lchown_by_names(&link, Some(&username), Some(&groupname));

        let _ = std::fs::remove_file(&link);
        let _ = std::fs::remove_file(&target);

        assert!(res.is_ok());
    }
}

// TODO: add more functions as needed, create user/group 
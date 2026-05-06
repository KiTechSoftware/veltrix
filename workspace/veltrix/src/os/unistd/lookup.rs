use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ffi::{CStr, CString};
use std::path::PathBuf;

use crate::os::paths::constants;
use crate::os::unistd::{Pid, getuid};

use super::{Uid, Gid, SubUidRange, SubGidRange, COMMON_ADMIN_GROUPS};

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
    let file = File::open(constants::PATH_SYSTEM_SUBUID_FILE).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        let parts: Vec<&str> = line.split(constants::SUBID_SEPARATOR).collect();
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
    let file = File::open(constants::PATH_SYSTEM_SUBGID_FILE).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        let parts: Vec<&str> = line.split(constants::SUBID_SEPARATOR).collect();
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

    let Ok(file) = File::open(constants::PATH_SYSTEM_GROUPS_FILE) else {
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

/// Returns the invoking username, accounting for `sudo` invocation.
pub fn invoking_username() -> Option<String> {
    let uid = super::invoking_uid();
    username_by_uid(uid)
}

/// Returns `true` if `/etc/sudoers` references a common admin group.
///
/// This checks for common sudoers group tokens such as `%sudo`, `%wheel`,
/// and `%admin`.
///
/// This is a lightweight heuristic and does not currently parse
/// `/etc/sudoers.d`, aliases, includes, LDAP, SSSD, or command-specific rules.
pub fn has_common_admin_group() -> bool {
    let Ok(file) = File::open(constants::PATH_SYSTEM_SUDOERS_FILE) else {
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
    std::env::current_dir()
}

/// Changes the current working directory.
pub fn chdir(path: impl AsRef<std::path::Path>) -> io::Result<()> {
    std::env::set_current_dir(path)
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

/// Returns the current user's home directory.
///
/// This checks `$HOME` first, then falls back to the passwd entry for the
/// current real UID.
pub fn home_dir() -> Option<PathBuf> {
    if let Some(home) = std::env::var_os(constants::USER_HOME_ENV) {
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

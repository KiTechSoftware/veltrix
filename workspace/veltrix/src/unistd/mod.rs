use std::collections::HashSet;
use std::env;
use std::ffi::{CStr, CString};
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

pub const COMMON_ADMIN_GROUPS: &[&str] = &["sudo", "wheel", "admin"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Uid(libc::uid_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Gid(libc::gid_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pid(libc::pid_t);

impl Uid {
    pub const fn from_raw(uid: u32) -> Self {
        Self(uid as libc::uid_t)
    }

    pub const fn as_raw(self) -> u32 {
        self.0 as u32
    }

    pub const fn is_root(self) -> bool {
        self.0 == 0
    }
}

impl Gid {
    pub const fn from_raw(gid: u32) -> Self {
        Self(gid as libc::gid_t)
    }

    pub const fn as_raw(self) -> u32 {
        self.0 as u32
    }
}

impl Pid {
    pub const fn from_raw(pid: i32) -> Self {
        Self(pid as libc::pid_t)
    }

    pub const fn as_raw(self) -> i32 {
        self.0 as i32
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

// Core identity

pub fn getuid() -> Uid {
    Uid(unsafe { libc::getuid() })
}

pub fn geteuid() -> Uid {
    Uid(unsafe { libc::geteuid() })
}

pub fn getgid() -> Gid {
    Gid(unsafe { libc::getgid() })
}

pub fn getegid() -> Gid {
    Gid(unsafe { libc::getegid() })
}

pub fn getpid() -> Pid {
    Pid(unsafe { libc::getpid() })
}

pub fn getppid() -> Pid {
    Pid(unsafe { libc::getppid() })
}

// User/group lookup

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

pub fn uid_by_username(username: &str) -> Option<Uid> {
    let username = CString::new(username).ok()?;
    let passwd = unsafe { libc::getpwnam(username.as_ptr()) };

    if passwd.is_null() {
        return None;
    }

    unsafe { Some(Uid((*passwd).pw_uid)) }
}

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

pub fn gid_by_groupname(groupname: &str) -> Option<Gid> {
    let groupname = CString::new(groupname).ok()?;
    let group = unsafe { libc::getgrnam(groupname.as_ptr()) };

    if group.is_null() {
        return None;
    }

    unsafe { Some(Gid((*group).gr_gid)) }
}

pub fn primary_gid_by_uid(uid: Uid) -> Option<Gid> {
    let passwd = unsafe { libc::getpwuid(uid.0) };

    if passwd.is_null() {
        return None;
    }

    unsafe { Some(Gid((*passwd).pw_gid)) }
}

/// Gets primary group plus supplementary groups listed in `/etc/group`.
///
/// Note: this may not include groups provided only by NSS, LDAP, SSSD,
/// Active Directory, or other external directory services.
pub fn groups_for_uid(uid: Uid) -> HashSet<String> {
    let mut groups = HashSet::new();

    if let Some(primary_gid) = primary_gid_by_uid(uid) {
        if let Some(group_name) = groupname_by_gid(primary_gid) {
            groups.insert(group_name);
        }
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

// Host/process environment

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

pub fn getcwd() -> io::Result<PathBuf> {
    env::current_dir()
}

pub fn chdir(path: impl AsRef<Path>) -> io::Result<()> {
    env::set_current_dir(path)
}

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

// Privilege helpers

pub fn is_root() -> bool {
    getuid().is_root()
}

pub fn is_effective_root() -> bool {
    geteuid().is_root()
}

pub fn user_in_group(uid: Uid, group: &str) -> bool {
    groups_for_uid(uid).contains(group)
}

pub fn user_in_admin_group(uid: Uid) -> bool {
    COMMON_ADMIN_GROUPS
        .iter()
        .any(|group| user_in_group(uid, group))
}

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
            let token = format!("%{}", group);

            if line.split_whitespace().any(|part| part == token) {
                return true;
            }
        }
    }

    false
}

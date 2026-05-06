use std::fmt;

/// Common Unix group names that often grant administrative privileges.
///
/// This list is heuristic and not an authoritative sudo policy check.
pub const COMMON_ADMIN_GROUPS: &[&str] = &["sudo", "wheel", "admin"];

#[cfg(feature = "legacy")]
pub const SUBUID_FILE: &str = crate::os::paths::constants::PATH_SYSTEM_SUBUID_FILE;
#[cfg(feature = "legacy")]
pub const SUBGID_FILE: &str = crate::os::paths::constants::PATH_SYSTEM_SUBGID_FILE;

/// A Unix user identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Uid(pub(crate) libc::uid_t);

/// A Unix group identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Gid(pub(crate) libc::gid_t);

/// A Unix process identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pid(pub(crate) libc::pid_t);

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
    std::env::var_os(crate::os::paths::constants::SUDO_UID_ENV)
        .and_then(|s| s.to_string_lossy().parse::<u32>().ok())
        .map(Uid::from_raw)
        .unwrap_or_else(getuid)
}

/// Returns `true` if the current real UID is root.
pub fn is_root() -> bool {
    getuid().is_root()
}

/// Returns `true` if the current effective UID is root.
pub fn is_effective_root() -> bool {
    geteuid().is_root()
}

use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Get the current real UID.
pub fn get_current_uid() -> u32 {
    unsafe { libc::getuid() as u32 }
}

/// Get the current effective UID.
pub fn get_effective_uid() -> u32 {
    unsafe { libc::geteuid() as u32 }
}

/// Get the username for a given UID (returns None if not found)
pub fn get_username_by_uid(uid: u32) -> Option<String> {
    let passwd = unsafe { libc::getpwuid(uid as libc::uid_t) };
    if passwd.is_null() {
        return None;
    }
    unsafe {
        let name = CStr::from_ptr((*passwd).pw_name);
        Some(name.to_string_lossy().into_owned())
    }
}

/// Get the UID for a given username (returns None if not found)
pub fn get_uid_by_username(username: &str) -> Option<u32> {
    let username = CString::new(username).ok()?;
    let passwd = unsafe { libc::getpwnam(username.as_ptr()) };
    if passwd.is_null() {
        return None;
    }
    unsafe { Some((*passwd).pw_uid as u32) }
}

/// Get the GID for a given username (returns None if not found)
pub fn get_gid_by_username(username: &str) -> Option<u32> {
    let username = CString::new(username).ok()?;
    let passwd = unsafe { libc::getpwnam(username.as_ptr()) };
    if passwd.is_null() {
        return None;
    }
    unsafe { Some((*passwd).pw_gid as u32) }
}

/// Get all group names for a given UID (by parsing /etc/group)
pub fn get_groups_for_uid(uid: u32) -> HashSet<String> {
    let mut groups = HashSet::new();
    if let Some(primary_gid) = get_primary_gid_by_uid(uid) {
        if let Some(group_name) = get_groupname_by_gid(primary_gid) {
            groups.insert(group_name);
        }
    }

    if let Some(username) = get_username_by_uid(uid) {
        if let Ok(file) = File::open("/etc/group") {
            let reader = BufReader::new(file);
            for line in reader.lines().map_while(Result::ok) {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 4 {
                    let group_name = parts[0];
                    let members = parts[3];
                    if members.split(',').any(|m| m.trim() == username) {
                        groups.insert(group_name.to_string());
                    }
                }
            }
        }
    }
    groups
}

/// Check if a user (by UID) is in a group by name
pub fn user_in_group(uid: u32, group: &str) -> bool {
    get_groups_for_uid(uid).contains(group)
}

/// Check if a user (by UID) is in a group by GID (primary group)
pub fn user_primary_group_is(uid: u32, gid: u32) -> bool {
    get_primary_gid_by_uid(uid).is_some_and(|primary_gid| primary_gid == gid)
}

/// check if user is in sudo group
pub fn user_in_sudo_group(uid: u32) -> bool {
    // common groups to check for sudo privileges
    let sudo_groups = ["sudo", "wheel", "admin"];
    sudo_groups.iter().any(|g| user_in_group(uid, g))
}

fn get_primary_gid_by_uid(uid: u32) -> Option<u32> {
    let passwd = unsafe { libc::getpwuid(uid as libc::uid_t) };
    if passwd.is_null() {
        return None;
    }
    unsafe { Some((*passwd).pw_gid as u32) }
}

fn get_groupname_by_gid(gid: u32) -> Option<String> {
    let group = unsafe { libc::getgrgid(gid as libc::gid_t) };
    if group.is_null() {
        return None;
    }
    unsafe {
        let name = CStr::from_ptr((*group).gr_name);
        Some(name.to_string_lossy().into_owned())
    }
}

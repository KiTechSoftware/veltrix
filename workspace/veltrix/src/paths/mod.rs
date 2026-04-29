//! Path helpers and common platform constants.
//!
//! This module centralizes well-known system and user path constants
//! and provides helpers to resolve system-wide and per-user paths using
//! environment variables (XDG) or sane defaults.

use std::{
    env,
    path::{Path, PathBuf},
};

use crate::error::{Result, VeltrixError};

// Common names

/// Name used for application data directories (e.g. `~/.local/share/<app>`).
pub const APPLICATIONS_DIR_NAME: &str = "applications";
/// User systemd unit directory name (under config).
pub const SYSTEMD_DIR_NAME: &str = "systemd";
/// Icon directories name.
pub const ICONS_DIR_NAME: &str = "icons";
/// Default user-local binary directory name.
pub const BIN_DIR_NAME: &str = "bin";

// System-wide paths
pub const PATH_SYSTEM_ROOT: &str = "/";
pub const PATH_SYSTEM_ROOT_HOME: &str = "/root";
pub const PATH_SYSTEM_HOME: &str = "/home";
pub const PATH_SYSTEM_BIN: &str = "/usr/bin";
pub const PATH_SYSTEM_LOCAL_BIN: &str = "/usr/local/bin";
pub const PATH_SYSTEM_LIB_DIR: &str = "/usr/lib";
pub const PATH_SYSTEM_LIBEXEC_DIR: &str = "/usr/libexec";
pub const PATH_SYSTEM_DATA_DIR: &str = "/usr/share";
pub const PATH_SYSTEM_CONFIG_DIR: &str = "/etc";
pub const PATH_SYSTEM_STATE_DIR: &str = "/var/lib";
pub const PATH_SYSTEM_CACHE_DIR: &str = "/var/cache";
pub const PATH_SYSTEM_LOG_DIR: &str = "/var/log";
pub const PATH_SYSTEM_RUNTIME_DIR: &str = "/run";
pub const PATH_SYSTEM_MAN1_DIR: &str = "/usr/share/man/man1";
pub const PATH_SYSTEM_DOC_DIR: &str = "/usr/share/doc";
pub const PATH_SYSTEM_APPLICATIONS_DIR: &str = "/usr/share/applications";
pub const PATH_SYSTEM_ICONS_DIR: &str = "/usr/share/icons";

pub const PATH_SYSTEMD_UNIT_DIR: &str = "/etc/systemd/system";

// User/XDG paths

pub const USER_HOME_ENV: &str = "HOME";
pub const XDG_CONFIG_DIR_ENV: &str = "XDG_CONFIG_HOME";
pub const XDG_STATE_DIR_ENV: &str = "XDG_STATE_HOME";
pub const XDG_CACHE_DIR_ENV: &str = "XDG_CACHE_HOME";
pub const XDG_DATA_DIR_ENV: &str = "XDG_DATA_HOME";
pub const XDG_RUNTIME_DIR_ENV: &str = "XDG_RUNTIME_DIR";

// User-level defaults
pub const USER_HOME_DIR: &str = "~";
pub const USER_CONFIG_DIR: &str = ".config";
pub const USER_STATE_DIR: &str = ".local/state";
pub const USER_CACHE_DIR: &str = ".cache";
pub const USER_DATA_DIR: &str = ".local/share";
pub const USER_BIN_DIR: &str = ".local/bin";
pub const USER_APPLICATIONS_DIR: &str = ".local/share/applications";
pub const USER_ICONS_DIR: &str = ".local/share/icons";
pub const USER_SYSTEMD_UNIT_DIR: &str = ".config/systemd/user";

// File extensions
pub const EXT_SEPERATOR: &str = ".";
pub const APPLICATION_DESKTOP_EXT: &str = "desktop";
pub const SYSTEMD_UNIT_EXT: &str = "service";
pub const MAN_PAGE_EXT: &str = "1.gz";

// System-wide paths

pub fn system_bin_path(bin_name: &str) -> PathBuf {
    PathBuf::from(PATH_SYSTEM_BIN).join(bin_name)
}

pub fn system_local_bin_path(bin_name: &str) -> PathBuf {
    PathBuf::from(PATH_SYSTEM_LOCAL_BIN).join(bin_name)
}

pub fn system_config_dir(app_name: &str) -> PathBuf {
    PathBuf::from(PATH_SYSTEM_CONFIG_DIR).join(app_name)
}

pub fn system_config_path(app_name: &str, config_file_name: &str) -> PathBuf {
    system_config_dir(app_name).join(config_file_name)
}

pub fn system_state_dir(app_name: &str) -> PathBuf {
    PathBuf::from(PATH_SYSTEM_STATE_DIR).join(app_name)
}

pub fn system_cache_dir(app_name: &str) -> PathBuf {
    PathBuf::from(PATH_SYSTEM_CACHE_DIR).join(app_name)
}

pub fn system_log_dir(app_name: &str) -> PathBuf {
    PathBuf::from(PATH_SYSTEM_LOG_DIR).join(app_name)
}

pub fn system_runtime_dir(app_name: &str) -> PathBuf {
    PathBuf::from(PATH_SYSTEM_RUNTIME_DIR).join(app_name)
}

pub fn system_data_dir(app_name: &str) -> PathBuf {
    PathBuf::from(PATH_SYSTEM_DATA_DIR).join(app_name)
}

pub fn system_lib_dir(app_name: &str) -> PathBuf {
    PathBuf::from(PATH_SYSTEM_LIB_DIR).join(app_name)
}

pub fn system_libexec_dir(app_name: &str) -> PathBuf {
    PathBuf::from(PATH_SYSTEM_LIBEXEC_DIR).join(app_name)
}

pub fn system_doc_dir(app_name: &str) -> PathBuf {
    PathBuf::from(PATH_SYSTEM_DOC_DIR).join(app_name)
}

pub fn system_man1_path(bin_name: &str) -> PathBuf {
    PathBuf::from(PATH_SYSTEM_MAN1_DIR).join(format!("{bin_name}{EXT_SEPERATOR}{MAN_PAGE_EXT}"))
}

pub fn system_desktop_entry_path(app_name: &str) -> PathBuf {
    PathBuf::from(PATH_SYSTEM_APPLICATIONS_DIR).join(format!(
        "{app_name}{EXT_SEPERATOR}{APPLICATION_DESKTOP_EXT}"
    ))
}

pub fn system_icon_dir(app_name: &str) -> PathBuf {
    PathBuf::from(PATH_SYSTEM_ICONS_DIR).join(app_name)
}

pub fn systemd_unit_path(app_name: &str) -> PathBuf {
    PathBuf::from(PATH_SYSTEMD_UNIT_DIR).join(systemd_unit_name(app_name))
}

pub fn systemd_unit_name(app_name: &str) -> String {
    format!("{app_name}{EXT_SEPERATOR}{SYSTEMD_UNIT_EXT}")
}

// User-level resolved paths

/// Resolve the user's configuration directory for `app_name`.
///
/// This consults `$XDG_CONFIG_HOME` and falls back to `~/.config`.
pub fn user_config_dir(app_name: &str) -> Result<PathBuf> {
    xdg_dir(XDG_CONFIG_DIR_ENV, &[USER_CONFIG_DIR], app_name)
}

pub fn user_config_path(app_name: &str, config_file_name: &str) -> Result<PathBuf> {
    Ok(user_config_dir(app_name)?.join(config_file_name))
}

pub fn user_state_dir(app_name: &str) -> Result<PathBuf> {
    xdg_dir(XDG_STATE_DIR_ENV, &[USER_STATE_DIR], app_name)
}

pub fn user_cache_dir(app_name: &str) -> Result<PathBuf> {
    xdg_dir(XDG_CACHE_DIR_ENV, &[USER_CACHE_DIR], app_name)
}

pub fn user_data_dir(app_name: &str) -> Result<PathBuf> {
    xdg_dir(XDG_DATA_DIR_ENV, &[USER_DATA_DIR], app_name)
}

pub fn user_runtime_dir(app_name: &str) -> Result<PathBuf> {
    let path = env::var_os(XDG_RUNTIME_DIR_ENV)
        .map(PathBuf::from)
        .ok_or_else(|| VeltrixError::env_missing(XDG_RUNTIME_DIR_ENV))?;
    if !path.is_absolute() {
        return Err(VeltrixError::env_invalid(
            XDG_RUNTIME_DIR_ENV,
            "must be absolute",
        ));
    }

    Ok(path.join(app_name))
}

pub fn user_log_dir(app_name: &str) -> Result<PathBuf> {
    Ok(user_state_dir(app_name)?.join("logs"))
}

/// Resolve the user's local `bin` directory, typically `~/.local/bin`.
pub fn user_bin_dir() -> Result<PathBuf> {
    Ok(home_dir()?.join(BIN_DIR_NAME))
}

pub fn user_bin_path(bin_name: &str) -> Result<PathBuf> {
    Ok(user_bin_dir()?.join(bin_name))
}

/// Resolve the path to the user's systemd unit directory (e.g. `~/.config/systemd/user`).
pub fn user_systemd_unit_dir() -> Result<PathBuf> {
    Ok(xdg_dir(XDG_CONFIG_DIR_ENV, &[USER_CONFIG_DIR], SYSTEMD_DIR_NAME)?.join("user"))
}

pub fn user_systemd_unit_path(app_name: &str) -> Result<PathBuf> {
    Ok(user_systemd_unit_dir()?.join(systemd_unit_name(app_name)))
}

pub fn user_desktop_entry_path(app_name: &str) -> Result<PathBuf> {
    Ok(user_data_dir(APPLICATIONS_DIR_NAME)?.join(format!(
        "{app_name}{EXT_SEPERATOR}{APPLICATION_DESKTOP_EXT}"
    )))
}

pub fn user_icon_dir(app_name: &str) -> Result<PathBuf> {
    Ok(user_data_dir(ICONS_DIR_NAME)?.join(app_name))
}

// Config resolution

pub fn resolve_config_path(
    app_name: &str,
    config_file_name: &str,
    explicit_config_path: Option<&Path>,
) -> Result<PathBuf> {
    if let Some(path) = explicit_config_path {
        return Ok(path.to_path_buf());
    }

    let user_path = user_config_path(app_name, config_file_name)?;
    if user_path.exists() {
        return Ok(user_path);
    }

    let system_path = system_config_path(app_name, config_file_name);
    if system_path.exists() {
        return Ok(system_path);
    }

    Ok(user_path)
}

/// Resolve the canonical new config path depending on whether `system` is true.
///
/// When `system` is true this returns the system config path, otherwise the per-user path.
pub fn resolve_new_config_path(
    app_name: &str,
    config_file_name: &str,
    system: bool,
) -> Result<PathBuf> {
    if system {
        Ok(system_config_path(app_name, config_file_name))
    } else {
        user_config_path(app_name, config_file_name)
    }
}

// App runtime defaults

pub fn app_config_dir(app_name: &str) -> Result<PathBuf> {
    user_config_dir(app_name)
}

pub fn app_state_dir(app_name: &str) -> Result<PathBuf> {
    user_state_dir(app_name)
}

pub fn app_cache_dir(app_name: &str) -> Result<PathBuf> {
    user_cache_dir(app_name)
}

pub fn app_data_dir(app_name: &str) -> Result<PathBuf> {
    user_data_dir(app_name)
}

pub fn app_runtime_dir(app_name: &str) -> Result<PathBuf> {
    user_runtime_dir(app_name)
}

// Helpers

pub fn expand_user_path(path: &str) -> Result<PathBuf> {
    if path.starts_with(USER_HOME_DIR) {
        Ok(home_dir()?.join(path.trim_start_matches(USER_HOME_DIR)))
    } else {
        Ok(PathBuf::from(path))
    }
}

pub fn home_dir() -> Result<PathBuf> {
    let path = env::var_os(USER_HOME_ENV)
        .map(PathBuf::from)
        .ok_or_else(|| VeltrixError::env_missing(USER_HOME_ENV))?;

    if !path.is_absolute() {
        return Err(VeltrixError::env_invalid(USER_HOME_ENV, "must be absolute"));
    }

    Ok(path)
}

pub fn xdg_dir(env_key: &str, fallback: &[&str], app_dir: &str) -> Result<PathBuf> {
    if let Some(value) = env::var_os(env_key) {
        let path = PathBuf::from(value);

        // XDG requires absolute paths. Relative values should be ignored.
        if path.is_absolute() {
            return Ok(path.join(app_dir));
        }
    }

    Ok(fallback
        .iter()
        .fold(home_dir()?, |path, part| path.join(part))
        .join(app_dir))
}

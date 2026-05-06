//! Path helpers and common platform constants.
//!
//! This module centralizes well-known system and user path constants
//! and provides helpers to resolve system-wide and per-user paths using
//! environment variables (XDG) or sane defaults.
pub mod constants;

use std::{
    env,
    path::{Path, PathBuf},
};

use crate::error::{Result, VeltrixError};

// System-wide paths

pub fn system_bin_path(bin_name: &str) -> PathBuf {
    PathBuf::from(constants::PATH_SYSTEM_BIN).join(bin_name)
}

pub fn system_local_bin_path(bin_name: &str) -> PathBuf {
    PathBuf::from(constants::PATH_SYSTEM_LOCAL_BIN).join(bin_name)
}

pub fn system_config_dir(app_name: &str) -> PathBuf {
    PathBuf::from(constants::PATH_SYSTEM_CONFIG_DIR).join(app_name)
}

pub fn system_config_path(app_name: &str, config_file_name: &str) -> PathBuf {
    system_config_dir(app_name).join(config_file_name)
}

pub fn system_state_dir(app_name: &str) -> PathBuf {
    PathBuf::from(constants::PATH_SYSTEM_STATE_DIR).join(app_name)
}

pub fn system_cache_dir(app_name: &str) -> PathBuf {
    PathBuf::from(constants::PATH_SYSTEM_CACHE_DIR).join(app_name)
}

pub fn system_log_dir(app_name: &str) -> PathBuf {
    PathBuf::from(constants::PATH_SYSTEM_LOG_DIR).join(app_name)
}

pub fn system_runtime_dir(app_name: &str) -> PathBuf {
    PathBuf::from(constants::PATH_SYSTEM_RUNTIME_DIR).join(app_name)
}

pub fn system_data_dir(app_name: &str) -> PathBuf {
    PathBuf::from(constants::PATH_SYSTEM_DATA_DIR).join(app_name)
}

pub fn system_lib_dir(app_name: &str) -> PathBuf {
    PathBuf::from(constants::PATH_SYSTEM_LIB_DIR).join(app_name)
}

pub fn system_libexec_dir(app_name: &str) -> PathBuf {
    PathBuf::from(constants::PATH_SYSTEM_LIBEXEC_DIR).join(app_name)
}

pub fn system_doc_dir(app_name: &str) -> PathBuf {
    PathBuf::from(constants::PATH_SYSTEM_DOC_DIR).join(app_name)
}

pub fn system_man1_path(bin_name: &str) -> PathBuf {
    PathBuf::from(constants::PATH_SYSTEM_MAN1_DIR).join(format!(
        "{bin_name}{}{}",
        constants::EXT_SEPERATOR,
        constants::MAN_PAGE_EXT
    ))
}

pub fn system_desktop_entry_path(app_name: &str) -> PathBuf {
    PathBuf::from(constants::PATH_SYSTEM_APPLICATIONS_DIR).join(format!(
        "{app_name}{}{}",
        constants::EXT_SEPERATOR,
        constants::APPLICATION_DESKTOP_EXT
    ))
}

pub fn system_icon_dir(app_name: &str) -> PathBuf {
    PathBuf::from(constants::PATH_SYSTEM_ICONS_DIR).join(app_name)
}

pub fn systemd_unit_path(app_name: &str) -> PathBuf {
    PathBuf::from(constants::PATH_SYSTEMD_UNIT_DIR).join(systemd_unit_name(app_name))
}

pub fn systemd_unit_name(app_name: &str) -> String {
    format!(
        "{app_name}{}{}",
        constants::EXT_SEPERATOR,
        constants::SYSTEMD_UNIT_EXT,
    )
}

// User-level resolved paths

/// Resolve the user's configuration directory for `app_name`.
///
/// This consults `$XDG_CONFIG_HOME` and falls back to `~/.config`.
pub fn user_config_dir(app_name: &str) -> Result<PathBuf> {
    xdg_dir(
        constants::XDG_CONFIG_DIR_ENV,
        &[constants::USER_CONFIG_DIR],
        app_name,
    )
}

pub fn user_config_path(app_name: &str, config_file_name: &str) -> Result<PathBuf> {
    Ok(user_config_dir(app_name)?.join(config_file_name))
}

pub fn user_state_dir(app_name: &str) -> Result<PathBuf> {
    xdg_dir(
        constants::XDG_STATE_DIR_ENV,
        &[constants::USER_STATE_DIR],
        app_name,
    )
}

pub fn user_cache_dir(app_name: &str) -> Result<PathBuf> {
    xdg_dir(
        constants::XDG_CACHE_DIR_ENV,
        &[constants::USER_CACHE_DIR],
        app_name,
    )
}

pub fn user_data_dir(app_name: &str) -> Result<PathBuf> {
    xdg_dir(
        constants::XDG_DATA_DIR_ENV,
        &[constants::USER_DATA_DIR],
        app_name,
    )
}

pub fn user_runtime_dir(app_name: &str) -> Result<PathBuf> {
    let path = env::var_os(constants::XDG_RUNTIME_DIR_ENV)
        .map(PathBuf::from)
        .ok_or_else(|| VeltrixError::env_missing(constants::XDG_RUNTIME_DIR_ENV))?;
    if !path.is_absolute() {
        return Err(VeltrixError::env_invalid(
            constants::XDG_RUNTIME_DIR_ENV,
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
    if cfg!(feature = "legacy") {
        return Ok(home_dir()?.join(".bin"));
    }
    Ok(home_dir()?.join(".local").join(constants::BIN_DIR_NAME))
}

pub fn user_bin_path(bin_name: &str) -> Result<PathBuf> {
    Ok(user_bin_dir()?.join(bin_name))
}

/// Resolve the path to the user's systemd unit directory (e.g. `~/.config/systemd/user`).
pub fn user_systemd_unit_dir() -> Result<PathBuf> {
    Ok(xdg_dir(
        constants::XDG_CONFIG_DIR_ENV,
        &[constants::USER_CONFIG_DIR],
        constants::SYSTEMD_DIR_NAME,
    )?
    .join("user"))
}

pub fn user_systemd_unit_path(app_name: &str) -> Result<PathBuf> {
    Ok(user_systemd_unit_dir()?.join(systemd_unit_name(app_name)))
}

pub fn user_desktop_entry_path(app_name: &str) -> Result<PathBuf> {
    Ok(
        user_data_dir(constants::APPLICATIONS_DIR_NAME)?.join(format!(
            "{app_name}{}{}",
            constants::EXT_SEPERATOR,
            constants::APPLICATION_DESKTOP_EXT
        )),
    )
}

pub fn user_icon_dir(app_name: &str) -> Result<PathBuf> {
    Ok(user_data_dir(constants::ICONS_DIR_NAME)?.join(app_name))
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
    if path.starts_with(constants::USER_HOME_DIR) {
        Ok(home_dir()?.join(path.trim_start_matches(constants::USER_HOME_DIR)))
    } else {
        Ok(PathBuf::from(path))
    }
}

pub fn home_dir() -> Result<PathBuf> {
    let path = env::var_os(constants::USER_HOME_ENV)
        .map(PathBuf::from)
        .ok_or_else(|| VeltrixError::env_missing(constants::USER_HOME_ENV))?;

    if !path.is_absolute() {
        return Err(VeltrixError::env_invalid(
            constants::USER_HOME_ENV,
            "must be absolute",
        ));
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

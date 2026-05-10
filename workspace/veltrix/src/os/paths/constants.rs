// Common names
// Key DIR is used for user-level directories paths (e.g. `~/.config`, `/etc`), while PATH is used for system file paths (e.g. `/etc/fstab`).
// This is a convention and not a strict rule, but it helps distinguish between relative and absolute paths in the code.

/// Name used for application data directories (e.g. `~/.local/share/<app>`).
pub const APPLICATIONS_DIR_NAME: &str = "applications";
/// User systemd unit directory name (under config).
pub const SYSTEMD_DIR_NAME: &str = "systemd";
/// Icon directories name.
pub const ICONS_DIR_NAME: &str = "icons";
/// Default user-local binary directory name.
pub const BIN_DIR_NAME: &str = "bin";

// System-wide paths
#[cfg(feature = "legacy")]
pub const PATH_SYSTEM_ROOT: &str = PATH_SYSTEM_ROOT_DIR;
#[cfg(feature = "legacy")]
pub const PATH_SYSTEM_ROOT_HOME: &str = PATH_SYSTEM_ROOT_HOME_DIR;
#[cfg(feature = "legacy")]
pub const PATH_SYSTEM_HOME: &str = PATH_SYSTEM_HOME_DIR;
#[cfg(feature = "legacy")]
pub const PATH_SYSTEM_BIN: &str = PATH_SYSTEM_BIN_DIR;
#[cfg(feature = "legacy")]
pub const PATH_SYSTEM_LOCAL_BIN: &str = PATH_SYSTEM_LOCAL_BIN_DIR;

// system directories
pub const PATH_SYSTEM_ROOT_DIR: &str = "/";
pub const PATH_SYSTEM_ROOT_HOME_DIR: &str = "/root";
pub const PATH_SYSTEM_HOME_DIR: &str = "/home";
pub const PATH_SYSTEM_BIN_DIR: &str = "/usr/bin";
pub const PATH_SYSTEM_LOCAL_BIN_DIR: &str = "/usr/local/bin";
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
pub const PATH_SYSTEM_FONTS_DIR: &str = "/usr/share/fonts";
pub const PATH_SYSTEM_SUDOERS_DIR: &str = "/etc/sudoers.d";

// System files
pub const PATH_SYSTEM_GROUPS_FILE: &str = "/etc/group";
pub const PATH_SYSTEM_PASSWD_FILE: &str = "/etc/passwd";
pub const PATH_SYSTEM_SUBUID_FILE: &str = "/etc/subuid";
pub const PATH_SYSTEM_SUBGID_FILE: &str = "/etc/subgid";
pub const PATH_SYSTEM_SUDOERS_FILE: &str = "/etc/sudoers";

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

pub const SUBID_SEPARATOR: &str = ":";
pub const SUDO_UID_ENV: &str = "SUDO_UID";
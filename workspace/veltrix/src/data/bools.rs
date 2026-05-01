use crate::error::{Result, VeltrixError};

/// Boolean parsing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolParseMode {
    /// Accept only `true` and `false`.
    Strict,
    /// Accept common configuration truthy/falsy words.
    Permissive,
}

/// Format a boolean as `yes` or `no`.
pub fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

/// Format a boolean as `on` or `off`.
pub fn on_off(value: bool) -> &'static str {
    if value { "on" } else { "off" }
}

/// Format a boolean as `enabled` or `disabled`.
pub fn enabled_disabled(value: bool) -> &'static str {
    if value { "enabled" } else { "disabled" }
}

/// Format a boolean as `true` or `false`.
pub fn true_false(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

/// Parse a boolean with the requested mode.
pub fn parse_bool(value: &str, mode: BoolParseMode) -> Result<bool> {
    let normalized = value.trim().to_ascii_lowercase();

    match normalized.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ if mode == BoolParseMode::Permissive => parse_truthy_falsy(&normalized),
        _ => Err(VeltrixError::validation(
            "bool",
            format!("unsupported boolean value: {value}"),
        )),
    }
}

/// Parse common truthy/falsy configuration values.
pub fn parse_truthy_falsy(value: &str) -> Result<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "t" | "1" | "y" | "yes" | "on" | "enabled" | "active" | "up" | "open"
        | "connected" | "pass" => Ok(true),
        "false" | "f" | "0" | "n" | "no" | "off" | "disabled" | "inactive" | "down" | "closed"
        | "disconnected" | "fail" => Ok(false),
        _ => Err(VeltrixError::validation(
            "bool",
            format!("unsupported truthy/falsy value: {value}"),
        )),
    }
}

/// Return whether a value is recognized as truthy.
pub fn is_true(value: &str) -> bool {
    parse_truthy_falsy(value).unwrap_or(false)
}

/// Return whether a value is recognized as falsy.
pub fn is_false(value: &str) -> bool {
    matches!(parse_truthy_falsy(value), Ok(false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_strict_and_permissive_bools() {
        assert!(parse_bool("true", BoolParseMode::Strict).unwrap());
        assert!(parse_bool("yes", BoolParseMode::Strict).is_err());
        assert!(parse_bool("yes", BoolParseMode::Permissive).unwrap());
        assert!(!parse_truthy_falsy("off").unwrap());
    }

    #[test]
    fn formats_common_pairs() {
        assert_eq!(yes_no(true), "yes");
        assert_eq!(on_off(false), "off");
        assert_eq!(enabled_disabled(true), "enabled");
        assert_eq!(true_false(false), "false");
    }
}

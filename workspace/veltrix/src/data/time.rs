use std::time::Duration;

use crate::error::{Result, VeltrixError};

/// Format a duration as compact `1h2m3s` text.
pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    match (hours, minutes, seconds) {
        (0, 0, seconds) => format!("{seconds}s"),
        (0, minutes, 0) => format!("{minutes}m"),
        (0, minutes, seconds) => format!("{minutes}m{seconds}s"),
        (hours, 0, 0) => format!("{hours}h"),
        (hours, minutes, 0) => format!("{hours}h{minutes}m"),
        (hours, minutes, seconds) => format!("{hours}h{minutes}m{seconds}s"),
    }
}

/// Parse a compact duration such as `30s`, `5m`, `2h`, or `1h30m5s`.
pub fn parse_duration(value: &str) -> Result<Duration> {
    let value = value.trim();

    if value.is_empty() {
        return Err(VeltrixError::validation(
            "duration",
            "duration must not be empty",
        ));
    }

    let mut total = 0_u64;
    let mut number = String::new();

    for ch in value.chars() {
        if ch.is_ascii_digit() {
            number.push(ch);
            continue;
        }

        let amount = number.parse::<u64>().map_err(|_| {
            VeltrixError::validation("duration", format!("missing amount before `{ch}`"))
        })?;
        number.clear();

        total += match ch {
            's' => amount,
            'm' => amount * 60,
            'h' => amount * 3600,
            'd' => amount * 86_400,
            _ => {
                return Err(VeltrixError::validation(
                    "duration",
                    format!("unsupported duration unit: {ch}"),
                ));
            }
        };
    }

    if !number.is_empty() {
        return Err(VeltrixError::validation(
            "duration",
            "duration is missing a unit",
        ));
    }

    Ok(Duration::from_secs(total))
}

/// Return a duration's total whole seconds.
pub fn seconds(duration: Duration) -> u64 {
    duration.as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_formats_compact_durations() {
        let duration = parse_duration("1h2m3s").unwrap();

        assert_eq!(duration.as_secs(), 3723);
        assert_eq!(format_duration(duration), "1h2m3s");
        assert_eq!(format_duration(Duration::from_secs(120)), "2m");
    }

    #[test]
    fn rejects_invalid_durations() {
        assert!(parse_duration("").is_err());
        assert!(parse_duration("10").is_err());
        assert!(parse_duration("1w").is_err());
    }
}

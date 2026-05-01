//! Operating-system and runtime clock helpers.
//!
//! This module asks the OS or runtime for clock state. Parsing and formatting
//! duration strings belongs in `veltrix::data::time`.

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::error::{Result, VeltrixError};

/// Return the current system wall-clock time.
pub fn now() -> SystemTime {
    SystemTime::now()
}

/// Return the current monotonic runtime instant.
pub fn monotonic() -> Instant {
    Instant::now()
}

/// Return elapsed monotonic time since an earlier instant.
pub fn elapsed_since(instant: Instant) -> Duration {
    instant.elapsed()
}

/// Return the current Unix timestamp as a non-negative duration since epoch.
pub fn unix_timestamp() -> Result<Duration> {
    now().duration_since(UNIX_EPOCH).map_err(|err| {
        VeltrixError::validation(
            "system_time",
            format!("system clock is before Unix epoch by {:?}", err.duration()),
        )
    })
}

/// Return system uptime from the platform clock source.
#[cfg(target_os = "linux")]
pub fn uptime() -> Result<Duration> {
    let uptime = std::fs::read_to_string("/proc/uptime")?;
    let seconds = uptime
        .split_whitespace()
        .next()
        .ok_or_else(|| VeltrixError::parsing("missing uptime value in /proc/uptime"))?;

    parse_seconds_duration(seconds)
}

/// Return system uptime from the platform clock source.
#[cfg(not(target_os = "linux"))]
pub fn uptime() -> Result<Duration> {
    Err(unsupported_clock("uptime"))
}

/// Return CPU time consumed by the current process.
#[cfg(target_os = "linux")]
pub fn process_cpu_time() -> Result<Duration> {
    clock_gettime_duration(CLOCK_PROCESS_CPUTIME_ID, "process_cpu_time")
}

/// Return CPU time consumed by the current process.
#[cfg(not(target_os = "linux"))]
pub fn process_cpu_time() -> Result<Duration> {
    Err(unsupported_clock("process_cpu_time"))
}

/// Return CPU time consumed by the current thread.
#[cfg(target_os = "linux")]
pub fn thread_cpu_time() -> Result<Duration> {
    clock_gettime_duration(CLOCK_THREAD_CPUTIME_ID, "thread_cpu_time")
}

/// Return CPU time consumed by the current thread.
#[cfg(not(target_os = "linux"))]
pub fn thread_cpu_time() -> Result<Duration> {
    Err(unsupported_clock("thread_cpu_time"))
}

fn parse_seconds_duration(value: &str) -> Result<Duration> {
    let (seconds, fraction) = value.split_once('.').unwrap_or((value, ""));
    let seconds = seconds
        .parse::<u64>()
        .map_err(|err| VeltrixError::parsing(format!("invalid seconds value `{value}`: {err}")))?;

    let nanos = if fraction.is_empty() {
        0
    } else {
        let mut padded = fraction.chars().take(9).collect::<String>();
        while padded.len() < 9 {
            padded.push('0');
        }
        padded.parse::<u32>().map_err(|err| {
            VeltrixError::parsing(format!("invalid fractional seconds value `{value}`: {err}"))
        })?
    };

    Ok(Duration::new(seconds, nanos))
}

#[cfg(not(target_os = "linux"))]
fn unsupported_clock(name: &str) -> VeltrixError {
    VeltrixError::validation(
        "clock",
        format!("{name} is not supported on this platform yet"),
    )
}

#[cfg(target_os = "linux")]
const CLOCK_PROCESS_CPUTIME_ID: std::os::raw::c_int = 2;
#[cfg(target_os = "linux")]
const CLOCK_THREAD_CPUTIME_ID: std::os::raw::c_int = 3;

#[cfg(target_os = "linux")]
#[repr(C)]
struct Timespec {
    tv_sec: std::os::raw::c_long,
    tv_nsec: std::os::raw::c_long,
}

#[cfg(target_os = "linux")]
unsafe extern "C" {
    fn clock_gettime(clk_id: std::os::raw::c_int, tp: *mut Timespec) -> std::os::raw::c_int;
}

#[cfg(target_os = "linux")]
fn clock_gettime_duration(clock_id: std::os::raw::c_int, name: &str) -> Result<Duration> {
    let mut timespec = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    // SAFETY: `timespec` points to valid writable memory for the duration of
    // the call and the clock IDs used by this module are Linux constants.
    let status = unsafe { clock_gettime(clock_id, &mut timespec) };
    if status != 0 {
        return Err(std::io::Error::last_os_error().into());
    }

    let seconds = u64::try_from(timespec.tv_sec)
        .map_err(|_| VeltrixError::parsing(format!("{name} returned a negative seconds value")))?;
    let nanos = u32::try_from(timespec.tv_nsec).map_err(|_| {
        VeltrixError::parsing(format!("{name} returned an invalid nanoseconds value"))
    })?;

    Ok(Duration::new(seconds, nanos))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix_timestamp_is_after_epoch() {
        assert!(unix_timestamp().unwrap().as_secs() > 0);
    }

    #[test]
    fn monotonic_elapsed_is_non_negative() {
        let start = monotonic();
        assert!(elapsed_since(start) < Duration::from_secs(1));
    }

    #[test]
    fn parses_fractional_seconds() {
        assert_eq!(
            parse_seconds_duration("12.345").unwrap(),
            Duration::new(12, 345_000_000)
        );
        assert_eq!(parse_seconds_duration("12").unwrap(), Duration::new(12, 0));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_runtime_clocks_return_values() {
        assert!(uptime().unwrap() > Duration::ZERO);
        assert!(process_cpu_time().is_ok());
        assert!(thread_cpu_time().is_ok());
    }
}

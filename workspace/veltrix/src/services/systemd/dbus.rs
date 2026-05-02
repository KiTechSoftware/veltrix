use crate::error::{Result, VeltrixError};
use crate::os::process::cmd::{spec::CmdSpec, std_cmd};

use super::{
    spec::{SystemdBackendUsed, SystemdDbusSpec, SystemdEmptyResponse, SystemdResponse},
    types::{SystemdJob, SystemdUnitStatus, SystemdUnitSummary},
};

const SYSTEMD_BUS_NAME: &str = "org.freedesktop.systemd1";
const SYSTEMD_MANAGER_PATH: &str = "/org/freedesktop/systemd1";
const SYSTEMD_MANAGER_INTERFACE: &str = "org.freedesktop.systemd1.Manager";
const SYSTEMD_UNIT_INTERFACE: &str = "org.freedesktop.systemd1.Unit";

/// systemd D-Bus client backed by `busctl`.
#[derive(Debug, Clone)]
pub struct SystemdDbusClient {
    spec: SystemdDbusSpec,
}

impl SystemdDbusClient {
    /// Create a systemd D-Bus client.
    pub fn new(spec: SystemdDbusSpec) -> Self {
        Self { spec }
    }

    /// Get the underlying spec.
    pub fn spec(&self) -> &SystemdDbusSpec {
        &self.spec
    }

    /// Start a unit through `org.freedesktop.systemd1.Manager.StartUnit`.
    pub fn start_unit(&self, unit: &str) -> Result<SystemdResponse<SystemdJob>> {
        self.call_unit_job("StartUnit", unit)
    }

    /// Stop a unit through `org.freedesktop.systemd1.Manager.StopUnit`.
    pub fn stop_unit(&self, unit: &str) -> Result<SystemdResponse<SystemdJob>> {
        self.call_unit_job("StopUnit", unit)
    }

    /// Restart a unit through `org.freedesktop.systemd1.Manager.RestartUnit`.
    pub fn restart_unit(&self, unit: &str) -> Result<SystemdResponse<SystemdJob>> {
        self.call_unit_job("RestartUnit", unit)
    }

    /// Reload a unit through `org.freedesktop.systemd1.Manager.ReloadUnit`.
    pub fn reload_unit(&self, unit: &str) -> Result<SystemdResponse<SystemdJob>> {
        self.call_unit_job("ReloadUnit", unit)
    }

    /// Reload the systemd manager configuration.
    pub fn daemon_reload(&self) -> Result<SystemdEmptyResponse> {
        self.manager_call(["Reload"])?;
        Ok(SystemdEmptyResponse::new(self.backend_used()))
    }

    /// Load a unit and read common status properties through D-Bus.
    pub fn status(&self, unit: &str) -> Result<SystemdResponse<SystemdUnitStatus>> {
        let path = self.load_unit_path(unit)?;
        let data = SystemdUnitStatus {
            unit: unit.to_string(),
            active_state: self.unit_property(&path, "ActiveState")?,
            sub_state: self.unit_property(&path, "SubState")?,
            load_state: self.unit_property(&path, "LoadState")?,
            description: self.unit_property(&path, "Description")?,
        };

        Ok(SystemdResponse::new(data, self.backend_used()))
    }

    /// Return whether a unit is currently active.
    pub fn is_active(&self, unit: &str) -> Result<SystemdResponse<bool>> {
        let status = self.status(unit)?;
        Ok(SystemdResponse::new(
            status.data.active_state.as_deref() == Some("active"),
            self.backend_used(),
        ))
    }

    /// Return whether a unit file is enabled.
    pub fn is_enabled(&self, unit: &str) -> Result<SystemdResponse<bool>> {
        let path = self.load_unit_path(unit)?;
        let enabled = matches!(
            self.unit_property(&path, "UnitFileState")?.as_deref(),
            Some("enabled" | "enabled-runtime")
        );

        Ok(SystemdResponse::new(enabled, self.backend_used()))
    }

    /// Return whether a unit is currently failed.
    pub fn is_failed(&self, unit: &str) -> Result<SystemdResponse<bool>> {
        let status = self.status(unit)?;
        Ok(SystemdResponse::new(
            status.data.active_state.as_deref() == Some("failed"),
            self.backend_used(),
        ))
    }

    /// List loaded units through `org.freedesktop.systemd1.Manager.ListUnits`.
    pub fn list_units(&self) -> Result<SystemdResponse<Vec<SystemdUnitSummary>>> {
        let output = self.manager_call(["ListUnits"])?;
        Ok(SystemdResponse::new(
            parse_list_units(&output)?,
            self.backend_used(),
        ))
    }

    fn call_unit_job(&self, method: &str, unit: &str) -> Result<SystemdResponse<SystemdJob>> {
        let output = self.manager_call([method, "ss", unit, "replace"])?;
        Ok(SystemdResponse::new(
            SystemdJob {
                path: parse_busctl_object_path(&output)?,
            },
            self.backend_used(),
        ))
    }

    fn load_unit_path(&self, unit: &str) -> Result<String> {
        let output = self.manager_call(["LoadUnit", "s", unit])?;
        parse_busctl_object_path(&output)
    }

    fn unit_property(&self, path: &str, property: &str) -> Result<Option<String>> {
        let output = self.run_busctl([
            "get-property",
            SYSTEMD_BUS_NAME,
            path,
            SYSTEMD_UNIT_INTERFACE,
            property,
        ])?;
        Ok(parse_busctl_string_property(&output))
    }

    fn manager_call<I, S>(&self, args: I) -> Result<String>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut command_args = vec![
            "call".to_string(),
            SYSTEMD_BUS_NAME.to_string(),
            SYSTEMD_MANAGER_PATH.to_string(),
            SYSTEMD_MANAGER_INTERFACE.to_string(),
        ];
        command_args.extend(args.into_iter().map(Into::into));
        self.run_busctl(command_args)
    }

    fn run_busctl<I, S>(&self, args: I) -> Result<String>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_command(base_busctl_cmd(&self.spec).args(args), "systemd")
    }

    fn backend_used(&self) -> SystemdBackendUsed {
        SystemdBackendUsed::Dbus {
            user: self.spec.user,
            sudo: self.spec.sudo,
            busctl: self.spec.busctl.clone(),
        }
    }
}

fn base_busctl_cmd(spec: &SystemdDbusSpec) -> CmdSpec {
    let mut cmd = CmdSpec::new(&spec.busctl);
    if spec.sudo {
        cmd = cmd.sudo();
    }
    if spec.user {
        cmd = cmd.arg("--user");
    }
    cmd
}

fn run_command(cmd: CmdSpec, service: &str) -> Result<String> {
    let result = std_cmd::run(cmd)?;
    if !result.status.success() {
        return Err(VeltrixError::service(
            service,
            String::from_utf8_lossy(&result.stderr).trim().to_string(),
        ));
    }

    String::from_utf8(result.stdout)
        .map_err(|err| VeltrixError::parsing(format!("invalid systemd D-Bus stdout: {err}")))
}

fn parse_busctl_object_path(output: &str) -> Result<String> {
    let trimmed = output.trim();
    for token in trimmed.split_whitespace().rev() {
        let token = token.trim_matches('"');
        if token.starts_with('/') {
            return Ok(token.to_string());
        }
    }

    Err(VeltrixError::parsing(format!(
        "systemd D-Bus output did not contain an object path: {trimmed}"
    )))
}

fn parse_busctl_string_property(output: &str) -> Option<String> {
    let trimmed = output.trim();
    if trimmed == "s \"\"" {
        return None;
    }

    if let Some(start) = trimmed.find('"') {
        let rest = &trimmed[start + 1..];
        if let Some(end) = rest.rfind('"') {
            return Some(rest[..end].to_string());
        }
    }

    trimmed
        .split_whitespace()
        .nth(1)
        .map(|value| value.trim_matches('"').to_string())
        .filter(|value| !value.is_empty())
}

fn parse_list_units(output: &str) -> Result<Vec<SystemdUnitSummary>> {
    let strings = quoted_strings(output);
    if strings.is_empty() {
        return Ok(Vec::new());
    }
    if !strings.len().is_multiple_of(7) {
        return Err(VeltrixError::parsing(format!(
            "unexpected systemd ListUnits D-Bus output: {} quoted fields",
            strings.len()
        )));
    }

    Ok(strings
        .chunks(7)
        .map(|chunk| SystemdUnitSummary {
            unit: chunk[0].clone(),
            description: optional_string(&chunk[1]),
            load_state: optional_string(&chunk[2]),
            active_state: optional_string(&chunk[3]),
            sub_state: optional_string(&chunk[4]),
        })
        .collect())
}

fn quoted_strings(output: &str) -> Vec<String> {
    let mut strings = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    let mut escaped = false;

    for ch in output.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }

        match ch {
            '\\' if in_string => escaped = true,
            '"' if in_string => {
                strings.push(std::mem::take(&mut current));
                in_string = false;
            }
            '"' => in_string = true,
            _ if in_string => current.push(ch),
            _ => {}
        }
    }

    strings
}

fn optional_string(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_busctl_object_paths() {
        assert_eq!(
            parse_busctl_object_path(r#"o "/org/freedesktop/systemd1/job/42""#).unwrap(),
            "/org/freedesktop/systemd1/job/42"
        );
        assert_eq!(
            parse_busctl_object_path(r#"/org/freedesktop/systemd1/unit/demo_2eservice"#).unwrap(),
            "/org/freedesktop/systemd1/unit/demo_2eservice"
        );
    }

    #[test]
    fn parses_busctl_string_properties() {
        assert_eq!(
            parse_busctl_string_property(r#"s "active""#).as_deref(),
            Some("active")
        );
        assert_eq!(parse_busctl_string_property(r#"s """#), None);
    }

    #[test]
    fn parses_list_units_output() {
        let units = parse_list_units(
            r#"a(ssssssouso) 1 "demo.service" "Demo Service" "loaded" "active" "running" "" /org/freedesktop/systemd1/unit/demo_2eservice 0 "" /"#,
        )
        .unwrap();

        assert_eq!(units.len(), 1);
        assert_eq!(units[0].unit, "demo.service");
        assert_eq!(units[0].description.as_deref(), Some("Demo Service"));
        assert_eq!(units[0].load_state.as_deref(), Some("loaded"));
        assert_eq!(units[0].active_state.as_deref(), Some("active"));
        assert_eq!(units[0].sub_state.as_deref(), Some("running"));
    }
}

use crate::error::{Result, VeltrixError};
use crate::os::process::cmd::{spec::CmdSpec, std_cmd};

use super::{
    spec::{SystemdBackendUsed, SystemdCliSpec, SystemdEmptyResponse, SystemdResponse},
    types::{
        SystemdDependency, SystemdJournal, SystemdJournalEntry, SystemdResourceLimit,
        SystemdUnitFile, SystemdUnitStatus,
    },
};

/// systemd CLI client backed by systemctl and journalctl.
#[derive(Debug, Clone)]
pub struct SystemdCliClient {
    spec: SystemdCliSpec,
}

impl SystemdCliClient {
    pub fn new(spec: SystemdCliSpec) -> Self {
        Self { spec }
    }

    pub fn spec(&self) -> &SystemdCliSpec {
        &self.spec
    }

    pub fn start_unit(&self, unit: &str) -> Result<SystemdEmptyResponse> {
        self.systemctl_empty(["start", unit])
    }

    pub fn stop_unit(&self, unit: &str) -> Result<SystemdEmptyResponse> {
        self.systemctl_empty(["stop", unit])
    }

    pub fn restart_unit(&self, unit: &str) -> Result<SystemdEmptyResponse> {
        self.systemctl_empty(["restart", unit])
    }

    pub fn reload_unit(&self, unit: &str) -> Result<SystemdEmptyResponse> {
        self.systemctl_empty(["reload", unit])
    }

    pub fn enable_unit(&self, unit: &str) -> Result<SystemdEmptyResponse> {
        self.systemctl_empty(["enable", unit])
    }

    pub fn enable_now_unit(&self, unit: &str) -> Result<SystemdEmptyResponse> {
        self.systemctl_empty(["enable", "--now", unit])
    }

    pub fn disable_unit(&self, unit: &str) -> Result<SystemdEmptyResponse> {
        self.systemctl_empty(["disable", unit])
    }

    pub fn mask_unit(&self, unit: &str) -> Result<SystemdEmptyResponse> {
        self.systemctl_empty(["mask", unit])
    }

    pub fn unmask_unit(&self, unit: &str) -> Result<SystemdEmptyResponse> {
        self.systemctl_empty(["unmask", unit])
    }

    pub fn daemon_reload(&self) -> Result<SystemdEmptyResponse> {
        self.systemctl_empty(["daemon-reload"])
    }

    pub fn status(&self, unit: &str) -> Result<SystemdResponse<SystemdUnitStatus>> {
        let output = self.systemctl_output(["show", unit, "--no-page"])?;
        let data = parse_status(unit, &output);

        Ok(SystemdResponse::new(data, self.backend_used()))
    }

    pub fn properties(&self, unit: &str) -> Result<SystemdResponse<String>> {
        let output = self.systemctl_output(["show", unit, "--no-page"])?;

        Ok(SystemdResponse::new(output, self.backend_used()))
    }

    pub fn dependencies(&self, unit: &str) -> Result<SystemdResponse<Vec<SystemdDependency>>> {
        let output = self.systemctl_output(["list-dependencies", unit, "--plain", "--no-pager"])?;
        let data = output
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|unit| SystemdDependency {
                unit: unit.to_string(),
            })
            .collect();

        Ok(SystemdResponse::new(data, self.backend_used()))
    }

    pub fn journal(
        &self,
        unit: &str,
        lines: Option<u32>,
    ) -> Result<SystemdResponse<SystemdJournal>> {
        let mut args = vec!["-u".to_string(), unit.to_string(), "--no-pager".to_string()];

        if let Some(lines) = lines {
            args.push("-n".to_string());
            args.push(lines.to_string());
        }

        let output = self.journalctl_output(args)?;
        Ok(SystemdResponse::new(
            SystemdJournal { output },
            self.backend_used(),
        ))
    }

    pub fn journal_entries(
        &self,
        unit: &str,
        lines: Option<u32>,
    ) -> Result<SystemdResponse<Vec<SystemdJournalEntry>>> {
        let mut args = vec![
            "-u".to_string(),
            unit.to_string(),
            "-o".to_string(),
            "json".to_string(),
            "--no-pager".to_string(),
        ];

        if let Some(lines) = lines {
            args.push("-n".to_string());
            args.push(lines.to_string());
        }

        let output = self.journalctl_output(args)?;
        Ok(SystemdResponse::new(
            parse_journal_entries(&output)?,
            self.backend_used(),
        ))
    }

    pub fn tail_journal(&self, unit: &str, lines: u32) -> Result<SystemdResponse<SystemdJournal>> {
        self.journal(unit, Some(lines))
    }

    pub fn tail_journal_entries(
        &self,
        unit: &str,
        lines: u32,
    ) -> Result<SystemdResponse<Vec<SystemdJournalEntry>>> {
        self.journal_entries(unit, Some(lines))
    }

    pub fn journal_since(
        &self,
        unit: &str,
        since: &str,
        until: Option<&str>,
    ) -> Result<SystemdResponse<SystemdJournal>> {
        let mut args = vec![
            "-u".to_string(),
            unit.to_string(),
            "--since".to_string(),
            since.to_string(),
            "--no-pager".to_string(),
        ];
        if let Some(until) = until {
            args.push("--until".to_string());
            args.push(until.to_string());
        }

        let output = self.journalctl_output(args)?;
        Ok(SystemdResponse::new(
            SystemdJournal { output },
            self.backend_used(),
        ))
    }

    pub fn journal_entries_since(
        &self,
        unit: &str,
        since: &str,
        until: Option<&str>,
    ) -> Result<SystemdResponse<Vec<SystemdJournalEntry>>> {
        let mut args = vec![
            "-u".to_string(),
            unit.to_string(),
            "--since".to_string(),
            since.to_string(),
            "-o".to_string(),
            "json".to_string(),
            "--no-pager".to_string(),
        ];
        if let Some(until) = until {
            args.push("--until".to_string());
            args.push(until.to_string());
        }

        let output = self.journalctl_output(args)?;
        Ok(SystemdResponse::new(
            parse_journal_entries(&output)?,
            self.backend_used(),
        ))
    }

    pub fn journal_boot(&self, unit: &str) -> Result<SystemdResponse<SystemdJournal>> {
        let output = self.journalctl_output(["-u", unit, "-b", "--no-pager"])?;
        Ok(SystemdResponse::new(
            SystemdJournal { output },
            self.backend_used(),
        ))
    }

    pub fn journal_entries_boot(
        &self,
        unit: &str,
    ) -> Result<SystemdResponse<Vec<SystemdJournalEntry>>> {
        let output = self.journalctl_output(["-u", unit, "-b", "-o", "json", "--no-pager"])?;
        Ok(SystemdResponse::new(
            parse_journal_entries(&output)?,
            self.backend_used(),
        ))
    }

    pub fn journal_priority(
        &self,
        unit: &str,
        priority: &str,
    ) -> Result<SystemdResponse<SystemdJournal>> {
        let output = self.journalctl_output(["-u", unit, "-p", priority, "--no-pager"])?;
        Ok(SystemdResponse::new(
            SystemdJournal { output },
            self.backend_used(),
        ))
    }

    pub fn journal_entries_priority(
        &self,
        unit: &str,
        priority: &str,
    ) -> Result<SystemdResponse<Vec<SystemdJournalEntry>>> {
        let output =
            self.journalctl_output(["-u", unit, "-p", priority, "-o", "json", "--no-pager"])?;
        Ok(SystemdResponse::new(
            parse_journal_entries(&output)?,
            self.backend_used(),
        ))
    }

    pub fn edit_unit(&self, unit: &str) -> Result<SystemdEmptyResponse> {
        self.systemctl_empty(["edit", unit])
    }

    pub fn edit_unit_drop_in(&self, unit: &str, drop_in: &str) -> Result<SystemdEmptyResponse> {
        self.systemctl_empty(["edit", unit, "--drop-in", drop_in])
    }

    pub fn edit_unit_full(&self, unit: &str) -> Result<SystemdEmptyResponse> {
        self.systemctl_empty(["edit", "--full", unit])
    }

    pub fn revert_unit(&self, unit: &str) -> Result<SystemdEmptyResponse> {
        self.systemctl_empty(["revert", unit])
    }

    pub fn list_unit_files(
        &self,
        pattern: Option<&str>,
    ) -> Result<SystemdResponse<Vec<SystemdUnitFile>>> {
        let mut args = vec!["list-unit-files".to_string(), "--no-pager".to_string()];
        if let Some(pattern) = pattern {
            args.push(pattern.to_string());
        }
        let output = self.systemctl_output(args)?;
        Ok(SystemdResponse::new(
            parse_unit_files(&output),
            self.backend_used(),
        ))
    }

    pub fn list_timers(&self) -> Result<SystemdResponse<String>> {
        let output = self.systemctl_output(["list-timers", "--all", "--no-pager"])?;
        Ok(SystemdResponse::new(output, self.backend_used()))
    }

    pub fn enable_timer(&self, timer: &str) -> Result<SystemdEmptyResponse> {
        self.enable_unit(timer)
    }

    pub fn disable_timer(&self, timer: &str) -> Result<SystemdEmptyResponse> {
        self.disable_unit(timer)
    }

    pub fn cat_unit(&self, unit: &str) -> Result<SystemdResponse<String>> {
        let output = self.systemctl_output(["cat", unit, "--no-pager"])?;
        Ok(SystemdResponse::new(output, self.backend_used()))
    }

    pub fn template_instance(template: &str, instance: &str) -> String {
        let template = template.trim_end_matches(".service").trim_end_matches('@');
        format!("{template}@{instance}.service")
    }

    pub fn set_resource_limits<I>(&self, unit: &str, limits: I) -> Result<SystemdEmptyResponse>
    where
        I: IntoIterator<Item = SystemdResourceLimit>,
    {
        let mut args = vec!["set-property".to_string(), unit.to_string()];
        args.extend(limits.into_iter().map(|limit| limit.assignment()));
        self.systemctl_empty(args)
    }

    pub fn watchdog_status(&self, unit: &str) -> Result<SystemdResponse<String>> {
        let output = self.systemctl_output([
            "show",
            unit,
            "-p",
            "WatchdogUSec",
            "-p",
            "WatchdogTimestamp",
        ])?;
        Ok(SystemdResponse::new(output, self.backend_used()))
    }

    pub fn set_watchdog_usec(
        &self,
        unit: &str,
        watchdog_usec: &str,
    ) -> Result<SystemdEmptyResponse> {
        self.set_resource_limits(
            unit,
            [SystemdResourceLimit::new("WatchdogUSec", watchdog_usec)],
        )
    }

    pub fn deploy_unit(&self, unit: &str) -> Result<SystemdEmptyResponse> {
        self.daemon_reload()?;
        self.enable_unit(unit)?;
        self.restart_unit(unit)
    }

    fn systemctl_empty<I, S>(&self, args: I) -> Result<SystemdEmptyResponse>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.systemctl_output(args)?;
        Ok(SystemdEmptyResponse::new(self.backend_used()))
    }

    fn systemctl_output<I, S>(&self, args: I) -> Result<String>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_command(base_systemctl_cmd(&self.spec).args(args), "systemd")
    }

    fn journalctl_output<I, S>(&self, args: I) -> Result<String>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        run_command(base_journalctl_cmd(&self.spec).args(args), "systemd")
    }

    fn backend_used(&self) -> SystemdBackendUsed {
        SystemdBackendUsed::Cli {
            user: self.spec.user,
            sudo: self.spec.sudo,
        }
    }
}

fn base_systemctl_cmd(spec: &SystemdCliSpec) -> CmdSpec {
    let mut cmd = CmdSpec::new(&spec.systemctl);
    if spec.sudo {
        cmd = cmd.sudo();
    }
    if spec.user {
        cmd = cmd.arg("--user");
    }
    cmd
}

fn base_journalctl_cmd(spec: &SystemdCliSpec) -> CmdSpec {
    let mut cmd = CmdSpec::new(&spec.journalctl);
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
        .map_err(|err| VeltrixError::parsing(format!("invalid systemd stdout: {err}")))
}

fn parse_status(unit: &str, output: &str) -> SystemdUnitStatus {
    SystemdUnitStatus {
        unit: unit.to_string(),
        active_state: property(output, "ActiveState"),
        sub_state: property(output, "SubState"),
        load_state: property(output, "LoadState"),
        description: property(output, "Description"),
    }
}

fn property(output: &str, key: &str) -> Option<String> {
    output.lines().find_map(|line| {
        let (name, value) = line.split_once('=')?;
        (name == key).then(|| value.to_string())
    })
}

fn parse_unit_files(output: &str) -> Vec<SystemdUnitFile> {
    output
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let name = parts.next()?;
            if name == "UNIT" || name.ends_with(".") {
                return None;
            }
            Some(SystemdUnitFile {
                name: name.to_string(),
                state: parts.next().map(ToOwned::to_owned),
            })
        })
        .collect()
}

fn parse_journal_entries(output: &str) -> Result<Vec<SystemdJournalEntry>> {
    output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let fields = serde_json::from_str(line).map_err(|err| {
                VeltrixError::parsing(format!("invalid systemd journal json: {err}"))
            })?;
            Ok(SystemdJournalEntry::from_fields(fields))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_systemd_show_output() {
        let status = parse_status(
            "demo.service",
            "Description=Demo\nLoadState=loaded\nActiveState=active\nSubState=running\n",
        );
        assert_eq!(status.active_state.as_deref(), Some("active"));
        assert_eq!(status.description.as_deref(), Some("Demo"));
    }

    #[test]
    fn builds_template_instance_names() {
        assert_eq!(
            SystemdCliClient::template_instance("worker@.service", "blue"),
            "worker@blue.service"
        );
        assert_eq!(
            SystemdCliClient::template_instance("worker", "blue"),
            "worker@blue.service"
        );
    }

    #[test]
    fn resource_limit_formats_assignment() {
        assert_eq!(
            SystemdResourceLimit::new("MemoryMax", "512M").assignment(),
            "MemoryMax=512M"
        );
    }

    #[test]
    fn parses_structured_journal_entries() {
        let entries = parse_journal_entries(
            r#"{"MESSAGE":"started","_SYSTEMD_UNIT":"demo.service","PRIORITY":"6","__REALTIME_TIMESTAMP":"1710000000000000"}"#,
        )
        .unwrap();

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].message.as_deref(), Some("started"));
        assert_eq!(entries[0].unit.as_deref(), Some("demo.service"));
        assert_eq!(entries[0].priority.as_deref(), Some("6"));
    }
}

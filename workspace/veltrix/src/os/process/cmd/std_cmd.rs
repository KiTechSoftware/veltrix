use std::{
    io,
    process::{Command, Output},
};

use crate::os::process::cmd::spec::CmdSpec;

impl CmdSpec {
    pub fn to_std_command(&self) -> Command {
        build_std(self)
    }
}

fn build_std(spec: &CmdSpec) -> Command {
    if spec.sudo {
        let mut cmd = Command::new("sudo");

        if let Some(uid) = spec.uid {
            cmd.arg("-u").arg(format!("#{uid}"));
        }

        if let Some(gid) = spec.gid {
            cmd.arg("-g").arg(format!("#{gid}"));
        }

        cmd.arg("--");
        cmd.arg(&spec.program);
        cmd.args(&spec.args);
        return cmd;
    }

    let mut cmd = Command::new(&spec.program);
    cmd.args(&spec.args);

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;

        if let Some(gid) = spec.gid {
            cmd.gid(gid);
        }

        if let Some(uid) = spec.uid {
            cmd.uid(uid);
        }
    }

    cmd
}

pub fn run(spec: CmdSpec) -> io::Result<Output> {
    let mut cmd = spec.to_std_command();
    cmd.output()
}

pub fn status_ok(spec: CmdSpec) -> io::Result<bool> {
    Ok(run(spec)?.status.success())
}
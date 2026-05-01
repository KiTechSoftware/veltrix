use std::io;

use super::spec::CmdSpec;
use tokio::process::Command;

#[cfg(unix)]
use std::os::unix::process::CommandExt;

impl CmdSpec {
    pub fn to_tokio_command(&self) -> Command {
        build_tokio(self)
    }
}

fn build_tokio(spec: &CmdSpec) -> Command {
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
        if let Some(gid) = spec.gid {
            cmd.gid(gid);
        }

        if let Some(uid) = spec.uid {
            cmd.uid(uid);
        }
    }

    cmd
}

pub async fn run(spec: CmdSpec) -> io::Result<std::process::Output> {
    let mut cmd = spec.to_tokio_command();
    cmd.output().await
}

pub async fn status_ok(spec: CmdSpec) -> io::Result<bool> {
    Ok(run(spec).await?.status.success())
}

#[derive(Debug, Clone)]
pub struct CmdSpec {
    pub program: String,
    pub args: Vec<String>,
    pub sudo: bool,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
}


impl CmdSpec {
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            sudo: false,
            uid: None,
            gid: None,
        }
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    /// If true, the command is executed via `sudo` regardless of current UID.
    /// This ensures consistent behavior across environments.
    pub fn sudo(mut self) -> Self {
        self.sudo = true;
        self
    }

    pub fn uid(mut self, uid: u32) -> Self {
        self.uid = Some(uid);
        self
    }

    pub fn gid(mut self, gid: u32) -> Self {
        self.gid = Some(gid);
        self
    }
}

#[cfg(feature = "unistd")]
impl CmdSpec {
    pub fn uid_unistd(mut self, uid: crate::unistd::Uid) -> Self {
        self.uid = Some(uid.as_raw());
        self
    }

    pub fn gid_unistd(mut self, gid: crate::unistd::Gid) -> Self {
        self.gid = Some(gid.as_raw());
        self
    }

    pub fn user(mut self, uid: crate::unistd::Uid) -> Self {
        self.uid = Some(uid.as_raw());
        self
    }

    pub fn group(mut self, gid: crate::unistd::Gid) -> Self {
        self.gid = Some(gid.as_raw());
        self
    }
}
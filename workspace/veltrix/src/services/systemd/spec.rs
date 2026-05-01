/// systemd CLI backend configuration.
#[derive(Debug, Clone)]
pub struct SystemdCliSpec {
    /// systemctl executable name or path.
    pub systemctl: String,
    /// journalctl executable name or path.
    pub journalctl: String,
    /// Whether commands target the user service manager.
    pub user: bool,
    /// Whether to execute through sudo.
    pub sudo: bool,
}

/// systemd D-Bus backend configuration.
#[derive(Debug, Clone)]
pub struct SystemdDbusSpec {
    /// busctl executable name or path.
    pub busctl: String,
    /// Whether commands target the user service manager.
    pub user: bool,
    /// Whether to execute busctl through sudo.
    pub sudo: bool,
}

impl Default for SystemdDbusSpec {
    fn default() -> Self {
        Self {
            busctl: "busctl".to_string(),
            user: false,
            sudo: false,
        }
    }
}

impl SystemdDbusSpec {
    /// Create a default systemd D-Bus spec.
    pub fn new() -> Self {
        Self::default()
    }

    /// Target the user service manager.
    pub fn user(mut self) -> Self {
        self.user = true;
        self
    }

    /// Execute busctl through sudo.
    pub fn sudo(mut self) -> Self {
        self.sudo = true;
        self
    }

    /// Set the busctl executable.
    pub fn busctl(mut self, binary: impl Into<String>) -> Self {
        self.busctl = binary.into();
        self
    }
}

impl Default for SystemdCliSpec {
    fn default() -> Self {
        Self {
            systemctl: "systemctl".to_string(),
            journalctl: "journalctl".to_string(),
            user: false,
            sudo: false,
        }
    }
}

impl SystemdCliSpec {
    /// Create a default systemd CLI spec.
    pub fn new() -> Self {
        Self::default()
    }

    /// Target the user service manager.
    pub fn user(mut self) -> Self {
        self.user = true;
        self
    }

    /// Execute commands through sudo.
    pub fn sudo(mut self) -> Self {
        self.sudo = true;
        self
    }

    /// Set the systemctl executable.
    pub fn systemctl(mut self, binary: impl Into<String>) -> Self {
        self.systemctl = binary.into();
        self
    }

    /// Set the journalctl executable.
    pub fn journalctl(mut self, binary: impl Into<String>) -> Self {
        self.journalctl = binary.into();
        self
    }
}

/// Metadata describing the systemd backend used for a response.
#[derive(Debug, Clone)]
pub enum SystemdBackendUsed {
    /// systemctl/journalctl CLI backend.
    Cli { user: bool, sudo: bool },
    /// org.freedesktop.systemd1 D-Bus backend via busctl.
    Dbus {
        /// Whether the user manager bus was used.
        user: bool,
        /// Whether busctl was executed through sudo.
        sudo: bool,
        /// busctl executable name or path.
        busctl: String,
    },
}

/// Response wrapper for systemd operations with data.
#[derive(Debug, Clone)]
pub struct SystemdResponse<T> {
    pub backend: SystemdBackendUsed,
    pub data: T,
}

impl<T> SystemdResponse<T> {
    pub fn new(data: T, backend: SystemdBackendUsed) -> Self {
        Self { backend, data }
    }
}

/// Response wrapper for successful systemd operations with no body.
#[derive(Debug, Clone)]
pub struct SystemdEmptyResponse {
    pub backend: SystemdBackendUsed,
}

impl SystemdEmptyResponse {
    pub fn new(backend: SystemdBackendUsed) -> Self {
        Self { backend }
    }
}

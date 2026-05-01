/// High-level systemd unit status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemdUnitStatus {
    pub unit: String,
    pub active_state: Option<String>,
    pub sub_state: Option<String>,
    pub load_state: Option<String>,
    pub description: Option<String>,
}

/// systemd unit-file listing entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemdUnitFile {
    pub name: String,
    pub state: Option<String>,
}

/// systemd dependency entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemdDependency {
    pub unit: String,
}

/// Journal output captured from journalctl.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemdJournal {
    pub output: String,
}

/// Unit resource limit assignment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemdResourceLimit {
    pub property: String,
    pub value: String,
}

impl SystemdResourceLimit {
    pub fn new(property: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            property: property.into(),
            value: value.into(),
        }
    }

    pub fn assignment(&self) -> String {
        format!("{}={}", self.property, self.value)
    }
}

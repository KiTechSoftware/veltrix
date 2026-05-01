use std::collections::BTreeMap;

use serde_json::Value;

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

/// Structured journal entry parsed from `journalctl -o json`.
#[derive(Debug, Clone, PartialEq)]
pub struct SystemdJournalEntry {
    pub message: Option<String>,
    pub unit: Option<String>,
    pub priority: Option<String>,
    pub timestamp_realtime_usec: Option<String>,
    pub fields: BTreeMap<String, Value>,
}

impl SystemdJournalEntry {
    pub fn from_fields(fields: BTreeMap<String, Value>) -> Self {
        Self {
            message: string_field(&fields, "MESSAGE"),
            unit: string_field(&fields, "_SYSTEMD_UNIT").or_else(|| string_field(&fields, "UNIT")),
            priority: string_field(&fields, "PRIORITY"),
            timestamp_realtime_usec: string_field(&fields, "__REALTIME_TIMESTAMP"),
            fields,
        }
    }
}

/// systemd D-Bus job object returned by manager lifecycle calls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemdJob {
    pub path: String,
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

fn string_field(fields: &BTreeMap<String, Value>, key: &str) -> Option<String> {
    fields.get(key).and_then(|value| match value {
        Value::String(value) => Some(value.clone()),
        Value::Number(value) => Some(value.to_string()),
        Value::Bool(value) => Some(value.to_string()),
        _ => None,
    })
}

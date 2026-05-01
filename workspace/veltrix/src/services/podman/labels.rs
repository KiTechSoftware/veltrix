use std::fmt;

use crate::error::{Result, VeltrixError};

use super::quadlet::{PODMAN_AUTO_UPDATE_LABEL, PodmanAutoUpdatePolicy};

/// A Podman label key/value pair.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PodmanLabel {
    key: String,
    value: String,
}

impl PodmanLabel {
    /// Create a Podman label.
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Result<Self> {
        let key = key.into();
        let value = value.into();

        validate_label_part("key", &key)?;
        validate_label_part("value", &value)?;

        Ok(Self { key, value })
    }

    /// Create Podman's auto-update label.
    pub fn auto_update(policy: PodmanAutoUpdatePolicy) -> Self {
        Self {
            key: PODMAN_AUTO_UPDATE_LABEL.to_string(),
            value: policy.as_label_value().to_string(),
        }
    }

    /// Return the label key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Return the label value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Return the `key=value` form accepted by Podman CLI and Quadlet.
    pub fn as_label_arg(&self) -> String {
        format!("{}={}", self.key, self.value)
    }
}

impl fmt::Display for PodmanLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_label_arg())
    }
}

/// A collection of Podman labels.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PodmanLabels {
    labels: Vec<PodmanLabel>,
}

impl PodmanLabels {
    /// Create an empty label collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create labels from `(key, value)` pairs.
    pub fn from_pairs<I, K, V>(pairs: I) -> Result<Self>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let mut labels = Self::new();

        for (key, value) in pairs {
            labels = labels.label(PodmanLabel::new(key, value)?);
        }

        Ok(labels)
    }

    /// Add a label.
    pub fn label(mut self, label: PodmanLabel) -> Self {
        self.labels.push(label);
        self
    }

    /// Add Podman's auto-update label.
    pub fn auto_update(self, policy: PodmanAutoUpdatePolicy) -> Self {
        self.label(PodmanLabel::auto_update(policy))
    }

    /// Return labels as a slice.
    pub fn as_slice(&self) -> &[PodmanLabel] {
        &self.labels
    }

    /// Return whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    /// Return CLI arguments in `--label key=value` form.
    pub fn to_cli_args(&self) -> Vec<String> {
        labels_to_cli_args(&self.labels)
    }
}

impl IntoIterator for PodmanLabels {
    type IntoIter = std::vec::IntoIter<PodmanLabel>;
    type Item = PodmanLabel;

    fn into_iter(self) -> Self::IntoIter {
        self.labels.into_iter()
    }
}

/// Convert labels to repeated `--label key=value` CLI arguments.
pub fn labels_to_cli_args(labels: &[PodmanLabel]) -> Vec<String> {
    let mut args = Vec::with_capacity(labels.len() * 2);

    for label in labels {
        args.push("--label".to_string());
        args.push(label.as_label_arg());
    }

    args
}

fn validate_label_part(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(VeltrixError::validation(
            field,
            "label field must not be empty",
        ));
    }

    if value.contains('\0') {
        return Err(VeltrixError::validation(
            field,
            "label field must not contain NUL bytes",
        ));
    }

    if value.contains('\n') || value.contains('\r') {
        return Err(VeltrixError::validation(
            field,
            "label field must not contain newlines",
        ));
    }

    if field == "key" && value.contains('=') {
        return Err(VeltrixError::validation(
            field,
            "label key must not contain '='",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_formats_for_podman() {
        let label = PodmanLabel::new("com.example.role", "web").expect("label is valid");

        assert_eq!(label.key(), "com.example.role");
        assert_eq!(label.value(), "web");
        assert_eq!(label.as_label_arg(), "com.example.role=web");
        assert_eq!(label.to_string(), "com.example.role=web");
    }

    #[test]
    fn label_collection_builds_cli_args() {
        let labels =
            PodmanLabels::from_pairs([("com.example.role", "web"), ("com.example.env", "dev")])
                .expect("labels are valid")
                .auto_update(PodmanAutoUpdatePolicy::Registry);

        assert_eq!(
            labels.to_cli_args(),
            [
                "--label",
                "com.example.role=web",
                "--label",
                "com.example.env=dev",
                "--label",
                "io.containers.autoupdate=registry",
            ]
        );
    }

    #[test]
    fn invalid_label_key_is_rejected() {
        let err = PodmanLabel::new("bad=key", "value").expect_err("key is invalid");

        assert!(err.to_string().contains("label key"));
    }
}

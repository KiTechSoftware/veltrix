use std::collections::BTreeMap;

use super::labels::PodmanLabel;

/// Podman auto-update label key recognized by `podman auto-update`.
pub const PODMAN_AUTO_UPDATE_LABEL: &str = "io.containers.autoupdate";

/// Podman auto-update policies supported by Quadlet and container labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PodmanAutoUpdatePolicy {
    /// Pull updates from the image registry.
    Registry,
    /// Use locally available images.
    Local,
}

impl PodmanAutoUpdatePolicy {
    /// Return the label value expected by Podman.
    pub fn as_label_value(self) -> &'static str {
        match self {
            Self::Registry => "registry",
            Self::Local => "local",
        }
    }

    /// Return a complete CLI label argument such as
    /// `io.containers.autoupdate=registry`.
    pub fn as_label_arg(self) -> String {
        format!("{PODMAN_AUTO_UPDATE_LABEL}={}", self.as_label_value())
    }
}

/// Quadlet unit file kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuadletUnitKind {
    /// A `.container` Quadlet file.
    Container,
    /// A `.pod` Quadlet file.
    Pod,
    /// A `.volume` Quadlet file.
    Volume,
    /// A `.network` Quadlet file.
    Network,
    /// A `.kube` Quadlet file.
    Kube,
}

impl QuadletUnitKind {
    /// Return the file extension used by systemd Quadlet.
    pub fn extension(self) -> &'static str {
        match self {
            Self::Container => "container",
            Self::Pod => "pod",
            Self::Volume => "volume",
            Self::Network => "network",
            Self::Kube => "kube",
        }
    }

    /// Return the primary section for this Quadlet kind.
    pub fn section(self) -> &'static str {
        match self {
            Self::Container => "Container",
            Self::Pod => "Pod",
            Self::Volume => "Volume",
            Self::Network => "Network",
            Self::Kube => "Kube",
        }
    }
}

/// Builder for simple Podman Quadlet unit files.
#[derive(Debug, Clone)]
pub struct QuadletUnit {
    kind: QuadletUnitKind,
    name: String,
    sections: BTreeMap<String, Vec<(String, String)>>,
}

impl QuadletUnit {
    /// Create a generic Quadlet unit.
    pub fn new(kind: QuadletUnitKind, name: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
            sections: BTreeMap::new(),
        }
    }

    /// Create a `.container` Quadlet unit with an image.
    pub fn container(name: impl Into<String>, image: impl Into<String>) -> Self {
        Self::new(QuadletUnitKind::Container, name).entry("Container", "Image", image)
    }

    /// Create a `.pod` Quadlet unit.
    pub fn pod(name: impl Into<String>) -> Self {
        Self::new(QuadletUnitKind::Pod, name)
    }

    /// Create a `.kube` Quadlet unit with a YAML path.
    pub fn kube(name: impl Into<String>, yaml: impl Into<String>) -> Self {
        Self::new(QuadletUnitKind::Kube, name).entry("Kube", "Yaml", yaml)
    }

    /// Add or append a key/value entry to a section.
    pub fn entry(
        mut self,
        section: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.sections
            .entry(section.into())
            .or_default()
            .push((key.into(), value.into()));
        self
    }

    /// Add an auto-update label to a container Quadlet.
    pub fn auto_update(self, policy: PodmanAutoUpdatePolicy) -> Self {
        self.label(PodmanLabel::auto_update(policy))
    }

    /// Add a label to a container Quadlet.
    pub fn label(self, label: PodmanLabel) -> Self {
        self.entry("Container", "Label", label.as_label_arg())
    }

    /// Add multiple labels to a container Quadlet.
    pub fn labels<I>(mut self, labels: I) -> Self
    where
        I: IntoIterator<Item = PodmanLabel>,
    {
        for label in labels {
            self = self.label(label);
        }

        self
    }

    /// Return this unit's Quadlet kind.
    pub fn kind(&self) -> QuadletUnitKind {
        self.kind
    }

    /// Return this unit's file name.
    pub fn file_name(&self) -> String {
        format!("{}.{}", self.name, self.kind.extension())
    }

    /// Render the Quadlet unit file contents.
    pub fn render(&self) -> String {
        let mut output = String::new();

        for (section, entries) in &self.sections {
            output.push('[');
            output.push_str(section);
            output.push_str("]\n");

            for (key, value) in entries {
                output.push_str(key);
                output.push('=');
                output.push_str(value);
                output.push('\n');
            }

            output.push('\n');
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_update_label_values_match_podman() {
        assert_eq!(
            PodmanAutoUpdatePolicy::Registry.as_label_value(),
            "registry"
        );
        assert_eq!(
            PodmanAutoUpdatePolicy::Local.as_label_arg(),
            "io.containers.autoupdate=local"
        );
    }

    #[test]
    fn container_quadlet_renders_expected_file() {
        let unit = QuadletUnit::container("web", "docker.io/library/caddy:latest")
            .entry("Container", "PublishPort", "8080:80")
            .label(PodmanLabel::new("com.example.role", "web").expect("label is valid"))
            .auto_update(PodmanAutoUpdatePolicy::Registry)
            .entry("Service", "Restart", "always");

        let rendered = unit.render();

        assert_eq!(unit.file_name(), "web.container");
        assert!(rendered.contains("[Container]\n"));
        assert!(rendered.contains("Image=docker.io/library/caddy:latest\n"));
        assert!(rendered.contains("PublishPort=8080:80\n"));
        assert!(rendered.contains("Label=com.example.role=web\n"));
        assert!(rendered.contains("Label=io.containers.autoupdate=registry\n"));
        assert!(rendered.contains("[Service]\nRestart=always\n"));
    }

    #[test]
    fn kube_quadlet_uses_kube_section() {
        let unit = QuadletUnit::kube("stack", "/srv/stack.yaml");

        assert_eq!(unit.kind(), QuadletUnitKind::Kube);
        assert_eq!(unit.file_name(), "stack.kube");
        assert!(unit.render().contains("[Kube]\nYaml=/srv/stack.yaml\n"));
    }
}

#[cfg(feature = "caddy")]
use veltrix::Error;
use veltrix::Result;

fn main() -> Result<()> {
    #[cfg(feature = "podman")]
    podman_example()?;

    #[cfg(feature = "docker")]
    docker_example();

    #[cfg(feature = "caddy")]
    caddy_example()?;

    #[cfg(feature = "systemd")]
    systemd_example();

    #[cfg(feature = "systemd-dbus")]
    systemd_dbus_example();

    #[cfg(feature = "technitium")]
    technitium_example()?;

    #[cfg(not(any(
        feature = "podman",
        feature = "docker",
        feature = "caddy",
        feature = "systemd",
        feature = "technitium"
    )))]
    {
        eprintln!("Enable one or more service features to run this example:");
        eprintln!(
            "  cargo run --manifest-path workspace/Cargo.toml -p veltrix --example services_demo --features podman,caddy,systemd,technitium"
        );
    }

    Ok(())
}

#[cfg(feature = "podman")]
fn podman_example() -> Result<()> {
    use veltrix::services::podman::{
        PodmanAutoUpdatePolicy, PodmanLabel, PodmanLabels, QuadletUnit,
    };

    let labels = PodmanLabels::from_pairs([
        ("com.example.app", "veltrix-demo"),
        ("com.example.role", "web"),
    ])?
    .auto_update(PodmanAutoUpdatePolicy::Registry);

    println!("podman label args: {:?}", labels.to_cli_args());

    let quadlet = QuadletUnit::container("veltrix-demo", "docker.io/library/caddy:latest")
        .label(PodmanLabel::new("com.example.role", "web")?)
        .auto_update(PodmanAutoUpdatePolicy::Registry);

    println!("quadlet file: {}", quadlet.file_name());
    println!("{}", quadlet.render());

    Ok(())
}

#[cfg(feature = "docker")]
fn docker_example() {
    use veltrix::services::docker::{DockerCliClient, DockerCliSpec, DockerComposeSpec};

    let docker = DockerCliClient::new(DockerCliSpec::new().sudo());
    let compose = DockerComposeSpec::new()
        .compose_file("compose.yaml")
        .project_name("veltrix-demo");

    println!("docker backend: {:?}", docker.spec());
    println!("compose spec: {:?}", compose);
}

#[cfg(feature = "caddy")]
fn caddy_example() -> Result<()> {
    use veltrix::services::caddy::{CaddyAdminClient, CaddyConfig};

    let config = CaddyConfig::local_https_reverse_proxy("app.local", ["127.0.0.1:3000"])?;
    let admin = CaddyAdminClient::localhost_default();

    println!("caddy admin endpoint: {:?}", admin.spec());
    let rendered = serde_json::to_string_pretty(&config)
        .map_err(|err| Error::parsing(format!("failed to render Caddy config: {err}")))?;
    println!("caddy config:\n{}", rendered);

    Ok(())
}

#[cfg(feature = "systemd")]
fn systemd_example() {
    use veltrix::services::systemd::{SystemdCliClient, SystemdCliSpec, SystemdResourceLimit};

    let user_systemd = SystemdCliClient::new(SystemdCliSpec::new().user());
    let instance = SystemdCliClient::template_instance("worker@.service", "blue");
    let memory = SystemdResourceLimit::new("MemoryMax", "512M");

    println!("systemd spec: {:?}", user_systemd.spec());
    println!("template instance: {}", instance);
    println!("resource assignment: {}", memory.assignment());
}

#[cfg(feature = "systemd-dbus")]
fn systemd_dbus_example() {
    use veltrix::services::systemd::{SystemdDbusClient, SystemdDbusSpec};

    let dbus = SystemdDbusClient::new(SystemdDbusSpec::new().user());

    println!("systemd D-Bus spec: {:?}", dbus.spec());
}

#[cfg(feature = "technitium")]
fn technitium_example() -> Result<()> {
    use veltrix::services::technitium::{
        TechnitiumAuth, TechnitiumClient, TechnitiumHttpSpec, TechnitiumRecordType,
    };

    let spec = TechnitiumHttpSpec::new("http://localhost:5380")
        .auth(TechnitiumAuth::session_token("example-token"));
    let client = TechnitiumClient::new(spec)?;

    println!("technitium client spec: {:?}", client.spec());
    println!("record type: {}", TechnitiumRecordType::A);
    println!(
        "Caddy ACME helper: set _acme-challenge.app.local TXT before requesting a certificate"
    );

    Ok(())
}

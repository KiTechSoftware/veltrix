use veltrix::{Result, os::paths};

fn main() -> Result<()> {
    let app = "veltrix-demo";

    println!(
        "system config path: {}",
        paths::system_config_path(app, "config.toml").display()
    );

    match paths::user_config_path(app, "config.toml") {
        Ok(p) => println!("user config path: {}", p.display()),
        Err(e) => println!("user config path error: {}", e),
    }

    println!("systemd unit name: {}", paths::systemd_unit_name(app));
    println!(
        "expand '~/.local/share': {}",
        paths::expand_user_path("~/.local/share")?.display()
    );

    Ok(())
}

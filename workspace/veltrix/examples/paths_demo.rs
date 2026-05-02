use veltrix::{
    Result,
    os::paths::{self, constants},
};

fn main() -> Result<()> {
    let app = "veltrix-demo";

    println!(
        "system bin: {}",
        paths::system_bin_path("veltrix").display()
    );
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
        "user desktop entry: {}",
        paths::user_desktop_entry_path(app)?.display()
    );
    println!(
        "expand '~/.local/share': {}",
        paths::expand_user_path("~/.local/share")?.display()
    );
    println!(
        "XDG config env key: {} -> default {}",
        constants::XDG_CONFIG_DIR_ENV,
        constants::USER_CONFIG_DIR
    );

    Ok(())
}

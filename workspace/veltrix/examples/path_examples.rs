
fn main() -> veltrix::Result<()> {
    println!(
        "User bin path for 'mybin': {}",
        veltrix::paths::user_bin_path("mybin")?.display()
    );
    Ok(())
}

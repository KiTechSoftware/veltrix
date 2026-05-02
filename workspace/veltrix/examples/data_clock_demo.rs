use veltrix::{Result, os::clock};

fn main() -> Result<()> {
    let start = clock::monotonic();

    println!("wall clock: {:?}", clock::now());
    println!("unix timestamp: {}s", clock::unix_timestamp()?.as_secs());
    println!("uptime: {:?}", clock::uptime()?);
    println!("process cpu: {:?}", clock::process_cpu_time()?);
    println!("thread cpu: {:?}", clock::thread_cpu_time()?);
    println!("example elapsed: {:?}", clock::elapsed_since(start));

    #[cfg(feature = "data-bools")]
    {
        use veltrix::data::bools::{self, BoolParseMode};

        let enabled = bools::parse_bool("yes", BoolParseMode::Permissive)?;
        println!("parse yes -> {}", bools::enabled_disabled(enabled));
    }

    #[cfg(feature = "data-time")]
    {
        use veltrix::data::time;

        let timeout = time::parse_duration("1h15m30s")?;
        println!(
            "parse 1h15m30s -> {} seconds -> {}",
            time::seconds(timeout),
            time::format_duration(timeout)
        );
    }

    #[cfg(not(any(feature = "data-bools", feature = "data-time")))]
    {
        println!("Enable `data-bools` and/or `data-time` to see value parsing helpers.");
    }

    Ok(())
}

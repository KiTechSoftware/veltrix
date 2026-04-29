#[cfg(feature = "emojis")]
fn main() {
    println!("Grinning: {}", veltrix::emojis::EMOJI_GRINNING_FACE);
    println!("Wink: {}", veltrix::emojis::EMOJI_WINKING_FACE);
    println!("Thumbs up: {}", veltrix::emojis::EMOJI_THUMBS_UP);
}

#[cfg(not(feature = "emojis"))]
fn main() {
    eprintln!("Enable the `emojis` feature to run this example:");
    eprintln!("  cargo run --example emojis_demo --features emojis");
}

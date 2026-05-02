fn main() {
    #[cfg(not(feature = "unicode-emojis"))]
    {
        eprintln!("This example needs emoji data, which is behind a feature flag.");
        eprintln!();
        eprintln!("Run it with:");
        eprintln!(
            "  cargo run --manifest-path workspace/Cargo.toml -p veltrix --example emojis_demo --features unicode-emojis"
        );
        eprintln!();
    }
    #[cfg(feature = "unicode-emojis")]
    {
        use veltrix::unicode::emojis::{
            EMOJI_GRINNING_FACE, EMOJI_THUMBS_UP, EMOJI_WINKING_FACE, UNICODE_EMOJI_VERSION,
            find_by_search_term,
        };

        println!("Unicode Emoji data: {}", UNICODE_EMOJI_VERSION);
        println!("Grinning: {}", EMOJI_GRINNING_FACE);
        println!("Wink: {}", EMOJI_WINKING_FACE);
        println!("Thumbs up: {}", EMOJI_THUMBS_UP);

        if let Some(match_) = find_by_search_term("rocket") {
            println!(
                "search rocket -> {} {} ({})",
                match_.emoji, match_.name, match_.unicode_version
            );
        }
    }
}

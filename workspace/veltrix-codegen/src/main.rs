use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod emoji;

#[derive(Debug, Parser)]
#[command(name = "veltrix-codegen")]
#[command(about = "General-purpose code generator for Veltrix generated data")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Generate emoji constants and metadata from Unicode emoji data.
    Emojis {
        /// Unicode emoji version, e.g. 17.0 or latest.
        #[arg(default_value = "latest")]
        version: String,

        /// Input Unicode emoji file. (emoji-test.txt or emoji-test-\[version\].txt)
        ///
        /// Defaults to:
        /// - data/unicode-emoji.txt for latest
        /// - data/unicode-emoji-\[version\].txt otherwise
        #[arg(long)]
        input: Option<PathBuf>,

        /// CLDR annotations XML file (unicode-cldr-en.xml or unicode-cldr-en-\[version\].xml)
        ///
        /// Defaults to:
        /// - data/unicode-cldr-en.xml for latest
        /// - data/unicode-cldr-en-\[version\].xml otherwise
        #[arg(long)]
        cldr: Option<PathBuf>,

        /// Output constants.rs path.
        #[arg(
            long,
            default_value = "workspace/veltrix/src/unicode/emojis/constants.rs"
        )]
        constants: PathBuf,

        /// Output details.rs path.
        #[arg(
            long,
            default_value = "workspace/veltrix/src/unicode/emojis/details.rs"
        )]
        details: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Emojis {
            version,
            input,
            cldr,
            constants,
            details,
        } => emoji::generate_emojis(&version, input, cldr, &constants, &details)?,
    }

    Ok(())
}

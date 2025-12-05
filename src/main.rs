mod assets;
mod audio;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mcsound")]
#[command(about = "Play Minecraft sounds from the command line")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available sounds
    List {
        /// Optional pattern to filter sounds
        pattern: Option<String>,
    },
    /// Play a sound by name
    Play {
        /// Sound path (e.g., mob/zombie/death)
        sound: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mc = assets::MinecraftAssets::load()?;

    match cli.command {
        Commands::List { pattern } => {
            let sounds = mc.list_sounds(pattern.as_deref());
            for sound in sounds {
                println!("{}", sound);
            }
        }
        Commands::Play { sound } => {
            let path = mc.resolve_sound(&sound)?;
            audio::play_ogg(&path)?;
        }
    }

    Ok(())
}

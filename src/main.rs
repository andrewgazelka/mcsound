mod assets;
mod audio;

use std::env;
use std::process::{Command, Stdio};

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

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
    Play(PlayArgs),
}

#[derive(Args)]
struct PlayArgs {
    /// Sound path (e.g., mob/zombie/death). If empty, does nothing.
    sound: Option<String>,

    /// Wait for playback to finish (default: plays in background)
    #[arg(short, long)]
    wait: bool,

    /// Internal flag: run in foreground (used by background spawn)
    #[arg(long, hide = true)]
    foreground: bool,
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
        Commands::Play(args) => {
            // Skip if no sound specified (disabled by default)
            let Some(sound) = args.sound.filter(|s| !s.is_empty()) else {
                return Ok(());
            };

            // If --wait or --foreground, play synchronously
            if args.wait || args.foreground {
                let path = mc.resolve_sound(&sound)?;
                audio::play_ogg(&path)?;
            } else {
                // Spawn self as background process and exit immediately
                let exe = env::current_exe()?;
                Command::new(exe)
                    .args(["play", "--foreground", &sound])
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()?;
            }
        }
    }

    Ok(())
}

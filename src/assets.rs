use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct AssetIndex {
    objects: HashMap<String, AssetObject>,
}

#[derive(Debug, Deserialize)]
struct AssetObject {
    hash: String,
    #[allow(dead_code)]
    size: u64,
}

pub struct MinecraftAssets {
    minecraft_dir: PathBuf,
    objects: HashMap<String, String>, // sound path -> hash
}

impl MinecraftAssets {
    pub fn load() -> Result<Self> {
        let minecraft_dir = find_minecraft_dir()?;
        let index_path = find_latest_index(&minecraft_dir)?;

        let content = fs::read_to_string(&index_path)
            .with_context(|| format!("Failed to read index file: {}", index_path.display()))?;

        let index: AssetIndex =
            serde_json::from_str(&content).context("Failed to parse asset index JSON")?;

        // Filter to only sound files and map to hash
        let objects: HashMap<String, String> = index
            .objects
            .into_iter()
            .filter(|(k, _)| k.starts_with("minecraft/sounds/") && k.ends_with(".ogg"))
            .map(|(k, v)| (k, v.hash))
            .collect();

        Ok(Self {
            minecraft_dir,
            objects,
        })
    }

    /// List all available sounds, optionally filtered by pattern
    pub fn list_sounds(&self, pattern: Option<&str>) -> Vec<&str> {
        let mut sounds: Vec<&str> = self
            .objects
            .keys()
            .filter_map(|k| {
                let name = k.strip_prefix("minecraft/sounds/")?.strip_suffix(".ogg")?;
                if let Some(p) = pattern {
                    if name.contains(p) {
                        Some(name)
                    } else {
                        None
                    }
                } else {
                    Some(name)
                }
            })
            .collect();
        sounds.sort();
        sounds
    }

    /// Resolve a sound name to its file path
    pub fn resolve_sound(&self, name: &str) -> Result<PathBuf> {
        let key = format!("minecraft/sounds/{}.ogg", name);
        let hash = self
            .objects
            .get(&key)
            .with_context(|| format!("Sound not found: {}", name))?;

        let path = self
            .minecraft_dir
            .join("assets")
            .join("objects")
            .join(&hash[..2])
            .join(hash);

        if !path.exists() {
            bail!("Sound file missing from disk: {}", path.display());
        }

        Ok(path)
    }
}

fn find_minecraft_dir() -> Result<PathBuf> {
    // Check env var first
    if let Ok(path) = env::var("MINECRAFT_HOME") {
        let path = PathBuf::from(path);
        if path.exists() {
            return Ok(path);
        }
    }

    // Auto-detect by OS
    let path = if cfg!(target_os = "macos") {
        dirs::home_dir().map(|h| h.join("Library/Application Support/minecraft"))
    } else if cfg!(target_os = "windows") {
        dirs::data_dir().map(|d| d.join(".minecraft"))
    } else {
        // Linux and others
        dirs::home_dir().map(|h| h.join(".minecraft"))
    };

    let path = path.context("Could not determine home directory")?;

    if path.exists() {
        Ok(path)
    } else {
        bail!(
            "Minecraft directory not found at {}. Set MINECRAFT_HOME env var or install Minecraft.",
            path.display()
        )
    }
}

fn find_latest_index(minecraft_dir: &PathBuf) -> Result<PathBuf> {
    let indexes_dir = minecraft_dir.join("assets").join("indexes");

    if !indexes_dir.exists() {
        bail!("No asset indexes found. Have you launched Minecraft at least once?");
    }

    // Find all .json files and pick the one with highest version number
    let mut indexes: Vec<_> = fs::read_dir(&indexes_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
        .collect();

    if indexes.is_empty() {
        bail!("No asset index files found in {}", indexes_dir.display());
    }

    // Sort by filename (version number) descending
    indexes.sort_by(|a, b| {
        let a_name = a.file_name();
        let b_name = b.file_name();
        // Parse as numbers if possible for proper sorting (26 > 8)
        let a_num: Option<u32> = a_name
            .to_str()
            .and_then(|s| s.strip_suffix(".json"))
            .and_then(|s| s.parse().ok());
        let b_num: Option<u32> = b_name
            .to_str()
            .and_then(|s| s.strip_suffix(".json"))
            .and_then(|s| s.parse().ok());

        match (a_num, b_num) {
            (Some(a), Some(b)) => b.cmp(&a),
            _ => b_name.cmp(&a_name),
        }
    });

    Ok(indexes[0].path())
}

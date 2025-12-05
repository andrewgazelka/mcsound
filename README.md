<p align="center">
  <img src=".github/assets/header.svg" alt="mcsound" width="100%"/>
</p>

<p align="center">
  <code>nix run github:andrewgazelka/mcsound -- play mob/zombie/death</code>
</p>

Play Minecraft sounds directly from your terminal. Reads from your local Minecraft installation.

## Usage

```bash
# List all sounds
mcsound list

# Filter sounds
mcsound list zombie

# Play a sound
mcsound play mob/zombie/death
```

## How It Works

Reads the Minecraft asset index to map sound names to hashed files in `.minecraft/assets/objects/`. Audio playback is bundled (no ffmpeg required).

---

<details>
<summary>Installation and configuration</summary>

**With Nix (recommended):**
```bash
nix run github:andrewgazelka/mcsound -- list zombie
```

**With Cargo:**
```bash
cargo install --git https://github.com/andrewgazelka/mcsound
```

**Minecraft path:** Auto-detects standard locations. Override with `MINECRAFT_HOME` env var.

**Requirement:** Minecraft Java Edition must have been launched at least once to download assets.

</details>

<details>
<summary>Claude Code Plugin</summary>

This repo is also a Claude Code plugin that adds Minecraft sound effects to Claude Code events.

**Install the plugin:**
```bash
claude plugins add github:andrewgazelka/mcsound
```

**Sound mappings:**
| Event | Sound |
|-------|-------|
| Session start | `random/door_open` |
| Session end | `random/door_close` |
| Task complete | `random/levelup` |
| Subagent complete | `random/orb` |
| Notification | `note/bell` |
| File write/edit | `block/amethyst/place1` |
| Bash command | `random/click` |
| Task spawn | `block/end_portal/endportal` |
| Web search/fetch | `block/end_portal/eyeplace1` |

</details>

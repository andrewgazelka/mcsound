<p align="center">
  <img src=".github/assets/header.svg" alt="mcsound" width="100%"/>
</p>

<p align="center">
  <strong>Add Minecraft sound effects to Claude Code</strong>
</p>

<p align="center">

```bash
claude plugin marketplace add andrewgazelka/mcsound
claude plugin install mcsound
```

</p>

---

A Claude Code plugin that plays Minecraft sounds for coding events — hear a door open when you start a session, level up when tasks complete, and amethyst chimes when files are saved.

## Sound Mappings

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

## Configuration

Override any sound via environment variables:

| Variable | Default |
|----------|---------|
| `MCSOUND_SESSION_START` | `random/door_open` |
| `MCSOUND_SESSION_END` | `random/door_close` |
| `MCSOUND_STOP` | `random/levelup` |
| `MCSOUND_SUBAGENT_STOP` | `random/orb` |
| `MCSOUND_NOTIFICATION` | `note/bell` |
| `MCSOUND_WRITE` | `block/amethyst/place1` |
| `MCSOUND_BASH` | `random/click` |
| `MCSOUND_TASK` | `block/end_portal/endportal` |
| `MCSOUND_WEB` | `block/end_portal/eyeplace1` |

```bash
# Example: change task complete sound
export MCSOUND_STOP="mob/villager/celebrate"
```

## Requirements

- Minecraft Java Edition must have been launched at least once (to download sound assets)
- Either [Nix](https://nixos.org/) or Rust/Cargo for building locally

## Installation

### With Nix
The plugin uses `nix run` for zero-install execution. No additional setup needed.

### Without Nix (Cargo)
If you don't have Nix installed, build the binary locally:

```bash
cd ~/.claude/plugins/marketplaces/mcsound
cargo build --release
```

The plugin will automatically use the built binary at `target/release/mcsound`.

### Prism Launcher / Alternative Minecraft Locations
If you use Prism Launcher or have Minecraft installed in a non-standard location, set the `MINECRAFT_HOME` environment variable:

```bash
export MINECRAFT_HOME="$HOME/.local/share/PrismLauncher"
```

---

<details>
<summary>Standalone CLI usage</summary>

Can also be used as a standalone CLI tool:

```bash
# Run directly with Nix
nix run github:andrewgazelka/mcsound -- play mob/zombie/death

# List all sounds
nix run github:andrewgazelka/mcsound -- list

# Filter sounds
nix run github:andrewgazelka/mcsound -- list zombie
```

**Install with Cargo:**
```bash
cargo install --git https://github.com/andrewgazelka/mcsound

mcsound play mob/zombie/death
mcsound list zombie
```

**Minecraft path:** Auto-detects standard locations. Override with `MINECRAFT_HOME` env var.

</details>

<details>
<summary>How it works</summary>

Reads the Minecraft asset index to map sound names to hashed files in `.minecraft/assets/objects/`. Audio playback is bundled (no ffmpeg required).

</details>

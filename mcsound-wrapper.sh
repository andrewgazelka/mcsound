#!/bin/sh

if command -v nix >/dev/null 2>&1; then
    exec nix run github:andrewgazelka/mcsound -- "$@"
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
LOCAL_BIN="$SCRIPT_DIR/target/release/mcsound"

if [ -x "$LOCAL_BIN" ]; then
    exec "$LOCAL_BIN" "$@"
fi

echo "Error: mcsound not found. Please either:" >&2
echo "  1. Install Nix, or" >&2
echo "  2. Build locally with: cd $SCRIPT_DIR && cargo build --release" >&2
exit 1

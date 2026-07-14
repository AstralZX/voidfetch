#!/bin/sh

# voidfetch installer
set -e

INSTALL_DIR="$HOME/.local/bin"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "[*] Building voidfetch..."
if command -v cargo >/dev/null 2>&1; then
    cd "$SCRIPT_DIR"
    cargo build --release 2>/dev/null
    BIN="$SCRIPT_DIR/target/release/voidfetch"
else
    echo "[-] cargo not found. Install Rust: https://rustup.rs"
    exit 1
fi

if [ ! -f "$BIN" ]; then
    echo "[-] Build failed"
    exit 1
fi

echo "[*] Installing to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"
cp "$BIN" "$INSTALL_DIR/voidfetch"

mkdir -p "$INSTALL_DIR/logos"
cp "$SCRIPT_DIR/logos/"*.txt "$INSTALL_DIR/logos/"

chmod +x "$INSTALL_DIR/voidfetch"

echo "[*] Checking PATH..."
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "[*] Adding $INSTALL_DIR to PATH..."
    for RC in ~/.bashrc ~/.zshrc; do
        if [ -f "$RC" ]; then
            if ! grep -q "$INSTALL_DIR" "$RC"; then
                echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$RC"
            fi
        fi
    done
    echo "[+] Added to shell configs. Restart your shell."
fi

mkdir -p "$HOME/.config/voidfetch"
if [ ! -f "$HOME/.config/voidfetch/config.css" ]; then
    "$INSTALL_DIR/voidfetch" --dump-config > "$HOME/.config/voidfetch/config.css"
    echo "[+] Created config at ~/.config/voidfetch/config.css"
fi

echo "[+] Installed successfully! Run 'voidfetch' to use."

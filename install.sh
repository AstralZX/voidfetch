# voidfetch installer

set -e

INSTALL_DIR="$HOME/.local/bin"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "[*] Creating install directory..."
mkdir -p "$INSTALL_DIR"

echo "[*] Installing voidfetch..."
cp "$SCRIPT_DIR/voidfetch" "$INSTALL_DIR/voidfetch"
cp "$SCRIPT_DIR/voidfetch.py" "$INSTALL_DIR/voidfetch.py"

mkdir -p "$INSTALL_DIR/logos"
cp "$SCRIPT_DIR/logos/"*.txt "$INSTALL_DIR/logos/"

chmod +x "$INSTALL_DIR/voidfetch"

echo "[*] Checking PATH..."
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "[!] $INSTALL_DIR not in PATH"
    echo "[*] Adding to ~/.bashrc and ~/.zshrc..."

    for RC in ~/.bashrc ~/.zshrc; do
        if [ -f "$RC" ]; then
            if ! grep -q "$INSTALL_DIR" "$RC"; then
                echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$RC"
            fi
        fi
    done

    echo "[+] Added PATH to shell configs. Restart your shell or run:"
    echo "    source ~/.bashrc"
fi

echo "[+] Installed successfully! Run 'voidfetch' to use."

#!/bin/bash
set -e

echo "Building fast-menu..."
cargo build --release

echo "Installing binary..."
mkdir -p ~/.local/bin
cp target/release/fast-menu ~/.local/bin/

echo "Installing desktop file..."
mkdir -p ~/.local/share/applications
cp data/fast-menu.desktop ~/.local/share/applications/

echo "Creating config directory..."
mkdir -p ~/.config/fast-menu/themes
cp data/config.default.toml ~/.config/fast-menu/config.toml 2>/dev/null || true
cp data/themes/*.css ~/.config/fast-menu/themes/ 2>/dev/null || true

echo "Installing GNOME Shell extension for window switcher..."
mkdir -p ~/.local/share/gnome-shell/extensions
cp -r data/gnome-extension/* ~/.local/share/gnome-shell/extensions/ 2>/dev/null || true

# Try to enable the extension (may need GNOME Shell restart)
if command -v gnome-extensions &> /dev/null; then
    gnome-extensions enable window-calls@domandoman.xyz 2>/dev/null || true
fi

echo ""
echo "Installation complete!"
echo ""
echo "To use fast-menu with Ctrl+Space:"
echo "1. Open GNOME Settings > Keyboard > Keyboard Shortcuts > Custom Shortcuts"
echo "2. Add a new shortcut:"
echo "   - Name: Fast Menu"
echo "   - Command: fast-menu toggle"
echo "   - Shortcut: Ctrl+Space"
echo ""
echo "Or run manually: fast-menu"
echo ""
echo "Note: The window switcher requires the window-calls GNOME extension."
echo "If windows don't appear, you may need to restart GNOME Shell (Alt+F2, type 'r', Enter)"
echo "or log out and log back in."

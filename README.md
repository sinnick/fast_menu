# Fast Menu

A terminal-style window switcher for GNOME/Wayland built with GTK4 and Rust.

## Features

- **Window Switching**: Quickly switch between open windows using fuzzy search
- **Terminal UI**: Minimal design with only 3 colors (black, white, cyan)
- **Keyboard Driven**: Navigate with arrow keys, select with Enter, cancel with Escape
- **Fast**: Written in Rust with GTK4 for native performance
- **Single Instance**: Uses D-Bus to ensure only one instance runs

## Requirements

- **OS**: Ubuntu 22.04+ or other Linux distribution with GNOME on Wayland
- **GNOME Shell**: Version 45 or later
- **Rust**: 1.70 or later
- **GTK4**: Development libraries

### Dependencies (Ubuntu/Debian)

```bash
sudo apt install libgtk-4-dev build-essential
```

### Dependencies (Fedora)

```bash
sudo dnf install gtk4-devel gcc
```

### Dependencies (Arch)

```bash
sudo pacman -S gtk4 base-devel
```

## Installation

1. Clone the repository:
```bash
git clone https://github.com/user/fast-menu.git
cd fast-menu
```

2. Run the install script:
```bash
./install.sh
```

This will:
- Build the project in release mode
- Install the binary to `~/.local/bin/`
- Install the desktop file
- Install the GNOME Shell extension for window listing
- Create the config directory at `~/.config/fast-menu/`

3. Restart GNOME Shell (or log out and back in) to enable the extension:
   - Press `Alt+F2`, type `r`, press Enter (X11 only)
   - Or log out and log back in (Wayland)

4. Enable the extension if not auto-enabled:
```bash
gnome-extensions enable window-calls@domandoman.xyz
```

## Usage

### Running

```bash
fast-menu          # Show the window
fast-menu toggle   # Toggle visibility
fast-menu show     # Show the window
fast-menu hide     # Hide the window
```

### Keyboard Shortcut

Set up a global keyboard shortcut in GNOME Settings:

1. Open **Settings** > **Keyboard** > **Keyboard Shortcuts** > **Custom Shortcuts**
2. Add a new shortcut:
   - **Name**: Fast Menu
   - **Command**: `fast-menu toggle`
   - **Shortcut**: `Super+Space` (or your preference)

### Controls

| Key | Action |
|-----|--------|
| `↑` `↓` | Navigate results |
| `Enter` | Switch to selected window |
| `Escape` | Close menu |
| Type | Filter windows by name |

## Configuration

Configuration file is located at `~/.config/fast-menu/config.toml`.

### Theme

Custom CSS themes can be placed in `~/.config/fast-menu/themes/`. The default theme is `dark.css`.

To modify colors, edit `~/.config/fast-menu/themes/dark.css`:

```css
/* Main colors */
window { background-color: #0c0c0c; border: 1px solid #1a8ac1; }
.result-left { color: #ffffff; }
.result-right { color: #1a8ac1; }
row:selected { background-color: #1a8ac1; }
row:selected label { color: #0c0c0c; }
```

## Troubleshooting

### Windows not appearing

1. Make sure the GNOME Shell extension is enabled:
```bash
gnome-extensions list --enabled | grep window-calls
```

2. Restart GNOME Shell or log out/in

3. Check if the extension is working:
```bash
gdbus call --session --dest org.gnome.Shell \
  --object-path /org/gnome/Shell/Extensions/Windows \
  --method org.gnome.Shell.Extensions.Windows.List
```

### Styles not loading

Make sure the theme file exists at `~/.config/fast-menu/themes/dark.css`

## Building from Source

```bash
cargo build --release
```

The binary will be at `target/release/fast-menu`.

## License

MIT

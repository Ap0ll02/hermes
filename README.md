# ⚡ Hermes Package Manager

![CI](https://github.com/yourusername/hermes/workflows/CI/badge.svg)
![Release](https://github.com/yourusername/hermes/workflows/Release/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

A beautiful TUI package manager for Arch Linux, written in Rust.

## Features

- 🔍 Real-time package search
- 📦 Install/remove packages with confirmation
- ✨ Beautiful terminal UI with Catppuccin theme
- ⚡ Fast and efficient using libalpm
- 🎯 Vim-style keybindings

!["Main Screen of the TUI"](./screenshots/main_screen.png) 
!["Help PopUp"](./screenshots/help.png) 

## Installation
```bash
# Download latest release
wget https://github.com/yourusername/hermes/releases/latest/download/hermes-x86_64-linux.tar.gz

# Extract and install
tar -xzf hermes-x86_64-linux.tar.gz
sudo mv hermes /usr/local/bin/

# Run
sudo hermes
```

## Development
```bash
# Clone repo
git clone https://github.com/yourusername/hermes.git
cd hermes

# Run tests
cargo test

# Build
cargo build --release

# Run locally
sudo ./target/release/hermes
```

## Keybindings

- `/` - Search mode
- `↑/k` - Move up
- `↓/j` - Move down
- `i` - Install package
- `r` - Remove package
- `?` - Help
- `q` - Quit

## License

TBD

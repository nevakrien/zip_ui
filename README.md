# Zip GUI Tool

A simple, cross-platform GUI application for zipping and unzipping files with native file dialogs. Built with Rust and egui.

## Features

- **Native file dialogs** - Uses the system file picker on Windows, macOS, and Linux
- **Cross-platform** - Works on Windows, macOS, and Linux
- **Simple interface** - Clean, intuitive GUI with two modes: Zip and Unzip
- **No installation required** - Single executable file

## Download

You can download pre-built executables from the [Releases](https://github.com/yourusername/zip_gui/releases) page.

- **Windows**: `zip-gui-windows-x64.zip`
- **macOS**: `zip-gui-macos-x64.tar.gz` (Intel) or `zip-gui-macos-arm64.tar.gz` (Apple Silicon)
- **Linux**: `zip-gui-linux-x64.tar.gz`

## Building from Source

If you want to build it yourself:

1. Install Rust (https://rustup.rs/)
2. Clone this repository
3. Run `cargo build --release`
4. The executable will be in `target/release/`

## Usage

### Zip Mode
1. Select "Zip Files" mode
2. Click "Select Files to Zip" and choose one or more files/folders
3. Click "Zip Selected Files" and choose where to save the zip file

### Unzip Mode
1. Select "Unzip Files" mode  
2. Click "Select Zip File" and choose a zip file
3. Click "Choose Extract Location" and select where to extract the files

## Requirements

- **Windows**: Windows 10 or later
- **macOS**: macOS 10.15 or later
- **Linux**: X11 or Wayland display server

## License

MIT License
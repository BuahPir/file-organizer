# File Organizer

A command-line utility to automatically organize files in a directory by their file types into categorized subdirectories.

## Features

- **Automatic Organization**: Sorts files into directories based on their extensions
- **Predefined Categories**: 
  - Images: `.jpg`, `.png`
  - Videos: `.mp4`, `.mkv`
  - Audio: `.mp3`, `.wav`
  - Documents: `.pdf`, `.docx`, `.xlsx`, `.pptx`, `.csv`, `.doc`
  - Compressed: `.zip`, `.rar`, `.7z`
  - Programs: `.exe`, `.msi`
  - Other: Unknown extensions get their own directories
- **Error Handling**: Comprehensive error messages using Anyhow
- **Safe Operations**: Skips files already in correct directories

## Installation

### Prerequisites
- Rust 1.70 or later
- Cargo

### Build
```bash
cargo build --release
```

## USAGE
### Basic Usage
```bash
cargo run -- <DIRECTORY_PATH>
```
#### Examples
- Organize current directory:
```bash
cargo run -- .
```
- Organize a specific folder:
```bash
cargo run -- ./Downloads
cargo run -- ./Documents
```
- Organize with absolute path:
```bash
cargo run -- "C:\Users\YourName\Downloads"
cargo run -- "G:\my-files"
```

## How It Works
1. Scans the specified directory for files
2. Reads each file's extension
3. Categorizes files based on predefined rules
4. Creates category subdirectories if they don't exist
5. Moves files to their respective category directories
6. Displays a summary of organized files

## Output Example
```
Moved: photo.jpg -> Images
Moved: video.mp4 -> Videos
Moved: document.pdf -> Documents
Skipped (already organized): image.png

Processed 4 file(s)
```

## Project Structure
```
file-organizer/
├── src/
│   └── main.rs          # Main application logic
├── Cargo.toml           # Project dependencies
└── README.md            # This file
```

## Dependencies
- clap: Command-line argument parsing
- anyhow: Error handling and context

## License
MIT

## Acknowledgments

This project was built with assistance from **GitHub Copilot** (powered by Claude Haiku 4.5), an AI programming assistant that helped with:
- Code structure and implementation
- Error handling patterns
- CLI design using Clap

---

**Built with AI assistance from GitHub Copilot**

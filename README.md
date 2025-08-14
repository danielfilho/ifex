# IFEX - EXIF Data Manager

[![CI](https://github.com/danielfilho/ifex/actions/workflows/ci.yml/badge.svg)](https://github.com/danielfilho/ifex/actions/workflows/ci.yml)
[![Release](https://github.com/danielfilho/ifex/actions/workflows/release.yml/badge.svg)](https://github.com/danielfilho/ifex/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A modern CLI tool for managing EXIF data in JPEG, TIFF, DNG, and RAW image files with structured equipment management. Built with Rust for performance and reliability.

> **📢 Version 1.103.0**: Latest stable release with enhanced features and improvements. Install via Homebrew or download from [releases](https://github.com/danielfilho/ifex/releases).

## Features

- 📷 **Equipment Management**: Separate CRUD operations for:
  - Cameras (maker, model)
  - Lenses (maker, model with aperture)
  - Films (maker, name, ISO)
  - Photographers (name, email)
  - Setups (camera + lens combinations)
- 📸 **EXIF Application**: Apply metadata to image files using:
  - Setup selection (pre-configured camera + lens)
  - Film selection (with ISO override option)
  - Photographer selection (from saved photographers)
- 🗑️ **EXIF Removal**: Erase all EXIF data from image files
- 🎯 **Multi-Format Support**: Works with:
  - JPEG/JPG files (direct EXIF modification)
  - TIFF files (direct EXIF modification)
  - DNG files (direct modification or XMP sidecar fallback)
  - RAW files (XMP sidecar files): CR2, CR3, NEF, NRW, ARW, SRF, ORF, RW2, RAF, SRW, PEF, X3F, and more
- 📁 **Recursive Processing**: Process subdirectories with user confirmation (defaults to yes)
- 🔍 **Fuzzy Search**: Type to filter in all selection menus
- 📝 **Smart Path Handling**: Supports paths with spaces, quoted paths, and escaped spaces
- 💾 **Local Storage**: Equipment data stored in `~/.config/ifex.json`
- 🔄 **Interactive Interface**: Easy-to-use menu system with enhanced UX
- ✅ **Batch Processing**: Process entire folders at once with detailed reporting
- 🛡️ **Robust Error Handling**: Graceful fallbacks and clear error messages
- 🔍 **EXIF Inspection**: View EXIF data from image files in formatted tables
- ⚡ **Performance**: Built with Rust for fast, reliable processing

## Installation

### From GitHub Releases (Recommended)

Download the latest binary for your platform from [GitHub Releases](https://github.com/danielfilho/ifex/releases):

- **Linux x86_64**: `ifex-linux-x86_64`
- **macOS Intel**: `ifex-macos-x86_64` 
- **macOS Apple Silicon**: `ifex-macos-aarch64`
- **Windows**: `ifex-windows-x86_64.exe`

Make the binary executable and move it to your PATH:

```bash
# Linux/macOS
chmod +x ifex-*
sudo mv ifex-* /usr/local/bin/ifex

# Or add to your PATH
export PATH="$PATH:/path/to/ifex"
```

### Homebrew (macOS/Linux)

```bash
# Install directly from the tap
brew install danielfilho/ifex/ifex

# Or tap first, then install
brew tap danielfilho/ifex
brew install ifex

# Upgrade to the latest version
brew upgrade danielfilho/ifex/ifex
```

### From Source

Requires [Rust](https://rustup.rs/) 1.70 or later:

```bash
git clone https://github.com/danielfilho/ifex.git
cd ifex
cargo install --path .
```

### Development Setup

```bash
git clone https://github.com/danielfilho/ifex.git
cd ifex
cargo build --release
```

## Usage

### Interactive Mode (Default)

```bash
ifex
```

### Equipment Management Only

```bash
ifex manage
```

### EXIF Inspection

```bash
ifex read /path/to/image.jpg
```

### Available Commands

- `ifex` - Run interactive mode with all options
- `ifex manage` - Equipment management only
- `ifex run` - Explicit interactive mode (same as default)
- `ifex read <file>` - Read and display EXIF data from an image file

## Equipment Management

The tool stores equipment data in a JSON file with separate collections for:

### Cameras
- Maker (e.g., "Canon", "Nikon")
- Model (e.g., "AE-1", "FM2")

### Lenses
- Maker (e.g., "Canon", "Nikon")
- Model (e.g., "FD", "AI")
- Focal Length (e.g., "50", "28-135") - in mm
- Aperture (e.g., "1.4", "2.8") - maximum aperture
- Mount (e.g., "FD", "F", "K")

### Films
- Maker (e.g., "Kodak", "Fuji")
- Name (e.g., "Portra 400", "Velvia 50")
- ISO rating

### Photographers
- Name (e.g., "John Doe", "Jane Smith")
- Email (optional, e.g., "john@photography.com")

### Setups
- Name (e.g., "Street Photography", "Studio Portraits")
- Camera reference
- Lens reference (optional - supports camera-only setups)

## Workflow

### EXIF Application

1. **Manage Equipment**: Add cameras, lenses, films, and photographers
2. **Create Setups**: Combine cameras with optional lenses (supports camera-only setups)
3. **Apply EXIF**: Select setup + film + photographer for batch processing

### EXIF Inspection

Use the read command to view EXIF data from any supported image file:

```bash
ifex read /path/to/image.jpg
```

This displays a formatted table with all EXIF fields including camera, lens, and film information.

## EXIF Fields Mapped

| Equipment Field | EXIF Tag |
|-----------------|----------|
| Camera Maker | Make |
| Camera Model | Model |
| Lens Maker | LensMake |
| Lens Model | LensModel |
| Focal Length | FocalLength |
| Aperture | FNumber |
| Film ISO | ISOSpeedRatings |
| Shot ISO | ISOSpeed |
| Photographer | Artist |

## Supported File Formats

### Direct EXIF Modification

- **JPEG/JPG**: Full EXIF read/write support
- **TIFF/TIF**: Full EXIF read/write support
- **DNG**: Adobe Digital Negative files (with fallback to XMP sidecar)

### XMP Sidecar Files

When direct EXIF modification isn't possible, IFEX creates `.xmp` sidecar files containing the metadata:

- **Canon RAW**: CR2, CR3
- **Nikon RAW**: NEF, NRW
- **Sony RAW**: ARW, SRF, SR2
- **Olympus RAW**: ORF
- **Panasonic RAW**: RW2
- **Fujifilm RAW**: RAF
- **Samsung RAW**: SRW
- **Pentax RAW**: PEF
- **Sigma RAW**: X3F
- **And many more**: ERF, MEF, MRW, DCR, KDC, 3FR, FFF, IIQ, K25, RWL

## Path Handling

IFEX intelligently handles various path formats:

- Simple paths: `/Users/user/photos`
- Paths with spaces: `/Users/user/My Photos`
- Quoted paths: `"/Users/user/My Photos"`
- Escaped paths: `/Users/user/My\ Photos`

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Linting

```bash
cargo clippy
cargo fmt --check
```

### Running Development Version

```bash
cargo run
cargo run -- manage
cargo run -- read /path/to/image.jpg
```

## Architecture

IFEX uses a modular Rust architecture with separated concerns:

- **CLI Module** (`src/cli.rs`) - Command-line argument parsing
- **Interface Module** (`src/interface.rs`) - Interactive menu system
- **Data Manager** (`src/data.rs`) - Equipment CRUD operations
- **Models** (`src/models.rs`) - Data structures with serialization
- **EXIF Manager** (`src/exif/exif_manager.rs`) - Main orchestrator
- **File Type Detection** (`src/exif/file_types.rs`) - Format identification
- **EXIF Tags** (`src/exif/tags.rs`) - Tag definitions and utilities
- **Format Processors** (`src/exif/processors.rs`) - Specialized handlers
- **Prompts** (`src/prompts.rs`) - Interactive user input utilities
- **Utils** (`src/utils.rs`) - Path handling and file utilities

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Run linting: `cargo clippy && cargo fmt`
6. Submit a pull request

## Requirements

- Rust 1.70.0 or higher (for building from source)
- Supported image file formats (see above)

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and changes.
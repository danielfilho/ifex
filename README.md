# IFEX - EXIF Data Manager

A modern CLI tool for managing EXIF data in JPEG, TIFF, DNG, and RAW image files with structured equipment management.

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

## Installation

### Global Installation (Recommended)

```bash
npm install -g ifex-cli
```

After installation, you can use `ifex` command from anywhere:

```bash
ifex
```

### Local Development

1. Clone this repository
2. Install dependencies:

   ```bash
   npm install
   ```

## Usage

### Interactive Mode (Default)

**Global Installation:**

```bash
ifex
```

**Local Development:**

```bash
npm start
# or
node src/index.js
```

### Equipment Management Only

**Global Installation:**

```bash
ifex manage
```

**Local Development:**

```bash
npm start manage
```

### Available Commands

**Global Installation:**

- `ifex` - Run interactive mode with all options
- `ifex manage` - Equipment management only
- `ifex run` - Explicit interactive mode
- `ifex check <file>` - Display EXIF data from an image file

**Local Development:**

- `npm start` - Run interactive mode with all options
- `npm start manage` - Equipment management only
- `npm start run` - Explicit interactive mode
- `npm start check <file>` - Display EXIF data from an image file

## Equipment Management

The tool stores equipment data in `~/.config/ifex.json` with separate collections for:

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
- Email (optional, e.g., "<john@photography.com>")

### Setups

- Name (e.g., "Street Photography", "Studio Portraits")
- Camera reference
- Lens reference

## Workflow

### EXIF Application

1. **Manage Equipment**: Add cameras, lenses, films, and photographers
2. **Create Setups**: Combine cameras with lenses
3. **Apply EXIF**: Select setup + film + photographer for batch processing

### EXIF Inspection

Use the check command to view EXIF data from any supported image file:

```bash
ifex check /path/to/image.jpg
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

## Architecture

IFEX uses a modular architecture with separated concerns for better maintainability and testing:

- **EXIF Manager** (`src/exif/exifManager.js`) - Main orchestrator
- **File Type Detection** (`src/exif/fileTypes.js`) - File format identification
- **EXIF Tags** (`src/exif/tags.js`) - Tag definitions and utilities
- **Format Processors** - Specialized handlers for different formats:
  - JPEG Processor (`src/exif/jpegProcessor.js`)
  - TIFF Processor (`src/exif/tiffProcessor.js`)
  - RAW Processor (`src/exif/rawProcessor.js`)
- **Sidecar Processor** (`src/exif/sidecarProcessor.js`) - XMP sidecar file generation
- **File Collector** (`src/exif/fileCollector.js`) - Recursive file discovery

## Testing

Run the comprehensive test suite:

```bash
npm test
```

Tests cover all modules including file types, EXIF tags, processors, and integration scenarios.

## Requirements

- Node.js 18.0.0 or higher
- Supported image file formats (see above)

## License

MIT

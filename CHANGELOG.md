# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.101.0] - 2025-07-27

### ✨ New Features
- **Optional lens support**: Equipment setups now support camera-only configurations
  - Create setups with just a camera (no lens required)
  - Option to select "No lens (camera only)" when creating or editing setups
  - EXIF metadata gracefully handles missing lens information
  - XMP sidecar files properly exclude lens data when not available

### 🔧 Improvements
- Enhanced setup management UI with clearer lens selection options
- Better visual indication for camera-only setups in setup listings
- Improved error handling for equipment validation

### 🏗️ Technical Changes
- Updated data models to support optional lens references
- Modified EXIF processors to conditionally include lens metadata
- Enhanced selection display to show "None (camera only)" for setups without lenses
- Updated all tests to work with optional lens structure

## [0.100.0] - 2025-07-25

### 🎯 Complete Rust Rewrite

This version represents a complete rewrite of IFEX from JavaScript/Node.js to Rust, bringing significant performance improvements, enhanced reliability, and better error handling.

#### ⚡ Performance & Reliability
- **Complete Rust rewrite**: From JavaScript/Node.js to Rust for maximum performance and memory safety
- **Native binary**: Single executable with no runtime dependencies
- **Cross-platform support**: Windows, macOS (Intel & Apple Silicon), and Linux binaries
- **Memory safety**: Zero-cost abstractions with compile-time guarantees

#### 🛠️ Core Features
- **Multi-format EXIF support**: JPEG, TIFF, DNG, and 20+ RAW camera formats
- **Equipment management**: Complete database for cameras, lenses, films, photographers, and setups
- **Interactive CLI**: Modern terminal interface with colored output and intuitive navigation
- **Batch processing**: Recursive directory scanning with progress feedback
- **ISO override**: Support for push/pull processing with custom ISO values

#### 🔧 Technical Improvements
- **Robust error handling**: Comprehensive error reporting with detailed feedback
- **Type safety**: Strong typing throughout the application prevents runtime errors
- **Modular architecture**: Clean separation of concerns with well-defined modules
- **Comprehensive testing**: 45+ unit and integration tests ensuring reliability
- **CI/CD pipeline**: Automated testing, formatting, and linting with GitHub Actions

#### 🎨 User Experience
- **Colored output**: Enhanced readability with syntax highlighting
- **Progress indicators**: Real-time feedback during batch operations
- **Clear navigation**: Intuitive menu system with easy-to-understand options
- **Flexible input**: Support for quoted paths, spaces, and various path formats

#### 📦 Development Quality
- **Clippy compliance**: Strict linting rules ensure code quality
- **Formatted code**: Consistent formatting with rustfmt
- **Documentation**: Comprehensive inline documentation and README
- **Security**: Forbids unsafe code and follows Rust security best practices

### Breaking Changes
- **Complete API rewrite**: This is not compatible with previous JavaScript versions
- **Configuration format**: New JSON-based configuration system
- **Command-line interface**: Updated CLI structure and options

### Migration
Users upgrading from previous versions will need to reconfigure their equipment database, as the underlying data format has changed to support the new Rust implementation.

## [0.3.0] - 2025-07-20

### Added

- 🎯 **Multi-Format Support**: Extended beyond JPEG to support TIFF, DNG, and RAW files
  - JPEG/JPG: Direct EXIF modification (existing functionality)
  - TIFF/TIF: Direct EXIF modification using UTIF library
  - DNG: Direct modification with fallback to XMP sidecar files
  - RAW formats: XMP sidecar file generation for CR2, CR3, NEF, NRW, ARW, SRF, ORF, RW2, RAF, SRW, PEF, X3F, and 20+ more formats
- 📁 **Recursive Directory Processing**: Process subdirectories with user confirmation (defaults to yes)
- 🔍 **Fuzzy Search**: Type-to-filter functionality in all selection menus
  - Real-time filtering as you type
  - Case-insensitive search
  - Partial match support
  - Enhanced user experience with instruction text
- 📝 **Smart Path Handling**: Robust support for various path formats
  - Paths with spaces: `/Users/user/My Photos`
  - Quoted paths: `"/Users/user/My Photos"`
  - Escaped paths: `/Users/user/My\ Photos`
  - Automatic trimming and cleanup
- 🛡️ **Enhanced Error Handling**: Robust validation and graceful fallbacks
  - Input validation for selection objects
  - UTIF decode/encode error handling with sidecar fallback
  - Clear, descriptive error messages
  - Graceful degradation for problematic files

### Improved

- **Case-Insensitive File Extension Detection**: More robust file type detection using `path.basename()` and `lastIndexOf()`
- **Enhanced File Type Detection**: Support for 25+ RAW camera formats
- **Better User Experience**: Updated prompts from "JPEG files" to "image files" to reflect multi-format support
- **Comprehensive Testing**: All 71 tests passing with enhanced coverage

### Technical

- Added `exifr` library for RAW file metadata reading
- Added `utif` library for TIFF/DNG processing
- Added `fuzzy` and `inquirer-autocomplete-prompt` for search functionality
- Enhanced ExifManager with format-specific processing methods
- Improved path validation and cleaning utilities
- Removed unused dependencies and imports

### Dependencies

- Added: `exifr@^7.1.3` for RAW file support
- Added: `utif@^3.1.0` for TIFF/DNG processing
- Added: `fuzzy@^0.1.3` for search functionality
- Added: `inquirer-autocomplete-prompt@^3.0.1` for enhanced UX

## [0.2.0] - 2025-07-20

### Added

- **Photographer Management**: Complete CRUD system for photographer information
  - Create photographers with name and optional email address
  - Email validation with proper regex patterns
  - Full CRUD operations (Create, Read, Update, Delete)
  - Integration with main workflow for photographer selection
- **Enhanced Navigation**: ESC key functionality for going back in CLI menus
  - Keyword-based cancellation system (type "esc", "escape", "back", or "cancel")
  - Consistent navigation experience across all CLI interfaces
  - Graceful error handling with CancelledError class
- **UUID Support**: Proper unique identifier generation
  - All models now use UUID v4 instead of timestamp-based IDs
  - Better data integrity and uniqueness guarantees
  - Added uuid dependency for reliable ID generation

### Changed

- **Test Infrastructure**: Improved test isolation
  - Tests now use temporary data files instead of overwriting user config
  - DataManager constructor accepts optional dataFile parameter
  - Comprehensive test coverage for all CRUD operations
- **CLI Workflow**: Enhanced user experience
  - Main workflow now includes photographer selection
  - Improved prompt utilities with cancellation support
  - Better error messages and validation feedback

### Fixed

- **Data Protection**: Tests no longer delete user's configuration file
- **ID Generation**: Replaced unreliable timestamp-based IDs with proper UUIDs
- **Navigation**: Added missing ESC key functionality throughout the application

### Technical

- Added `promptUtils.js` for enhanced CLI prompt management
- Updated all model classes to use UUID v4 generation
- Improved test isolation with temporary file system usage
- Enhanced error handling and user feedback systems
- Comprehensive test suite with 58+ passing tests

## [Unreleased]

### Planned

- Additional export formats for equipment data
- Batch operations for multiple photographer assignments
- Enhanced EXIF metadata validation and error reporting

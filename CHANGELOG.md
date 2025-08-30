# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### ✨ New Features
- **Enhanced read command**: Extended EXIF inspection capabilities
  - Support for multiple file inputs: `ifex read file1.jpg file2.png file3.tiff`
  - Directory processing: `ifex read /path/to/photos/` scans all supported image files
  - JSON output format: `ifex read --json` for structured data output suitable for scripts and APIs
  - Batch EXIF inspection with improved error handling for each file
- **Improved CLI interface**: Updated command descriptions and help text to reflect new capabilities

### 🔧 Improvements
- Enhanced file processing logic to handle multiple inputs efficiently
- Better error reporting for unsupported file formats and missing files
- Improved user experience with clearer output formatting for multiple files
- Added file format validation before processing

### 🏗️ Technical Changes
- Refactored `check_exif_data()` function to accept multiple paths and JSON output flag
- Integrated `FileSelector::scan_directory()` for directory processing
- Added JSON serialization for EXIF data output
- Enhanced error handling with per-file validation and reporting

## [1.103.3] - 2025-08-24

### 🐛 Bug Fixes
- **Fixed EXIF/IPTC metadata display in Google Photos**: Completely rewrote EXIF segment generation for JPEG files
  - Resolved "Truncated IFD" errors that prevented metadata readers from parsing EXIF data
  - Fixed malformed TIFF header structure in EXIF segments
  - Corrected field ordering requirements (entries now properly sorted by tag number)
  - Fixed offset calculations for external data storage
  - Ensured specification compliance with TIFF 6.0 and EXIF 2.32 standards
- **Improved EXIF data compatibility**: Metadata now properly displays in Google Photos, Apple Photos, and other applications
- **Enhanced data integrity**: All EXIF fields (camera make/model, lens info, ISO, focal length) are now correctly embedded

### 🔧 Code Quality
- Resolved all Clippy warnings for cleaner, more maintainable code
- Fixed lossless casting warnings and code organization issues
- Improved error handling and memory safety in EXIF processing

## [1.103.0] - 2025-08-14

### ✨ New Features
- Enhanced file path autocompletion with directory navigation
- Improved interactive prompts with double-tab directory listing
- Added comprehensive example files for testing and documentation

### 🔧 Improvements
- Code quality improvements with all Clippy warnings resolved
- Enhanced error handling and user feedback mechanisms
- Improved file selector functionality and user experience
- Better path handling with expanded tilde (~) support
- Optimized memory usage with proper resource management

### 🏗️ Technical Changes
- Major version bump to 1.x series for stability milestone
- Comprehensive code refactoring for better maintainability
- Enhanced test coverage and reliability
- Improved code organization and structure
- Performance optimizations and memory leak fixes

### 🐛 Bug Fixes
- Fixed significant memory drop warnings
- Resolved format string optimization issues
- Corrected file path handling edge cases
- Fixed various linting issues for cleaner codebase

## [0.102.0] - 2025-08-14

### ✨ New Features
- Enhanced file selector functionality for better user experience
- Improved file management and processing capabilities

### 🔧 Improvements  
- Updated codebase structure and organization
- Enhanced testing coverage and reliability
- Better error handling and user feedback

### 🏗️ Technical Changes
- Added new file selector module for streamlined file operations
- Improved code modularity and maintainability
- Enhanced test suite with additional test cases

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

### 📚 Documentation
- Added Homebrew upgrade instructions for easier package management

### 🐛 Bug Fixes
- Removed duplicate option for reading EXIF metadata

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

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

//! Utility functions for file handling and path processing.
//!
//! This module provides helper functions for cleaning user input paths,
//! determining supported image file formats, and extracting file type
//! information from file extensions.

use std::path::Path;

/// Cleans user-provided path input by removing quotes and handling escape sequences.
///
/// Removes surrounding single or double quotes from the input string,
/// and converts escaped spaces (backslash-space) to regular spaces.
/// This helps handle paths that users copy from file managers or shells.
#[must_use]
pub fn clean_path(input: &str) -> String {
  let trimmed = input.trim();

  if (trimmed.starts_with('"') && trimmed.ends_with('"'))
    || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
  {
    trimmed[1..trimmed.len() - 1].to_string()
  } else {
    trimmed.replace("\\ ", " ")
  }
}

/// Checks if a file path has a supported image format extension.
///
/// Returns true if the file extension matches any of the supported formats
/// including JPEG, TIFF, DNG, and various RAW formats from different camera manufacturers.
/// The check is case-insensitive.
#[must_use]
pub fn is_supported_image_format(path: &Path) -> bool {
  if let Some(extension) = path.extension() {
    if let Some(ext_str) = extension.to_str() {
      let ext_lower = ext_str.to_lowercase();
      matches!(
        ext_lower.as_str(),
        "jpg"
          | "jpeg"
          | "tif"
          | "tiff"
          | "dng"
          | "cr2"
          | "cr3"
          | "nef"
          | "nrw"
          | "arw"
          | "srf"
          | "sr2"
          | "orf"
          | "rw2"
          | "raf"
          | "srw"
          | "pef"
          | "x3f"
          | "erf"
          | "mef"
          | "mrw"
          | "dcr"
          | "kdc"
          | "3fr"
          | "fff"
          | "iiq"
          | "k25"
          | "rwl"
      )
    } else {
      false
    }
  } else {
    false
  }
}

/// Determines the file type category from a file path's extension.
///
/// Maps file extensions to broad categories used for EXIF processing:
/// - JPEG files return "jpeg"
/// - TIFF files return "tiff"
/// - DNG files return "dng"
/// - All other supported formats return "raw"
///
/// Returns None if the file has no extension or an unsupported extension.
#[must_use]
pub fn get_file_type(path: &Path) -> Option<String> {
  path.extension().and_then(|extension| {
    extension.to_str().map(|ext_str| {
      let ext_lower = ext_str.to_lowercase();
      match ext_lower.as_str() {
        "jpg" | "jpeg" => "jpeg".to_string(),
        "tif" | "tiff" => "tiff".to_string(),
        "dng" => "dng".to_string(),
        _ => "raw".to_string(),
      }
    })
  })
}

//! File type classification for EXIF processing.
//!
//! This module defines the file type enumeration used to categorize different
//! image formats and determine the appropriate processing strategy for each type.

use std::path::Path;

/// Enumeration of supported image file types for EXIF processing.
///
/// Each variant represents a category of image files that require
/// different approaches for EXIF metadata handling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileType {
  /// JPEG files - support direct EXIF embedding
  Jpeg,
  /// TIFF files - support direct EXIF embedding
  Tiff,
  /// Adobe DNG files - digital negative format
  Dng,
  /// RAW camera files - require XMP sidecar files for metadata
  Raw,
}

impl FileType {
  /// Determines the file type from a file path's extension.
  ///
  /// Examines the file extension and maps it to the appropriate `FileType` variant.
  /// Returns None for unsupported or missing file extensions.
  #[must_use]
  pub fn from_path(path: &Path) -> Option<Self> {
    if let Some(extension) = path.extension() {
      if let Some(ext_str) = extension.to_str() {
        let ext_lower = ext_str.to_lowercase();
        match ext_lower.as_str() {
          "jpg" | "jpeg" => Some(Self::Jpeg),
          "tif" | "tiff" => Some(Self::Tiff),
          "dng" => Some(Self::Dng),
          "cr2" | "cr3" | "nef" | "nrw" | "arw" | "srf" | "sr2" | "orf" | "rw2" | "raf" | "srw"
          | "pef" | "x3f" | "erf" | "mef" | "mrw" | "dcr" | "kdc" | "3fr" | "fff" | "iiq"
          | "k25" | "rwl" => Some(Self::Raw),
          _ => None,
        }
      } else {
        None
      }
    } else {
      None
    }
  }

  /// Checks if the file type supports direct EXIF embedding.
  ///
  /// Returns true for JPEG and TIFF files that can have EXIF data
  /// embedded directly in the file structure.
  #[must_use]
  pub const fn supports_direct_exif(&self) -> bool {
    matches!(self, Self::Jpeg | Self::Tiff)
  }

  /// Checks if the file type is a DNG file.
  ///
  /// DNG files require special handling as they are Adobe's digital negative format.
  #[must_use]
  pub const fn supports_dng_processing(&self) -> bool {
    matches!(self, Self::Dng)
  }

  /// Checks if the file type requires XMP sidecar files for metadata.
  ///
  /// Raw camera files cannot be modified directly, so metadata is stored
  /// in separate XMP files alongside the original raw file.
  #[must_use]
  pub const fn requires_sidecar(&self) -> bool {
    matches!(self, Self::Raw)
  }

  /// Returns a string representation of the file type.
  ///
  /// Provides a lowercase string identifier for the file type,
  /// useful for display and logging purposes.
  #[must_use]
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Jpeg => "jpeg",
      Self::Tiff => "tiff",
      Self::Dng => "dng",
      Self::Raw => "raw",
    }
  }
}

//! EXIF management and batch processing functionality.
//! 
//! This module provides the main interface for applying and erasing EXIF metadata
//! from image files. It handles batch processing of directories, file type detection,
//! and coordination with the appropriate file type processors.

use crate::models::Selection;
use crate::utils::{get_file_type, is_supported_image_format};
use std::path::Path;
use walkdir::WalkDir;

/// Result of a batch EXIF processing operation.
/// 
/// Contains overall success status, descriptive message, and detailed
/// statistics about the processing results.
#[derive(Debug)]
pub struct ProcessingResult {
  /// Whether the overall operation succeeded
  pub success: bool,
  /// Descriptive message about the operation result
  pub message: String,
  /// Detailed statistics about processed files
  pub results: ProcessingStats,
}

/// Statistics about files processed during an EXIF operation.
/// 
/// Tracks the number of successfully processed files, failed files,
/// and detailed results for each individual file.
#[derive(Debug)]
pub struct ProcessingStats {
  /// Number of files successfully processed
  pub processed: usize,
  /// Number of files that failed to process
  pub failed: usize,
  /// Detailed results for each file that was processed
  pub files: Vec<FileResult>,
}

/// Result information for a single file processing operation.
/// 
/// Contains the file name, success status, detected file type,
/// and any error message if processing failed.
#[derive(Debug)]
pub struct FileResult {
  /// Name of the processed file
  pub name: String,
  /// Whether the file was processed successfully
  pub success: bool,
  /// Detected file type (jpeg, tiff, dng, raw)
  pub file_type: Option<String>,
  /// Error message if processing failed
  pub error: Option<String>,
}

/// Main EXIF processing manager.
/// 
/// Provides methods for batch processing of image files, handling both
/// EXIF application and erasure operations across supported file formats.
pub struct ExifManager;

impl ExifManager {
  /// Creates a new ExifManager instance.
  pub fn new() -> Self {
    Self
  }

  /// Processes all supported image files in a folder.
  /// 
  /// Walks through the specified folder (optionally recursively) and applies
  /// the requested operation ("apply" or "erase") to all supported image files.
  /// For "apply" operations, a Selection containing equipment information is required.
  /// 
  /// Returns a ProcessingResult with statistics and detailed results for each file.
  pub async fn process_folder(
    &self,
    folder_path: &Path,
    selection: Option<&Selection>,
    operation: &str,
    recursive: bool,
  ) -> ProcessingResult {
    self.process_folder_with_iso(folder_path, selection, operation, recursive, None).await
  }

  /// Walks through the specified folder with optional custom shot ISO.
  /// 
  /// Supports custom ISO for push/pull processing. If shot_iso is None, uses film's base ISO.
  /// Returns a ProcessingResult with statistics and detailed results for each file.
  pub async fn process_folder_with_iso(
    &self,
    folder_path: &Path,
    selection: Option<&Selection>,
    operation: &str,
    recursive: bool,
    shot_iso: Option<u32>,
  ) -> ProcessingResult {
    let mut stats = ProcessingStats {
      processed: 0,
      failed: 0,
      files: Vec::new(),
    };

    let walker = if recursive {
      WalkDir::new(folder_path)
    } else {
      WalkDir::new(folder_path).max_depth(1)
    };

    for entry in walker {
      match entry {
        Ok(entry) => {
          let path = entry.path();

          if path.is_file() && is_supported_image_format(path) {
            let file_name = path
              .file_name()
              .unwrap_or_default()
              .to_string_lossy()
              .to_string();

            let file_type = get_file_type(path);

            let result = match operation {
              "apply" => self.apply_exif_with_iso(path, selection.unwrap(), shot_iso).await,
              "erase" => self.erase_exif(path).await,
              _ => Err("Unknown operation".into()),
            };

            match result {
              Ok(_) => {
                stats.processed += 1;
                stats.files.push(FileResult {
                  name: file_name,
                  success: true,
                  file_type,
                  error: None,
                });
              }
              Err(e) => {
                stats.failed += 1;
                stats.files.push(FileResult {
                  name: file_name,
                  success: false,
                  file_type,
                  error: Some(e.to_string()),
                });
              }
            }
          }
        }
        Err(e) => {
          eprintln!("Error reading directory entry: {}", e);
        }
      }
    }

    if stats.processed > 0 || stats.failed > 0 {
      ProcessingResult {
        success: true,
        message: "Processing completed".to_string(),
        results: stats,
      }
    } else {
      ProcessingResult {
        success: false,
        message: "No supported image files found".to_string(),
        results: stats,
      }
    }
  }

  /// Applies EXIF metadata to a single image file.
  /// 
  /// Determines the file type and delegates to the appropriate processor
  /// to apply the EXIF metadata from the provided equipment selection.
  #[allow(dead_code)]
  async fn apply_exif(
    &self,
    path: &Path,
    selection: &Selection,
  ) -> Result<(), Box<dyn std::error::Error>> {
    use crate::exif::file_types::FileType;
    use crate::exif::processors::{JpegProcessor, RawProcessor, TiffProcessor};

    let file_type = FileType::from_path(path)
      .ok_or_else(|| format!("Unsupported file type: {}", path.display()))?;

    match file_type {
      FileType::Jpeg => JpegProcessor::apply_exif(path, selection),
      FileType::Tiff => TiffProcessor::apply_exif(path, selection),
      FileType::Dng => TiffProcessor::apply_exif(path, selection),
      FileType::Raw => RawProcessor::apply_exif(path, selection),
    }
  }

  /// Applies EXIF metadata to a single image file with optional custom shot ISO.
  /// 
  /// Determines the file type and delegates to the appropriate processor.
  /// If shot_iso is provided, uses that instead of the film's base ISO for push/pull processing.
  async fn apply_exif_with_iso(
    &self,
    path: &Path,
    selection: &Selection,
    shot_iso: Option<u32>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    use crate::exif::file_types::FileType;
    use crate::exif::processors::{JpegProcessor, RawProcessor, TiffProcessor};

    let file_type = FileType::from_path(path)
      .ok_or_else(|| format!("Unsupported file type: {}", path.display()))?;

    match file_type {
      FileType::Jpeg => JpegProcessor::apply_exif_with_iso(path, selection, shot_iso),
      FileType::Tiff => TiffProcessor::apply_exif_with_iso(path, selection, shot_iso),
      FileType::Dng => TiffProcessor::apply_exif_with_iso(path, selection, shot_iso),
      FileType::Raw => RawProcessor::apply_exif_with_iso(path, selection, shot_iso),
    }
  }

  /// Erases EXIF metadata from a single image file.
  /// 
  /// Determines the file type and delegates to the appropriate processor
  /// to remove all EXIF metadata from the file.
  async fn erase_exif(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use crate::exif::file_types::FileType;
    use crate::exif::processors::{JpegProcessor, RawProcessor, TiffProcessor};

    let file_type = FileType::from_path(path)
      .ok_or_else(|| format!("Unsupported file type: {}", path.display()))?;

    match file_type {
      FileType::Jpeg => JpegProcessor::erase_exif(path),
      FileType::Tiff => TiffProcessor::erase_exif(path),
      FileType::Dng => TiffProcessor::erase_exif(path),
      FileType::Raw => RawProcessor::erase_exif(path),
    }
  }

  /// Reads EXIF metadata from an image file.
  /// 
  /// Determines the file type and delegates to the appropriate processor
  /// to extract all available EXIF metadata as key-value pairs.
  /// 
  /// Returns a vector of (tag_name, value) tuples sorted by tag name.
  pub fn read_exif_data(path: &Path) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    use crate::exif::file_types::FileType;
    use crate::exif::processors::{JpegProcessor, RawProcessor, TiffProcessor};

    let file_type = FileType::from_path(path)
      .ok_or_else(|| format!("Unsupported file type: {}", path.display()))?;

    match file_type {
      FileType::Jpeg => JpegProcessor::read_exif(path),
      FileType::Tiff => TiffProcessor::read_exif(path),
      FileType::Dng => TiffProcessor::read_exif(path),
      FileType::Raw => RawProcessor::read_exif(path),
    }
  }
}

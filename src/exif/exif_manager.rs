//! EXIF management and batch processing functionality.
//!
//! This module provides the main interface for applying and erasing EXIF metadata
//! from image files. It handles batch processing of directories, file type detection,
//! and coordination with the appropriate file type processors.

use crate::exif::file_types::FileType;
use crate::exif::processors::{JpegProcessor, RawProcessor, TiffProcessor};
use crate::models::Selection;
use crate::utils::{get_file_type, is_supported_image_format};
use chrono::{DateTime, Local, NaiveDateTime};
use std::path::{Path, PathBuf};
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

impl Default for ExifManager {
  fn default() -> Self {
    Self::new()
  }
}

impl ExifManager {
  /// Creates a new `ExifManager` instance.
  #[must_use]
  pub const fn new() -> Self {
    Self
  }

  /// Checks if all files have the same creation date
  fn check_identical_dates(&self, files: &[PathBuf]) -> Result<bool, Box<dyn std::error::Error>> {
    if files.len() <= 1 {
      return Ok(false);
    }

    let mut creation_dates = Vec::new();

    for file_path in files {
      if let Ok(date) = self.get_creation_date(file_path) {
        creation_dates.push(date);
      } else {
        // If we can't read the date from any file, assume they're not identical
        return Ok(false);
      }
    }

    if creation_dates.is_empty() {
      return Ok(false);
    }

    // Check if all dates are the same (within 1 second to account for precision differences)
    let first_date = creation_dates[0];
    Ok(creation_dates.iter().all(|&date| {
      (date - first_date).abs() <= chrono::Duration::seconds(1)
    }))
  }

  /// Gets the creation date from EXIF data
  fn get_creation_date(&self, file_path: &Path) -> Result<DateTime<Local>, Box<dyn std::error::Error>> {
    let exif_data = Self::read_exif_data(file_path)?;
    
    // Look for DateTimeOriginal first, then DateTime, then DateTimeDigitized
    for (tag_name, value) in &exif_data {
      if tag_name == "Date/Time Original" || tag_name == "Date/Time" || tag_name == "Date/Time Digitized" {
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(value, "%Y:%m:%d %H:%M:%S") {
          return Ok(DateTime::from_naive_utc_and_offset(naive_dt, *Local::now().offset()));
        }
      }
    }
    
    Err("No valid creation date found in EXIF data".into())
  }

  /// Prompts user whether to set identical dates for different timestamps
  fn prompt_set_identical_dates(&self) -> Result<bool, Box<dyn std::error::Error>> {
    use crate::prompts::PromptUtils;
    
    println!("The photos have different creation dates.");
    match PromptUtils::prompt_confirm(
      "Do you want to set the same creation date for all photos with 1-second increments?",
      false,
    )? {
      Some(response) => Ok(response),
      None => Ok(false),
    }
  }

  /// Adjusts creation dates with 1-second increments
  fn adjust_creation_dates(&self, files: &[PathBuf]) -> Result<(), Box<dyn std::error::Error>> {
    if files.is_empty() {
      return Ok(());
    }

    // Sort files by filename for consistent ordering
    let mut sorted_files = files.to_vec();
    sorted_files.sort_by(|a, b| {
      a.file_name().unwrap_or_default().cmp(b.file_name().unwrap_or_default())
    });

    // Get the original creation date from the first file
    let base_date = self.get_creation_date(&sorted_files[0])?;

    for (index, file_path) in sorted_files.iter().enumerate() {
      if index == 0 {
        // Keep the original date for the first file
        continue;
      }

      // Add 1 second for each subsequent file
      let new_date = base_date + chrono::Duration::seconds(i64::try_from(index).unwrap_or(0));
      self.set_creation_date(file_path, new_date)?;
    }

    Ok(())
  }

  /// Sets the creation date in EXIF data
  fn set_creation_date(&self, file_path: &Path, new_date: DateTime<Local>) -> Result<(), Box<dyn std::error::Error>> {
    // Format the date for EXIF
    let date_string = new_date.format("%Y:%m:%d %H:%M:%S").to_string();

    let file_type = FileType::from_path(file_path)
      .ok_or_else(|| format!("Unsupported file type: {}", file_path.display()))?;

    match file_type {
      FileType::Jpeg => JpegProcessor::set_creation_date(file_path, &date_string),
      FileType::Tiff => TiffProcessor::set_creation_date(file_path, &date_string),
      FileType::Dng => TiffProcessor::set_creation_date(file_path, &date_string),
      FileType::Raw => RawProcessor::set_creation_date(file_path, &date_string),
    }
  }

  /// Handles the date adjustment logic for a set of files
  fn handle_date_adjustment(&self, file_paths: &[PathBuf]) -> Result<(), Box<dyn std::error::Error>> {
    let has_identical_dates = self.check_identical_dates(file_paths)?;

    if has_identical_dates {
      println!("All photos have the same creation date. Adjusting with 1-second increments...");
      self.adjust_creation_dates(file_paths)?;
      println!("✅ Creation dates adjusted successfully!");
    } else {
      // Ask user if they want to set identical dates for different timestamps
      if self.prompt_set_identical_dates()? {
        self.adjust_creation_dates(file_paths)?;
        println!("✅ Creation dates set with 1-second increments!");
      }
    }

    Ok(())
  }

  /// Handles the date adjustment logic for a set of files with --one-sec flag
  fn handle_date_adjustment_with_one_sec(&self, file_paths: &[PathBuf], one_sec: bool) -> Result<(), Box<dyn std::error::Error>> {
    if !one_sec {
      return Ok(());
    }

    let has_identical_dates = self.check_identical_dates(file_paths)?;

    if has_identical_dates {
      println!("All photos have the same creation date. Adjusting with 1-second increments...");
      self.adjust_creation_dates(file_paths)?;
      println!("✅ Creation dates adjusted successfully!");
    } else {
      // Ask user if they want to set identical dates for different timestamps
      if self.prompt_set_identical_dates()? {
        self.adjust_creation_dates(file_paths)?;
        println!("✅ Creation dates set with 1-second increments!");
      }
    }

    Ok(())
  }

  /// Processes all supported image files in a folder.
  ///
  /// Walks through the specified folder recursively and applies
  /// the requested operation ("apply" or "erase") to all supported image files.
  /// For "apply" operations, a Selection containing equipment information is required.
  ///
  /// Returns a `ProcessingResult` with statistics and detailed results for each file.
  #[must_use]
  pub fn process_folder(
    &self,
    folder_path: &Path,
    selection: Option<&Selection>,
    operation: &str,
  ) -> ProcessingResult {
    self.process_folder_with_iso(folder_path, selection, operation, None)
  }

  /// Walks through the specified folder with optional custom shot ISO.
  ///
  /// Supports custom ISO for push/pull processing. If `shot_iso` is None, uses film's base ISO.
  /// Returns a `ProcessingResult` with statistics and detailed results for each file.
  #[must_use]
  pub fn process_folder_with_iso(
    &self,
    folder_path: &Path,
    selection: Option<&Selection>,
    operation: &str,
    shot_iso: Option<u32>,
  ) -> ProcessingResult {
    let mut stats = ProcessingStats {
      processed: 0,
      failed: 0,
      files: Vec::new(),
    };

    let walker = WalkDir::new(folder_path);

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
              "apply" => self.apply_exif_with_iso(path, selection.unwrap(), shot_iso),
              "erase" => self.erase_exif(path),
              _ => Err("Unknown operation".into()),
            };

            match result {
              Ok(()) => {
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
          eprintln!("Error reading directory entry: {e}");
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

  /// Processes a specific list of selected files with optional custom shot ISO.
  ///
  /// Applies the requested operation ("apply" or "erase") to the provided list of files.
  /// For "apply" operations, a Selection containing equipment information is required.
  /// Supports custom ISO for push/pull processing.
  /// Also handles automatic date adjustment for photos with identical creation dates.
  ///
  /// Returns a `ProcessingResult` with statistics and detailed results for each file.
  #[must_use]
  pub fn process_selected_files(
    &self,
    file_paths: &[PathBuf],
    selection: Option<&Selection>,
    operation: &str,
    shot_iso: Option<u32>,
  ) -> ProcessingResult {
    // Handle date adjustment logic before processing EXIF
    if operation == "apply" && file_paths.len() > 1 {
      if let Err(e) = self.handle_date_adjustment(file_paths) {
        eprintln!("Warning: Failed to adjust creation dates: {e}");
      }
    }
    
    self.process_files_internal(file_paths, selection, operation, shot_iso)
  }

  /// Processes a specific list of selected files with optional custom shot ISO and --one-sec flag.
  ///
  /// Applies the requested operation ("apply" or "erase") to the provided list of files.
  /// For "apply" operations, a Selection containing equipment information is required.
  /// Supports custom ISO for push/pull processing.
  /// Only handles date adjustment if the --one-sec flag is enabled.
  ///
  /// Returns a `ProcessingResult` with statistics and detailed results for each file.
  #[must_use]
  pub fn process_selected_files_with_one_sec(
    &self,
    file_paths: &[PathBuf],
    selection: Option<&Selection>,
    operation: &str,
    shot_iso: Option<u32>,
    one_sec: bool,
  ) -> ProcessingResult {
    // Handle date adjustment logic before processing EXIF only if --one-sec is enabled
    if operation == "apply" && file_paths.len() > 1 {
      if let Err(e) = self.handle_date_adjustment_with_one_sec(file_paths, one_sec) {
        eprintln!("Warning: Failed to adjust creation dates: {e}");
      }
    }
    
    self.process_files_internal(file_paths, selection, operation, shot_iso)
  }

  /// Internal method to process files without date adjustment logic
  fn process_files_internal(
    &self,
    file_paths: &[PathBuf],
    selection: Option<&Selection>,
    operation: &str,
    shot_iso: Option<u32>,
  ) -> ProcessingResult {
    let mut stats = ProcessingStats {
      processed: 0,
      failed: 0,
      files: Vec::new(),
    };

    for file_path in file_paths {
      if file_path.is_file() && is_supported_image_format(file_path) {
        let file_name = file_path
          .file_name()
          .unwrap_or_default()
          .to_string_lossy()
          .to_string();

        let file_type = get_file_type(file_path);

        let result = match operation {
          "apply" => self.apply_exif_with_iso(file_path, selection.unwrap(), shot_iso),
          "erase" => self.erase_exif(file_path),
          _ => Err("Unknown operation".into()),
        };

        match result {
          Ok(()) => {
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

    if stats.processed > 0 || stats.failed > 0 {
      ProcessingResult {
        success: true,
        message: "Processing completed".to_string(),
        results: stats,
      }
    } else {
      ProcessingResult {
        success: false,
        message: "No valid files to process".to_string(),
        results: stats,
      }
    }
  }

  /// Applies EXIF metadata to a single image file.
  ///
  /// Determines the file type and delegates to the appropriate processor
  /// to apply the EXIF metadata from the provided equipment selection.
  #[allow(dead_code)]
  fn apply_exif(
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
  /// If `shot_iso` is provided, uses that instead of the film's base ISO for push/pull processing.
  fn apply_exif_with_iso(
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
  fn erase_exif(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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
  /// Returns a vector of (`tag_name`, value) tuples sorted by tag name.
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

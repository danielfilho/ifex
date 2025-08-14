//! Interactive file selection interface for EXIF processing.
//!
//! This module provides an interactive interface allowing users to select
//! specific files from a directory for EXIF processing operations.
//! Uses multi-select functionality with arrow key navigation and spacebar selection.

use crate::utils::is_supported_image_format;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Interactive file selector for choosing specific files from a directory.
///
/// Provides functionality to scan directories for supported image files
/// and present them in an interactive multi-select interface.
pub struct FileSelector;

impl FileSelector {
  /// Creates a new `FileSelector` instance.
  #[must_use]
  pub const fn new() -> Self {
    Self
  }

  /// Scans a directory for supported image files.
  ///
  /// Walks through the specified directory (optionally recursively) and
  /// collects all supported image files. Returns a vector of file paths
  /// sorted by filename for consistent presentation.
  ///
  /// # Arguments
  ///
  /// * `folder_path` - The directory to scan for image files
  /// * `recursive` - Whether to scan subdirectories recursively
  ///
  /// # Returns
  ///
  /// A vector of `PathBuf` objects representing all supported image files found.
  #[must_use]
  pub fn scan_directory(folder_path: &Path, recursive: bool) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let walker = if recursive {
      WalkDir::new(folder_path)
    } else {
      WalkDir::new(folder_path).max_depth(1)
    };

    for entry in walker.into_iter().flatten() {
      let path = entry.path();
      if path.is_file() && is_supported_image_format(path) {
        files.push(path.to_path_buf());
      }
    }

    // Sort files by filename for consistent presentation
    files.sort_by(|a, b| {
      a.file_name()
        .unwrap_or_default()
        .cmp(b.file_name().unwrap_or_default())
    });

    files
  }

  /// Formats a file path for display in the selection interface.
  ///
  /// Creates a user-friendly display string showing the relative path
  /// from the base directory, or just the filename if it's a direct child.
  ///
  /// # Arguments
  ///
  /// * `file_path` - The full path to the file
  /// * `base_path` - The base directory path for relative display
  ///
  /// # Returns
  ///
  /// A formatted string suitable for display in the selection interface.
  #[must_use]
  pub fn format_file_for_display(file_path: &Path, base_path: &Path) -> String {
    if let Ok(relative_path) = file_path.strip_prefix(base_path) {
      relative_path.display().to_string()
    } else {
      file_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
    }
  }
}

impl Default for FileSelector {
  fn default() -> Self {
    Self::new()
  }
}
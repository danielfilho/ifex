//! Test file selection functionality

use ifex::file_selector::FileSelector;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_scan_directory_finds_supported_files() {
  let temp_dir = TempDir::new().expect("Failed to create temp directory");
  let temp_path = temp_dir.path();

  // Create some test files
  fs::write(temp_path.join("image1.jpg"), b"fake jpeg data").expect("Failed to write test file");
  fs::write(temp_path.join("image2.png"), b"fake png data").expect("Failed to write test file"); // Not supported
  fs::write(temp_path.join("image3.tiff"), b"fake tiff data").expect("Failed to write test file");
  fs::write(temp_path.join("document.txt"), b"text file").expect("Failed to write test file"); // Not supported

  // Test scanning
  let files = FileSelector::scan_directory(temp_path);

  // Should find 2 supported image files (jpg and tiff)
  assert_eq!(files.len(), 2);

  let filenames: Vec<String> = files
    .iter()
    .map(|f| f.file_name().unwrap().to_string_lossy().to_string())
    .collect();

  assert!(filenames.contains(&"image1.jpg".to_string()));
  assert!(filenames.contains(&"image3.tiff".to_string()));
  assert!(!filenames.contains(&"image2.png".to_string()));
  assert!(!filenames.contains(&"document.txt".to_string()));
}

#[test]
fn test_scan_directory_recursive() {
  let temp_dir = TempDir::new().expect("Failed to create temp directory");
  let temp_path = temp_dir.path();

  // Create subdirectory
  let sub_dir = temp_path.join("subdir");
  fs::create_dir(&sub_dir).expect("Failed to create subdirectory");

  // Create files in root and subdirectory
  fs::write(temp_path.join("root.jpg"), b"fake jpeg data").expect("Failed to write test file");
  fs::write(sub_dir.join("sub.tiff"), b"fake tiff data").expect("Failed to write test file");

  // Test recursive scanning (now the default and only behavior)
  let files = FileSelector::scan_directory(temp_path);
  assert_eq!(files.len(), 2); // Should find both root.jpg and subdir/sub.tiff
}

#[test]
fn test_format_file_for_display() {
  let base_path = Path::new("/Users/test");
  let file_path = Path::new("/Users/test/images/photo.jpg");

  let display = FileSelector::format_file_for_display(file_path, base_path);
  assert_eq!(display, "images/photo.jpg");

  // Test with direct child
  let direct_child = Path::new("/Users/test/photo.jpg");
  let display2 = FileSelector::format_file_for_display(direct_child, base_path);
  assert_eq!(display2, "photo.jpg");
}

#[test]
fn test_empty_directory() {
  let temp_dir = TempDir::new().expect("Failed to create temp directory");
  let temp_path = temp_dir.path();

  let files = FileSelector::scan_directory(temp_path);
  assert_eq!(files.len(), 0);
}

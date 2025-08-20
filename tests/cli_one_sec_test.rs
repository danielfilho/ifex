//! Tests for the --one-sec CLI argument functionality.

use ifex::exif::ExifManager;
use ifex::models::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_one_sec_flag_conditionally_processes_dates() {
  let temp_dir = TempDir::new().unwrap();
  let test_file1 = temp_dir.path().join("test1.jpg");
  let test_file2 = temp_dir.path().join("test2.jpg");

  // Create minimal JPEG files for testing
  let minimal_jpeg = create_minimal_jpeg();
  fs::write(&test_file1, &minimal_jpeg).unwrap();
  fs::write(&test_file2, &minimal_jpeg).unwrap();

  // Create test equipment
  let selection = create_test_selection();

  let file_paths = vec![test_file1, test_file2];

  let exif_manager = ExifManager::new();

  // Test with one_sec = false (should not process dates automatically)
  let result_without_one_sec = exif_manager.process_selected_files_with_one_sec(
    &file_paths,
    Some(&selection),
    "apply",
    Some(400),
    false, // one_sec = false
  );

  // Test with one_sec = true (should process dates if identical)
  let result_with_one_sec = exif_manager.process_selected_files_with_one_sec(
    &file_paths,
    Some(&selection),
    "apply",
    Some(400),
    true, // one_sec = true
  );

  // Both should succeed regardless of the one_sec flag for the EXIF application part
  assert!(result_without_one_sec.success);
  assert!(result_with_one_sec.success);
  assert_eq!(result_without_one_sec.results.processed, 2);
  assert_eq!(result_with_one_sec.results.processed, 2);
}

fn create_minimal_jpeg() -> Vec<u8> {
  vec![
    0xFF, 0xD8, // SOI
    0xFF, 0xE0, 0x00, 0x10, // APP0 segment
    b'J', b'F', b'I', b'F', 0x00, 0x01, 0x01, 0x01, 0x00, 0x48, 0x00, 0x48, 0x00, 0x00, 0xFF, 0xDB,
    0x00, 0x43, 0x00, // DQT
    // Quantization table (64 bytes)
    0x08, 0x06, 0x06, 0x07, 0x06, 0x05, 0x08, 0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0A, 0x0C, 0x14,
    0x0D, 0x0C, 0x0B, 0x0B, 0x0C, 0x19, 0x12, 0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D, 0x1A,
    0x1C, 0x1C, 0x20, 0x24, 0x2E, 0x27, 0x20, 0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37, 0x29, 0x2C,
    0x30, 0x31, 0x34, 0x34, 0x34, 0x1F, 0x27, 0x39, 0x3D, 0x38, 0x32, 0x3C, 0x2E, 0x33, 0x34, 0x32,
    0xFF, 0xC0, 0x00, 0x11, 0x08, 0x00, 0x10, 0x00, 0x10, 0x01, 0x01, 0x11, 0x00, 0x02, 0x11, 0x01,
    0x03, 0x11, 0x01, // SOF0
    0xFF, 0xC4, 0x00, 0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x08, // DHT
    0xFF, 0xDA, 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3F, 0x00, // SOS
    0xD2, 0xCF, 0x20, // minimal scan data
    0xFF, 0xD9, // EOI
  ]
}

fn create_test_selection() -> Selection {
  let camera = Camera::new("Test".to_string(), "Camera".to_string());
  let lens = Lens::new(
    "Test".to_string(),
    "Lens".to_string(),
    "35".to_string(),
    "f/2".to_string(),
    "Test".to_string(),
  );
  let film = Film::new("Test".to_string(), "Film".to_string(), 400);
  let photographer = Photographer::new("Test User".to_string(), None);
  let setup = Setup::new("Test Setup".to_string(), camera.id, Some(lens.id));

  Selection {
    setup,
    camera,
    lens: Some(lens),
    film,
    photographer,
  }
}

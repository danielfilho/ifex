use ifex::models::*;
use ifex::exif::processors::JpegProcessor;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_exif_apply_and_read_with_focal_length() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.jpg");
    
    // Create a minimal JPEG file for testing
    let minimal_jpeg = vec![
        0xFF, 0xD8, // SOI
        0xFF, 0xE0, 0x00, 0x10, // APP0 segment
        b'J', b'F', b'I', b'F', 0x00, 0x01, 0x01, 0x01, 0x00, 0x48, 0x00, 0x48, 0x00, 0x00,
        0xFF, 0xDB, 0x00, 0x43, 0x00, // DQT
        // Quantization table (64 bytes)
        0x08, 0x06, 0x06, 0x07, 0x06, 0x05, 0x08, 0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0A, 0x0C, 0x14,
        0x0D, 0x0C, 0x0B, 0x0B, 0x0C, 0x19, 0x12, 0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D, 0x1A,
        0x1C, 0x1C, 0x20, 0x24, 0x2E, 0x27, 0x20, 0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37, 0x29, 0x2C,
        0x30, 0x31, 0x34, 0x34, 0x34, 0x1F, 0x27, 0x39, 0x3D, 0x38, 0x32, 0x3C, 0x2E, 0x33, 0x34, 0x32,
        0xFF, 0xC0, 0x00, 0x11, 0x08, 0x00, 0x10, 0x00, 0x10, 0x01, 0x01, 0x11, 0x00, 0x02, 0x11, 0x01, 0x03, 0x11, 0x01, // SOF0
        0xFF, 0xC4, 0x00, 0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, // DHT
        0xFF, 0xDA, 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3F, 0x00, // SOS
        0xD2, 0xCF, 0x20, // minimal scan data
        0xFF, 0xD9 // EOI
    ];
    
    fs::write(&test_file, minimal_jpeg).unwrap();
    
    // Create test equipment with focal length that can be parsed
    let camera = Camera::new("Test".to_string(), "Camera".to_string());
    let lens = Lens::new(
        "Test".to_string(),
        "Lens".to_string(),
        "35".to_string(), // This should be parseable as f32
        "f/2".to_string(),
        "Test".to_string(),
    );
    let film = Film::new("Test".to_string(), "Film".to_string(), 400);
    let photographer = Photographer::new("Test User".to_string(), None);
    let setup = Setup::new("Test Setup".to_string(), camera.id.clone(), lens.id.clone());
    
    let selection = Selection {
        setup,
        camera,
        lens,
        film,
        photographer,
    };
    
    // Apply EXIF data (this should not fail with truncated IFD count)
    let result = JpegProcessor::apply_exif_with_iso(&test_file, &selection, Some(800));
    assert!(result.is_ok(), "Failed to apply EXIF data: {:?}", result.err());
    
    // Try to read back the EXIF data (this should not fail with truncated IFD count)
    let read_result = JpegProcessor::read_exif(&test_file);
    assert!(read_result.is_ok(), "Failed to read EXIF data after applying: {:?}", read_result.err());
    
    let exif_data = read_result.unwrap();
    
    // Verify some of the EXIF data was written correctly
    let has_make = exif_data.iter().any(|(key, value)| key.contains("Make") && value.contains("Test"));
    let has_model = exif_data.iter().any(|(key, value)| key.contains("Model") && value.contains("Camera"));
    let has_iso = exif_data.iter().any(|(key, value)| key.contains("ISO") && value.contains("800"));
    
    assert!(has_make, "Make field not found in EXIF data");
    assert!(has_model, "Model field not found in EXIF data");
    assert!(has_iso, "ISO field not found in EXIF data");
}

#[test]
fn test_exif_apply_and_read_without_focal_length() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test2.jpg");
    
    // Create a minimal JPEG file for testing
    let minimal_jpeg = vec![
        0xFF, 0xD8, // SOI
        0xFF, 0xE0, 0x00, 0x10, // APP0 segment
        b'J', b'F', b'I', b'F', 0x00, 0x01, 0x01, 0x01, 0x00, 0x48, 0x00, 0x48, 0x00, 0x00,
        0xFF, 0xDB, 0x00, 0x43, 0x00, // DQT
        // Quantization table (64 bytes)
        0x08, 0x06, 0x06, 0x07, 0x06, 0x05, 0x08, 0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0A, 0x0C, 0x14,
        0x0D, 0x0C, 0x0B, 0x0B, 0x0C, 0x19, 0x12, 0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D, 0x1A,
        0x1C, 0x1C, 0x20, 0x24, 0x2E, 0x27, 0x20, 0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37, 0x29, 0x2C,
        0x30, 0x31, 0x34, 0x34, 0x34, 0x1F, 0x27, 0x39, 0x3D, 0x38, 0x32, 0x3C, 0x2E, 0x33, 0x34, 0x32,
        0xFF, 0xC0, 0x00, 0x11, 0x08, 0x00, 0x10, 0x00, 0x10, 0x01, 0x01, 0x11, 0x00, 0x02, 0x11, 0x01, 0x03, 0x11, 0x01, // SOF0
        0xFF, 0xC4, 0x00, 0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, // DHT
        0xFF, 0xDA, 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3F, 0x00, // SOS
        0xD2, 0xCF, 0x20, // minimal scan data
        0xFF, 0xD9 // EOI
    ];
    
    fs::write(&test_file, minimal_jpeg).unwrap();
    
    // Create test equipment with focal length that cannot be parsed (to test different entry count)
    let camera = Camera::new("Test".to_string(), "Camera".to_string());
    let lens = Lens::new(
        "Test".to_string(),
        "Lens".to_string(),
        "non-numeric".to_string(), // This should NOT be parseable as f32
        "f/2".to_string(),
        "Test".to_string(),
    );
    let film = Film::new("Test".to_string(), "Film".to_string(), 200);
    let photographer = Photographer::new("Test User".to_string(), None);
    let setup = Setup::new("Test Setup".to_string(), camera.id.clone(), lens.id.clone());
    
    let selection = Selection {
        setup,
        camera,
        lens,
        film,
        photographer,
    };
    
    // Apply EXIF data (this should not fail with truncated IFD count)
    let result = JpegProcessor::apply_exif_with_iso(&test_file, &selection, None);
    assert!(result.is_ok(), "Failed to apply EXIF data: {:?}", result.err());
    
    // Try to read back the EXIF data (this should not fail with truncated IFD count)
    let read_result = JpegProcessor::read_exif(&test_file);
    assert!(read_result.is_ok(), "Failed to read EXIF data after applying: {:?}", read_result.err());
    
    let exif_data = read_result.unwrap();
    
    // Verify some of the EXIF data was written correctly
    let has_make = exif_data.iter().any(|(key, value)| key.contains("Make") && value.contains("Test"));
    let has_model = exif_data.iter().any(|(key, value)| key.contains("Model") && value.contains("Camera"));
    let has_iso = exif_data.iter().any(|(key, value)| key.contains("ISO") && value.contains("200"));
    
    assert!(has_make, "Make field not found in EXIF data");
    assert!(has_model, "Model field not found in EXIF data");
    assert!(has_iso, "ISO field not found in EXIF data");
}

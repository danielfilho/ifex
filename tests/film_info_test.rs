//! Test for film information in EXIF data

use ifex::{
    exif::processors::JpegProcessor,
    models::{Camera, Film, Lens, Photographer, Selection, Setup},
};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_film_info_in_exif_segment() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create the equipment that matches the user's setup
    let camera = Camera::new("Leica".to_string(), "M7".to_string());
    let lens = Lens::new(
        "Leica".to_string(),
        "Summicron Asph. V5 35mm".to_string(),
        "35".to_string(),
        "2".to_string(),
        "M".to_string(),
    );
    let film = Film::new("Fujifilm".to_string(), "Santacolor 100".to_string(), 100);
    let photographer = Photographer::new("Daniel Filho".to_string(), None);
    let setup = Setup::new("Leica M7 + Summicron".to_string(), camera.id, Some(lens.id));

    let selection = Selection {
        setup,
        camera,
        lens: Some(lens),
        film,
        photographer,
    };

    // Create a minimal JPEG file for testing
    let test_image_path = temp_dir.path().join("test.jpg");
    
    // Minimal JPEG file structure
    let jpeg_data = vec![
        0xFF, 0xD8, // JPEG SOI marker
        0xFF, 0xDA, // SOS marker (start of scan)
        0x00, 0x0C, // Length
        0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3F, 0x00, // SOS data
        0xFF, 0xD9, // EOI marker
    ];
    
    fs::write(&test_image_path, &jpeg_data).unwrap();

    // Apply EXIF data including film information
    let result = JpegProcessor::apply_exif(&test_image_path, &selection);
    assert!(result.is_ok(), "Failed to apply EXIF: {result:?}");

    // Read back the EXIF data to verify
    let exif_data = JpegProcessor::read_exif(&test_image_path).unwrap();
    
    println!("EXIF data after applying film information:");
    for (tag, value) in &exif_data {
        println!("  {tag}: {value}");
    }

    // Check if film information is present
    let has_film_info = exif_data.iter().any(|(tag, value)| {
        tag.contains("Film") && value.contains("Santacolor 100")
    });

    assert!(
        has_film_info,
        "Film information should be present in EXIF data. Available tags: {exif_data:#?}"
    );

    // Also check for other expected data
    let has_camera_make = exif_data.iter().any(|(tag, value)| {
        tag.contains("Make") && value.contains("Leica")
    });
    
    let has_lens_model = exif_data.iter().any(|(tag, value)| {
        tag.contains("Lens Model") && value.contains("Summicron")
    });

    assert!(has_camera_make, "Camera make should be present");
    assert!(has_lens_model, "Lens model should be present");
}
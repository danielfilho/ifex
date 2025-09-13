use ifex::{
    exif::processors::JpegProcessor,
    models::{Camera, Film, Lens, Photographer, Selection, Setup},
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create test JPEG
    let jpeg_data = vec![
        0xFF, 0xD8, // JPEG SOI marker
        0xFF, 0xDA, // SOS marker (start of scan)
        0x00, 0x0C, // Length
        0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3F, 0x00, // SOS data
        0xFF, 0xD9, // EOI marker
    ];
    
    let test_path = "/tmp/test_film.jpg";
    fs::write(test_path, &jpeg_data)?;

    // Create equipment selection
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
    let setup = Setup::new("Test Setup".to_string(), camera.id, Some(lens.id));

    let selection = Selection {
        setup,
        camera,
        lens: Some(lens),
        film,
        photographer,
    };

    // Apply EXIF
    println!("Applying EXIF data...");
    JpegProcessor::apply_exif(std::path::Path::new(test_path), &selection)?;

    // Read back
    println!("Reading EXIF data back...");
    let exif_data = JpegProcessor::read_exif(std::path::Path::new(test_path))?;
    
    for (tag, value) in &exif_data {
        println!("  {}: {}", tag, value);
    }

    // Check specifically for Image Description
    let image_desc = exif_data.iter().find(|(tag, _)| tag.contains("Image Description"));
    match image_desc {
        Some((tag, value)) => {
            println!("\n✅ Found Image Description: {} = {}", tag, value);
            if value.contains("Santacolor 100") {
                println!("✅ Film information is present!");
            } else {
                println!("❌ Film information is missing from Image Description");
            }
        }
        None => println!("❌ No Image Description field found"),
    }

    // Clean up
    std::fs::remove_file(test_path).ok();

    Ok(())
}
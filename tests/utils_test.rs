use ifex::utils::*;
use std::path::Path;

#[test]
fn test_clean_path_removes_double_quotes() {
    let input = r#""/path/to/file.jpg""#;
    assert_eq!(clean_path(input), "/path/to/file.jpg");
}

#[test]
fn test_clean_path_removes_single_quotes() {
    let input = "'/path/to/file.jpg'";
    assert_eq!(clean_path(input), "/path/to/file.jpg");
}

#[test]
fn test_clean_path_handles_escaped_spaces() {
    let input = r"/path/with\ spaces/file.jpg";
    assert_eq!(clean_path(input), "/path/with spaces/file.jpg");
}

#[test]
fn test_clean_path_no_quotes() {
    let input = "/path/to/file.jpg";
    assert_eq!(clean_path(input), "/path/to/file.jpg");
}

#[test]
fn test_clean_path_trims_whitespace() {
    let input = "  /path/to/file.jpg  ";
    assert_eq!(clean_path(input), "/path/to/file.jpg");
}

#[test]
fn test_is_supported_image_format_jpeg() {
    assert!(is_supported_image_format(Path::new("test.jpg")));
    assert!(is_supported_image_format(Path::new("test.jpeg")));
    assert!(is_supported_image_format(Path::new("test.JPG")));
    assert!(is_supported_image_format(Path::new("test.JPEG")));
}

#[test]
fn test_is_supported_image_format_tiff() {
    assert!(is_supported_image_format(Path::new("test.tif")));
    assert!(is_supported_image_format(Path::new("test.tiff")));
    assert!(is_supported_image_format(Path::new("test.TIF")));
    assert!(is_supported_image_format(Path::new("test.TIFF")));
}

#[test]
fn test_is_supported_image_format_dng() {
    assert!(is_supported_image_format(Path::new("test.dng")));
    assert!(is_supported_image_format(Path::new("test.DNG")));
}

#[test]
fn test_is_supported_image_format_raw_formats() {
    assert!(is_supported_image_format(Path::new("test.cr2")));
    assert!(is_supported_image_format(Path::new("test.nef")));
    assert!(is_supported_image_format(Path::new("test.arw")));
    assert!(is_supported_image_format(Path::new("test.orf")));
    assert!(is_supported_image_format(Path::new("test.rw2")));
    assert!(is_supported_image_format(Path::new("test.raf")));
}

#[test]
fn test_is_supported_image_format_unsupported() {
    assert!(!is_supported_image_format(Path::new("test.png")));
    assert!(!is_supported_image_format(Path::new("test.txt")));
    assert!(!is_supported_image_format(Path::new("test")));
}

#[test]
fn test_get_file_type_jpeg() {
    assert_eq!(get_file_type(Path::new("test.jpg")), Some("jpeg".to_string()));
    assert_eq!(get_file_type(Path::new("test.jpeg")), Some("jpeg".to_string()));
    assert_eq!(get_file_type(Path::new("test.JPG")), Some("jpeg".to_string()));
}

#[test]
fn test_get_file_type_tiff() {
    assert_eq!(get_file_type(Path::new("test.tif")), Some("tiff".to_string()));
    assert_eq!(get_file_type(Path::new("test.tiff")), Some("tiff".to_string()));
    assert_eq!(get_file_type(Path::new("test.TIF")), Some("tiff".to_string()));
}

#[test]
fn test_get_file_type_dng() {
    assert_eq!(get_file_type(Path::new("test.dng")), Some("dng".to_string()));
    assert_eq!(get_file_type(Path::new("test.DNG")), Some("dng".to_string()));
}

#[test]
fn test_get_file_type_raw() {
    assert_eq!(get_file_type(Path::new("test.cr2")), Some("raw".to_string()));
    assert_eq!(get_file_type(Path::new("test.nef")), Some("raw".to_string()));
    assert_eq!(get_file_type(Path::new("test.arw")), Some("raw".to_string()));
}

#[test]
fn test_get_file_type_no_extension() {
    assert_eq!(get_file_type(Path::new("test")), None);
}

#[test]
fn test_get_file_type_unsupported() {
    assert_eq!(get_file_type(Path::new("test.png")), Some("raw".to_string()));
    assert_eq!(get_file_type(Path::new("test.txt")), Some("raw".to_string()));
}
use ifex::exif::file_types::FileType;
use std::path::Path;

#[test]
fn test_file_type_from_path_jpeg() {
  assert_eq!(
    FileType::from_path(Path::new("test.jpg")),
    Some(FileType::Jpeg)
  );
  assert_eq!(
    FileType::from_path(Path::new("test.jpeg")),
    Some(FileType::Jpeg)
  );
  assert_eq!(
    FileType::from_path(Path::new("test.JPG")),
    Some(FileType::Jpeg)
  );
  assert_eq!(
    FileType::from_path(Path::new("test.JPEG")),
    Some(FileType::Jpeg)
  );
}

#[test]
fn test_file_type_from_path_tiff() {
  assert_eq!(
    FileType::from_path(Path::new("test.tif")),
    Some(FileType::Tiff)
  );
  assert_eq!(
    FileType::from_path(Path::new("test.tiff")),
    Some(FileType::Tiff)
  );
  assert_eq!(
    FileType::from_path(Path::new("test.TIF")),
    Some(FileType::Tiff)
  );
  assert_eq!(
    FileType::from_path(Path::new("test.TIFF")),
    Some(FileType::Tiff)
  );
}

#[test]
fn test_file_type_from_path_dng() {
  assert_eq!(
    FileType::from_path(Path::new("test.dng")),
    Some(FileType::Dng)
  );
  assert_eq!(
    FileType::from_path(Path::new("test.DNG")),
    Some(FileType::Dng)
  );
}

#[test]
fn test_file_type_from_path_raw() {
  assert_eq!(
    FileType::from_path(Path::new("test.cr2")),
    Some(FileType::Raw)
  );
  assert_eq!(
    FileType::from_path(Path::new("test.nef")),
    Some(FileType::Raw)
  );
  assert_eq!(
    FileType::from_path(Path::new("test.arw")),
    Some(FileType::Raw)
  );
  assert_eq!(
    FileType::from_path(Path::new("test.orf")),
    Some(FileType::Raw)
  );
  assert_eq!(
    FileType::from_path(Path::new("test.rw2")),
    Some(FileType::Raw)
  );
  assert_eq!(
    FileType::from_path(Path::new("test.raf")),
    Some(FileType::Raw)
  );
}

#[test]
fn test_file_type_from_path_unsupported() {
  assert_eq!(FileType::from_path(Path::new("test.png")), None);
  assert_eq!(FileType::from_path(Path::new("test.txt")), None);
  assert_eq!(FileType::from_path(Path::new("test")), None);
}

#[test]
fn test_supports_direct_exif() {
  assert!(FileType::Jpeg.supports_direct_exif());
  assert!(FileType::Tiff.supports_direct_exif());
  assert!(!FileType::Dng.supports_direct_exif());
  assert!(!FileType::Raw.supports_direct_exif());
}

#[test]
fn test_supports_dng_processing() {
  assert!(!FileType::Jpeg.supports_dng_processing());
  assert!(!FileType::Tiff.supports_dng_processing());
  assert!(FileType::Dng.supports_dng_processing());
  assert!(!FileType::Raw.supports_dng_processing());
}

#[test]
fn test_requires_sidecar() {
  assert!(!FileType::Jpeg.requires_sidecar());
  assert!(!FileType::Tiff.requires_sidecar());
  assert!(!FileType::Dng.requires_sidecar());
  assert!(FileType::Raw.requires_sidecar());
}

#[test]
fn test_as_str() {
  assert_eq!(FileType::Jpeg.as_str(), "jpeg");
  assert_eq!(FileType::Tiff.as_str(), "tiff");
  assert_eq!(FileType::Dng.as_str(), "dng");
  assert_eq!(FileType::Raw.as_str(), "raw");
}

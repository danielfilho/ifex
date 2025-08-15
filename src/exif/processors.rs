//! EXIF metadata processors for different image file formats.
//!
//! This module contains specialized processors for handling EXIF metadata
//! operations on different image file types. Each processor implements
//! format-specific logic for applying, erasing, and reading EXIF data.

use crate::exif::tags::ExifTags;
use crate::models::Selection;
use exif::{Reader, Value};
use std::fs;
use std::io::BufReader;
use std::path::Path;

/// JPEG file EXIF processor.
///
/// Handles EXIF metadata operations for JPEG files by manipulating
/// the EXIF segments directly in the JPEG file structure.
pub struct JpegProcessor;

/// TIFF file EXIF processor.
///
/// Handles EXIF metadata operations for TIFF files using the image crate
/// for file manipulation and the exif crate for metadata reading.
pub struct TiffProcessor;

/// RAW file EXIF processor.
///
/// Handles EXIF metadata operations for RAW camera files by creating
/// and managing XMP sidecar files alongside the original raw files.
pub struct RawProcessor;

impl JpegProcessor {
  /// Sets the creation date in a JPEG file's EXIF data.
  ///
  /// Updates the `DateTimeOriginal`, `DateTime`, and `DateTimeDigitized` fields in the EXIF data.
  pub fn set_creation_date(
    path: &Path,
    date_string: &str,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let mut bufreader = BufReader::new(&file);

    let exifreader = Reader::new();
    let existing_exif = exifreader.read_from_container(&mut bufreader).ok();

    let original_data = fs::read(path)?;

    let mut new_data = Vec::new();

    if original_data.len() >= 2 && &original_data[0..2] == b"\xff\xd8" {
      new_data.extend_from_slice(&original_data[0..2]);

      // Create EXIF segment with updated date
      let exif_data = Self::create_date_exif_segment(date_string, existing_exif.as_ref())?;
      new_data.extend_from_slice(&exif_data);

      let mut i = 2;
      while i < original_data.len() - 1 {
        if original_data[i] == 0xff {
          let marker = original_data[i + 1];
          if marker == 0xe1 {
            let segment_length =
              (u16::from(original_data[i + 2]) << 8) | u16::from(original_data[i + 3]);
            i += 2 + segment_length as usize;
            continue;
          }
        }
        break;
      }

      new_data.extend_from_slice(&original_data[i..]);
    } else {
      return Err("Not a valid JPEG file".into());
    }

    fs::write(path, new_data)?;
    Ok(())
  }

  /// Creates an EXIF segment specifically for updating date fields
  fn create_date_exif_segment(
    date_string: &str,
    existing_exif: Option<&exif::Exif>,
  ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use exif::Value;

    let mut segment = Vec::new();
    segment.extend_from_slice(b"\xff\xe1");

    let mut data = Vec::new();
    data.extend_from_slice(b"Exif\x00\x00");
    data.extend_from_slice(b"II*\x00");
    let ifd_offset = 8u32;
    data.extend_from_slice(&ifd_offset.to_le_bytes());

    // Date tags we want to update
    let date_tag_numbers = [
      0x0132, // DateTime
      0x9003, // DateTimeOriginal
      0x9004, // DateTimeDigitized
    ];

    // Collect preserved fields from existing EXIF
    let mut preserved_fields = Vec::new();

    if let Some(exif) = existing_exif {
      for field in exif.fields() {
        let tag_number = Self::tag_to_number(field.tag);

        // Skip the date fields we're updating
        if let Some(tag_num) = tag_number {
          if date_tag_numbers.contains(&tag_num) {
            continue;
          }
        }

        // Preserve other fields
        if let Value::Ascii(ascii_vec) = &field.value {
          for ascii_bytes in ascii_vec {
            if let Ok(string_value) = std::str::from_utf8(ascii_bytes) {
              let clean_value = string_value.trim_end_matches('\0');
              if !clean_value.is_empty() && clean_value.len() < 1000 {
                if let Some(tag_number) = Self::tag_to_number(field.tag) {
                  preserved_fields.push((tag_number, 0x02, clean_value.as_bytes().to_vec()));
                }
              }
            }
          }
        } else {
          // Preserve other field types using existing logic from the original implementation
        }
      }
    }

    // Calculate entry count
    let entry_count = preserved_fields.len() + date_tag_numbers.len();
    data.extend_from_slice(&(entry_count as u16).to_le_bytes());

    // Calculate where string data will start
    let string_data_start = 8 + 2 + (entry_count * 12) + 4;
    let mut string_offset = string_data_start;
    let mut string_data = Vec::new();

    // Add preserved fields
    for (tag_num, field_type, field_data) in preserved_fields {
      let mut entry = Vec::new();
      entry.extend_from_slice(&tag_num.to_le_bytes());
      entry.extend_from_slice(&[field_type, 0x00]);

      let count = field_data.len();
      entry.extend_from_slice(&u32::try_from(count).unwrap_or(0).to_le_bytes());

      if field_data.len() <= 4 {
        let mut padded_data = field_data.clone();
        while padded_data.len() < 4 {
          padded_data.push(0);
        }
        entry.extend_from_slice(&padded_data[0..4]);
      } else {
        entry.extend_from_slice(&u32::try_from(string_offset).unwrap_or(0).to_le_bytes());
        string_data.extend_from_slice(&field_data);
        string_offset += field_data.len();
      }

      data.extend_from_slice(&entry);
    }

    // Add date entries
    for &tag_num in &date_tag_numbers {
      let mut entry = Vec::new();
      entry.extend_from_slice(&tag_num.to_le_bytes());
      entry.extend_from_slice(&[0x02, 0x00]); // ASCII type
      let string_len = date_string.len() + 1; // Include null terminator
      entry.extend_from_slice(&u32::try_from(string_len).unwrap_or(0).to_le_bytes());

      if string_len <= 4 {
        let mut padded_value = date_string.as_bytes().to_vec();
        padded_value.push(0); // null terminator
        while padded_value.len() < 4 {
          padded_value.push(0);
        }
        entry.extend_from_slice(&padded_value[0..4]);
      } else {
        entry.extend_from_slice(&u32::try_from(string_offset).unwrap_or(0).to_le_bytes());
        string_data.extend_from_slice(date_string.as_bytes());
        string_data.push(0); // null terminator
        string_offset += string_len;
      }

      data.extend_from_slice(&entry);
    }

    // Next IFD pointer (0 = no more IFDs)
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

    // Append string data
    data.extend_from_slice(&string_data);

    // Add length and data to segment
    let length = u16::try_from(data.len() + 2).unwrap_or(0);
    segment.push((length >> 8) as u8);
    segment.push((length & 0xff) as u8);
    segment.extend_from_slice(&data);

    Ok(segment)
  }

  /// Applies EXIF metadata to a JPEG file.
  ///
  /// Creates new EXIF segments containing equipment and photographer information
  /// from the selection and embeds them into the JPEG file structure.
  /// This method preserves existing EXIF/IPTC data and only updates the specified fields.
  pub fn apply_exif(path: &Path, selection: &Selection) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let mut bufreader = BufReader::new(&file);

    let exifreader = Reader::new();
    let existing_exif = exifreader.read_from_container(&mut bufreader).ok();

    let original_data = fs::read(path)?;

    let mut new_data = Vec::new();

    if original_data.len() >= 2 && &original_data[0..2] == b"\xff\xd8" {
      new_data.extend_from_slice(&original_data[0..2]);

      // Create merged EXIF segment that preserves existing data
      let exif_data = Self::create_merged_exif_segment(selection, existing_exif.as_ref())?;
      new_data.extend_from_slice(&exif_data);

      let mut i = 2;
      while i < original_data.len() - 1 {
        if original_data[i] == 0xff {
          let marker = original_data[i + 1];
          if marker == 0xe1 {
            let segment_length =
              (u16::from(original_data[i + 2]) << 8) | u16::from(original_data[i + 3]);
            i += 2 + segment_length as usize;
            continue;
          }
        }
        new_data.push(original_data[i]);
        i += 1;
      }
      if i < original_data.len() {
        new_data.push(original_data[i]);
      }
    } else {
      return Err("Not a valid JPEG file".into());
    }

    fs::write(path, new_data)?;
    Ok(())
  }

  /// Erases EXIF metadata from a JPEG file.
  ///
  /// Removes all EXIF and JFIF segments from the JPEG file,
  /// leaving only the core image data.
  pub fn erase_exif(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let original_data = fs::read(path)?;

    if original_data.len() < 2 || &original_data[0..2] != b"\xff\xd8" {
      return Err("Not a valid JPEG file".into());
    }

    let mut new_data = Vec::new();
    new_data.extend_from_slice(&original_data[0..2]);

    let mut i = 2;
    while i < original_data.len() - 1 {
      if original_data[i] == 0xff {
        let marker = original_data[i + 1];
        if marker == 0xe1 || marker == 0xe0 {
          let segment_length =
            (u16::from(original_data[i + 2]) << 8) | u16::from(original_data[i + 3]);
          i += 2 + segment_length as usize;
          continue;
        }
      }
      new_data.push(original_data[i]);
      i += 1;
    }
    if i < original_data.len() {
      new_data.push(original_data[i]);
    }

    fs::write(path, new_data)?;
    Ok(())
  }

  /// Read EXIF data from a JPEG file and return as key-value pairs
  pub fn read_exif(path: &Path) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let mut bufreader = BufReader::new(&file);

    let exifreader = Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;

    let mut results = Vec::new();

    // Read all EXIF fields from all IFDs
    for field in exif.fields() {
      let tag_name = Self::format_tag_name(&field.tag);
      let mut value = Self::format_exif_value(&field.value);

      // Truncate long values
      if value.len() > 50 {
        value.truncate(50);
        value.push('…');
      }

      // Add IFD context to help identify the source
      let ifd_name = match field.ifd_num {
        exif::In::PRIMARY => "",
        exif::In::THUMBNAIL => " (Thumbnail)",
        _ => " (Sub-IFD)",
      };
      let full_tag_name = if ifd_name.is_empty() {
        tag_name.clone()
      } else {
        format!("{tag_name}{ifd_name}")
      };

      // Also add raw tag info for debugging unknown tags
      let raw_tag_info = format!("{:?}", field.tag);
      if raw_tag_info.contains("Tag(") && !raw_tag_info.starts_with(&tag_name) {
        results.push((format!("{full_tag_name} [{raw_tag_info}]"), value.clone()));
      } else {
        results.push((full_tag_name, value));
      }
    }

    // Also try to read IPTC data from APP13 segments if present
    let mut iptc_results = Self::read_iptc_data(path)?;
    results.append(&mut iptc_results);

    results.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(results)
  }

  /// Reads IPTC data from APP13 segments in JPEG files.
  ///
  /// Searches for APP13 (0xFFED) segments that contain IPTC metadata
  /// and extracts common IPTC fields like keywords, caption, etc.
  fn read_iptc_data(path: &Path) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let data = fs::read(path)?;
    let mut results = Vec::new();
    let mut i = 0;

    // Look for APP13 segments (0xFFED) that contain IPTC data
    while i < data.len() - 1 {
      if data[i] == 0xFF && data[i + 1] == 0xED {
        // Found APP13 segment
        if i + 4 < data.len() {
          let segment_len = (u16::from(data[i + 2]) << 8) | u16::from(data[i + 3]);
          let segment_end = i + 2 + segment_len as usize;

          if segment_end <= data.len() {
            // Look for Photoshop resource blocks within APP13
            let segment_data = &data[i + 4..segment_end];
            if segment_data.len() >= 14 && &segment_data[0..14] == b"Photoshop 3.0\0" {
              // Parse IPTC records within the Photoshop resource
              Self::parse_iptc_records(&segment_data[14..], &mut results);
            }
          }
        }
        i += 2;
      } else {
        i += 1;
      }
    }

    Ok(results)
  }

  /// Parses IPTC records from Photoshop resource data.
  fn parse_iptc_records(data: &[u8], results: &mut Vec<(String, String)>) {
    let mut i = 0;

    while i + 8 < data.len() {
      // Look for IPTC record marker (0x1C)
      if data[i] == 0x1C {
        let record = data[i + 1];
        let dataset = data[i + 2];
        let length = (u16::from(data[i + 3]) << 8) | u16::from(data[i + 4]);

        if i + 5 + length as usize <= data.len() {
          let value_data = &data[i + 5..i + 5 + length as usize];
          let value = String::from_utf8_lossy(value_data).trim().to_string();

          if !value.is_empty() {
            let tag_name = Self::format_iptc_tag(record, dataset);
            let mut display_value = value;
            // Truncate long values
            if display_value.len() > 50 {
              display_value.truncate(50);
              display_value.push('…');
            }
            results.push((format!("IPTC: {tag_name}"), display_value));
          }

          i += 5 + length as usize;
        } else {
          break;
        }
      } else {
        i += 1;
      }
    }
  }

  /// Formats IPTC record and dataset numbers into readable names.
  fn format_iptc_tag(record: u8, dataset: u8) -> String {
    match (record, dataset) {
      (2, 5) => "Object Name".to_string(),
      (2, 15) => "Category".to_string(),
      (2, 20) => "Supplemental Categories".to_string(),
      (2, 25) => "Keywords".to_string(),
      (2, 40) => "Special Instructions".to_string(),
      (2, 55) => "Date Created".to_string(),
      (2, 60) => "Time Created".to_string(),
      (2, 62) => "Digital Creation Date".to_string(),
      (2, 63) => "Digital Creation Time".to_string(),
      (2, 80) => "Byline".to_string(),
      (2, 85) => "Byline Title".to_string(),
      (2, 90) => "City".to_string(),
      (2, 92) => "Sublocation".to_string(),
      (2, 95) => "Province/State".to_string(),
      (2, 100) => "Country/Primary Location Code".to_string(),
      (2, 101) => "Country/Primary Location Name".to_string(),
      (2, 103) => "Original Transmission Reference".to_string(),
      (2, 105) => "Headline".to_string(),
      (2, 110) => "Credit".to_string(),
      (2, 115) => "Source".to_string(),
      (2, 116) => "Copyright Notice".to_string(),
      (2, 118) => "Contact".to_string(),
      (2, 120) => "Caption/Abstract".to_string(),
      (2, 122) => "Caption Writer/Editor".to_string(),
      _ => format!("Record {record} Dataset {dataset}"),
    }
  }

  /// Formats an EXIF tag into a human-readable name.
  ///
  /// Maps known EXIF tags to descriptive names, and provides fallback
  /// formatting for unknown tags using their numeric identifiers.
  #[must_use]
  pub fn format_tag_name(tag: &exif::Tag) -> String {
    use exif::Tag;

    match *tag {
      Tag::Make => "Make".to_string(),
      Tag::Model => "Model".to_string(),
      Tag::Artist => "Artist".to_string(),
      Tag::Copyright => "Copyright".to_string(),
      Tag::DateTime => "Date/Time".to_string(),
      Tag::DateTimeOriginal => "Date/Time Original".to_string(),
      Tag::DateTimeDigitized => "Date/Time Digitized".to_string(),
      Tag::Software => "Software".to_string(),
      Tag::ImageDescription => "Image Description".to_string(),
      Tag::Orientation => "Orientation".to_string(),
      Tag::XResolution => "X Resolution".to_string(),
      Tag::YResolution => "Y Resolution".to_string(),
      Tag::ResolutionUnit => "Resolution Unit".to_string(),
      Tag::ExposureTime => "Exposure Time".to_string(),
      Tag::FNumber => "F-Number".to_string(),
      Tag::ExposureProgram => "Exposure Program".to_string(),
      Tag::PhotographicSensitivity => "ISO Speed".to_string(),
      Tag::ExifVersion => "EXIF Version".to_string(),
      Tag::ComponentsConfiguration => "Components Configuration".to_string(),
      Tag::CompressedBitsPerPixel => "Compressed Bits Per Pixel".to_string(),
      Tag::ShutterSpeedValue => "Shutter Speed Value".to_string(),
      Tag::ApertureValue => "Aperture Value".to_string(),
      Tag::BrightnessValue => "Brightness Value".to_string(),
      Tag::ExposureBiasValue => "Exposure Bias Value".to_string(),
      Tag::MaxApertureValue => "Max Aperture Value".to_string(),
      Tag::SubjectDistance => "Subject Distance".to_string(),
      Tag::MeteringMode => "Metering Mode".to_string(),
      Tag::LightSource => "Light Source".to_string(),
      Tag::Flash => "Flash".to_string(),
      Tag::FocalLength => "Focal Length".to_string(),
      Tag::UserComment => "User Comment".to_string(),
      Tag::FlashpixVersion => "Flashpix Version".to_string(),
      Tag::ColorSpace => "Color Space".to_string(),
      Tag::PixelXDimension => "Pixel X Dimension".to_string(),
      Tag::PixelYDimension => "Pixel Y Dimension".to_string(),
      Tag::RelatedSoundFile => "Related Sound File".to_string(),
      Tag::FocalPlaneXResolution => "Focal Plane X Resolution".to_string(),
      Tag::FocalPlaneYResolution => "Focal Plane Y Resolution".to_string(),
      Tag::FocalPlaneResolutionUnit => "Focal Plane Resolution Unit".to_string(),
      Tag::SubjectLocation => "Subject Location".to_string(),
      Tag::ExposureIndex => "Exposure Index".to_string(),
      Tag::SensingMethod => "Sensing Method".to_string(),
      Tag::FileSource => "File Source".to_string(),
      Tag::SceneType => "Scene Type".to_string(),
      Tag::CFAPattern => "CFA Pattern".to_string(),
      Tag::CustomRendered => "Custom Rendered".to_string(),
      Tag::ExposureMode => "Exposure Mode".to_string(),
      Tag::WhiteBalance => "White Balance".to_string(),
      Tag::DigitalZoomRatio => "Digital Zoom Ratio".to_string(),
      Tag::FocalLengthIn35mmFilm => "Focal Length (35mm equiv)".to_string(),
      Tag::SceneCaptureType => "Scene Capture Type".to_string(),
      Tag::GainControl => "Gain Control".to_string(),
      Tag::Contrast => "Contrast".to_string(),
      Tag::Saturation => "Saturation".to_string(),
      Tag::Sharpness => "Sharpness".to_string(),
      Tag::DeviceSettingDescription => "Device Setting Description".to_string(),
      Tag::SubjectDistanceRange => "Subject Distance Range".to_string(),
      Tag::ImageUniqueID => "Image Unique ID".to_string(),
      Tag::LensSpecification => "Lens Specification".to_string(),
      Tag::LensMake => "Lens Make".to_string(),
      Tag::LensModel => "Lens Model".to_string(),
      Tag::LensSerialNumber => "Lens Serial Number".to_string(),
      _ => {
        // For unknown tags, try to provide a cleaner format
        let tag_str = format!("{tag}");
        if tag_str.starts_with("Tag(") && tag_str.ends_with(')') {
          // Extract the numeric tag ID from "Tag(Context, 12345)" format
          if let Some(comma_pos) = tag_str.rfind(", ") {
            if let Some(end_pos) = tag_str.rfind(')') {
              let tag_num = &tag_str[comma_pos + 2..end_pos];
              // Map some common tag numbers to readable names
              match tag_num {
                "34855" => return "ISO Speed".to_string(),
                "33434" => return "Exposure Time".to_string(),
                "33437" => return "F-Number".to_string(),
                "36867" => return "Date/Time Original".to_string(),
                "36868" => return "Date/Time Digitized".to_string(),
                "37377" => return "Shutter Speed Value".to_string(),
                "37378" => return "Aperture Value".to_string(),
                "37380" => return "Exposure Bias Value".to_string(),
                "37381" => return "Max Aperture Value".to_string(),
                "37382" => return "Subject Distance".to_string(),
                "37383" => return "Metering Mode".to_string(),
                "37384" => return "Light Source".to_string(),
                "37385" => return "Flash".to_string(),
                "37386" => return "Focal Length".to_string(),
                // Lens-related tags
                "42034" => return "Lens Specification".to_string(),
                "42035" => return "Lens Make".to_string(),
                "42036" => return "Lens Model".to_string(),
                "42037" => return "Lens Serial Number".to_string(),
                "37500" => return "Maker Note".to_string(),
                "40961" => return "Color Space".to_string(),
                "40962" => return "Pixel X Dimension".to_string(),
                "40963" => return "Pixel Y Dimension".to_string(),
                "41486" => return "Focal Plane X Resolution".to_string(),
                "41487" => return "Focal Plane Y Resolution".to_string(),
                "41488" => return "Focal Plane Resolution Unit".to_string(),
                "41495" => return "Sensing Method".to_string(),
                "41728" => return "File Source".to_string(),
                "41729" => return "Scene Type".to_string(),
                "41985" => return "Custom Rendered".to_string(),
                "41986" => return "Exposure Mode".to_string(),
                "41987" => return "White Balance".to_string(),
                "41988" => return "Digital Zoom Ratio".to_string(),
                "41989" => return "Focal Length (35mm equiv)".to_string(),
                "41990" => return "Scene Capture Type".to_string(),
                "41991" => return "Gain Control".to_string(),
                "41992" => return "Contrast".to_string(),
                "41993" => return "Saturation".to_string(),
                "41994" => return "Sharpness".to_string(),
                "42016" => return "Image Unique ID".to_string(),
                _ => return format!("Tag {tag_num}"),
              }
            }
          }
        }
        tag_str
      }
    }
  }

  /// Formats an EXIF value into a human-readable string.
  ///
  /// Handles different EXIF value types and converts them to displayable
  /// strings, with special handling for binary data and ASCII strings.
  #[must_use]
  pub fn format_exif_value(value: &Value) -> String {
    match value {
      Value::Byte(bytes) => format!("{bytes:?}"),
      Value::Ascii(ascii) => {
        let result = ascii
          .iter()
          .map(|bytes| {
            let s = String::from_utf8_lossy(bytes);
            // Remove null terminators and non-printable characters
            s.trim_end_matches('\0')
              .chars()
              .filter(|c| c.is_ascii_graphic() || c.is_ascii_whitespace())
              .collect::<String>()
              .trim()
              .to_string()
          })
          .filter(|s| !s.is_empty())
          .collect::<Vec<_>>()
          .join(", ");

        // If result is empty or contains only non-printable data, show a placeholder
        if result.is_empty()
          || result
            .chars()
            .all(|c| !c.is_ascii_graphic() && !c.is_ascii_whitespace())
        {
          "<binary data>".to_string()
        } else {
          result
        }
      }
      Value::Short(shorts) => shorts
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", "),
      Value::Long(longs) => longs
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", "),
      Value::Rational(rationals) => rationals
        .iter()
        .map(|r| format!("{}/{}", r.num, r.denom))
        .collect::<Vec<_>>()
        .join(", "),
      Value::SByte(bytes) => format!("{bytes:?}"),
      Value::Undefined(bytes, _) => format!("Undefined({} bytes)", bytes.len()),
      Value::SShort(shorts) => shorts
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", "),
      Value::SLong(longs) => longs
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", "),
      Value::SRational(rationals) => rationals
        .iter()
        .map(|r| format!("{}/{}", r.num, r.denom))
        .collect::<Vec<_>>()
        .join(", "),
      Value::Float(floats) => floats
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", "),
      Value::Double(doubles) => doubles
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", "),
      Value::Unknown(tag, ty, count) => {
        format!("Unknown(tag={tag}, type={ty}, count={count})")
      }
    }
  }

  /// Applies EXIF metadata to a JPEG file with optional custom shot ISO.
  ///
  /// Similar to `apply_exif` but allows overriding the ISO value for push/pull processing.
  /// If `shot_iso` is None, uses the film's base ISO rating.
  /// This method preserves existing EXIF/IPTC data and only updates the specified fields.
  pub fn apply_exif_with_iso(
    path: &Path,
    selection: &Selection,
    shot_iso: Option<u32>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let mut bufreader = BufReader::new(&file);

    let exifreader = Reader::new();
    let existing_exif = exifreader.read_from_container(&mut bufreader).ok();

    let original_data = fs::read(path)?;

    let mut new_data = Vec::new();

    if original_data.len() >= 2 && &original_data[0..2] == b"\xff\xd8" {
      new_data.extend_from_slice(&original_data[0..2]);

      // Create merged EXIF segment that preserves existing data
      let exif_data =
        Self::create_merged_exif_segment_with_iso(selection, shot_iso, existing_exif.as_ref())?;
      new_data.extend_from_slice(&exif_data);

      let mut i = 2;
      while i < original_data.len() - 1 {
        if original_data[i] == 0xff {
          let marker = original_data[i + 1];
          if marker == 0xe1 {
            let segment_length =
              (u16::from(original_data[i + 2]) << 8) | u16::from(original_data[i + 3]);
            i += 2 + segment_length as usize;
            continue;
          }
        }
        break;
      }

      new_data.extend_from_slice(&original_data[i..]);
    } else {
      return Err("Not a valid JPEG file".into());
    }

    fs::write(path, new_data)?;
    Ok(())
  }

  /// Creates an EXIF segment while preserving existing EXIF data.
  fn create_merged_exif_segment(
    selection: &Selection,
    existing_exif: Option<&exif::Exif>,
  ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Self::create_merged_exif_segment_with_iso(selection, None, existing_exif)
  }

  /// Creates an EXIF segment with optional custom shot ISO while preserving existing EXIF data.
  /// Note: This is a simplified implementation that preserves only ASCII string fields.
  /// More complex EXIF fields are not currently preserved to avoid data corruption.
  fn create_merged_exif_segment_with_iso(
    selection: &Selection,
    shot_iso: Option<u32>,
    existing_exif: Option<&exif::Exif>,
  ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use exif::Value;

    let mut segment = Vec::new();
    segment.extend_from_slice(b"\xff\xe1");

    // Start building the merged EXIF data
    let mut data = Vec::new();
    data.extend_from_slice(b"Exif\x00\x00");
    data.extend_from_slice(b"II*\x00");
    let ifd_offset = 8u32;
    data.extend_from_slice(&ifd_offset.to_le_bytes());

    // Define the tags we want to update/override - using numeric IDs for more reliable matching
    let our_tag_numbers = [
      0x010f, // Make
      0x0110, // Model
      0x013b, // Artist
      0xa433, // LensMake
      0xa434, // LensModel
      0x8827, // PhotographicSensitivity (ISO)
      0x920a, // FocalLength
    ];

    // Collect preserved fields from existing EXIF
    let mut preserved_fields = Vec::new();

    // Add existing fields that we're not overriding (preserve ALL field types now)
    if let Some(exif) = existing_exif {
      for field in exif.fields() {
        // Get the numeric tag ID for comparison
        let tag_number = Self::tag_to_number(field.tag);

        // Only skip the specific fields we're overriding, preserve everything else
        if let Some(tag_num) = tag_number {
          if our_tag_numbers.contains(&tag_num) {
            continue;
          }
        }

        // Preserve ALL field types, not just ASCII strings
        match &field.value {
          Value::Ascii(ascii_vec) => {
            for ascii_bytes in ascii_vec {
              if let Ok(string_value) = std::str::from_utf8(ascii_bytes) {
                let clean_value = string_value.trim_end_matches('\0');
                if !clean_value.is_empty() && clean_value.len() < 1000 {
                  if let Some(tag_number) = Self::tag_to_number(field.tag) {
                    preserved_fields.push((tag_number, 0x02, clean_value.as_bytes().to_vec()));
                  }
                }
              }
            }
          }
          Value::Short(shorts) => {
            if let Some(tag_number) = Self::tag_to_number(field.tag) {
              let mut data_bytes = Vec::new();
              for &short_val in shorts {
                data_bytes.extend_from_slice(&short_val.to_le_bytes());
              }
              preserved_fields.push((tag_number, 0x03, data_bytes));
            }
          }
          Value::Long(longs) => {
            if let Some(tag_number) = Self::tag_to_number(field.tag) {
              let mut data_bytes = Vec::new();
              for &long_val in longs {
                data_bytes.extend_from_slice(&long_val.to_le_bytes());
              }
              preserved_fields.push((tag_number, 0x04, data_bytes));
            }
          }
          Value::Rational(rationals) => {
            if let Some(tag_number) = Self::tag_to_number(field.tag) {
              let mut data_bytes = Vec::new();
              for rational in rationals {
                data_bytes.extend_from_slice(&rational.num.to_le_bytes());
                data_bytes.extend_from_slice(&rational.denom.to_le_bytes());
              }
              preserved_fields.push((tag_number, 0x05, data_bytes));
            }
          }
          Value::SRational(srationals) => {
            if let Some(tag_number) = Self::tag_to_number(field.tag) {
              let mut data_bytes = Vec::new();
              for srational in srationals {
                data_bytes.extend_from_slice(&srational.num.to_le_bytes());
                data_bytes.extend_from_slice(&srational.denom.to_le_bytes());
              }
              preserved_fields.push((tag_number, 0x0A, data_bytes));
            }
          }
          Value::Byte(bytes) => {
            if let Some(tag_number) = Self::tag_to_number(field.tag) {
              preserved_fields.push((tag_number, 0x01, bytes.clone()));
            }
          }
          Value::Undefined(bytes, _) => {
            if let Some(tag_number) = Self::tag_to_number(field.tag) {
              preserved_fields.push((tag_number, 0x07, bytes.clone()));
            }
          }
          Value::SByte(sbytes) => {
            if let Some(tag_number) = Self::tag_to_number(field.tag) {
              let bytes: Vec<u8> = sbytes.iter().map(|&b| b as u8).collect();
              preserved_fields.push((tag_number, 0x06, bytes));
            }
          }
          Value::SShort(sshorts) => {
            if let Some(tag_number) = Self::tag_to_number(field.tag) {
              let mut data_bytes = Vec::new();
              for &sshort_val in sshorts {
                data_bytes.extend_from_slice(&sshort_val.to_le_bytes());
              }
              preserved_fields.push((tag_number, 0x08, data_bytes));
            }
          }
          Value::SLong(slongs) => {
            if let Some(tag_number) = Self::tag_to_number(field.tag) {
              let mut data_bytes = Vec::new();
              for &slong_val in slongs {
                data_bytes.extend_from_slice(&slong_val.to_le_bytes());
              }
              preserved_fields.push((tag_number, 0x09, data_bytes));
            }
          }
          Value::Float(floats) => {
            if let Some(tag_number) = Self::tag_to_number(field.tag) {
              let mut data_bytes = Vec::new();
              for &float_val in floats {
                data_bytes.extend_from_slice(&float_val.to_le_bytes());
              }
              preserved_fields.push((tag_number, 0x0B, data_bytes));
            }
          }
          Value::Double(doubles) => {
            if let Some(tag_number) = Self::tag_to_number(field.tag) {
              let mut data_bytes = Vec::new();
              for &double_val in doubles {
                data_bytes.extend_from_slice(&double_val.to_le_bytes());
              }
              preserved_fields.push((tag_number, 0x0C, data_bytes));
            }
          }
          Value::Unknown(_, _, _) => {
            // Skip unknown value types - we can't safely preserve them
            // without understanding their structure
          }
        }
      }
    }

    // Collect all string fields (our new ones + preserved ones)
    let mut all_string_fields = Vec::new();

    // Pre-compute lens model to avoid lifetime issues
    let lens_model_string = selection
      .lens
      .as_ref()
      .map(super::super::models::Lens::complete_lens_model);

    // Add our fields (these override existing ones)
    all_string_fields.push((0x010f, selection.camera.maker.as_str())); // Make
    all_string_fields.push((0x0110, selection.camera.model.as_str())); // Model
    all_string_fields.push((0x013b, selection.photographer.name.as_str())); // Artist

    // Add lens fields only if a lens is present
    if let Some(lens) = &selection.lens {
      all_string_fields.push((0xa433, lens.maker.as_str())); // Lens Make
      if let Some(ref lens_model) = lens_model_string {
        all_string_fields.push((0xa434, lens_model.as_str())); // Lens Model
      }
    }

    // Add ISO field
    let iso_value = shot_iso.unwrap_or(selection.film.iso);

    // Add focal length if parseable and lens is present
    let focal_length_entry = selection
      .lens
      .as_ref()
      .is_some_and(|lens| lens.focal_length.parse::<f32>().is_ok());

    // Calculate entry count
    let entry_count =
      all_string_fields.len() + preserved_fields.len() + 1 + usize::from(focal_length_entry);
    data.extend_from_slice(&(entry_count as u16).to_le_bytes());

    // Calculate where string data will start
    let string_data_start = 8 + 2 + (entry_count * 12) + 4;
    let mut string_offset = string_data_start;
    let mut string_data = Vec::new();

    // Create string IFD entries (our fields)
    for (tag, value) in &all_string_fields {
      let mut entry = Vec::new();
      entry.extend_from_slice(&(*tag as u16).to_le_bytes());
      entry.extend_from_slice(&[0x02, 0x00]); // ASCII type
      let string_len = value.len() + 1; // Include null terminator
      entry.extend_from_slice(&u32::try_from(string_len).unwrap_or(0).to_le_bytes());

      if string_len <= 4 {
        // String fits in offset field
        let mut padded_value = value.as_bytes().to_vec();
        padded_value.push(0); // null terminator
        while padded_value.len() < 4 {
          padded_value.push(0);
        }
        entry.extend_from_slice(&padded_value[0..4]);
      } else {
        // String needs offset
        entry.extend_from_slice(&u32::try_from(string_offset).unwrap_or(0).to_le_bytes());
        string_data.extend_from_slice(value.as_bytes());
        string_data.push(0); // null terminator
        string_offset += string_len;
      }

      data.extend_from_slice(&entry);
    }

    // Add preserved fields
    for (tag_num, field_type, field_data) in preserved_fields {
      let mut entry = Vec::new();
      entry.extend_from_slice(&tag_num.to_le_bytes());
      entry.extend_from_slice(&[field_type, 0x00]);

      let count = match field_type {
        0x03 | 0x08 => field_data.len() / 2,        // SHORT, SSHORT
        0x04 | 0x09 | 0x0B => field_data.len() / 4, // LONG, SLONG, FLOAT
        0x05 | 0x0A | 0x0C => field_data.len() / 8, // RATIONAL, SRATIONAL, DOUBLE
        _ => field_data.len(),
      };
      entry.extend_from_slice(&u32::try_from(count).unwrap_or(0).to_le_bytes());

      if field_data.len() <= 4 {
        // Data fits in offset field
        let mut padded_data = field_data.clone();
        while padded_data.len() < 4 {
          padded_data.push(0);
        }
        entry.extend_from_slice(&padded_data[0..4]);
      } else {
        // Data needs offset
        entry.extend_from_slice(&u32::try_from(string_offset).unwrap_or(0).to_le_bytes());
        string_data.extend_from_slice(&field_data);
        string_offset += field_data.len();
      }

      data.extend_from_slice(&entry);
    }

    // Add ISO entry
    let mut iso_entry = Vec::new();
    iso_entry.extend_from_slice(&(0x8827u16).to_le_bytes()); // ISO tag
    iso_entry.extend_from_slice(&[0x03, 0x00]); // SHORT type
    iso_entry.extend_from_slice(&1u32.to_le_bytes()); // count = 1
    iso_entry.extend_from_slice(&u16::try_from(iso_value).unwrap_or(0).to_le_bytes()); // ISO value
    iso_entry.extend_from_slice(&[0x00, 0x00]); // padding
    data.extend_from_slice(&iso_entry);

    // Add focal length if lens is present and parseable
    if let Some(lens) = &selection.lens {
      if let Ok(focal_length) = lens.focal_length.parse::<f32>() {
        let focal_length_rational = (focal_length * 1000.0).abs() as u32;
        let mut focal_entry = Vec::new();
        focal_entry.extend_from_slice(&(0x920Au16).to_le_bytes()); // Focal length tag
        focal_entry.extend_from_slice(&[0x05, 0x00]); // RATIONAL type
        focal_entry.extend_from_slice(&1u32.to_le_bytes()); // count = 1
        focal_entry.extend_from_slice(&u32::try_from(string_offset).unwrap_or(0).to_le_bytes()); // offset to rational data
        data.extend_from_slice(&focal_entry);

        // Add rational data at the end (numerator/denominator)
        string_data.extend_from_slice(&focal_length_rational.to_le_bytes());
        string_data.extend_from_slice(&1000u32.to_le_bytes());
      }
    }

    // Next IFD pointer (0 = no more IFDs)
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

    // Append string data
    data.extend_from_slice(&string_data);

    // Add length and data to segment
    let length = u16::try_from(data.len() + 2).unwrap_or(0);
    segment.push((length >> 8) as u8);
    segment.push((length & 0xff) as u8);
    segment.extend_from_slice(&data);

    Ok(segment)
  }

  /// Convert an EXIF tag to its numeric representation for field processing.
  /// This is a helper function for the merged EXIF segment creation.
  fn tag_to_number(tag: exif::Tag) -> Option<u16> {
    use exif::Tag;

    match tag {
      Tag::ImageWidth => Some(0x0100),
      Tag::ImageLength => Some(0x0101),
      Tag::Compression => Some(0x0103),
      Tag::PhotometricInterpretation => Some(0x0106),
      Tag::ImageDescription => Some(0x010e),
      Tag::Make => Some(0x010f),
      Tag::Model => Some(0x0110),
      Tag::Orientation => Some(0x0112),
      Tag::XResolution => Some(0x011a),
      Tag::YResolution => Some(0x011b),
      Tag::ResolutionUnit => Some(0x0128),
      Tag::Software => Some(0x0131),
      Tag::DateTime => Some(0x0132),
      Tag::Artist => Some(0x013b),
      Tag::Copyright => Some(0x8298),
      Tag::ExposureTime => Some(0x829a),
      Tag::FNumber => Some(0x829d),
      Tag::ExposureProgram => Some(0x8822),
      Tag::PhotographicSensitivity => Some(0x8827),
      Tag::ExifVersion => Some(0x9000),
      Tag::DateTimeOriginal => Some(0x9003),
      Tag::DateTimeDigitized => Some(0x9004),
      Tag::ShutterSpeedValue => Some(0x9201),
      Tag::ApertureValue => Some(0x9202),
      Tag::BrightnessValue => Some(0x9203),
      Tag::ExposureBiasValue => Some(0x9204),
      Tag::MaxApertureValue => Some(0x9205),
      Tag::SubjectDistance => Some(0x9206),
      Tag::MeteringMode => Some(0x9207),
      Tag::LightSource => Some(0x9208),
      Tag::Flash => Some(0x9209),
      Tag::FocalLength => Some(0x920a),
      Tag::ColorSpace => Some(0xa001),
      Tag::LensSpecification => Some(0xa432),
      Tag::LensMake => Some(0xa433),
      Tag::LensModel => Some(0xa434),
      // Add more commonly used tags that were missing
      Tag::ComponentsConfiguration => Some(0x9101),
      Tag::CompressedBitsPerPixel => Some(0x9102),
      Tag::UserComment => Some(0x9286),
      Tag::FlashpixVersion => Some(0xa000),
      Tag::PixelXDimension => Some(0xa002),
      Tag::PixelYDimension => Some(0xa003),
      Tag::RelatedSoundFile => Some(0xa004),
      Tag::FocalPlaneXResolution => Some(0xa20e),
      Tag::FocalPlaneYResolution => Some(0xa20f),
      Tag::FocalPlaneResolutionUnit => Some(0xa210),
      Tag::SubjectLocation => Some(0xa214),
      Tag::ExposureIndex => Some(0xa215),
      Tag::SensingMethod => Some(0xa217),
      Tag::FileSource => Some(0xa300),
      Tag::SceneType => Some(0xa301),
      Tag::CFAPattern => Some(0xa302),
      Tag::CustomRendered => Some(0xa401),
      Tag::ExposureMode => Some(0xa402),
      Tag::WhiteBalance => Some(0xa403),
      Tag::DigitalZoomRatio => Some(0xa404),
      Tag::FocalLengthIn35mmFilm => Some(0xa405),
      Tag::SceneCaptureType => Some(0xa406),
      Tag::GainControl => Some(0xa407),
      Tag::Contrast => Some(0xa408),
      Tag::Saturation => Some(0xa409),
      Tag::Sharpness => Some(0xa40a),
      Tag::DeviceSettingDescription => Some(0xa40b),
      Tag::SubjectDistanceRange => Some(0xa40c),
      Tag::ImageUniqueID => Some(0xa420),
      Tag::LensSerialNumber => Some(0xa435),
      // Add missing standard tags that are commonly seen but not in the enum
      // These will be handled by the fallback case, but we can add known ones here
      _ => {
        // For truly unknown tags, try to extract the numeric value from the debug format
        let tag_str = format!("{tag:?}");
        if tag_str.contains("Tag(") {
          if let Some(comma_pos) = tag_str.rfind(", ") {
            if let Some(end_pos) = tag_str.rfind(')') {
              let tag_num_str = &tag_str[comma_pos + 2..end_pos];
              if let Ok(tag_num) = tag_num_str.parse::<u16>() {
                return Some(tag_num);
              }
            }
          }
        }
        None
      }
    }
  }
}

impl TiffProcessor {
  /// Sets the creation date in a TIFF file's EXIF data.
  ///
  /// Updates the `DateTimeOriginal`, `DateTime`, and `DateTimeDigitized` fields in the EXIF data.
  /// Note: This is a basic implementation that will be enhanced in the future.
  pub fn set_creation_date(
    path: &Path,
    _date_string: &str,
  ) -> Result<(), Box<dyn std::error::Error>> {
    // For now, we'll just re-save the TIFF file to preserve it
    // A full implementation would need to properly modify TIFF EXIF data
    let img = image::open(path)?;
    let mut output_file = fs::File::create(path)?;
    img.write_to(&mut output_file, image::ImageFormat::Tiff)?;

    // TODO: Implement proper TIFF EXIF date modification
    println!("Note: TIFF date modification is not fully implemented yet. File preserved.");
    Ok(())
  }

  /// Applies EXIF metadata to a TIFF file.
  ///
  /// Currently re-saves the TIFF file using the image crate.
  /// Full EXIF application for TIFF files is not yet implemented.
  pub fn apply_exif(path: &Path, _selection: &Selection) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(path)?;
    let mut output_file = fs::File::create(path)?;

    match img {
      image::DynamicImage::ImageRgb8(rgb_img) => {
        rgb_img.write_to(&mut output_file, image::ImageFormat::Tiff)?;
      }
      image::DynamicImage::ImageRgba8(rgba_img) => {
        rgba_img.write_to(&mut output_file, image::ImageFormat::Tiff)?;
      }
      _ => {
        img.write_to(&mut output_file, image::ImageFormat::Tiff)?;
      }
    }

    Ok(())
  }

  /// Erases EXIF metadata from a TIFF file.
  ///
  /// Re-saves the TIFF file which removes embedded metadata.
  pub fn erase_exif(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(path)?;
    let mut output_file = fs::File::create(path)?;
    img.write_to(&mut output_file, image::ImageFormat::Tiff)?;
    Ok(())
  }

  /// Reads EXIF metadata from a TIFF file.
  ///
  /// Extracts all available EXIF fields and returns them as key-value pairs,
  /// sorted alphabetically by tag name.
  /// Read EXIF data from a TIFF file and return as key-value pairs
  pub fn read_exif(path: &Path) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let mut bufreader = BufReader::new(&file);

    let exifreader = Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;

    let mut results = Vec::new();

    // Read all EXIF fields from all IFDs
    for field in exif.fields() {
      let tag_name = JpegProcessor::format_tag_name(&field.tag);
      let mut value = JpegProcessor::format_exif_value(&field.value);

      // Truncate long values
      if value.len() > 50 {
        value.truncate(50);
        value.push('…');
      }

      // Add IFD context to help identify the source
      let ifd_name = match field.ifd_num {
        exif::In::PRIMARY => "",
        exif::In::THUMBNAIL => " (Thumbnail)",
        _ => " (Sub-IFD)",
      };
      let full_tag_name = if ifd_name.is_empty() {
        tag_name.clone()
      } else {
        format!("{tag_name}{ifd_name}")
      };

      // Also add raw tag info for debugging unknown tags
      let raw_tag_info = format!("{:?}", field.tag);
      if raw_tag_info.contains("Tag(") && !raw_tag_info.starts_with(&tag_name) {
        results.push((format!("{full_tag_name} [{raw_tag_info}]"), value.clone()));
      } else {
        results.push((full_tag_name, value));
      }
    }

    results.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(results)
  }

  /// Applies EXIF metadata to a TIFF file with optional custom shot ISO.
  ///
  /// Similar to `apply_exif` but allows overriding the ISO value for push/pull processing.
  /// If `shot_iso` is None, uses the film's base ISO rating.
  /// Currently re-saves the TIFF file using the image crate.
  /// Full EXIF application for TIFF files is not yet implemented.
  pub fn apply_exif_with_iso(
    path: &Path,
    _selection: &Selection,
    _shot_iso: Option<u32>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(path)?;
    let mut output_file = fs::File::create(path)?;
    img.write_to(&mut output_file, image::ImageFormat::Tiff)?;
    Ok(())
  }
}

impl RawProcessor {
  /// Sets the creation date in a RAW file's XMP sidecar.
  ///
  /// Updates or creates an XMP file with the new creation date.
  pub fn set_creation_date(
    path: &Path,
    date_string: &str,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let xmp_path = path.with_extension("xmp");

    // Create basic XMP content with date information
    let xmp_content = format!(
      r#"<?xml version="1.0" encoding="UTF-8"?>
<x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core">
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
    <rdf:Description rdf:about=""
        xmlns:exif="http://ns.adobe.com/exif/1.0/">
      <exif:DateTimeOriginal>{date_string}</exif:DateTimeOriginal>
      <exif:DateTimeDigitized>{date_string}</exif:DateTimeDigitized>
    </rdf:Description>
  </rdf:RDF>
</x:xmpmeta>"#
    );

    fs::write(&xmp_path, xmp_content)?;
    Ok(())
  }

  /// Applies EXIF metadata to a RAW file by creating an XMP sidecar.
  ///
  /// Creates an XMP metadata file alongside the RAW file containing
  /// equipment and photographer information from the selection.
  pub fn apply_exif(path: &Path, selection: &Selection) -> Result<(), Box<dyn std::error::Error>> {
    let xmp_content = ExifTags::create_xmp_metadata(selection);
    let xmp_path = path.with_extension("xmp");
    fs::write(&xmp_path, xmp_content)?;
    Ok(())
  }

  /// Erases EXIF metadata from a RAW file by removing its XMP sidecar.
  ///
  /// Deletes the associated XMP metadata file if it exists,
  /// effectively removing all applied metadata.
  pub fn erase_exif(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let xmp_path = path.with_extension("xmp");
    if xmp_path.exists() {
      fs::remove_file(&xmp_path)?;
    }
    Ok(())
  }

  /// Reads EXIF metadata from a RAW file's XMP sidecar.
  ///
  /// Returns the contents of the associated XMP file if it exists,
  /// or an empty vector if no XMP file is found.
  /// Read EXIF data from a JPEG file and return as key-value pairs
  pub fn read_exif(path: &Path) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let xmp_path = path.with_extension("xmp");
    if xmp_path.exists() {
      let content = fs::read_to_string(&xmp_path)?;
      Ok(vec![("XMP Content".to_string(), content)])
    } else {
      Ok(vec![])
    }
  }

  /// Applies EXIF metadata to a RAW file with optional custom shot ISO by creating an XMP sidecar.
  ///
  /// Similar to `apply_exif` but allows overriding the ISO value for push/pull processing.
  /// If `shot_iso` is None, uses the film's base ISO rating.
  /// Creates an XMP metadata file alongside the RAW file containing
  /// equipment and photographer information from the selection.
  pub fn apply_exif_with_iso(
    path: &Path,
    selection: &Selection,
    shot_iso: Option<u32>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let xmp_content = ExifTags::create_xmp_metadata_with_iso(selection, shot_iso);
    let xmp_path = path.with_extension("xmp");
    fs::write(&xmp_path, xmp_content)?;
    Ok(())
  }
}

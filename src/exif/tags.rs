//! EXIF tag processing and metadata generation utilities.
//!
//! This module provides functionality for converting equipment selections into
//! various metadata formats including EXIF tag mappings and XMP metadata structures.

use crate::models::Selection;
use std::collections::HashMap;

/// Utility struct for converting equipment selections to EXIF metadata formats.
///
/// Provides methods for generating EXIF tag mappings, XMP metadata, and
/// extracting specific tag values from equipment selections.
pub struct ExifTags;

impl ExifTags {
  /// Creates a `HashMap` of EXIF tags from an equipment selection.
  ///
  /// Converts the equipment selection into standard EXIF tag names and values,
  /// including camera make/model, lens information, film ISO, and photographer name.
  ///
  /// Returns a `HashMap` where keys are EXIF tag names and values are the corresponding data.
  #[must_use]
  pub fn create_exif_object(selection: &Selection) -> HashMap<String, String> {
    let mut exif_data = HashMap::new();

    exif_data.insert("Make".to_string(), selection.camera.maker.clone());
    exif_data.insert("Model".to_string(), selection.camera.model.clone());

    // Only add lens data if a lens is present
    if let Some(lens) = &selection.lens {
      exif_data.insert("LensMake".to_string(), lens.maker.clone());
      exif_data.insert("LensModel".to_string(), lens.lens_model_with_aperture());
      exif_data.insert("FocalLength".to_string(), lens.focal_length.clone());
      exif_data.insert("FNumber".to_string(), lens.aperture.clone());
    }

    exif_data.insert(
      "ISOSpeedRatings".to_string(),
      selection.film.iso.to_string(),
    );
    exif_data.insert("ISOSpeed".to_string(), selection.film.iso.to_string());
    exif_data.insert("Artist".to_string(), selection.photographer.name.clone());

    exif_data
  }

  /// Creates a `HashMap` of EXIF tags from an equipment selection with custom shot ISO.
  ///
  /// Similar to `create_exif_object` but allows overriding the ISO value for push/pull processing.
  /// If `shot_iso` is None, uses the film's base ISO rating.
  ///
  /// Returns a `HashMap` where keys are EXIF tag names and values are the corresponding data.
  #[must_use]
  pub fn create_exif_object_with_iso(
    selection: &Selection,
    shot_iso: Option<u32>,
  ) -> HashMap<String, String> {
    let mut exif_data = HashMap::new();

    exif_data.insert("Make".to_string(), selection.camera.maker.clone());
    exif_data.insert("Model".to_string(), selection.camera.model.clone());

    // Only add lens data if a lens is present
    if let Some(lens) = &selection.lens {
      exif_data.insert("LensMake".to_string(), lens.maker.clone());
      exif_data.insert("LensModel".to_string(), lens.lens_model_with_aperture());
      exif_data.insert("FocalLength".to_string(), lens.focal_length.clone());
      exif_data.insert("FNumber".to_string(), lens.aperture.clone());
    }

    // ISOSpeedRatings always uses the film's base ISO rating
    exif_data.insert(
      "ISOSpeedRatings".to_string(),
      selection.film.iso.to_string(),
    );
    // ISOSpeed uses the actual photographed ISO (shot_iso if provided, otherwise film ISO)
    let photographed_iso = shot_iso.unwrap_or(selection.film.iso);
    exif_data.insert("ISOSpeed".to_string(), photographed_iso.to_string());
    exif_data.insert("Artist".to_string(), selection.photographer.name.clone());

    exif_data
  }

  /// Gets the value for a specific EXIF tag from an equipment selection.
  ///
  /// Looks up the requested tag name and returns the corresponding value
  /// from the equipment selection, or None if the tag is not supported.
  ///
  /// Supported tags include Make, Model, `LensMake`, `LensModel`, `FocalLength`,
  /// `FNumber`, `ISOSpeedRatings`, `ISOSpeed`, and Artist.
  #[must_use]
  pub fn get_tag_value(tag: &str, selection: &Selection) -> Option<String> {
    match tag {
      "Make" => Some(selection.camera.maker.clone()),
      "Model" => Some(selection.camera.model.clone()),
      "LensMake" => selection.lens.as_ref().map(|lens| lens.maker.clone()),
      "LensModel" => selection
        .lens
        .as_ref()
        .map(super::super::models::Lens::lens_model_with_aperture),
      "FocalLength" => selection
        .lens
        .as_ref()
        .map(|lens| lens.focal_length.clone()),
      "FNumber" => selection.lens.as_ref().map(|lens| lens.aperture.clone()),
      "ISOSpeedRatings" | "ISOSpeed" => Some(selection.film.iso.to_string()),
      "Artist" => Some(selection.photographer.name.clone()),
      _ => None,
    }
  }

  /// Gets the value for a specific EXIF tag with optional custom shot ISO.
  ///
  /// Similar to `get_tag_value` but allows overriding the ISO value for push/pull processing.
  /// If `shot_iso` is None, uses the film's base ISO rating.
  #[must_use]
  pub fn get_tag_value_with_iso(
    tag: &str,
    selection: &Selection,
    shot_iso: Option<u32>,
  ) -> Option<String> {
    match tag {
      "Make" => Some(selection.camera.maker.clone()),
      "Model" => Some(selection.camera.model.clone()),
      "LensMake" => selection.lens.as_ref().map(|lens| lens.maker.clone()),
      "LensModel" => selection
        .lens
        .as_ref()
        .map(super::super::models::Lens::lens_model_with_aperture),
      "FocalLength" => selection
        .lens
        .as_ref()
        .map(|lens| lens.focal_length.clone()),
      "FNumber" => selection.lens.as_ref().map(|lens| lens.aperture.clone()),
      "ISOSpeedRatings" | "ISOSpeed" => {
        let iso_value = shot_iso.unwrap_or(selection.film.iso);
        Some(iso_value.to_string())
      }
      "Artist" => Some(selection.photographer.name.clone()),
      _ => None,
    }
  }

  /// Creates XMP metadata XML from an equipment selection.
  ///
  /// Generates a complete XMP metadata structure containing camera, lens,
  /// film, and photographer information formatted as XML for use with
  /// RAW image files as sidecar metadata.
  ///
  /// The XMP follows Adobe's metadata standards and includes appropriate
  /// namespaces for TIFF, EXIF, Dublin Core, and auxiliary data.
  #[must_use]
  pub fn create_xmp_metadata(selection: &Selection) -> String {
    let lens_metadata = if let Some(lens) = &selection.lens {
      format!(
        r"      <aux:LensModel>{}</aux:LensModel>
      <exif:FocalLength>{}</exif:FocalLength>
      <exif:FNumber>{}</exif:FNumber>",
        lens.lens_model_with_aperture(),
        lens.focal_length,
        lens.aperture
      )
    } else {
      String::new()
    };

    format!(
      r#"<?xml version="1.0" encoding="UTF-8"?>
<x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core">
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
    <rdf:Description rdf:about=""
        xmlns:tiff="http://ns.adobe.com/tiff/1.0/"
        xmlns:exif="http://ns.adobe.com/exif/1.0/"
        xmlns:dc="http://purl.org/dc/elements/1.1/"
        xmlns:aux="http://ns.adobe.com/exif/1.0/aux/">
      <tiff:Make>{}</tiff:Make>
      <tiff:Model>{}</tiff:Model>
{}
      <exif:ISOSpeedRatings>
        <rdf:Bag>
          <rdf:li>{}</rdf:li>
        </rdf:Bag>
      </exif:ISOSpeedRatings>
      <dc:creator>
        <rdf:Bag>
          <rdf:li>{}</rdf:li>
        </rdf:Bag>
      </dc:creator>
    </rdf:Description>
  </rdf:RDF>
</x:xmpmeta>"#,
      selection.camera.maker,
      selection.camera.model,
      lens_metadata,
      selection.film.iso,
      selection.photographer.name
    )
  }

  /// Creates XMP metadata XML from an equipment selection with custom shot ISO.
  ///
  /// Similar to `create_xmp_metadata` but allows overriding the ISO value for push/pull processing.
  /// If `shot_iso` is None, uses the film's base ISO rating.
  #[must_use]
  pub fn create_xmp_metadata_with_iso(selection: &Selection, shot_iso: Option<u32>) -> String {
    let iso_value = shot_iso.unwrap_or(selection.film.iso);
    let lens_metadata = if let Some(lens) = &selection.lens {
      format!(
        r"      <aux:LensModel>{}</aux:LensModel>
      <exif:FocalLength>{}</exif:FocalLength>
      <exif:FNumber>{}</exif:FNumber>",
        lens.lens_model_with_aperture(),
        lens.focal_length,
        lens.aperture
      )
    } else {
      String::new()
    };

    format!(
      r#"<?xml version="1.0" encoding="UTF-8"?>
<x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core">
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
    <rdf:Description rdf:about=""
        xmlns:tiff="http://ns.adobe.com/tiff/1.0/"
        xmlns:exif="http://ns.adobe.com/exif/1.0/"
        xmlns:dc="http://purl.org/dc/elements/1.1/"
        xmlns:aux="http://ns.adobe.com/exif/1.0/aux/">
      <tiff:Make>{}</tiff:Make>
      <tiff:Model>{}</tiff:Model>
{}
      <exif:ISOSpeedRatings>
        <rdf:Bag>
          <rdf:li>{}</rdf:li>
        </rdf:Bag>
      </exif:ISOSpeedRatings>
      <dc:creator>
        <rdf:Bag>
          <rdf:li>{}</rdf:li>
        </rdf:Bag>
      </dc:creator>
    </rdf:Description>
  </rdf:RDF>
</x:xmpmeta>"#,
      selection.camera.maker,
      selection.camera.model,
      lens_metadata,
      iso_value,
      selection.photographer.name
    )
  }
}

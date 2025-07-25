//! Configuration management for IFEX application settings and persistent data.
//!
//! This module handles loading, saving, and managing the application's configuration
//! data including cameras, lenses, films, photographers, and equipment setups.

use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration containing all photography equipment data.
///
/// This structure holds collections of all photography equipment and photographer
/// information that can be used to apply EXIF data to images.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
  /// List of cameras available for EXIF metadata
  pub cameras: Vec<crate::models::Camera>,
  /// List of lenses available for EXIF metadata
  pub lenses: Vec<crate::models::Lens>,
  /// List of film stocks available for EXIF metadata
  pub films: Vec<crate::models::Film>,
  /// List of photographers available for EXIF metadata
  pub photographers: Vec<crate::models::Photographer>,
  /// List of equipment setups (camera + lens combinations)
  pub setups: Vec<crate::models::Setup>,
}

impl Config {
  /// Returns the path to the configuration file.
  ///
  /// The configuration file is stored in the user's config directory as "ifex.json".
  /// On macOS this is typically `~/Library/Application Support/ifex.json`.
  pub fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = config_dir().ok_or("Could not find config directory")?;
    Ok(config_dir.join("ifex.json"))
  }

  /// Loads the configuration from the config file.
  ///
  /// If the config file doesn't exist, returns a default empty configuration.
  /// Otherwise, deserializes the JSON content into a Config struct.
  pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
    let path = Self::config_path()?;

    if !path.exists() {
      return Ok(Self::default());
    }

    let content = fs::read_to_string(&path)?;
    let config: Self = serde_json::from_str(&content)?;
    Ok(config)
  }

  /// Saves the current configuration to the config file.
  ///
  /// Creates the parent directory if it doesn't exist, then serializes
  /// the configuration to pretty-printed JSON and writes it to disk.
  pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
    let path = Self::config_path()?;

    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(self)?;
    fs::write(&path, content)?;
    Ok(())
  }
}

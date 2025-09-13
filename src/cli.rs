//! Command-line interface definitions and parsing

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Main CLI structure for parsing command-line arguments
#[derive(Parser)]
#[command(
    name = "ifex",
    version = env!("CARGO_PKG_VERSION"),
    about = "A modern CLI tool for managing EXIF data in JPEG, TIFF, DNG and RAW image files with structured equipment management",
    long_about = None
)]
pub struct Cli {
  /// Optional subcommand to execute
  #[command(subcommand)]
  pub command: Option<Commands>,

  /// Enable automatic creation date adjustment with 1-second increments for photos with identical timestamps
  #[arg(long = "one-sec")]
  pub one_sec: bool,
}

/// Available CLI commands
#[derive(Subcommand)]
pub enum Commands {
  /// Interactive mode to apply or erase EXIF data
  Run,
  /// Manage cameras, lenses, films, and setups
  Manage,
  /// Read and display EXIF data from an image file in a formatted table
  Read {
    /// Path to the image file
    file: PathBuf,
    /// Output EXIF data in JSON format
    #[arg(long = "json")]
    json: bool,
  },
}

impl Cli {
  /// Parse command-line arguments and return a Cli instance
  #[must_use]
  pub fn parse_args() -> Self {
    Self::parse()
  }
}

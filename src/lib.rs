//! IFEX - EXIF Data Manager
//!
//! A modern CLI tool for managing EXIF data in JPEG, TIFF, DNG, and RAW image files
//! with structured equipment management. Built with Rust for performance and reliability.

/// Command-line interface module
pub mod cli;
/// Configuration management module
pub mod config;
/// Data management and persistence module
pub mod data;
/// EXIF processing and manipulation module
pub mod exif;
/// Interactive user interface module
pub mod interface;
/// Data model definitions module
pub mod models;
/// User prompt utilities module
pub mod prompts;
/// Utility functions and helpers module
pub mod utils;

pub use cli::*;
pub use config::*;
pub use data::*;
pub use exif::*;
pub use interface::*;
pub use models::*;

/// Type alias for Result with boxed error
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

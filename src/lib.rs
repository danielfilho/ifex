//! IFEX - EXIF Data Manager
//!
//! A modern CLI tool for managing EXIF data in JPEG, TIFF, DNG, and RAW image files
//! with structured equipment management. Built with Rust for performance and reliability.
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::unused_self)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::unused_async)]
#![allow(clippy::needless_pass_by_ref_mut)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::cognitive_complexity)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::only_used_in_recursion)]

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
pub use exif::ExifManager;
pub use interface::*;
pub use models::*;

/// Type alias for Result with boxed error
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

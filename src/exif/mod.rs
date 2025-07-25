//! EXIF metadata processing module.
//!
//! This module provides functionality for reading, writing, and erasing EXIF metadata
//! from various image file formats including JPEG, TIFF, DNG, and RAW files.
//! It handles different file types through specialized processors and provides
//! a unified interface for EXIF operations.

pub mod exif_manager;
pub mod file_types;
pub mod processors;
pub mod tags;

pub use exif_manager::ExifManager;
pub use file_types::*;
pub use processors::*;
pub use tags::*;

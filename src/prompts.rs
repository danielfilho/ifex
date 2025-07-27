//! User prompt utilities for interactive input and display formatting.
//!
//! This module provides utility functions for handling user input through
//! interactive prompts, as well as formatting functions for displaying
//! information in a user-friendly manner.

use crate::models::Selection;
use colored::Colorize;
use inquire::{Confirm, CustomType, InquireError, Select, Text};

/// Utility struct providing static methods for user interaction and display formatting.
///
/// This struct contains methods for prompting user input, handling cancellation,
/// and formatting output for display in the terminal interface.
pub struct PromptUtils;

impl PromptUtils {
  /// Handles cancellation of inquire prompts gracefully.
  ///
  /// Converts `InquireError::OperationCanceled` to None, preserves successful
  /// results as Some(T), and propagates other errors. Shows a user-friendly
  /// message when operations are cancelled.
  ///
  /// # Errors
  ///
  /// Returns an error if the inquire operation fails for reasons other than cancellation.
  pub fn handle_cancellation<T>(
    result: Result<T, InquireError>,
  ) -> Result<Option<T>, Box<dyn std::error::Error>> {
    match result {
      Ok(value) => Ok(Some(value)),
      Err(InquireError::OperationCanceled) => {
        println!("{}", "Operation cancelled by user.".yellow());
        Ok(None)
      }
      Err(e) => Err(Box::new(e)),
    }
  }

  /// Prompts the user for text input.
  ///
  /// Shows the provided message and waits for user input.
  /// Returns None if the user cancels the operation.
  ///
  /// # Errors
  ///
  /// Returns an error if the prompt fails for reasons other than user cancellation.
  pub fn prompt_text(message: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let result = Text::new(message).prompt();
    Self::handle_cancellation(result)
  }

  /// Prompts the user for text input with a default value.
  ///
  /// Shows the provided message and waits for user input, pre-filling with the default.
  /// Returns None if the user cancels the operation.
  ///
  /// # Errors
  ///
  /// Returns an error if the prompt fails for reasons other than user cancellation.
  pub fn prompt_text_with_default(
    message: &str,
    default: &str,
  ) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let result = Text::new(message).with_default(default).prompt();
    Self::handle_cancellation(result)
  }

  /// Prompts the user for a yes/no confirmation.
  ///
  /// Shows the provided message with a default value.
  /// Returns None if the user cancels the operation.
  ///
  /// # Errors
  ///
  /// Returns an error if the prompt fails for reasons other than user cancellation.
  pub fn prompt_confirm(
    message: &str,
    default: bool,
  ) -> Result<Option<bool>, Box<dyn std::error::Error>> {
    let result = Confirm::new(message).with_default(default).prompt();
    Self::handle_cancellation(result)
  }

  /// Prompts the user for numeric input of a specific type.
  ///
  /// Shows the provided message and parses the input to the specified numeric type.
  /// Returns None if the user cancels the operation or if parsing fails.
  ///
  /// # Errors
  ///
  /// Returns an error if the prompt fails for reasons other than user cancellation or parsing errors.
  pub fn prompt_number<T>(message: &str) -> Result<Option<T>, Box<dyn std::error::Error>>
  where
    T: std::str::FromStr + Clone + std::fmt::Display,
    T::Err: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static,
  {
    let result = CustomType::<T>::new(message).prompt();
    Self::handle_cancellation(result)
  }

  /// Prompts the user for numeric input with a default value.
  ///
  /// Shows the provided message and waits for numeric input, pre-filling with the default.
  /// Returns None if the user cancels the operation.
  ///
  /// # Errors
  ///
  /// Returns an error if the prompt fails for reasons other than user cancellation.
  pub fn prompt_number_with_default<T>(
    message: &str,
    default: T,
  ) -> Result<Option<T>, Box<dyn std::error::Error>>
  where
    T: std::str::FromStr + Clone + std::fmt::Display,
    T::Err: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static,
  {
    let result = CustomType::<T>::new(message).with_default(default).prompt();
    Self::handle_cancellation(result)
  }

  /// Prompts the user to select from a list of options.
  ///
  /// Shows the provided message and a list of selectable options.
  /// Returns None if no options are available or if the user cancels.
  ///
  /// # Errors
  ///
  /// Returns an error if the prompt fails for reasons other than user cancellation.
  pub fn select_from_list<T: std::fmt::Display>(
    message: &str,
    options: Vec<T>,
  ) -> Result<Option<T>, Box<dyn std::error::Error>> {
    if options.is_empty() {
      println!("{}", "No options available.".yellow());
      return Ok(None);
    }

    let result = Select::new(message, options).prompt();
    Self::handle_cancellation(result)
  }

  /// Displays a formatted equipment selection summary.
  ///
  /// Shows all selected equipment and photographer information in a
  /// color-coded, user-friendly format for confirmation.
  pub fn display_selection(selection: &Selection) {
    println!("\n{}", "📋 Selected Configuration:".blue().bold());
    println!("  {} {}", "Setup:".cyan(), selection.setup.display_name());
    println!("  {} {}", "Camera:".cyan(), selection.camera.display_name());
    if let Some(lens) = &selection.lens {
      println!("  {} {}", "Lens:".cyan(), lens.display_name());
    } else {
      println!("  {} {}", "Lens:".cyan(), "None (camera only)".italic());
    }
    println!("  {} {}", "Film:".cyan(), selection.film.display_name());
    println!(
      "  {} {}",
      "Photographer:".cyan(),
      selection.photographer.display_name()
    );
    println!();
  }

  /// Formats a table header for EXIF data display.
  ///
  /// Creates a formatted table header with specified column widths
  /// for displaying EXIF tag information in a structured layout.
  #[must_use]
  pub fn format_table_header(tag_width: usize, value_width: usize) -> String {
    let tag_header = "EXIF Tag".to_string();
    let value_header = "Value".to_string();

    format!(
      " {} │ {} \n{}",
      format!("{tag_header:tag_width$}").cyan(),
      format!("{value_header:value_width$}").cyan(),
      "─".repeat(tag_width + value_width + 3).cyan()
    )
  }

  /// Formats a table row for EXIF data display.
  ///
  /// Creates a formatted table row with the specified tag and value,
  /// using the provided column widths for consistent alignment.
  #[must_use]
  pub fn format_table_row(tag: &str, value: &str, tag_width: usize, value_width: usize) -> String {
    format!(
      " {} │ {} ",
      format!("{tag:tag_width$}").yellow(),
      format!("{value:value_width$}").white()
    )
  }

  /// Formats a table footer for EXIF data display.
  ///
  /// Creates a formatted table footer with the specified column widths
  /// to close the table structure started by `format_table_header`.
  #[must_use]
  pub const fn format_table_footer(_tag_width: usize, _value_width: usize) -> String {
    String::new()
  }
}

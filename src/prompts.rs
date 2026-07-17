//! User prompt utilities for interactive input and display formatting.
//!
//! This module provides utility functions for handling user input through
//! interactive prompts, as well as formatting functions for displaying
//! information in a user-friendly manner.

use crate::{file_selector::FileSelector, models::Selection};
use colored::Colorize;
use inquire::{
  autocompletion::Autocomplete, Confirm, CustomType, InquireError, MultiSelect, Select, Text,
};
use std::{
  fs,
  path::{Path, PathBuf},
  sync::{Arc, Mutex},
  time::{SystemTime, UNIX_EPOCH},
};

const MAX_DISPLAY: usize = 10;

/// Path autocompleter for file system paths.
///
/// Implements the Autocomplete trait to provide tab completion for folder paths,
/// supporting tilde expansion and directory traversal.
/// On double tab, displays a file/folder listing.
#[derive(Clone)]
struct PathAutocompleter {
  last_input: Arc<Mutex<String>>,
  last_tab_time: Arc<Mutex<u64>>,
}

impl PathAutocompleter {
  fn new() -> Self {
    Self {
      last_input: Arc::new(Mutex::new(String::new())),
      last_tab_time: Arc::new(Mutex::new(0)),
    }
  }

  fn get_current_time() -> u64 {
    SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap_or_default()
      .as_millis() as u64
  }

  fn is_double_tab(&self, input: &str) -> bool {
    let now = Self::get_current_time();
    let mut last_input = self.last_input.lock().unwrap();
    let mut last_time = self.last_tab_time.lock().unwrap();

    let is_double = input == *last_input && now - *last_time < 500; // 500ms threshold

    *last_input = input.to_string();
    drop(last_input);
    *last_time = now;

    is_double
  }

  fn show_directory_listing(&self, input: &str) {
    let expanded_input = if input.starts_with('~') {
      if let Some(home) = dirs::home_dir() {
        input.replacen('~', &home.to_string_lossy(), 1)
      } else {
        input.to_string()
      }
    } else {
      input.to_string()
    };

    let path = Path::new(&expanded_input);
    let dir_to_list = if expanded_input.is_empty() {
      Path::new(".")
    } else if expanded_input.ends_with('/') || expanded_input.ends_with('\\') {
      path
    } else if let Some(parent) = path.parent() {
      parent
    } else {
      Path::new(".")
    };

    println!(
      "\n{}",
      format!(
        "📂 Contents of {} (continue typing or press Tab to autocomplete):",
        dir_to_list.display()
      )
      .cyan()
      .bold()
    );

    if let Ok(entries) = fs::read_dir(dir_to_list) {
      let mut files = Vec::new();
      let mut dirs = Vec::new();

      for entry in entries.flatten() {
        let file_name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden files/directories unless input starts with .
        if file_name.starts_with('.')
          && !expanded_input.ends_with("/.")
          && !expanded_input.contains("/.")
        {
          continue;
        }

        if entry.path().is_dir() {
          dirs.push(format!("{file_name}/"));
        } else {
          files.push(file_name);
        }
      }

      // Sort directories and files separately
      dirs.sort();
      files.sort();

      // Check if both are empty before iterating
      let is_empty = dirs.is_empty() && files.is_empty();

      if is_empty {
        println!("  {}", "Empty directory".yellow().italic());
      } else {
        // Display in a more compact format - show first 10 items
        let total_items = dirs.len() + files.len();
        let mut displayed = 0;

        // Display directories first
        for dir in &dirs {
          if displayed < MAX_DISPLAY {
            println!("  {} {}", "📁".blue(), dir.blue());
            displayed += 1;
          } else {
            break;
          }
        }

        // Then display files
        for file in &files {
          if displayed < MAX_DISPLAY {
            println!("  {} {}", "📄".white(), file);
            displayed += 1;
          } else {
            break;
          }
        }

        if total_items > MAX_DISPLAY {
          println!(
            "  {} {}",
            "...".yellow(),
            format!("and {} more items", total_items - MAX_DISPLAY).yellow()
          );
        }
      }
    } else {
      println!("  {}", "Cannot read directory".red());
    }

    println!(); // Empty line for better readability
  }
}

impl Autocomplete for PathAutocompleter {
  fn get_suggestions(
    &mut self,
    input: &str,
  ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    // Check for double tab and show directory listing
    if self.is_double_tab(input) {
      self.show_directory_listing(input);
      // Return empty suggestions so the input stays the same
      return Ok(vec![]);
    }

    let suggestions = PromptUtils::internal_path_autocompleter(input);
    Ok(suggestions)
  }

  fn get_completion(
    &mut self,
    input: &str,
    highlighted_suggestion: Option<String>,
  ) -> Result<inquire::autocompletion::Replacement, Box<dyn std::error::Error + Send + Sync>> {
    if let Some(suggestion) = highlighted_suggestion {
      Ok(inquire::autocompletion::Replacement::Some(suggestion))
    } else {
      // If there's no highlighted suggestion, try to find a common prefix
      let suggestions = PromptUtils::internal_path_autocompleter(input);
      match suggestions.len().cmp(&1) {
        std::cmp::Ordering::Equal => {
          // If there's exactly one suggestion, use it
          Ok(inquire::autocompletion::Replacement::Some(
            suggestions[0].clone(),
          ))
        }
        std::cmp::Ordering::Greater => {
          // If there are multiple suggestions, find common prefix
          let common_prefix = find_common_prefix(&suggestions);
          if common_prefix.len() > input.len() {
            Ok(inquire::autocompletion::Replacement::Some(common_prefix))
          } else {
            Ok(inquire::autocompletion::Replacement::None)
          }
        }
        std::cmp::Ordering::Less => Ok(inquire::autocompletion::Replacement::None),
      }
    }
  }
}

/// Find the longest common prefix among a list of strings
fn find_common_prefix(strings: &[String]) -> String {
  if strings.is_empty() {
    return String::new();
  }

  if strings.len() == 1 {
    return strings[0].clone();
  }

  let mut prefix = String::new();
  let first = &strings[0];

  for (i, ch) in first.chars().enumerate() {
    if strings.iter().all(|s| s.chars().nth(i) == Some(ch)) {
      prefix.push(ch);
    } else {
      break;
    }
  }

  prefix
}

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

  /// Path autocompletion function for folder paths.
  ///
  /// Provides tab completion for file system paths, supporting tilde expansion
  /// and directory traversal. Returns suggestions for directories that match
  /// the current input.
  #[must_use]
  pub fn path_autocompleter(input: &str) -> Vec<String> {
    Self::internal_path_autocompleter(input)
  }

  /// Internal path autocompletion implementation.
  fn internal_path_autocompleter(input: &str) -> Vec<String> {
    let expanded_input = if input.starts_with('~') {
      if let Some(home) = dirs::home_dir() {
        input.replacen('~', &home.to_string_lossy(), 1)
      } else {
        input.to_string()
      }
    } else {
      input.to_string()
    };

    let path = Path::new(&expanded_input);

    // If the path ends with a separator or is empty, list contents of the directory
    let (dir_to_search, prefix) = if expanded_input.is_empty() {
      (Path::new("."), String::new())
    } else if expanded_input.ends_with('/') || expanded_input.ends_with('\\') {
      // When path ends with '/', list all contents of that directory
      (path, String::new())
    } else {
      // Split into directory and file prefix for partial completion
      if let Some(parent) = path.parent() {
        let filename = path.file_name().unwrap_or_default().to_string_lossy();
        (parent, filename.to_string())
      } else {
        (Path::new("."), expanded_input.clone())
      }
    };

    let mut suggestions = Vec::new();

    if let Ok(entries) = fs::read_dir(dir_to_search) {
      for entry in entries.flatten() {
        let file_name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden files/directories (starting with .)
        if file_name.starts_with('.') && !prefix.starts_with('.') {
          continue;
        }

        // Only suggest directories and files that match the prefix
        if file_name.starts_with(&prefix) {
          let full_path = entry.path();

          // Build the suggestion based on the original input context
          let suggestion = if expanded_input.ends_with('/') || expanded_input.ends_with('\\') {
            // When input ends with separator, append the filename to the input
            if full_path.is_dir() {
              format!("{expanded_input}{file_name}/")
            } else {
              format!("{expanded_input}{file_name}")
            }
          } else {
            // When input doesn't end with separator, replace the last component
            if let Some(parent) = Path::new(&expanded_input).parent() {
              if parent.to_string_lossy().is_empty() || parent == Path::new(".") {
                if full_path.is_dir() {
                  format!("{file_name}/")
                } else {
                  file_name.clone()
                }
              } else if full_path.is_dir() {
                format!("{}/{}/", parent.to_string_lossy(), file_name)
              } else {
                format!("{}/{}", parent.to_string_lossy(), file_name)
              }
            } else if full_path.is_dir() {
              format!("{file_name}/")
            } else {
              file_name.clone()
            }
          };

          // Convert back to use tilde if the original input started with ~
          let final_suggestion = if input.starts_with('~') {
            if let Some(home) = dirs::home_dir() {
              suggestion.replace(&home.to_string_lossy().to_string(), "~")
            } else {
              suggestion
            }
          } else {
            suggestion
          };

          suggestions.push(final_suggestion);
        }
      }
    }

    // Sort suggestions with directories first
    suggestions.sort_by(|a, b| {
      let a_is_dir = a.ends_with('/');
      let b_is_dir = b.ends_with('/');
      match (a_is_dir, b_is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.cmp(b),
      }
    });

    // Limit to first 20 suggestions to avoid overwhelming the user
    suggestions.into_iter().take(20).collect()
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

  /// Prompts the user for a file system path with autocompletion.
  ///
  /// Shows the provided message and waits for user input, providing tab completion
  /// for file system paths. Supports tilde expansion (~) and shows directory suggestions.
  /// Returns None if the user cancels the operation.
  ///
  /// # Errors
  ///
  /// Returns an error if the prompt fails for reasons other than user cancellation.
  pub fn prompt_path(message: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let result = Text::new(message)
      .with_autocomplete(PathAutocompleter::new())
      .with_help_message("Tab: autocomplete paths, /: show directory listing, ~ for home directory")
      .prompt();
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

  /// Prompts the user to select files from a folder interactively.
  ///
  /// Scans the specified folder for supported image files and presents them
  /// in an interactive multi-select interface. Users can navigate with arrow keys,
  /// select/deselect files with spacebar, and confirm with Enter.
  ///
  /// # Arguments
  ///
  /// * `folder_path` - The directory to scan for image files
  ///
  /// # Returns
  ///
  /// A vector of selected file paths, or None if the user cancels or no files are available.
  ///
  /// # Errors
  ///
  /// Returns an error if the prompt fails for reasons other than user cancellation.
  pub fn select_files_from_folder(
    folder_path: &Path,
  ) -> Result<Option<Vec<PathBuf>>, Box<dyn std::error::Error>> {
    let files = FileSelector::scan_directory(folder_path);

    if files.is_empty() {
      println!(
        "{}",
        "No supported image files found in the specified folder.".yellow()
      );
      return Ok(None);
    }

    println!(
      "{}",
      format!(
        "\n📂 Found {} supported image file{} in folder",
        files.len(),
        if files.len() == 1 { "" } else { "s" }
      )
      .blue()
    );

    // First, ask if user wants to select all files
    let select_all_option = Self::prompt_confirm("Select all files?", false)?;

    if select_all_option == Some(true) {
      println!(
        "{}",
        format!(
          "✅ Selected all {} file{} for processing",
          files.len(),
          if files.len() == 1 { "" } else { "s" }
        )
        .green()
      );
      return Ok(Some(files));
    } else if select_all_option.is_none() {
      // User cancelled
      return Ok(None);
    }

    println!(
      "{}",
      "Use arrow keys to navigate, spacebar to select/deselect, Enter to confirm:".cyan()
    );

    // Create display options with relative paths
    let display_options: Vec<String> = files
      .iter()
      .map(|file_path| FileSelector::format_file_for_display(file_path, folder_path))
      .collect();

    let result = MultiSelect::new("Select files to apply EXIF data:", display_options).prompt();

    match Self::handle_cancellation(result)? {
      Some(selected_displays) => {
        // Map selected display strings back to full paths
        let selected_files: Vec<PathBuf> = selected_displays
          .iter()
          .filter_map(|display| {
            files.iter().find(|file_path| {
              FileSelector::format_file_for_display(file_path, folder_path) == *display
            })
          })
          .cloned()
          .collect();

        if selected_files.is_empty() {
          println!("{}", "No files selected.".yellow());
          Ok(None)
        } else {
          println!(
            "{}",
            format!(
              "✅ Selected {} file{} for processing",
              selected_files.len(),
              if selected_files.len() == 1 { "" } else { "s" }
            )
            .green()
          );
          Ok(Some(selected_files))
        }
      }
      None => Ok(None),
    }
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

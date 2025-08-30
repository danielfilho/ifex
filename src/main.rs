//! IFEX CLI application entry point
#![allow(clippy::multiple_crate_versions)]

use colored::Colorize;
use ifex::{
  cli::{Cli, Commands},
  interface::Interface,
  Result,
};
use std::process;

/// Main application entry point
fn main() {
  let cli = Cli::parse_args();

  let result = match &cli.command {
    Some(Commands::Manage) => run_management(),
    Some(Commands::Read { paths, json }) => check_exif_data(paths, *json),
    Some(Commands::Run) | None => run_interactive(cli.one_sec),
  };

  if let Err(e) = result {
    eprintln!("{}", format!("Error: {e}").red());
    process::exit(1);
  }
}

/// Run the interactive main menu interface
fn run_interactive(one_sec: bool) -> Result<()> {
  println!("{}", "🏷️  IFEX - EXIF Data Manager\n".blue());

  let mut interface = Interface::new(one_sec)?;
  interface.run_main_menu()?;
  Ok(())
}

/// Run the equipment management interface
fn run_management() -> Result<()> {
  println!("{}", "🏷️  IFEX - Equipment Manager\n".blue());

  let mut interface = Interface::new(false)?;
  interface.run_management_menu()?;
  Ok(())
}

/// Check and display EXIF data from image files or directories
#[allow(clippy::unnecessary_wraps, clippy::too_many_lines)]
fn check_exif_data(paths: &[std::path::PathBuf], json: bool) -> Result<()> {
  use ifex::{prompts::PromptUtils, ExifManager, file_selector::FileSelector, utils::is_supported_image_format};
  use serde_json::{json, Value};

  // Collect all image files from the provided paths
  let mut all_files = Vec::new();
  
  for path in paths {
    if path.is_file() {
      if is_supported_image_format(path) {
        all_files.push(path.clone());
      } else if json {
        // In JSON mode, we still need to include unsupported files with error
        all_files.push(path.clone());
      } else {
        println!("{}", format!("⚠️  Unsupported file format: {}", path.display()).yellow());
      }
    } else if path.is_dir() {
      let directory_files = FileSelector::scan_directory(path);
      all_files.extend(directory_files);
    } else if json {
      // Path doesn't exist - we'll handle this in the processing loop
      all_files.push(path.clone());
    } else {
      println!("{}", format!("❌ Path does not exist: {}", path.display()).red());
    }
  }

  if json {
    // JSON output format - always return an array
    let mut json_results = Vec::new();
    
    for file in &all_files {
      let mut file_result = serde_json::Map::new();
      file_result.insert("file".to_string(), json!(file.display().to_string()));
      
      if !file.exists() {
        file_result.insert("error".to_string(), json!("File does not exist"));
      } else if !is_supported_image_format(file) {
        file_result.insert("error".to_string(), json!("Unsupported file format"));
      } else {
        match ExifManager::read_exif_data(file) {
          Ok(exif_data) => {
            let exif_map: std::collections::HashMap<String, String> = exif_data
              .into_iter()
              .collect();
            file_result.insert("exif".to_string(), json!(exif_map));
          }
          Err(e) => {
            file_result.insert("error".to_string(), json!(format!("Error reading EXIF data: {e}")));
          }
        }
      }
      
      json_results.push(Value::Object(file_result));
    }
    
    match serde_json::to_string_pretty(&json_results) {
      Ok(json_string) => println!("{json_string}"),
      Err(e) => println!("{{\"error\": \"Failed to serialize JSON: {e}\"}}"),
    }
  } else {
    // Original table format for each file
    for file in &all_files {
      if !file.exists() {
        println!("{}", format!("❌ File does not exist: {}", file.display()).red());
        continue;
      }
      
      if !is_supported_image_format(file) {
        println!("{}", format!("⚠️  Unsupported file format: {}", file.display()).yellow());
        continue;
      }
      
      println!(
        "{}",
        format!("📷 EXIF Data for: {}\n", file.display()).blue()
      );

      match ExifManager::read_exif_data(file) {
        Ok(exif_data) => {
          if exif_data.is_empty() {
            println!("{}", "⚠️  No EXIF data found in this image.".yellow());
          } else {
            println!(
              "{}",
              format!("📷 EXIF Data ({} entries)\n", exif_data.len()).blue()
            );

            let max_tag_length = exif_data
              .iter()
              .map(|(tag, _)| tag.len())
              .max()
              .unwrap_or(15);
            let max_value_length = exif_data
              .iter()
              .map(|(_, value)| value.len())
              .max()
              .unwrap_or(20);

            println!(
              "{}",
              PromptUtils::format_table_header(max_tag_length, max_value_length)
            );

            for (tag, value) in &exif_data {
              println!(
                "{}",
                PromptUtils::format_table_row(tag, value, max_tag_length, max_value_length)
              );
            }

            println!(
              "{}",
              PromptUtils::format_table_footer(max_tag_length, max_value_length)
            );
          }
        }
        Err(e) => {
          println!("{}", format!("❌ Error reading EXIF data: {e}").red());
        }
      }
      
      // Add spacing between files if there are multiple
      if all_files.len() > 1 {
        println!();
      }
    }
  }

  Ok(())
}

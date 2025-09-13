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
    Some(Commands::Read { file, json }) => check_exif_data(file, *json),
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

/// Check and display EXIF data from an image file
#[allow(clippy::unnecessary_wraps)]
fn check_exif_data(file: &std::path::Path, json_output: bool) -> Result<()> {
  use ifex::{prompts::PromptUtils, ExifManager};

  println!(
    "{}",
    format!("📷 EXIF Data for: {}\n", file.display()).blue()
  );

  if !file.exists() {
    println!("{}", "❌ File does not exist".red());
    return Ok(());
  }

  match ExifManager::read_exif_data(file) {
    Ok(exif_data) => {
      if exif_data.is_empty() {
        if json_output {
          println!("[]");
        } else {
          println!("{}", "⚠️  No EXIF data found in this image.".yellow());
        }
      } else if json_output {
        // JSON output
        let json_data: serde_json::Value = exif_data
          .iter()
          .map(|(tag, value)| (tag.clone(), serde_json::Value::String(value.clone())))
          .collect::<serde_json::Map<String, serde_json::Value>>()
          .into();
        match serde_json::to_string_pretty(&json_data) {
          Ok(json_str) => println!("{json_str}"),
          Err(e) => eprintln!("{}", format!("Error serializing JSON: {e}").red()),
        }
      } else {
        // Table output
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
      if json_output {
        let error_json = serde_json::json!({
          "error": format!("Error reading EXIF data: {}", e)
        });
        match serde_json::to_string_pretty(&error_json) {
          Ok(json_str) => println!("{json_str}"),
          Err(e) => eprintln!("{}", format!("Error serializing error JSON: {e}").red()),
        }
      } else {
        println!("{}", format!("❌ Error reading EXIF data: {e}").red());
      }
    }
  }

  Ok(())
}

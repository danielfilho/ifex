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
    Some(Commands::Read { file }) => check_exif_data(file),
    Some(Commands::Run) | None => run_interactive(),
  };

  if let Err(e) = result {
    eprintln!("{}", format!("Error: {e}").red());
    process::exit(1);
  }
}

/// Run the interactive main menu interface
fn run_interactive() -> Result<()> {
  println!("{}", "🏷️  IFEX - EXIF Data Manager\n".blue());

  let mut interface = Interface::new()?;
  interface.run_main_menu()?;
  Ok(())
}

/// Run the equipment management interface
fn run_management() -> Result<()> {
  println!("{}", "🏷️  IFEX - Equipment Manager\n".blue());

  let mut interface = Interface::new()?;
  interface.run_management_menu()?;
  Ok(())
}

/// Check and display EXIF data from an image file
#[allow(clippy::unnecessary_wraps)]
fn check_exif_data(file: &std::path::Path) -> Result<()> {
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

  Ok(())
}

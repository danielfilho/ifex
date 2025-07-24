//! User interface module providing interactive menus and workflows.
//!
//! This module contains the main application interface that allows users to
//! interactively apply or erase EXIF data from images, as well as manage
//! their photography equipment database through various menu systems.

use crate::{data::DataManager, models::{Selection, Setup, Film, Photographer, Camera, Lens}, prompts::PromptUtils, utils::clean_path};
use colored::Colorize;
use std::path::PathBuf;

/// Main application interface providing interactive menu systems.
///
/// This struct handles all user interactions, from the main menu through
/// equipment management and EXIF processing workflows. It manages the
/// underlying data and coordinates between user input and EXIF operations.
pub struct Interface {
  data_manager: DataManager,
}

impl Interface {
  /// Creates a new Interface instance.
  ///
  /// Initializes the data manager by loading the configuration from disk.
  /// Returns an error if the configuration cannot be loaded.
  pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
    let data_manager = DataManager::new()?;
    Ok(Self { data_manager })
  }

  /// Runs the main application menu loop.
  ///
  /// Displays the primary menu with options to apply EXIF data, erase EXIF data,
  /// manage equipment, or exit the application. Continues running until the user
  /// chooses to exit or cancels the operation.
  pub async fn run_main_menu(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    loop {
      let options = vec![
        "Apply EXIF data to images",
        "Erase EXIF data from images",
        "Manage equipment",
        "Exit",
      ];

      if let Some(choice) = PromptUtils::select_from_list("What would you like to do?", options)? {
        match choice {
          "Apply EXIF data to images" => {
            if let Err(e) = self.handle_apply_exif().await {
              eprintln!("{}", format!("Error: {e}").red());
            }
          }
          "Erase EXIF data from images" => {
            if let Err(e) = self.handle_erase_exif().await {
              eprintln!("{}", format!("Error: {e}").red());
            }
          }
          "Manage equipment" => {
            if let Err(e) = self.run_management_menu().await {
              eprintln!("{}", format!("Error: {e}").red());
            }
          }
          "Exit" => {
            println!("{}", "👋 Goodbye!".blue());
            break;
          }
          _ => {}
        }
      } else {
        break;
      }
    }
    Ok(())
  }

  /// Handles the EXIF application workflow.
  ///
  /// Guides the user through selecting equipment, choosing a folder path,
  /// and applying EXIF metadata to supported image files in the specified location.
  async fn handle_apply_exif(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    let (selection, shot_iso) = self.select_setup_film_and_iso().await?;
    if selection.is_none() || shot_iso.is_none() {
      println!(
        "{}",
        "No valid setup, film, and ISO selected. Returning to main menu.".yellow()
      );
      return Ok(());
    }
    let selection = selection.unwrap();
    let shot_iso = shot_iso.unwrap();

    PromptUtils::display_selection(&selection);

    let folder_path = self.prompt_folder_path()?;
    if folder_path.is_none() {
      return Ok(());
    }
    let _folder_path = folder_path.unwrap();

    let recursive = self.prompt_recursive_processing()?;
    if recursive.is_none() {
      return Ok(());
    }
    let _recursive = recursive.unwrap();

    println!("{}", "\n📝 Applying EXIF data...\n".blue());

    let exif_manager = crate::ExifManager::new();
    let result = exif_manager
      .process_folder_with_iso(
        &_folder_path,
        Some(&selection),
        "apply",
        _recursive,
        Some(shot_iso),
      );

    if result.success {
      println!(
        "{}",
        format!(
          "✅ Successfully processed {} files",
          result.results.processed
        )
        .green()
      );
      if result.results.failed > 0 {
        println!(
          "{}",
          format!("❌ Failed to process {} files", result.results.failed).red()
        );
      }

      println!("\n📊 Processing Results:");
      for file in &result.results.files {
        let status = if file.success {
          "✓".green()
        } else {
          "✗".red()
        };
        let type_label = file
          .file_type
          .as_ref()
          .map(|t| format!("[{}]", t.to_uppercase()))
          .unwrap_or_default();
        println!(
          "  {} {} {}",
          status,
          file.name,
          type_label.as_str().bright_black()
        );
        if let Some(error) = &file.error {
          println!("    {}", format!("Error: {error}").red());
        }
      }
    } else {
      println!("{}", format!("❌ Error: {}", result.message).red());
    }
    Ok(())
  }

  /// Handles the EXIF erasure workflow.
  ///
  /// Guides the user through selecting a folder path and confirmation,
  /// then erases EXIF metadata from supported image files in the specified location.
  async fn handle_erase_exif(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    let folder_path = self.prompt_folder_path()?;
    if folder_path.is_none() {
      return Ok(());
    }
    let _folder_path = folder_path.unwrap();

    let recursive = self.prompt_recursive_processing()?;
    if recursive.is_none() {
      return Ok(());
    }
    let _recursive = recursive.unwrap();

    let confirmed = self.confirm_erase_exif()?;
    if confirmed != Some(true) {
      println!("{}", "Operation cancelled.".yellow());
      return Ok(());
    }

    println!("{}", "\n🗑️  Erasing EXIF data...\n".blue());

    let exif_manager = crate::ExifManager::new();
    let result = exif_manager
      .process_folder(&_folder_path, None, "erase", _recursive);

    if result.success {
      println!(
        "{}",
        format!(
          "✅ Successfully processed {} files",
          result.results.processed
        )
        .green()
      );
      if result.results.failed > 0 {
        println!(
          "{}",
          format!("❌ Failed to process {} files", result.results.failed).red()
        );
      }

      println!("\n📊 Processing Results:");
      for file in &result.results.files {
        let status = if file.success {
          "✓".green()
        } else {
          "✗".red()
        };
        let type_label = file
          .file_type
          .as_ref()
          .map(|t| format!("[{}]", t.to_uppercase()))
          .unwrap_or_default();
        println!(
          "  {} {} {}",
          status,
          file.name,
          type_label.as_str().bright_black()
        );
        if let Some(error) = &file.error {
          println!("    {}", format!("Error: {error}").red());
        }
      }
    } else {
      println!("{}", format!("❌ Error: {}", result.message).red());
    }
    Ok(())
  }

  /// Guides the user through selecting a complete equipment configuration.
  ///
  /// Prompts the user to select a setup (camera + lens), film, photographer, and ISO,
  /// then creates a Selection object containing all the necessary information
  /// for EXIF metadata application.
  async fn select_setup_film_and_iso(
    &self,
  ) -> Result<(Option<Selection>, Option<u32>), Box<dyn std::error::Error>> {
    let setups = self.data_manager.get_setups();
    if setups.is_empty() {
      println!(
        "{}",
        "No setups available. Please create a setup first.".yellow()
      );
      return Ok((None, None));
    }

    let setup_options: Vec<String> = setups.iter().map(Setup::display_name).collect();
    let selected_setup_name = PromptUtils::select_from_list("Select a setup:", setup_options)?;
    if selected_setup_name.is_none() {
      return Ok((None, None));
    }
    let selected_setup_name = selected_setup_name.unwrap();
    let selected_setup = setups
      .iter()
      .find(|s| s.display_name() == selected_setup_name)
      .unwrap();

    let films = self.data_manager.get_films();
    if films.is_empty() {
      println!(
        "{}",
        "No films available. Please create a film first.".yellow()
      );
      return Ok((None, None));
    }

    let film_options: Vec<String> = films.iter().map(Film::display_name).collect();
    let selected_film_name = PromptUtils::select_from_list("Select a film:", film_options)?;
    if selected_film_name.is_none() {
      return Ok((None, None));
    }
    let selected_film_name = selected_film_name.unwrap();
    let selected_film = films
      .iter()
      .find(|f| f.display_name() == selected_film_name)
      .unwrap();

    let photographers = self.data_manager.get_photographers();
    if photographers.is_empty() {
      println!(
        "{}",
        "No photographers available. Please create a photographer first.".yellow()
      );
      return Ok((None, None));
    }

    // Prompt for shot ISO (defaults to film's base ISO)
    let shot_iso = self.prompt_shot_iso(selected_film.iso)?;
    if shot_iso.is_none() {
      return Ok((None, None));
    }
    let shot_iso = shot_iso.unwrap();

    let photographer_options: Vec<String> =
      photographers.iter().map(Photographer::display_name).collect();
    let selected_photographer_name =
      PromptUtils::select_from_list("Select a photographer:", photographer_options)?;
    if selected_photographer_name.is_none() {
      return Ok((None, None));
    }
    let selected_photographer_name = selected_photographer_name.unwrap();
    let selected_photographer = photographers
      .iter()
      .find(|p| p.display_name() == selected_photographer_name)
      .unwrap();

    let selection = self
      .data_manager
      .create_selection(
        selected_setup.id,
        selected_film.id,
        selected_photographer.id,
      )
      .map_err(|e| format!("Error creating selection: {e}"))?;

    Ok((Some(selection), Some(shot_iso)))
  }

  /// Prompts the user to enter the ISO speed at which the film was shot.
  ///
  /// Defaults to the film's base ISO rating. Useful for push/pull processing.
  /// Returns None if the user cancels the operation.
  fn prompt_shot_iso(&self, default_iso: u32) -> Result<Option<u32>, Box<dyn std::error::Error>> {
    println!("{}", format!("📸 Film base ISO: {default_iso}").cyan());

    let prompt_text = format!(
      "Enter the ISO speed you shot at (press Enter for {default_iso}): "
    );

    if let Some(input) = PromptUtils::prompt_text(&prompt_text)? {
      if input.trim().is_empty() {
        Ok(Some(default_iso))
      } else if let Ok(iso) = input.trim().parse::<u32>() {
        if iso > 0 && iso <= 6400 {
          Ok(Some(iso))
        } else {
          println!("{}", "❌ ISO must be between 1 and 6400".red());
          self.prompt_shot_iso(default_iso)
        }
      } else {
        println!("{}", "❌ Invalid ISO value. Please enter a number.".red());
        self.prompt_shot_iso(default_iso)
      }
    } else {
      Ok(None)
    }
  }

  /// Prompts the user to enter a folder path for image processing.
  ///
  /// Cleans the input path by removing quotes and handling escaped spaces.
  /// Returns None if the user cancels the operation.
  fn prompt_folder_path(&self) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    if let Some(path_str) = PromptUtils::prompt_text("Enter the folder path:")? {
      let cleaned_path = clean_path(&path_str);
      Ok(Some(PathBuf::from(cleaned_path)))
    } else {
      Ok(None)
    }
  }

  /// Prompts the user whether to process subdirectories recursively.
  ///
  /// Defaults to true (recursive processing enabled).
  /// Returns None if the user cancels the operation.
  fn prompt_recursive_processing(&self) -> Result<Option<bool>, Box<dyn std::error::Error>> {
    PromptUtils::prompt_confirm("Process subdirectories recursively?", true)
  }

  /// Prompts the user to confirm EXIF data erasure.
  ///
  /// Shows a warning that the operation cannot be undone.
  /// Defaults to false (do not erase). Returns None if the user cancels.
  fn confirm_erase_exif(&self) -> Result<Option<bool>, Box<dyn std::error::Error>> {
    PromptUtils::prompt_confirm(
      "Are you sure you want to erase EXIF data? This cannot be undone.",
      false,
    )
  }

  /// Runs the equipment management menu loop.
  ///
  /// Provides options to manage cameras, lenses, films, photographers, and setups.
  /// Returns to the main menu when the user selects "Back to main menu".
  ///
  /// # Errors
  ///
  /// Returns an error if there are issues with user prompts or data management operations.
  pub async fn run_management_menu(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    loop {
      let options = vec![
        "Manage Cameras",
        "Manage Lenses",
        "Manage Films",
        "Manage Photographers",
        "Manage Setups",
        "Back to main menu",
      ];

      if let Some(choice) = PromptUtils::select_from_list("Equipment Management", options)? {
        match choice {
          "Manage Cameras" => self.manage_cameras().await?,
          "Manage Lenses" => self.manage_lenses().await?,
          "Manage Films" => self.manage_films().await?,
          "Manage Photographers" => self.manage_photographers().await?,
          "Manage Setups" => self.manage_setups().await?,
          "Back to main menu" => break,
          _ => {}
        }
      } else {
        break;
      }
    }
    Ok(())
  }

  /// Handles camera management operations.
  ///
  /// Provides options to view, add, and delete cameras in the configuration.
  /// Prevents deletion of cameras that are currently used in setups.
  async fn manage_cameras(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "\n📷 Camera Management\n".blue().bold());

    loop {
      let options = vec![
        "View all cameras",
        "Add new camera",
        "Edit camera",
        "Delete camera",
        "Back",
      ];

      if let Some(choice) = PromptUtils::select_from_list("Camera Management", options)? {
        match choice {
          "View all cameras" => {
            let cameras = self.data_manager.get_cameras();
            if cameras.is_empty() {
              println!("{}", "No cameras found.".yellow());
            } else {
              println!("{}", "📷 Cameras:".cyan().bold());
              for camera in cameras {
                println!("  • {}", camera.display_name());
              }
            }
          }
          "Add new camera" => {
            if let (Some(maker), Some(model)) = (
              PromptUtils::prompt_text("Camera maker:")?,
              PromptUtils::prompt_text("Camera model:")?,
            ) {
              let camera = self.data_manager.add_camera(maker, model);
              self.data_manager.save()?;
              println!(
                "{}",
                format!("✅ Added camera: {}", camera.display_name()).green()
              );
            }
          }
          "Edit camera" => {
            let cameras = self.data_manager.get_cameras();
            if cameras.is_empty() {
              println!("{}", "No cameras to edit.".yellow());
            } else {
              let camera_options: Vec<String> = cameras.iter().map(Camera::display_name).collect();
              if let Some(selected_name) =
                PromptUtils::select_from_list("Select camera to edit:", camera_options)?
              {
                if let Some(camera) = cameras.iter().find(|c| c.display_name() == selected_name) {
                  let old_name = camera.display_name();
                  if let (Some(maker), Some(model)) = (
                    PromptUtils::prompt_text_with_default("Camera maker:", &camera.maker)?,
                    PromptUtils::prompt_text_with_default("Camera model:", &camera.model)?,
                  ) {
                    if self.data_manager.edit_camera(camera.id, maker, model) {
                      self.data_manager.save()?;
                      println!("{}", format!("✅ Updated camera: {old_name}").green());
                    } else {
                      println!("{}", "❌ Failed to update camera.".red());
                    }
                  }
                }
              }
            }
          }
          "Delete camera" => {
            let cameras = self.data_manager.get_cameras();
            if cameras.is_empty() {
              println!("{}", "No cameras to delete.".yellow());
            } else {
              let camera_options: Vec<String> = cameras.iter().map(Camera::display_name).collect();
              if let Some(selected_name) =
                PromptUtils::select_from_list("Select camera to delete:", camera_options)?
              {
                if let Some(camera) = cameras.iter().find(|c| c.display_name() == selected_name) {
                  let camera_id = camera.id;
                  let camera_name = camera.display_name();
                  match self.data_manager.delete_camera(camera_id) {
                    Ok(()) => {
                      self.data_manager.save()?;
                      println!("{}", format!("✅ Deleted camera: {camera_name}").green());
                    }
                    Err(e) => {
                      println!("{}", format!("❌ Error: {e}").red());
                    }
                  }
                }
              }
            }
          }
          "Back" => break,
          _ => {}
        }
      } else {
        break;
      }
    }
    Ok(())
  }

  /// Handles lens management operations.
  ///
  /// Provides options to view, add, and delete lenses in the configuration.
  /// Prevents deletion of lenses that are currently used in setups.
  async fn manage_lenses(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "\n🔍 Lens Management\n".blue().bold());

    loop {
      let options = vec![
        "View all lenses",
        "Add new lens",
        "Edit lens",
        "Delete lens",
        "Back",
      ];

      if let Some(choice) = PromptUtils::select_from_list("Lens Management", options)? {
        match choice {
          "View all lenses" => {
            let lenses = self.data_manager.get_lenses();
            if lenses.is_empty() {
              println!("{}", "No lenses found.".yellow());
            } else {
              println!("{}", "🔍 Lenses:".cyan().bold());
              for lens in lenses {
                println!("  • {}", lens.display_name());
              }
            }
          }
          "Add new lens" => {
            if let (Some(maker), Some(model), Some(focal_length), Some(aperture), Some(mount)) = (
              PromptUtils::prompt_text("Lens maker:")?,
              PromptUtils::prompt_text("Lens model:")?,
              PromptUtils::prompt_text("Focal length (mm):")?,
              PromptUtils::prompt_text("Maximum aperture (f/):")?,
              PromptUtils::prompt_text("Mount type:")?,
            ) {
              let lens = self
                .data_manager
                .add_lens(maker, model, focal_length, aperture, mount);
              self.data_manager.save()?;
              println!(
                "{}",
                format!("✅ Added lens: {}", lens.display_name()).green()
              );
            }
          }
          "Edit lens" => {
            let lenses = self.data_manager.get_lenses();
            if lenses.is_empty() {
              println!("{}", "No lenses to edit.".yellow());
            } else {
              let lens_options: Vec<String> = lenses.iter().map(Lens::display_name).collect();
              if let Some(selected_name) =
                PromptUtils::select_from_list("Select lens to edit:", lens_options)?
              {
                if let Some(lens) = lenses.iter().find(|l| l.display_name() == selected_name) {
                  let old_name = lens.display_name();
                  if let (
                    Some(maker),
                    Some(model),
                    Some(focal_length),
                    Some(aperture),
                    Some(mount),
                  ) = (
                    PromptUtils::prompt_text_with_default("Lens maker:", &lens.maker)?,
                    PromptUtils::prompt_text_with_default("Lens model:", &lens.model)?,
                    PromptUtils::prompt_text_with_default(
                      "Focal length (mm):",
                      &lens.focal_length,
                    )?,
                    PromptUtils::prompt_text_with_default(
                      "Maximum aperture (f/):",
                      &lens.aperture,
                    )?,
                    PromptUtils::prompt_text_with_default("Mount type:", &lens.mount)?,
                  ) {
                    if self.data_manager.edit_lens(
                      lens.id,
                      maker,
                      model,
                      focal_length,
                      aperture,
                      mount,
                    ) {
                      self.data_manager.save()?;
                      println!("{}", format!("✅ Updated lens: {old_name}").green());
                    } else {
                      println!("{}", "❌ Failed to update lens.".red());
                    }
                  }
                }
              }
            }
          }
          "Delete lens" => {
            let lenses = self.data_manager.get_lenses();
            if lenses.is_empty() {
              println!("{}", "No lenses to delete.".yellow());
            } else {
              let lens_options: Vec<String> = lenses.iter().map(Lens::display_name).collect();
              if let Some(selected_name) =
                PromptUtils::select_from_list("Select lens to delete:", lens_options)?
              {
                if let Some(lens) = lenses.iter().find(|l| l.display_name() == selected_name) {
                  let lens_id = lens.id;
                  let lens_name = lens.display_name();
                  match self.data_manager.delete_lens(lens_id) {
                    Ok(()) => {
                      self.data_manager.save()?;
                      println!("{}", format!("✅ Deleted lens: {lens_name}").green());
                    }
                    Err(e) => {
                      println!("{}", format!("❌ Error: {e}").red());
                    }
                  }
                }
              }
            }
          }
          "Back" => break,
          _ => {}
        }
      } else {
        break;
      }
    }
    Ok(())
  }

  /// Handles film management operations.
  ///
  /// Provides options to view, add, and delete film stocks in the configuration.
  /// Films can be deleted without restriction as they are not referenced by other entities.
  async fn manage_films(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "\n🎞️ Film Management\n".blue().bold());

    loop {
      let options = vec![
        "View all films",
        "Add new film",
        "Edit film",
        "Delete film",
        "Back",
      ];

      if let Some(choice) = PromptUtils::select_from_list("Film Management", options)? {
        match choice {
          "View all films" => {
            let films = self.data_manager.get_films();
            if films.is_empty() {
              println!("{}", "No films found.".yellow());
            } else {
              println!("{}", "🎞️ Films:".cyan().bold());
              for film in films {
                println!("  • {}", film.display_name());
              }
            }
          }
          "Add new film" => {
            if let (Some(maker), Some(name), Some(iso)) = (
              PromptUtils::prompt_text("Film maker:")?,
              PromptUtils::prompt_text("Film name:")?,
              PromptUtils::prompt_number::<u32>("ISO rating:")?,
            ) {
              let film = self.data_manager.add_film(maker, name, iso);
              self.data_manager.save()?;
              println!(
                "{}",
                format!("✅ Added film: {}", film.display_name()).green()
              );
            }
          }
          "Edit film" => {
            let films = self.data_manager.get_films();
            if films.is_empty() {
              println!("{}", "No films to edit.".yellow());
            } else {
              let film_options: Vec<String> = films.iter().map(Film::display_name).collect();
              if let Some(selected_name) =
                PromptUtils::select_from_list("Select film to edit:", film_options)?
              {
                if let Some(film) = films.iter().find(|f| f.display_name() == selected_name) {
                  let old_name = film.display_name();
                  if let (Some(maker), Some(name), Some(iso)) = (
                    PromptUtils::prompt_text_with_default("Film maker:", &film.maker)?,
                    PromptUtils::prompt_text_with_default("Film name:", &film.name)?,
                    PromptUtils::prompt_number_with_default::<u32>("ISO rating:", film.iso)?,
                  ) {
                    if self.data_manager.edit_film(film.id, maker, name, iso) {
                      self.data_manager.save()?;
                      println!("{}", format!("✅ Updated film: {old_name}").green());
                    } else {
                      println!("{}", "❌ Failed to update film.".red());
                    }
                  }
                }
              }
            }
          }
          "Delete film" => {
            let films = self.data_manager.get_films();
            if films.is_empty() {
              println!("{}", "No films to delete.".yellow());
            } else {
              let film_options: Vec<String> = films.iter().map(Film::display_name).collect();
              if let Some(selected_name) =
                PromptUtils::select_from_list("Select film to delete:", film_options)?
              {
                if let Some(film) = films.iter().find(|f| f.display_name() == selected_name) {
                  let film_id = film.id;
                  let film_name = film.display_name();
                  self.data_manager.delete_film(film_id);
                  self.data_manager.save()?;
                  println!("{}", format!("✅ Deleted film: {film_name}").green());
                }
              }
            }
          }
          "Back" => break,
          _ => {}
        }
      } else {
        break;
      }
    }
    Ok(())
  }

  /// Handles photographer management operations.
  ///
  /// Provides options to view, add, and delete photographers in the configuration.
  /// Photographers can be deleted without restriction as they are not referenced by other entities.
  async fn manage_photographers(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "\n👤 Photographer Management\n".blue().bold());

    loop {
      let options = vec![
        "View all photographers",
        "Add new photographer",
        "Edit photographer",
        "Delete photographer",
        "Back",
      ];

      if let Some(choice) = PromptUtils::select_from_list("Photographer Management", options)? {
        match choice {
          "View all photographers" => {
            let photographers = self.data_manager.get_photographers();
            if photographers.is_empty() {
              println!("{}", "No photographers found.".yellow());
            } else {
              println!("{}", "👤 Photographers:".cyan().bold());
              for photographer in photographers {
                println!("  • {}", photographer.display_name());
              }
            }
          }
          "Add new photographer" => {
            if let Some(name) = PromptUtils::prompt_text("Photographer name:")? {
              let email = PromptUtils::prompt_text("Email (optional):")?;
              let email = if email.as_ref().map_or(true, |e| e.trim().is_empty()) {
                None
              } else {
                email
              };

              let photographer = self.data_manager.add_photographer(name, email);
              self.data_manager.save()?;
              println!(
                "{}",
                format!("✅ Added photographer: {}", photographer.display_name()).green()
              );
            }
          }
          "Edit photographer" => {
            let photographers = self.data_manager.get_photographers();
            if photographers.is_empty() {
              println!("{}", "No photographers to edit.".yellow());
            } else {
              let photographer_options: Vec<String> =
                photographers.iter().map(Photographer::display_name).collect();
              if let Some(selected_name) =
                PromptUtils::select_from_list("Select photographer to edit:", photographer_options)?
              {
                if let Some(photographer) = photographers
                  .iter()
                  .find(|p| p.display_name() == selected_name)
                {
                  let old_name = photographer.display_name();
                  if let Some(name) =
                    PromptUtils::prompt_text_with_default("Photographer name:", &photographer.name)?
                  {
                    let current_email = photographer.email.as_deref().unwrap_or("");
                    let email =
                      PromptUtils::prompt_text_with_default("Email (optional):", current_email)?;
                    let email = if email.as_ref().map_or(true, |e| e.trim().is_empty()) {
                      None
                    } else {
                      email
                    };

                    if self
                      .data_manager
                      .edit_photographer(photographer.id, name, email)
                    {
                      self.data_manager.save()?;
                      println!(
                        "{}",
                        format!("✅ Updated photographer: {old_name}").green()
                      );
                    } else {
                      println!("{}", "❌ Failed to update photographer.".red());
                    }
                  }
                }
              }
            }
          }
          "Delete photographer" => {
            let photographers = self.data_manager.get_photographers();
            if photographers.is_empty() {
              println!("{}", "No photographers to delete.".yellow());
            } else {
              let photographer_options: Vec<String> =
                photographers.iter().map(Photographer::display_name).collect();
              if let Some(selected_name) = PromptUtils::select_from_list(
                "Select photographer to delete:",
                photographer_options,
              )? {
                if let Some(photographer) = photographers
                  .iter()
                  .find(|p| p.display_name() == selected_name)
                {
                  let photographer_id = photographer.id;
                  let photographer_name = photographer.display_name();
                  self.data_manager.delete_photographer(photographer_id);
                  self.data_manager.save()?;
                  println!(
                    "{}",
                    format!("✅ Deleted photographer: {photographer_name}").green()
                  );
                }
              }
            }
          }
          "Back" => break,
          _ => {}
        }
      } else {
        break;
      }
    }
    Ok(())
  }

  /// Handles equipment setup management operations.
  ///
  /// Provides options to view, add, and delete equipment setups (camera + lens combinations).
  /// Validates that referenced cameras and lenses exist before creating new setups.
  async fn manage_setups(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "\n⚙️ Setup Management\n".blue().bold());

    loop {
      let options = vec![
        "View all setups",
        "Add new setup",
        "Edit setup",
        "Delete setup",
        "Back",
      ];

      if let Some(choice) = PromptUtils::select_from_list("Setup Management", options)? {
        match choice {
          "View all setups" => {
            let setups = self.data_manager.get_setups();
            if setups.is_empty() {
              println!("{}", "No setups found.".yellow());
            } else {
              println!("{}", "⚙️ Setups:".cyan().bold());
              for setup in setups {
                if let (Some(camera), Some(lens)) = (
                  self.data_manager.get_camera_by_id(setup.camera_id),
                  self.data_manager.get_lens_by_id(setup.lens_id),
                ) {
                  println!(
                    "  • {} ({} + {})",
                    setup.display_name(),
                    camera.display_name(),
                    lens.display_name()
                  );
                }
              }
            }
          }
          "Add new setup" => {
            let cameras = self.data_manager.get_cameras();
            let lenses = self.data_manager.get_lenses();

            if cameras.is_empty() {
              println!(
                "{}",
                "No cameras available. Please add cameras first.".yellow()
              );
              continue;
            }
            if lenses.is_empty() {
              println!(
                "{}",
                "No lenses available. Please add lenses first.".yellow()
              );
              continue;
            }

            if let Some(name) = PromptUtils::prompt_text("Setup name:")? {
              let camera_options: Vec<String> = cameras.iter().map(Camera::display_name).collect();
              if let Some(selected_camera_name) =
                PromptUtils::select_from_list("Select camera:", camera_options)?
              {
                let selected_camera = cameras
                  .iter()
                  .find(|c| c.display_name() == selected_camera_name)
                  .expect("Selected camera should exist");

                let lens_options: Vec<String> = lenses.iter().map(Lens::display_name).collect();
                if let Some(selected_lens_name) =
                  PromptUtils::select_from_list("Select lens:", lens_options)?
                {
                  let selected_lens = lenses
                    .iter()
                    .find(|l| l.display_name() == selected_lens_name)
                    .expect("Selected lens should exist");

                  match self
                    .data_manager
                    .add_setup(name, selected_camera.id, selected_lens.id)
                  {
                    Ok(setup) => {
                      self.data_manager.save()?;
                      println!(
                        "{}",
                        format!("✅ Added setup: {}", setup.display_name()).green()
                      );
                    }
                    Err(e) => {
                      println!("{}", format!("❌ Error: {e}").red());
                    }
                  }
                }
              }
            }
          }
          "Edit setup" => {
            let setups = self.data_manager.get_setups();
            if setups.is_empty() {
              println!("{}", "No setups to edit.".yellow());
            } else {
              let setup_options: Vec<String> = setups.iter().map(Setup::display_name).collect();
              if let Some(selected_name) =
                PromptUtils::select_from_list("Select setup to edit:", setup_options)?
              {
                if let Some(setup) = setups.iter().find(|s| s.display_name() == selected_name) {
                  let old_name = setup.display_name();

                  let cameras = self.data_manager.get_cameras();
                  let lenses = self.data_manager.get_lenses();

                  if cameras.is_empty() {
                    println!(
                      "{}",
                      "No cameras available. Please add cameras first.".yellow()
                    );
                    continue;
                  }
                  if lenses.is_empty() {
                    println!(
                      "{}",
                      "No lenses available. Please add lenses first.".yellow()
                    );
                    continue;
                  }

                  if let Some(name) =
                    PromptUtils::prompt_text_with_default("Setup name:", &setup.name)?
                  {
                    let camera_options: Vec<String> =
                      cameras.iter().map(Camera::display_name).collect();
                    if let Some(selected_camera_name) =
                      PromptUtils::select_from_list("Select camera:", camera_options)?
                    {
                      let selected_camera = cameras
                        .iter()
                        .find(|c| c.display_name() == selected_camera_name)
                        .expect("Selected camera should exist");

                      let lens_options: Vec<String> =
                        lenses.iter().map(Lens::display_name).collect();
                      if let Some(selected_lens_name) =
                        PromptUtils::select_from_list("Select lens:", lens_options)?
                      {
                        let selected_lens = lenses
                          .iter()
                          .find(|l| l.display_name() == selected_lens_name)
                          .expect("Selected lens should exist");

                        match self.data_manager.edit_setup(
                          setup.id,
                          name,
                          selected_camera.id,
                          selected_lens.id,
                        ) {
                          Ok(true) => {
                            self.data_manager.save()?;
                            println!("{}", format!("✅ Updated setup: {old_name}").green());
                          }
                          Ok(false) => {
                            println!("{}", "❌ Failed to update setup.".red());
                          }
                          Err(e) => {
                            println!("{}", format!("❌ Error: {e}").red());
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
          "Delete setup" => {
            let setups = self.data_manager.get_setups();
            if setups.is_empty() {
              println!("{}", "No setups to delete.".yellow());
            } else {
              let setup_options: Vec<String> = setups.iter().map(Setup::display_name).collect();
              if let Some(selected_name) =
                PromptUtils::select_from_list("Select setup to delete:", setup_options)?
              {
                if let Some(setup) = setups.iter().find(|s| s.display_name() == selected_name) {
                  let setup_id = setup.id;
                  let setup_name = setup.display_name();
                  self.data_manager.delete_setup(setup_id);
                  self.data_manager.save()?;
                  println!("{}", format!("✅ Deleted setup: {setup_name}").green());
                }
              }
            }
          }
          "Back" => break,
          _ => {}
        }
      } else {
        break;
      }
    }
    Ok(())
  }
}

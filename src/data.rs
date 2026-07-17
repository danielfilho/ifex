//! Data management layer for IFEX equipment and configuration operations.
//!
//! This module provides a high-level interface for managing photography equipment
//! data including cameras, lenses, films, photographers, and equipment setups.
//! It wraps the configuration system and provides CRUD operations.

use crate::{
  config::Config,
  models::{Camera, Film, Lens, Photographer, Selection, Setup},
};
use uuid::Uuid;

/// Data manager for handling all equipment and configuration operations.
///
/// This struct provides methods to add, retrieve, and delete photography equipment,
/// as well as create complete equipment selections for EXIF metadata application.
pub struct DataManager {
  config: Config,
}

impl DataManager {
  /// Creates a new `DataManager` by loading the configuration from disk.
  ///
  /// Returns an error if the configuration cannot be loaded.
  pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
    let config = Config::load()?;
    Ok(Self { config })
  }

  /// Saves the current configuration to disk.
  ///
  /// Returns an error if the configuration cannot be saved.
  pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
    self.config.save()
  }

  /// Adds a new camera to the configuration.
  ///
  /// Creates a new camera with the specified maker and model, adds it to the
  /// configuration, and returns the created camera.
  pub fn add_camera(&mut self, maker: String, model: String) -> Camera {
    let camera = Camera::new(maker, model);
    self.config.cameras.push(camera.clone());
    camera
  }

  /// Adds a new lens to the configuration.
  ///
  /// Creates a new lens with the specified parameters, adds it to the
  /// configuration, and returns the created lens.
  pub fn add_lens(
    &mut self,
    maker: String,
    model: String,
    focal_length: String,
    aperture: String,
    mount: String,
  ) -> Lens {
    let lens = Lens::new(maker, model, focal_length, aperture, mount);
    self.config.lenses.push(lens.clone());
    lens
  }

  /// Adds a new film stock to the configuration.
  ///
  /// Creates a new film with the specified maker, name, and ISO rating,
  /// adds it to the configuration, and returns the created film.
  pub fn add_film(&mut self, maker: String, name: String, iso: u32) -> Film {
    let film = Film::new(maker, name, iso);
    self.config.films.push(film.clone());
    film
  }

  /// Adds a new photographer to the configuration.
  ///
  /// Creates a new photographer with the specified name and optional email,
  /// adds it to the configuration, and returns the created photographer.
  pub fn add_photographer(&mut self, name: String, email: Option<String>) -> Photographer {
    let photographer = Photographer::new(name, email);
    self.config.photographers.push(photographer.clone());
    photographer
  }

  /// Adds a new equipment setup to the configuration.
  ///
  /// Creates a new setup that combines a camera and optionally a lens. Returns an error
  /// if the camera ID cannot be found in the configuration, or if a lens ID is provided
  /// but cannot be found.
  pub fn add_setup(
    &mut self,
    name: String,
    camera_id: Uuid,
    lens_id: Option<Uuid>,
  ) -> Result<Setup, String> {
    if !self.config.cameras.iter().any(|c| c.id == camera_id) {
      return Err("Camera not found".to_string());
    }
    if let Some(lens_id) = lens_id {
      if !self.config.lenses.iter().any(|l| l.id == lens_id) {
        return Err("Lens not found".to_string());
      }
    }

    let setup = Setup::new(name, camera_id, lens_id);
    self.config.setups.push(setup.clone());
    Ok(setup)
  }

  /// Returns a reference to all cameras in the configuration.
  #[must_use]
  pub const fn get_cameras(&self) -> &Vec<Camera> {
    &self.config.cameras
  }

  /// Returns a reference to all lenses in the configuration.
  #[must_use]
  pub const fn get_lenses(&self) -> &Vec<Lens> {
    &self.config.lenses
  }

  /// Returns a reference to all films in the configuration.
  #[must_use]
  pub const fn get_films(&self) -> &Vec<Film> {
    &self.config.films
  }

  /// Returns a reference to all photographers in the configuration.
  #[must_use]
  pub const fn get_photographers(&self) -> &Vec<Photographer> {
    &self.config.photographers
  }

  /// Returns a reference to all setups in the configuration.
  #[must_use]
  pub const fn get_setups(&self) -> &Vec<Setup> {
    &self.config.setups
  }

  /// Finds a camera by its unique ID.
  ///
  /// Returns `Some(&Camera)` if found, `None` otherwise.
  #[must_use]
  pub fn get_camera_by_id(&self, id: Uuid) -> Option<&Camera> {
    self.config.cameras.iter().find(|c| c.id == id)
  }

  /// Finds a lens by its unique ID.
  ///
  /// Returns `Some(&Lens)` if found, `None` otherwise.
  #[must_use]
  pub fn get_lens_by_id(&self, id: Uuid) -> Option<&Lens> {
    self.config.lenses.iter().find(|l| l.id == id)
  }

  /// Finds a film by its unique ID.
  ///
  /// Returns `Some(&Film)` if found, `None` otherwise.
  #[must_use]
  pub fn get_film_by_id(&self, id: Uuid) -> Option<&Film> {
    self.config.films.iter().find(|f| f.id == id)
  }

  /// Finds a photographer by their unique ID.
  ///
  /// Returns `Some(&Photographer)` if found, `None` otherwise.
  #[must_use]
  pub fn get_photographer_by_id(&self, id: Uuid) -> Option<&Photographer> {
    self.config.photographers.iter().find(|p| p.id == id)
  }

  /// Finds a setup by its unique ID.
  ///
  /// Returns `Some(&Setup)` if found, `None` otherwise.
  #[must_use]
  pub fn get_setup_by_id(&self, id: Uuid) -> Option<&Setup> {
    self.config.setups.iter().find(|s| s.id == id)
  }

  /// Creates a complete equipment selection for EXIF metadata application.
  ///
  /// Combines a setup (camera + optional lens), film, and photographer into a single
  /// Selection object that contains all necessary information for applying
  /// EXIF metadata to images. Returns an error if any of the specified IDs
  /// cannot be found in the configuration.
  pub fn create_selection(
    &self,
    setup_id: Uuid,
    film_id: Uuid,
    photographer_id: Uuid,
  ) -> Result<Selection, String> {
    let setup = self.get_setup_by_id(setup_id).ok_or("Setup not found")?;
    let camera = self
      .get_camera_by_id(setup.camera_id)
      .ok_or("Camera not found")?;
    let lens = if let Some(lens_id) = setup.lens_id {
      Some(
        self
          .get_lens_by_id(lens_id)
          .ok_or("Lens not found")?
          .clone(),
      )
    } else {
      None
    };
    let film = self.get_film_by_id(film_id).ok_or("Film not found")?;
    let photographer = self
      .get_photographer_by_id(photographer_id)
      .ok_or("Photographer not found")?;

    Ok(Selection {
      setup: setup.clone(),
      camera: camera.clone(),
      lens,
      film: film.clone(),
      photographer: photographer.clone(),
    })
  }

  /// Deletes a camera from the configuration.
  ///
  /// Returns an error if the camera is currently used in any setups.
  /// This prevents data integrity issues by ensuring referenced cameras
  /// are not deleted.
  pub fn delete_camera(&mut self, id: Uuid) -> Result<(), String> {
    if self.config.setups.iter().any(|s| s.camera_id == id) {
      return Err("Cannot delete camera that is used in setups".to_string());
    }
    self.config.cameras.retain(|c| c.id != id);
    Ok(())
  }

  /// Deletes a lens from the configuration.
  ///
  /// Returns an error if the lens is currently used in any setups.
  /// This prevents data integrity issues by ensuring referenced lenses
  /// are not deleted.
  pub fn delete_lens(&mut self, id: Uuid) -> Result<(), String> {
    if self.config.setups.iter().any(|s| s.lens_id == Some(id)) {
      return Err("Cannot delete lens that is used in setups".to_string());
    }
    self.config.lenses.retain(|l| l.id != id);
    Ok(())
  }

  /// Deletes a film from the configuration.
  ///
  /// Films can be safely deleted without checking for references
  /// since they are not referenced by other entities.
  pub fn delete_film(&mut self, id: Uuid) {
    self.config.films.retain(|f| f.id != id);
  }

  /// Deletes a photographer from the configuration.
  ///
  /// Photographers can be safely deleted without checking for references
  /// since they are not referenced by other entities.
  pub fn delete_photographer(&mut self, id: Uuid) {
    self.config.photographers.retain(|p| p.id != id);
  }

  /// Deletes a setup from the configuration.
  ///
  /// Setups can be safely deleted without checking for references
  /// since they are not referenced by other entities.
  pub fn delete_setup(&mut self, id: Uuid) {
    self.config.setups.retain(|s| s.id != id);
  }

  /// Updates an existing camera in the configuration.
  ///
  /// Returns true if the camera was found and updated, false otherwise.
  pub fn edit_camera(&mut self, id: Uuid, maker: String, model: String) -> bool {
    if let Some(camera) = self.config.cameras.iter_mut().find(|c| c.id == id) {
      camera.maker = maker;
      camera.model = model;
      true
    } else {
      false
    }
  }

  /// Updates an existing lens in the configuration.
  ///
  /// Returns true if the lens was found and updated, false otherwise.
  pub fn edit_lens(
    &mut self,
    id: Uuid,
    maker: String,
    model: String,
    focal_length: String,
    aperture: String,
    mount: String,
  ) -> bool {
    if let Some(lens) = self.config.lenses.iter_mut().find(|l| l.id == id) {
      lens.maker = maker;
      lens.model = model;
      lens.focal_length = focal_length;
      lens.aperture = aperture;
      lens.mount = mount;
      true
    } else {
      false
    }
  }

  /// Updates an existing film in the configuration.
  ///
  /// Returns true if the film was found and updated, false otherwise.
  pub fn edit_film(&mut self, id: Uuid, maker: String, name: String, iso: u32) -> bool {
    if let Some(film) = self.config.films.iter_mut().find(|f| f.id == id) {
      film.maker = maker;
      film.name = name;
      film.iso = iso;
      true
    } else {
      false
    }
  }

  /// Updates an existing photographer in the configuration.
  ///
  /// Returns true if the photographer was found and updated, false otherwise.
  pub fn edit_photographer(&mut self, id: Uuid, name: String, email: Option<String>) -> bool {
    if let Some(photographer) = self.config.photographers.iter_mut().find(|p| p.id == id) {
      photographer.name = name;
      photographer.email = email;
      true
    } else {
      false
    }
  }

  /// Updates an existing setup in the configuration.
  ///
  /// Returns true if the setup was found and updated, false otherwise.
  pub fn edit_setup(
    &mut self,
    id: Uuid,
    name: String,
    camera_id: Uuid,
    lens_id: Option<Uuid>,
  ) -> Result<bool, String> {
    // Validate that the referenced camera exists
    if self.get_camera_by_id(camera_id).is_none() {
      return Err("Camera not found".to_string());
    }
    // Validate that the lens exists if one is provided
    if let Some(lens_id) = lens_id {
      if self.get_lens_by_id(lens_id).is_none() {
        return Err("Lens not found".to_string());
      }
    }

    if let Some(setup) = self.config.setups.iter_mut().find(|s| s.id == id) {
      setup.name = name;
      setup.camera_id = camera_id;
      setup.lens_id = lens_id;
      Ok(true)
    } else {
      Ok(false)
    }
  }
}

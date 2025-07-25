//! Data model definitions for IFEX equipment and selections

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Camera equipment model
/// Camera equipment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
  /// Unique identifier for the camera
  pub id: Uuid,
  /// Camera manufacturer (e.g., "Canon", "Nikon")
  pub maker: String,
  /// Camera model name (e.g., "EOS R5", "D850")
  pub model: String,
  /// Timestamp when the camera was added to the system
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

impl Camera {
  /// Creates a new camera with the specified maker and model.
  ///
  /// Automatically generates a unique ID and sets the creation timestamp.
  #[must_use]
  pub fn new(maker: String, model: String) -> Self {
    Self {
      id: Uuid::new_v4(),
      maker,
      model,
      created_at: Utc::now(),
    }
  }

  /// Returns a human-readable display name for the camera.
  ///
  /// Format: "Maker Model" (e.g., "Canon EOS R5")
  #[must_use]
  pub fn display_name(&self) -> String {
    format!("{} {}", self.maker, self.model)
  }
}

/// Lens equipment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lens {
  /// Unique identifier for the lens
  pub id: Uuid,
  /// Lens manufacturer (e.g., "Canon", "Sigma")
  pub maker: String,
  /// Lens model name
  pub model: String,
  /// Focal length specification (e.g., "50", "24-70")
  #[serde(rename = "focalLength")]
  pub focal_length: String,
  /// Maximum aperture specification (e.g., "1.4", "2.8")
  pub aperture: String,
  /// Lens mount type (e.g., "EF", "Z", "E")
  pub mount: String,
  /// Timestamp when the lens was added to the system
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

impl Lens {
  /// Creates a new lens with the specified parameters.
  ///
  /// Automatically generates a unique ID and sets the creation timestamp.
  #[must_use]
  pub fn new(
    maker: String,
    model: String,
    focal_length: String,
    aperture: String,
    mount: String,
  ) -> Self {
    Self {
      id: Uuid::new_v4(),
      maker,
      model,
      focal_length,
      aperture,
      mount,
      created_at: Utc::now(),
    }
  }

  /// Returns a human-readable display name for the lens.
  ///
  /// Format: "Maker Model Focalmm f/Aperture" (e.g., "Canon EF 50mm f/1.4")
  #[must_use]
  pub fn display_name(&self) -> String {
    format!(
      "{} {} {}mm f/{}",
      self.maker, self.model, self.focal_length, self.aperture
    )
  }

  /// Returns the lens model name combined with its aperture specification.
  ///
  /// Format: "Model f/Aperture" (e.g., "EF 50mm f/1.4")
  #[must_use]
  pub fn lens_model_with_aperture(&self) -> String {
    format!("{} f/{}", self.model, self.aperture)
  }

  /// Returns the complete lens model for EXIF including focal length.
  ///
  /// Format: "Model `FocalLength` f/Aperture" (e.g., "Summicron 35mm f/2")
  #[must_use]
  pub fn complete_lens_model(&self) -> String {
    format!("{} {}mm f/{}", self.model, self.focal_length, self.aperture)
  }
}

/// Film stock model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Film {
  /// Unique identifier for the film
  pub id: Uuid,
  /// Film manufacturer (e.g., "Kodak", "Fujifilm")
  pub maker: String,
  /// Film stock name (e.g., "Tri-X", "Velvia 50")
  pub name: String,
  /// ISO/ASA rating of the film
  pub iso: u32,
  /// Timestamp when the film was added to the system
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

impl Film {
  /// Creates a new film stock with the specified parameters.
  ///
  /// Automatically generates a unique ID and sets the creation timestamp.
  #[must_use]
  pub fn new(maker: String, name: String, iso: u32) -> Self {
    Self {
      id: Uuid::new_v4(),
      maker,
      name,
      iso,
      created_at: Utc::now(),
    }
  }

  /// Returns a human-readable display name for the film.
  ///
  /// Format: "Maker Name (ISO rating)" (e.g., "Kodak Tri-X (ISO 400)")
  #[must_use]
  pub fn display_name(&self) -> String {
    format!("{} {} (ISO {})", self.maker, self.name, self.iso)
  }
}

/// Photographer model for attribution and contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photographer {
  /// Unique identifier for the photographer
  pub id: Uuid,
  /// Photographer's name
  pub name: String,
  /// Optional email address for the photographer
  pub email: Option<String>,
  /// Timestamp when the photographer was added to the system
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

impl Photographer {
  /// Creates a new photographer with the specified name and optional email.
  ///
  /// Automatically generates a unique ID and sets the creation timestamp.
  #[must_use]
  pub fn new(name: String, email: Option<String>) -> Self {
    Self {
      id: Uuid::new_v4(),
      name,
      email,
      created_at: Utc::now(),
    }
  }

  /// Returns a human-readable display name for the photographer.
  ///
  /// If email is provided, format: "Name <email>"
  /// Otherwise, format: "Name"
  #[must_use]
  pub fn display_name(&self) -> String {
    self.email.as_ref().map_or_else(
      || self.name.clone(),
      |email| format!("{} <{}>", self.name, email),
    )
  }
}

/// Equipment setup combining a camera and lens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setup {
  /// Unique identifier for the setup
  pub id: Uuid,
  /// User-defined name for the setup
  pub name: String,
  /// Reference to the camera used in this setup
  #[serde(rename = "cameraId")]
  pub camera_id: Uuid,
  /// Reference to the lens used in this setup
  #[serde(rename = "lensId")]
  pub lens_id: Uuid,
  /// Timestamp when the setup was created
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

impl Setup {
  /// Creates a new equipment setup with the specified name and equipment IDs.
  ///
  /// Automatically generates a unique ID and sets the creation timestamp.
  #[must_use]
  pub fn new(name: String, camera_id: Uuid, lens_id: Uuid) -> Self {
    Self {
      id: Uuid::new_v4(),
      name,
      camera_id,
      lens_id,
      created_at: Utc::now(),
    }
  }

  /// Returns the display name for the setup.
  ///
  /// Currently just returns the user-defined name.
  #[must_use]
  pub fn display_name(&self) -> String {
    self.name.clone()
  }
}

/// Complete equipment selection for EXIF metadata application.
///
/// This struct combines all the necessary equipment and photographer information
/// needed to apply comprehensive EXIF metadata to images. It includes the setup
/// (camera + lens combination), film stock, and photographer details.
#[derive(Debug, Clone)]
pub struct Selection {
  /// The equipment setup (camera + lens combination)
  pub setup: Setup,
  /// The camera used for the photographs
  pub camera: Camera,
  /// The lens used for the photographs
  pub lens: Lens,
  /// The film stock used for the photographs
  pub film: Film,
  /// The photographer who took the photographs
  pub photographer: Photographer,
}

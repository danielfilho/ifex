//! Integration tests for the ifex library workflows.

use ifex::config::Config;
use ifex::models::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_full_workflow_config_and_models() {
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join("test_config.json");

  // Create equipment
  let camera = Camera::new("Canon".to_string(), "EOS R5".to_string());
  let lens = Lens::new(
    "Canon".to_string(),
    "EF 85mm".to_string(),
    "85".to_string(),
    "1.2".to_string(),
    "EF".to_string(),
  );
  let film = Film::new("Fujifilm".to_string(), "Velvia 50".to_string(), 50);
  let photographer = Photographer::new(
    "Jane Smith".to_string(),
    Some("jane@example.com".to_string()),
  );
  let setup = Setup::new("Portrait Setup".to_string(), camera.id, Some(lens.id));

  // Create config
  let mut config = Config::default();
  config.cameras.push(camera);
  config.lenses.push(lens);
  config.films.push(film);
  config.photographers.push(photographer);
  config.setups.push(setup);

  // Save config
  let content = serde_json::to_string_pretty(&config).unwrap();
  fs::write(&config_path, content).unwrap();

  // Load and verify
  let loaded_content = fs::read_to_string(&config_path).unwrap();
  let loaded_config: Config = serde_json::from_str(&loaded_content).unwrap();

  assert_eq!(loaded_config.cameras.len(), 1);
  assert_eq!(loaded_config.lenses.len(), 1);
  assert_eq!(loaded_config.films.len(), 1);
  assert_eq!(loaded_config.photographers.len(), 1);
  assert_eq!(loaded_config.setups.len(), 1);

  // Verify data integrity
  let loaded_camera = &loaded_config.cameras[0];
  let loaded_lens = &loaded_config.lenses[0];
  let loaded_setup = &loaded_config.setups[0];

  assert_eq!(loaded_camera.display_name(), "Canon EOS R5");
  assert_eq!(loaded_lens.display_name(), "Canon EF 85mm 85mm f/1.2");
  assert_eq!(loaded_setup.camera_id, loaded_camera.id);
  assert_eq!(loaded_setup.lens_id, Some(loaded_lens.id));
}


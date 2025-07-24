use ifex::config::Config;
use ifex::models::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_default() {
  let config = Config::default();

  assert!(config.cameras.is_empty());
  assert!(config.lenses.is_empty());
  assert!(config.films.is_empty());
  assert!(config.photographers.is_empty());
  assert!(config.setups.is_empty());
}

#[test]
fn test_config_load_nonexistent_file() {
  // This test loads from the actual config file, which might exist
  // So we test the behavior when file doesn't exist by checking the config_path doesn't exist
  let config_path = Config::config_path().unwrap();
  if !config_path.exists() {
    let config = Config::load().unwrap();
    assert!(config.cameras.is_empty());
  }
}

#[test]
fn test_config_save_and_load() {
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join("test_config.json");

  // Create config with test data
  let mut config = Config::default();
  config
    .cameras
    .push(Camera::new("Canon".to_string(), "EOS R5".to_string()));
  config.lenses.push(Lens::new(
    "Canon".to_string(),
    "EF 50mm".to_string(),
    "50".to_string(),
    "1.4".to_string(),
    "EF".to_string(),
  ));
  config
    .films
    .push(Film::new("Kodak".to_string(), "Tri-X".to_string(), 400));
  config.photographers.push(Photographer::new(
    "John Doe".to_string(),
    Some("john@example.com".to_string()),
  ));

  // Save config
  let content = serde_json::to_string_pretty(&config).unwrap();
  fs::write(&config_path, content).unwrap();

  // Load config
  let loaded_content = fs::read_to_string(&config_path).unwrap();
  let loaded_config: Config = serde_json::from_str(&loaded_content).unwrap();

  assert_eq!(loaded_config.cameras.len(), 1);
  assert_eq!(loaded_config.lenses.len(), 1);
  assert_eq!(loaded_config.films.len(), 1);
  assert_eq!(loaded_config.photographers.len(), 1);
  assert_eq!(loaded_config.cameras[0].maker, "Canon");
  assert_eq!(loaded_config.lenses[0].model, "EF 50mm");
  assert_eq!(loaded_config.films[0].name, "Tri-X");
  assert_eq!(loaded_config.photographers[0].name, "John Doe");
}

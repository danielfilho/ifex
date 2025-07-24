use ifex::models::*;
use uuid::Uuid;

#[test]
fn test_camera_new() {
  let camera = Camera::new("Canon".to_string(), "EOS R5".to_string());

  assert_eq!(camera.maker, "Canon");
  assert_eq!(camera.model, "EOS R5");
  assert!(!camera.id.is_nil());
}

#[test]
fn test_camera_display_name() {
  let camera = Camera::new("Canon".to_string(), "EOS R5".to_string());
  assert_eq!(camera.display_name(), "Canon EOS R5");
}

#[test]
fn test_lens_new() {
  let lens = Lens::new(
    "Canon".to_string(),
    "EF 50mm".to_string(),
    "50".to_string(),
    "1.4".to_string(),
    "EF".to_string(),
  );

  assert_eq!(lens.maker, "Canon");
  assert_eq!(lens.model, "EF 50mm");
  assert_eq!(lens.focal_length, "50");
  assert_eq!(lens.aperture, "1.4");
  assert_eq!(lens.mount, "EF");
  assert!(!lens.id.is_nil());
}

#[test]
fn test_lens_display_name() {
  let lens = Lens::new(
    "Canon".to_string(),
    "EF 50mm".to_string(),
    "50".to_string(),
    "1.4".to_string(),
    "EF".to_string(),
  );
  assert_eq!(lens.display_name(), "Canon EF 50mm 50mm f/1.4");
}

#[test]
fn test_lens_model_with_aperture() {
  let lens = Lens::new(
    "Canon".to_string(),
    "EF 50mm".to_string(),
    "50".to_string(),
    "1.4".to_string(),
    "EF".to_string(),
  );
  assert_eq!(lens.lens_model_with_aperture(), "EF 50mm f/1.4");
}

#[test]
fn test_film_new() {
  let film = Film::new("Kodak".to_string(), "Tri-X".to_string(), 400);

  assert_eq!(film.maker, "Kodak");
  assert_eq!(film.name, "Tri-X");
  assert_eq!(film.iso, 400);
  assert!(!film.id.is_nil());
}

#[test]
fn test_film_display_name() {
  let film = Film::new("Kodak".to_string(), "Tri-X".to_string(), 400);
  assert_eq!(film.display_name(), "Kodak Tri-X (ISO 400)");
}

#[test]
fn test_photographer_new_with_email() {
  let photographer =
    Photographer::new("John Doe".to_string(), Some("john@example.com".to_string()));

  assert_eq!(photographer.name, "John Doe");
  assert_eq!(photographer.email, Some("john@example.com".to_string()));
  assert!(!photographer.id.is_nil());
}

#[test]
fn test_photographer_new_without_email() {
  let photographer = Photographer::new("Jane Doe".to_string(), None);

  assert_eq!(photographer.name, "Jane Doe");
  assert_eq!(photographer.email, None);
  assert!(!photographer.id.is_nil());
}

#[test]
fn test_photographer_display_name_with_email() {
  let photographer =
    Photographer::new("John Doe".to_string(), Some("john@example.com".to_string()));
  assert_eq!(photographer.display_name(), "John Doe <john@example.com>");
}

#[test]
fn test_photographer_display_name_without_email() {
  let photographer = Photographer::new("Jane Doe".to_string(), None);
  assert_eq!(photographer.display_name(), "Jane Doe");
}

#[test]
fn test_setup_new() {
  let camera_id = Uuid::new_v4();
  let lens_id = Uuid::new_v4();
  let setup = Setup::new("My Setup".to_string(), camera_id, lens_id);

  assert_eq!(setup.name, "My Setup");
  assert_eq!(setup.camera_id, camera_id);
  assert_eq!(setup.lens_id, lens_id);
  assert!(!setup.id.is_nil());
}

#[test]
fn test_setup_display_name() {
  let camera_id = Uuid::new_v4();
  let lens_id = Uuid::new_v4();
  let setup = Setup::new("My Setup".to_string(), camera_id, lens_id);
  assert_eq!(setup.display_name(), "My Setup");
}

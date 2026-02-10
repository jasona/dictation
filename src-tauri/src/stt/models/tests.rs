use super::*;
use std::fs;

/// Create a unique temp dir per test to avoid parallel test interference.
fn unique_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join("vozr_test")
        .join(name);
    let _ = fs::remove_dir_all(&dir);
    let _ = fs::create_dir_all(&dir);
    dir
}

#[test]
fn list_models_returns_all_catalog_entries() {
    let dir = unique_dir("list_all");
    let models = list_models(&dir);
    assert_eq!(models.len(), 6);

    let ids: Vec<&str> = models.iter().map(|m| m.id.as_str()).collect();
    assert!(ids.contains(&"tiny.en"));
    assert!(ids.contains(&"base.en"));
    assert!(ids.contains(&"small.en"));
    assert!(ids.contains(&"small"));
    assert!(ids.contains(&"medium.en"));
    assert!(ids.contains(&"medium"));
}

#[test]
fn list_models_reports_not_downloaded_by_default() {
    let dir = unique_dir("list_not_dl");
    let models = list_models(&dir);
    for model in &models {
        assert!(!model.downloaded, "Model {} should not be downloaded", model.id);
    }
}

#[test]
fn model_path_returns_correct_path() {
    let dir = unique_dir("path_correct");
    let path = model_path(&dir, "base.en");
    assert!(path.is_some());
    let p = path.unwrap();
    assert!(p.ends_with("ggml-base.en.bin"));
}

#[test]
fn model_path_unknown_returns_none() {
    let dir = unique_dir("path_unknown");
    assert!(model_path(&dir, "nonexistent").is_none());
}

#[test]
fn is_downloaded_false_when_missing() {
    let dir = unique_dir("dl_false");
    assert!(!is_downloaded(&dir, "tiny.en"));
}

#[test]
fn is_downloaded_true_when_file_exists() {
    let dir = unique_dir("dl_true");
    let path = model_path(&dir, "tiny.en").unwrap();
    fs::write(&path, b"fake model data").unwrap();

    assert!(is_downloaded(&dir, "tiny.en"));
}

#[test]
fn delete_model_removes_file() {
    let dir = unique_dir("delete_file");
    let path = model_path(&dir, "tiny.en").unwrap();
    fs::write(&path, b"fake model data").unwrap();

    assert!(path.exists());
    delete_model(&dir, "tiny.en").unwrap();
    assert!(!path.exists());
}

#[test]
fn delete_model_unknown_returns_error() {
    let dir = unique_dir("delete_unknown");
    let result = delete_model(&dir, "nonexistent");
    assert!(result.is_err());
}

#[test]
fn delete_model_not_downloaded_is_ok() {
    let dir = unique_dir("delete_not_dl");
    let result = delete_model(&dir, "tiny.en");
    assert!(result.is_ok());
}

#[test]
fn models_dir_creates_directory() {
    let dir = unique_dir("models_dir_create");
    let mdir = models_dir(&dir);
    assert!(mdir.exists());
    assert!(mdir.ends_with("whisper"));
}

#[test]
fn model_sizes_are_reasonable() {
    let dir = unique_dir("sizes");
    let models = list_models(&dir);

    let tiny = models.iter().find(|m| m.id == "tiny.en").unwrap();
    assert!(tiny.size_bytes > 50_000_000 && tiny.size_bytes < 100_000_000);

    let base = models.iter().find(|m| m.id == "base.en").unwrap();
    assert!(base.size_bytes > 100_000_000 && base.size_bytes < 200_000_000);

    let medium = models.iter().find(|m| m.id == "medium.en").unwrap();
    assert!(medium.size_bytes > 1_000_000_000);
}

use super::*;
use std::path::PathBuf;

// ---- Model catalog tests (always run) ----

#[test]
fn catalog_has_models() {
    assert!(!LLM_MODEL_CATALOG.is_empty());
}

#[test]
fn catalog_entry_has_valid_fields() {
    for entry in LLM_MODEL_CATALOG {
        assert!(!entry.id.is_empty());
        assert!(!entry.name.is_empty());
        assert!(!entry.filename.is_empty());
        assert!(!entry.url.is_empty());
        assert!(entry.size_bytes > 0);
        assert!(!entry.description.is_empty());
        assert!(entry.filename.ends_with(".gguf"));
    }
}

#[test]
fn model_path_returns_none_for_unknown() {
    let dir = PathBuf::from("/tmp/test-llm-models");
    assert!(model_path(&dir, "nonexistent-model").is_none());
}

#[test]
fn model_path_returns_path_for_known() {
    let dir = PathBuf::from("/tmp/test-llm-models");
    let path = model_path(&dir, "phi3-mini-q4");
    assert!(path.is_some());
    let path = path.unwrap();
    assert!(path.to_string_lossy().contains("Phi-3-mini-4k-instruct-q4.gguf"));
}

#[test]
fn list_models_includes_all_catalog_entries() {
    let dir = PathBuf::from("/tmp/test-llm-models-list");
    let models = list_models(&dir);
    assert_eq!(models.len(), LLM_MODEL_CATALOG.len());
    assert_eq!(models[0].id, "phi3-mini-q4");
    assert!(!models[0].downloaded); // Not actually downloaded
}

#[test]
fn is_downloaded_false_for_missing() {
    let dir = PathBuf::from("/tmp/test-llm-models-missing");
    assert!(!is_downloaded(&dir, "phi3-mini-q4"));
}

#[test]
fn engine_starts_unloaded() {
    let engine = LlmEngine::new();
    assert!(!engine.is_loaded());
    assert!(engine.current_model_id().is_none());
}

#[test]
fn engine_clean_text_fails_when_not_loaded() {
    let engine = LlmEngine::new();
    let result = engine.clean_text("hello");
    assert!(result.is_err());
}

#[test]
fn engine_load_fails_without_feature() {
    // When local-llm feature is disabled, load_model should return an error
    #[cfg(not(feature = "local-llm"))]
    {
        let engine = LlmEngine::new();
        let result = engine.load_model("/nonexistent/path", "test");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not enabled"));
    }
}

// ---- Engine tests requiring downloaded model ----

#[test]
#[ignore]
fn engine_load_and_clean() {
    let engine = LlmEngine::new();
    // This test requires the phi3-mini-q4 model to be downloaded
    let model_path = std::env::var("LLM_TEST_MODEL_PATH")
        .expect("Set LLM_TEST_MODEL_PATH to run this test");

    engine.load_model(&model_path, "phi3-mini-q4").unwrap();
    assert!(engine.is_loaded());
    assert_eq!(engine.current_model_id(), Some("phi3-mini-q4".to_string()));

    let result = engine.clean_text("um so basically i went to the store").unwrap();
    assert!(!result.is_empty());

    engine.unload_model();
    assert!(!engine.is_loaded());
}

#[test]
#[ignore]
fn engine_clean_latency_under_5s() {
    let engine = LlmEngine::new();
    let model_path = std::env::var("LLM_TEST_MODEL_PATH")
        .expect("Set LLM_TEST_MODEL_PATH to run this test");

    engine.load_model(&model_path, "phi3-mini-q4").unwrap();

    let start = std::time::Instant::now();
    let _ = engine.clean_text("um so basically i went to the store");
    let elapsed = start.elapsed();

    engine.unload_model();

    assert!(
        elapsed.as_secs() < 5,
        "Cleanup took {:?}, expected <5s",
        elapsed
    );
}

use super::*;

#[test]
fn engine_starts_unloaded() {
    let engine = WhisperEngine::new();
    assert!(!engine.is_loaded());
    assert!(engine.current_model_id().is_none());
}

#[test]
fn load_nonexistent_model_returns_error() {
    let engine = WhisperEngine::new();
    let result = engine.load_model("/nonexistent/path/model.bin", "test", false);
    assert!(result.is_err());
    assert!(!engine.is_loaded());
}

#[test]
fn unload_when_not_loaded_is_safe() {
    let engine = WhisperEngine::new();
    engine.unload_model(); // Should not panic
    assert!(!engine.is_loaded());
}

#[test]
fn transcribe_without_model_returns_error() {
    let engine = WhisperEngine::new();
    let audio = vec![0.0f32; 16000]; // 1 second of silence
    let result = engine.transcribe(&audio, Some("en"));
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No Whisper model loaded"));
}

#[test]
fn available_backends_includes_cpu() {
    let backends = available_backends();
    assert!(!backends.is_empty());

    let cpu = backends.iter().find(|b| b.id == "cpu").unwrap();
    assert!(cpu.available);
    assert_eq!(cpu.name, "CPU");
}

#[test]
fn available_backends_lists_gpu_options() {
    let backends = available_backends();

    // CUDA and Vulkan should be listed (even if not available)
    assert!(backends.iter().any(|b| b.id == "cuda"));
    assert!(backends.iter().any(|b| b.id == "vulkan"));
}

#[test]
fn gpu_backends_not_available_without_features() {
    let backends = available_backends();

    // Without cuda/vulkan features enabled, these should be unavailable
    let cuda = backends.iter().find(|b| b.id == "cuda").unwrap();
    assert!(!cuda.available);

    let vulkan = backends.iter().find(|b| b.id == "vulkan").unwrap();
    assert!(!vulkan.available);
}

#[test]
fn transcription_result_fields() {
    // Verify the struct can be constructed and serialized
    let result = TranscriptionResult {
        text: "Hello world".to_string(),
        duration_ms: 1500,
        audio_duration_ms: 5000,
    };

    assert_eq!(result.text, "Hello world");
    assert_eq!(result.duration_ms, 1500);
    assert_eq!(result.audio_duration_ms, 5000);

    // Should be serializable
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("Hello world"));
}

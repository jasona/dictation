pub mod models;
pub mod whisper;

use models::ModelInfo;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, Runtime};
use whisper::{GpuBackendInfo, TranscriptionResult, WhisperEngine};

/// Tauri-managed state for the STT subsystem.
pub struct SttState {
    pub engine: WhisperEngine,
    /// App data directory for model storage.
    pub app_data_dir: Mutex<Option<PathBuf>>,
    /// User's preferred GPU backend ("cpu", "cuda", "vulkan").
    pub gpu_backend: Mutex<String>,
}

impl SttState {
    pub fn new() -> Self {
        Self {
            engine: WhisperEngine::new(),
            app_data_dir: Mutex::new(None),
            gpu_backend: Mutex::new("cpu".to_string()),
        }
    }

    fn data_dir(&self) -> Result<PathBuf, String> {
        self.app_data_dir
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| "App data directory not configured".to_string())
    }
}

/// Try to preload the user's selected Whisper model on startup.
/// Non-fatal — logs warnings but does not fail.
pub fn preload_model<R: Runtime>(app: &AppHandle<R>, state: &SttState, model_id: &str) {
    let data_dir = match state.data_dir() {
        Ok(d) => d,
        Err(_) => return,
    };

    let path = match models::model_path(&data_dir, model_id) {
        Some(p) if p.exists() => p,
        _ => {
            log::info!(
                "Whisper model '{}' not downloaded, skipping preload",
                model_id
            );
            return;
        }
    };

    let use_gpu = *state.gpu_backend.lock().unwrap() != "cpu";
    let path_str = path.to_string_lossy().to_string();

    // Load on a background thread to not block startup
    let engine_model_id = model_id.to_string();
    let stt_state: tauri::State<'_, SttState> = app.state();

    match stt_state.engine.load_model(&path_str, &engine_model_id, use_gpu) {
        Ok(()) => log::info!("Whisper model '{}' preloaded", model_id),
        Err(e) => {
            log::warn!("Failed to preload Whisper model '{}': {}", model_id, e);
            let _ = app.emit(
                "stt://model-load-error",
                format!(
                    "Failed to load speech model '{}'. Try re-downloading or using a smaller model.",
                    model_id
                ),
            );
            crate::tray::set_state(app, crate::tray::TrayState::Error);
        }
    }
}

// --------------- Tauri commands ---------------

#[tauri::command]
pub fn list_whisper_models(state: tauri::State<'_, SttState>) -> Result<Vec<ModelInfo>, String> {
    let data_dir = state.data_dir()?;
    Ok(models::list_models(&data_dir))
}

#[tauri::command]
pub fn download_whisper_model(
    app: AppHandle,
    model_id: String,
    state: tauri::State<'_, SttState>,
) -> Result<String, String> {
    let data_dir = state.data_dir()?;

    // Run download on a background thread (blocking I/O)
    let path = models::download_model(&app, &data_dir, &model_id)?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn delete_whisper_model(
    model_id: String,
    state: tauri::State<'_, SttState>,
) -> Result<(), String> {
    let data_dir = state.data_dir()?;

    // If the deleted model is currently loaded, unload it
    if state.engine.current_model_id().as_deref() == Some(&model_id) {
        state.engine.unload_model();
    }

    models::delete_model(&data_dir, &model_id)
}

#[tauri::command]
pub fn load_whisper_model(
    model_id: String,
    state: tauri::State<'_, SttState>,
) -> Result<(), String> {
    let data_dir = state.data_dir()?;

    let path = models::model_path(&data_dir, &model_id)
        .ok_or_else(|| format!("Unknown model: {}", model_id))?;

    if !path.exists() {
        return Err(format!(
            "Model '{}' is not downloaded. Download it first.",
            model_id
        ));
    }

    let use_gpu = *state.gpu_backend.lock().unwrap() != "cpu";
    state
        .engine
        .load_model(&path.to_string_lossy(), &model_id, use_gpu)
}

#[tauri::command]
pub fn unload_whisper_model(state: tauri::State<'_, SttState>) {
    state.engine.unload_model();
}

#[tauri::command]
pub fn get_current_whisper_model(state: tauri::State<'_, SttState>) -> Option<String> {
    state.engine.current_model_id()
}

#[tauri::command]
pub fn transcribe(
    audio: Vec<f32>,
    state: tauri::State<'_, SttState>,
) -> Result<TranscriptionResult, String> {
    state.engine.transcribe(&audio, Some("en"))
}

#[tauri::command]
pub fn get_gpu_backends() -> Vec<GpuBackendInfo> {
    whisper::available_backends()
}

#[tauri::command]
pub fn set_gpu_backend(
    backend: String,
    state: tauri::State<'_, SttState>,
) -> Result<(), String> {
    let backends = whisper::available_backends();
    if !backends.iter().any(|b| b.id == backend && b.available) {
        return Err(format!("GPU backend '{}' is not available", backend));
    }
    *state.gpu_backend.lock().unwrap() = backend;
    Ok(())
}

#[tauri::command]
pub fn get_gpu_backend(state: tauri::State<'_, SttState>) -> String {
    state.gpu_backend.lock().unwrap().clone()
}

use std::sync::Mutex;
use std::time::Instant;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// Wrapper around WhisperContext for thread-safe access.
/// WhisperContext is not Send/Sync, so we hold it behind a Mutex
/// and only access it from one thread at a time.
pub struct WhisperEngine {
    context: Mutex<Option<WhisperContextWrapper>>,
    current_model_id: Mutex<Option<String>>,
}

/// Wrapper to make WhisperContext usable across threads.
/// Safety: WhisperContext uses FFI pointers internally. We ensure single-threaded
/// access through the outer Mutex in WhisperEngine.
struct WhisperContextWrapper(WhisperContext);
unsafe impl Send for WhisperContextWrapper {}

impl WhisperEngine {
    pub fn new() -> Self {
        Self {
            context: Mutex::new(None),
            current_model_id: Mutex::new(None),
        }
    }

    /// Load a Whisper GGML model from the given file path.
    pub fn load_model(&self, model_path: &str, model_id: &str, use_gpu: bool) -> Result<(), String> {
        let mut params = WhisperContextParameters::default();
        params.use_gpu(use_gpu);

        let ctx = WhisperContext::new_with_params(model_path, params)
            .map_err(|e| format!("Failed to load Whisper model '{}': {:?}", model_path, e))?;

        log::info!("Whisper model '{}' loaded from {}", model_id, model_path);

        *self.context.lock().unwrap() = Some(WhisperContextWrapper(ctx));
        *self.current_model_id.lock().unwrap() = Some(model_id.to_string());
        Ok(())
    }

    /// Unload the current model, freeing memory.
    pub fn unload_model(&self) {
        *self.context.lock().unwrap() = None;
        *self.current_model_id.lock().unwrap() = None;
        log::info!("Whisper model unloaded");
    }

    /// Check if a model is currently loaded.
    pub fn is_loaded(&self) -> bool {
        self.context.lock().unwrap().is_some()
    }

    /// Get the ID of the currently loaded model.
    pub fn current_model_id(&self) -> Option<String> {
        self.current_model_id.lock().unwrap().clone()
    }

    /// Transcribe audio data (f32 PCM, 16kHz, mono).
    /// Returns the transcribed text.
    pub fn transcribe(&self, audio: &[f32], language: Option<&str>) -> Result<TranscriptionResult, String> {
        let mut guard = self.context.lock().unwrap();
        let wrapper = guard
            .as_mut()
            .ok_or_else(|| "No Whisper model loaded. Download and load a model first.".to_string())?;

        let ctx = &wrapper.0;
        let mut state = ctx
            .create_state()
            .map_err(|e| format!("Failed to create Whisper state: {:?}", e))?;

        // Configure transcription parameters
        let mut params = FullParams::new(SamplingStrategy::BeamSearch {
            beam_size: 5,
            patience: -1.0,
        });

        params.set_language(language.or(Some("en")));
        params.set_no_timestamps(true);
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_suppress_blank(true);

        // Use available CPU threads (cap at 4 to avoid hogging)
        let n_threads = std::thread::available_parallelism()
            .map(|n| n.get().min(4) as i32)
            .unwrap_or(2);
        params.set_n_threads(n_threads);

        let start = Instant::now();

        state
            .full(params, audio)
            .map_err(|e| format!("Whisper transcription failed: {:?}", e))?;

        let duration_ms = start.elapsed().as_millis() as u64;
        let audio_duration_ms = (audio.len() as f64 / 16.0) as u64; // 16kHz = 16 samples/ms

        // Collect segments
        let n_segments = state
            .full_n_segments()
            .map_err(|e| format!("Failed to get segment count: {:?}", e))?;

        let mut text = String::new();
        for i in 0..n_segments {
            let segment = state
                .full_get_segment_text(i)
                .map_err(|e| format!("Failed to get segment {}: {:?}", i, e))?;
            text.push_str(&segment);
        }

        let text = text.trim().to_string();

        log::info!(
            "Transcription complete: {}ms audio in {}ms ({:.1}x realtime), {} segments, {} chars",
            audio_duration_ms,
            duration_ms,
            if duration_ms > 0 {
                audio_duration_ms as f64 / duration_ms as f64
            } else {
                0.0
            },
            n_segments,
            text.len()
        );

        Ok(TranscriptionResult {
            text,
            duration_ms,
            audio_duration_ms,
        })
    }
}

/// Result of a transcription.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TranscriptionResult {
    /// The transcribed text.
    pub text: String,
    /// How long the transcription took (ms).
    pub duration_ms: u64,
    /// Duration of the input audio (ms).
    pub audio_duration_ms: u64,
}

/// Available GPU backends (compile-time detection).
#[derive(Debug, Clone, serde::Serialize)]
pub struct GpuBackendInfo {
    pub id: String,
    pub name: String,
    pub available: bool,
}

/// Detect available GPU backends based on compiled features.
pub fn available_backends() -> Vec<GpuBackendInfo> {
    let mut backends = vec![GpuBackendInfo {
        id: "cpu".to_string(),
        name: "CPU".to_string(),
        available: true,
    }];

    backends.push(GpuBackendInfo {
        id: "cuda".to_string(),
        name: "CUDA (NVIDIA)".to_string(),
        available: cfg!(feature = "cuda"),
    });

    backends.push(GpuBackendInfo {
        id: "vulkan".to_string(),
        name: "Vulkan".to_string(),
        available: cfg!(feature = "vulkan"),
    });

    backends
}

#[cfg(test)]
mod tests;

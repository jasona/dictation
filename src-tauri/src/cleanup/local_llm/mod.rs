// Local LLM cleanup via llama.cpp bindings

use super::TextCleaner;
use serde::Serialize;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Runtime};

#[cfg(feature = "local-llm")]
const CLEANUP_PROMPT: &str = "Clean up the following dictated text. Fix grammar and punctuation. Remove filler words. Do NOT change technical terms, names, or meaning. Do NOT add content. Return only the cleaned text.";

// ---- Model catalog ----

/// Metadata for an LLM model available for download.
#[derive(Debug, Clone, Serialize)]
pub struct LlmModelInfo {
    pub id: String,
    pub name: String,
    pub filename: String,
    pub size_bytes: u64,
    pub description: String,
    pub downloaded: bool,
}

struct LlmModelCatalogEntry {
    id: &'static str,
    name: &'static str,
    filename: &'static str,
    url: &'static str,
    size_bytes: u64,
    description: &'static str,
}

const LLM_MODEL_CATALOG: &[LlmModelCatalogEntry] = &[LlmModelCatalogEntry {
    id: "phi3-mini-q4",
    name: "Phi-3 Mini (Q4)",
    filename: "Phi-3-mini-4k-instruct-q4.gguf",
    url: "https://huggingface.co/microsoft/Phi-3-mini-4k-instruct-gguf/resolve/main/Phi-3-mini-4k-instruct-q4.gguf",
    size_bytes: 2_318_377_216, // ~2.2GB
    description: "Fast, good quality text cleanup (~2.2 GB)",
}];

/// Download progress event payload for LLM model downloads.
#[derive(Clone, Serialize)]
pub struct LlmDownloadProgressEvent {
    pub model_id: String,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub percent: f32,
}

/// Get the LLM models directory, creating it if needed.
pub fn models_dir(app_data_dir: &Path) -> PathBuf {
    let dir = app_data_dir.join("models").join("llm");
    let _ = fs::create_dir_all(&dir);
    dir
}

/// Get the file path for an LLM model by ID.
pub fn model_path(app_data_dir: &Path, model_id: &str) -> Option<PathBuf> {
    let entry = LLM_MODEL_CATALOG.iter().find(|m| m.id == model_id)?;
    Some(models_dir(app_data_dir).join(entry.filename))
}

/// Check if an LLM model is downloaded.
pub fn is_downloaded(app_data_dir: &Path, model_id: &str) -> bool {
    model_path(app_data_dir, model_id)
        .map(|p| p.exists())
        .unwrap_or(false)
}

/// List all LLM models with download status.
pub fn list_models(app_data_dir: &Path) -> Vec<LlmModelInfo> {
    LLM_MODEL_CATALOG
        .iter()
        .map(|entry| LlmModelInfo {
            id: entry.id.to_string(),
            name: entry.name.to_string(),
            filename: entry.filename.to_string(),
            size_bytes: entry.size_bytes,
            description: entry.description.to_string(),
            downloaded: is_downloaded(app_data_dir, entry.id),
        })
        .collect()
}

/// Delete a downloaded LLM model.
pub fn delete_model(app_data_dir: &Path, model_id: &str) -> Result<(), String> {
    let path = model_path(app_data_dir, model_id)
        .ok_or_else(|| format!("Unknown LLM model: {}", model_id))?;

    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("Failed to delete LLM model: {}", e))?;
        log::info!("Deleted LLM model: {} ({})", model_id, path.display());
    }
    Ok(())
}

/// Download an LLM model from HuggingFace with progress events.
pub fn download_model<R: Runtime>(
    app: &AppHandle<R>,
    app_data_dir: &Path,
    model_id: &str,
) -> Result<PathBuf, String> {
    let entry = LLM_MODEL_CATALOG
        .iter()
        .find(|m| m.id == model_id)
        .ok_or_else(|| format!("Unknown LLM model: {}", model_id))?;

    let dir = models_dir(app_data_dir);
    let dest = dir.join(entry.filename);

    if dest.exists() {
        log::info!(
            "LLM model {} already downloaded at {}",
            model_id,
            dest.display()
        );
        return Ok(dest);
    }

    log::info!("Downloading LLM model {} from {}", model_id, entry.url);

    let client = reqwest::blocking::Client::builder()
        .timeout(None)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get(entry.url)
        .send()
        .map_err(|e| format!("Download request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Download failed: HTTP {}", response.status()));
    }

    let total_bytes = response.content_length().unwrap_or(entry.size_bytes);

    let tmp_dest = dir.join(format!("{}.tmp", entry.filename));
    let mut file =
        fs::File::create(&tmp_dest).map_err(|e| format!("Failed to create file: {}", e))?;

    let mut downloaded: u64 = 0;
    let mut buf = vec![0u8; 64 * 1024];
    let mut reader = response;

    loop {
        let bytes_read = reader
            .read(&mut buf)
            .map_err(|e| format!("Download read error: {}", e))?;

        if bytes_read == 0 {
            break;
        }

        file.write_all(&buf[..bytes_read])
            .map_err(|e| format!("File write error: {}", e))?;

        downloaded += bytes_read as u64;

        let percent = if total_bytes > 0 {
            (downloaded as f32 / total_bytes as f32) * 100.0
        } else {
            0.0
        };

        let _ = app.emit(
            "llm://download-progress",
            LlmDownloadProgressEvent {
                model_id: model_id.to_string(),
                downloaded_bytes: downloaded,
                total_bytes,
                percent,
            },
        );
    }

    file.flush()
        .map_err(|e| format!("File flush error: {}", e))?;
    drop(file);

    fs::rename(&tmp_dest, &dest)
        .map_err(|e| format!("Failed to finalize download: {}", e))?;

    log::info!(
        "LLM model {} downloaded successfully ({} bytes)",
        model_id,
        downloaded
    );

    Ok(dest)
}

// ---- LLM Engine ----

/// Local LLM engine for text cleanup.
/// Feature-gated behind `local-llm`.
pub struct LlmEngine {
    #[cfg(feature = "local-llm")]
    model: Mutex<Option<LlmModelWrapper>>,
    #[cfg(not(feature = "local-llm"))]
    _phantom: (),
    model_id: Mutex<Option<String>>,
}

#[cfg(feature = "local-llm")]
struct LlmModelWrapper(llama_cpp_2::model::LlamaModel);

#[cfg(feature = "local-llm")]
unsafe impl Send for LlmModelWrapper {}

impl LlmEngine {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "local-llm")]
            model: Mutex::new(None),
            #[cfg(not(feature = "local-llm"))]
            _phantom: (),
            model_id: Mutex::new(None),
        }
    }

    pub fn is_loaded(&self) -> bool {
        #[cfg(feature = "local-llm")]
        {
            self.model.lock().unwrap().is_some()
        }
        #[cfg(not(feature = "local-llm"))]
        false
    }

    pub fn current_model_id(&self) -> Option<String> {
        self.model_id.lock().unwrap().clone()
    }

    #[cfg(feature = "local-llm")]
    pub fn load_model(&self, model_path: &str, model_id: &str) -> Result<(), String> {
        use llama_cpp_2::model::params::LlamaModelParams;
        use llama_cpp_2::model::LlamaModel;

        let params = LlamaModelParams::default();
        let model = LlamaModel::load_from_file(model_path, &params)
            .map_err(|e| format!("Failed to load LLM model: {:?}", e))?;

        *self.model.lock().unwrap() = Some(LlmModelWrapper(model));
        *self.model_id.lock().unwrap() = Some(model_id.to_string());
        log::info!("LLM model '{}' loaded from {}", model_id, model_path);
        Ok(())
    }

    #[cfg(not(feature = "local-llm"))]
    pub fn load_model(&self, _model_path: &str, _model_id: &str) -> Result<(), String> {
        Err("Local LLM support is not enabled. Build with --features local-llm".to_string())
    }

    pub fn unload_model(&self) {
        #[cfg(feature = "local-llm")]
        {
            *self.model.lock().unwrap() = None;
        }
        *self.model_id.lock().unwrap() = None;
        log::info!("LLM model unloaded");
    }

    #[cfg(feature = "local-llm")]
    pub fn clean_text(&self, text: &str) -> Result<String, String> {
        use llama_cpp_2::context::params::LlamaContextParams;
        use llama_cpp_2::llama_batch::LlamaBatch;
        use llama_cpp_2::token::data_array::LlamaTokenDataArray;

        let guard = self.model.lock().unwrap();
        let wrapper = guard
            .as_ref()
            .ok_or_else(|| "No LLM model loaded".to_string())?;
        let model = &wrapper.0;

        // Format prompt using Phi-3 chat template
        let prompt = format!(
            "<|system|>\n{}<|end|>\n<|user|>\n{}<|end|>\n<|assistant|>\n",
            CLEANUP_PROMPT, text
        );

        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(std::num::NonZeroU32::new(2048));
        let mut ctx = model
            .new_context(&llama_cpp_2::llama_backend::LlamaBackend::init()
                .map_err(|e| format!("Failed to init llama backend: {:?}", e))?,
                ctx_params,
            )
            .map_err(|e| format!("Failed to create context: {:?}", e))?;

        // Tokenize
        let tokens = model
            .str_to_token(&prompt, llama_cpp_2::model::AddBos::Always)
            .map_err(|e| format!("Tokenization failed: {:?}", e))?;

        // Create batch and add tokens
        let mut batch = LlamaBatch::new(2048, 1);
        for (i, token) in tokens.iter().enumerate() {
            let is_last = i == tokens.len() - 1;
            batch.add(*token, i as i32, &[0], is_last)
                .map_err(|e| format!("Failed to add token to batch: {:?}", e))?;
        }

        ctx.decode(&mut batch)
            .map_err(|e| format!("Decode failed: {:?}", e))?;

        // Generate tokens
        let mut output_tokens = Vec::new();
        let max_tokens = 512;

        for _ in 0..max_tokens {
            let candidates = ctx.candidates_ith(batch.n_tokens() - 1);
            let mut candidates_data = LlamaTokenDataArray::from_iter(candidates, false);

            candidates_data.sample_temp(Some(&mut ctx), 0.1);
            let new_token = candidates_data.sample_token(Some(&mut ctx));

            if model.is_eog_token(new_token) {
                break;
            }

            output_tokens.push(new_token);

            batch.clear();
            batch
                .add(new_token, (tokens.len() + output_tokens.len() - 1) as i32, &[0], true)
                .map_err(|e| format!("Failed to add token: {:?}", e))?;

            ctx.decode(&mut batch)
                .map_err(|e| format!("Decode failed: {:?}", e))?;
        }

        // Detokenize
        let result: String = output_tokens
            .iter()
            .map(|t| model.token_to_str(*t, llama_cpp_2::token::LlamaTokenAttr::empty()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Detokenization failed: {:?}", e))?
            .join("");

        // Strip any remaining template tokens
        let result = result
            .replace("<|end|>", "")
            .replace("<|assistant|>", "")
            .trim()
            .to_string();

        Ok(result)
    }

    #[cfg(not(feature = "local-llm"))]
    pub fn clean_text(&self, _text: &str) -> Result<String, String> {
        Err("Local LLM support is not enabled. Build with --features local-llm".to_string())
    }
}

/// Implements TextCleaner for the local LLM engine.
pub struct LocalLlmCleaner<'a> {
    engine: &'a LlmEngine,
}

impl<'a> LocalLlmCleaner<'a> {
    pub fn new(engine: &'a LlmEngine) -> Self {
        Self { engine }
    }
}

impl TextCleaner for LocalLlmCleaner<'_> {
    fn clean(&self, text: &str) -> Result<String, String> {
        self.engine.clean_text(text)
    }
}

#[cfg(test)]
mod tests;

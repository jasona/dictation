use serde::Serialize;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, Runtime};

/// Base URL for downloading whisper.cpp GGML models from HuggingFace.
const HF_BASE_URL: &str =
    "https://huggingface.co/ggerganov/whisper.cpp/resolve/main";

/// Metadata for a single Whisper model.
#[derive(Debug, Clone, Serialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub filename: String,
    pub size_bytes: u64,
    pub description: String,
    pub downloaded: bool,
}

/// Catalog entry (compile-time).
struct ModelCatalogEntry {
    id: &'static str,
    name: &'static str,
    filename: &'static str,
    size_bytes: u64,
    description: &'static str,
}

const MODEL_CATALOG: &[ModelCatalogEntry] = &[
    ModelCatalogEntry {
        id: "tiny.en",
        name: "Tiny (English)",
        filename: "ggml-tiny.en.bin",
        size_bytes: 77_704_715,
        description: "Fastest, least accurate (~75 MB)",
    },
    ModelCatalogEntry {
        id: "base.en",
        name: "Base (English)",
        filename: "ggml-base.en.bin",
        size_bytes: 147_964_211,
        description: "Good balance of speed and accuracy (~150 MB)",
    },
    ModelCatalogEntry {
        id: "small.en",
        name: "Small (English)",
        filename: "ggml-small.en.bin",
        size_bytes: 487_601_967,
        description: "More accurate, slower (~500 MB)",
    },
    ModelCatalogEntry {
        id: "small",
        name: "Small (Multilingual)",
        filename: "ggml-small.bin",
        size_bytes: 487_601_967,
        description: "Multilingual support (~500 MB)",
    },
    ModelCatalogEntry {
        id: "medium.en",
        name: "Medium (English)",
        filename: "ggml-medium.en.bin",
        size_bytes: 1_533_774_781,
        description: "High accuracy (~1.5 GB)",
    },
    ModelCatalogEntry {
        id: "medium",
        name: "Medium (Multilingual)",
        filename: "ggml-medium.bin",
        size_bytes: 1_533_774_781,
        description: "High accuracy, multilingual (~1.5 GB)",
    },
];

/// Download progress event payload.
#[derive(Clone, Serialize)]
pub struct DownloadProgressEvent {
    pub model_id: String,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub percent: f32,
}

/// Get the models directory, creating it if needed.
pub fn models_dir(app_data_dir: &Path) -> PathBuf {
    let dir = app_data_dir.join("models").join("whisper");
    let _ = fs::create_dir_all(&dir);
    dir
}

/// Get the file path for a model by ID.
pub fn model_path(app_data_dir: &Path, model_id: &str) -> Option<PathBuf> {
    let entry = MODEL_CATALOG.iter().find(|m| m.id == model_id)?;
    Some(models_dir(app_data_dir).join(entry.filename))
}

/// Check if a model is downloaded.
pub fn is_downloaded(app_data_dir: &Path, model_id: &str) -> bool {
    model_path(app_data_dir, model_id)
        .map(|p| p.exists())
        .unwrap_or(false)
}

/// List all models with download status.
pub fn list_models(app_data_dir: &Path) -> Vec<ModelInfo> {
    MODEL_CATALOG
        .iter()
        .map(|entry| ModelInfo {
            id: entry.id.to_string(),
            name: entry.name.to_string(),
            filename: entry.filename.to_string(),
            size_bytes: entry.size_bytes,
            description: entry.description.to_string(),
            downloaded: is_downloaded(app_data_dir, entry.id),
        })
        .collect()
}

/// Delete a downloaded model.
pub fn delete_model(app_data_dir: &Path, model_id: &str) -> Result<(), String> {
    let path = model_path(app_data_dir, model_id)
        .ok_or_else(|| format!("Unknown model: {}", model_id))?;

    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("Failed to delete model: {}", e))?;
        log::info!("Deleted model: {} ({})", model_id, path.display());
    }
    Ok(())
}

/// Download a model from HuggingFace with progress events.
/// Runs synchronously — call from a background thread.
pub fn download_model<R: Runtime>(
    app: &AppHandle<R>,
    app_data_dir: &Path,
    model_id: &str,
) -> Result<PathBuf, String> {
    let entry = MODEL_CATALOG
        .iter()
        .find(|m| m.id == model_id)
        .ok_or_else(|| format!("Unknown model: {}", model_id))?;

    let dir = models_dir(app_data_dir);
    let dest = dir.join(entry.filename);

    // Skip if already downloaded
    if dest.exists() {
        log::info!("Model {} already downloaded at {}", model_id, dest.display());
        return Ok(dest);
    }

    let url = format!("{}/{}", HF_BASE_URL, entry.filename);
    log::info!("Downloading model {} from {}", model_id, url);

    let client = reqwest::blocking::Client::builder()
        .timeout(None) // No timeout for large downloads
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .map_err(|e| format!("Download request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Download failed: HTTP {}",
            response.status()
        ));
    }

    let total_bytes = response.content_length().unwrap_or(entry.size_bytes);

    // Write to a temp file first, then rename (atomic)
    let tmp_dest = dir.join(format!("{}.tmp", entry.filename));
    let mut file = fs::File::create(&tmp_dest)
        .map_err(|e| format!("Failed to create file: {}", e))?;

    let mut downloaded: u64 = 0;
    let mut buf = vec![0u8; 64 * 1024]; // 64KB chunks
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

        // Emit progress every 64KB chunk
        let percent = if total_bytes > 0 {
            (downloaded as f32 / total_bytes as f32) * 100.0
        } else {
            0.0
        };

        let _ = app.emit(
            "model://download-progress",
            DownloadProgressEvent {
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

    // Rename tmp to final
    fs::rename(&tmp_dest, &dest)
        .map_err(|e| format!("Failed to finalize download: {}", e))?;

    log::info!(
        "Model {} downloaded successfully ({} bytes)",
        model_id,
        downloaded
    );

    Ok(dest)
}

#[cfg(test)]
mod tests;

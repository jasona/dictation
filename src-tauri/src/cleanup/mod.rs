pub mod cloud_llm;
pub mod local_llm;
pub mod rules;

use rules::RuleCleaner;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Instant;
use tauri::AppHandle;

// ---- Trait ----

/// Trait for text cleanup implementations.
pub trait TextCleaner {
    fn clean(&self, text: &str) -> Result<String, String>;
}

// ---- Enums ----

/// Which cleanup tier to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CleanupTier {
    Rules,
    LocalLlm,
    CloudLlm,
}

/// Which cloud LLM provider to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CloudProvider {
    OpenAi,
    Anthropic,
}

// ---- Result ----

/// Result of a cleanup operation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanupResult {
    pub text: String,
    pub tier_used: CleanupTier,
    pub duration_ms: u64,
}

// ---- State ----

/// Tauri-managed state for the cleanup subsystem.
pub struct CleanupState {
    pub tier: Mutex<CleanupTier>,
    pub cloud_provider: Mutex<CloudProvider>,
    pub http_client: reqwest::blocking::Client,
    pub llm_engine: local_llm::LlmEngine,
    pub app_data_dir: Mutex<Option<PathBuf>>,
}

impl CleanupState {
    pub fn new() -> Self {
        Self {
            tier: Mutex::new(CleanupTier::Rules),
            cloud_provider: Mutex::new(CloudProvider::OpenAi),
            http_client: reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            llm_engine: local_llm::LlmEngine::new(),
            app_data_dir: Mutex::new(None),
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

// ---- Orchestrator ----

/// Run cleanup with fallback cascade.
/// Tries the selected tier first, falls back to lower tiers on failure.
/// Rules tier is terminal and always succeeds.
pub fn run_cleanup(state: &CleanupState, text: &str) -> CleanupResult {
    let selected_tier = *state.tier.lock().unwrap();
    let start = Instant::now();

    // Try tiers in cascade order
    let tiers_to_try = match selected_tier {
        CleanupTier::CloudLlm => vec![CleanupTier::CloudLlm, CleanupTier::LocalLlm, CleanupTier::Rules],
        CleanupTier::LocalLlm => vec![CleanupTier::LocalLlm, CleanupTier::Rules],
        CleanupTier::Rules => vec![CleanupTier::Rules],
    };

    for tier in tiers_to_try {
        let result = match tier {
            CleanupTier::CloudLlm => {
                let provider = *state.cloud_provider.lock().unwrap();
                let cleaner = cloud_llm::CloudCleaner::new(&state.http_client, provider);
                cleaner.clean(text)
            }
            CleanupTier::LocalLlm => {
                let cleaner = local_llm::LocalLlmCleaner::new(&state.llm_engine);
                cleaner.clean(text)
            }
            CleanupTier::Rules => {
                let cleaner = RuleCleaner;
                cleaner.clean(text)
            }
        };

        match result {
            Ok(cleaned) => {
                if tier != selected_tier {
                    log::warn!(
                        "Cleanup fell back from {:?} to {:?}",
                        selected_tier,
                        tier
                    );
                }
                return CleanupResult {
                    text: cleaned,
                    tier_used: tier,
                    duration_ms: start.elapsed().as_millis() as u64,
                };
            }
            Err(e) => {
                log::warn!("Cleanup tier {:?} failed: {}", tier, e);
            }
        }
    }

    // This should never happen since Rules always succeeds,
    // but return original text as ultimate fallback
    CleanupResult {
        text: text.to_string(),
        tier_used: CleanupTier::Rules,
        duration_ms: start.elapsed().as_millis() as u64,
    }
}

// ---- Tauri commands ----

#[tauri::command]
pub fn cleanup_text(
    text: String,
    state: tauri::State<'_, CleanupState>,
) -> CleanupResult {
    run_cleanup(&state, &text)
}

#[tauri::command]
pub fn get_cleanup_tier(state: tauri::State<'_, CleanupState>) -> CleanupTier {
    *state.tier.lock().unwrap()
}

#[tauri::command]
pub fn set_cleanup_tier(
    tier: CleanupTier,
    state: tauri::State<'_, CleanupState>,
) {
    *state.tier.lock().unwrap() = tier;
    log::info!("Cleanup tier set to {:?}", tier);
}

#[tauri::command]
pub fn get_cloud_provider(state: tauri::State<'_, CleanupState>) -> CloudProvider {
    *state.cloud_provider.lock().unwrap()
}

#[tauri::command]
pub fn set_cloud_provider(
    provider: CloudProvider,
    state: tauri::State<'_, CleanupState>,
) {
    *state.cloud_provider.lock().unwrap() = provider;
    log::info!("Cloud provider set to {:?}", provider);
}

#[tauri::command]
pub fn save_api_key(
    provider: CloudProvider,
    key: String,
) -> Result<(), String> {
    cloud_llm::save_api_key(&provider, &key)
}

#[tauri::command]
pub fn get_api_key_exists(provider: CloudProvider) -> bool {
    cloud_llm::has_api_key(&provider)
}

#[tauri::command]
pub fn delete_api_key(provider: CloudProvider) -> Result<(), String> {
    cloud_llm::delete_api_key(&provider)
}

#[tauri::command]
pub fn test_api_key(
    provider: CloudProvider,
    state: tauri::State<'_, CleanupState>,
) -> Result<(), String> {
    cloud_llm::test_cloud_key(&state.http_client, &provider)
}

// ---- LLM model commands ----

#[tauri::command]
pub fn list_llm_models(
    state: tauri::State<'_, CleanupState>,
) -> Result<Vec<local_llm::LlmModelInfo>, String> {
    let data_dir = state.data_dir()?;
    Ok(local_llm::list_models(&data_dir))
}

#[tauri::command]
pub fn download_llm_model(
    app: AppHandle,
    model_id: String,
    state: tauri::State<'_, CleanupState>,
) -> Result<String, String> {
    let data_dir = state.data_dir()?;
    let path = local_llm::download_model(&app, &data_dir, &model_id)?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn delete_llm_model(
    model_id: String,
    state: tauri::State<'_, CleanupState>,
) -> Result<(), String> {
    let data_dir = state.data_dir()?;

    // Unload if this model is currently loaded
    if state.llm_engine.current_model_id().as_deref() == Some(&model_id) {
        state.llm_engine.unload_model();
    }

    local_llm::delete_model(&data_dir, &model_id)
}

#[tauri::command]
pub fn load_llm_model(
    model_id: String,
    state: tauri::State<'_, CleanupState>,
) -> Result<(), String> {
    let data_dir = state.data_dir()?;

    let path = local_llm::model_path(&data_dir, &model_id)
        .ok_or_else(|| format!("Unknown LLM model: {}", model_id))?;

    if !path.exists() {
        return Err(format!(
            "LLM model '{}' is not downloaded. Download it first.",
            model_id
        ));
    }

    state
        .llm_engine
        .load_model(&path.to_string_lossy(), &model_id)
}

#[tauri::command]
pub fn unload_llm_model(state: tauri::State<'_, CleanupState>) {
    state.llm_engine.unload_model();
}

#[tauri::command]
pub fn get_current_llm_model(state: tauri::State<'_, CleanupState>) -> Option<String> {
    state.llm_engine.current_model_id()
}

#[cfg(test)]
mod tests;

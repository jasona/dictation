pub mod audio;
pub mod cleanup;
pub mod hotkey;
pub mod injection;
mod pipeline;
pub mod settings;
pub mod stt;
pub mod tray;

use audio::AudioState;
use cleanup::CleanupState;
use hotkey::HotkeyState;
use injection::InjectionState;
use stt::SttState;
use tauri::{Emitter, Manager, WebviewWindow};

// ---- Pill window commands ----

#[tauri::command]
fn show_pill_window(app: tauri::AppHandle) -> Result<(), String> {
    let win: WebviewWindow = app
        .get_webview_window("pill")
        .ok_or_else(|| "Pill window not found".to_string())?;
    win.show().map_err(|e| format!("Failed to show pill: {}", e))?;
    // Don't steal focus — the target app needs to stay focused for text injection
    Ok(())
}

#[tauri::command]
fn hide_pill_window(app: tauri::AppHandle) -> Result<(), String> {
    let win: WebviewWindow = app
        .get_webview_window("pill")
        .ok_or_else(|| "Pill window not found".to_string())?;
    win.hide().map_err(|e| format!("Failed to hide pill: {}", e))?;
    Ok(())
}

// ---- Onboarding window commands ----

#[tauri::command]
fn show_onboarding_window(app: tauri::AppHandle) -> Result<(), String> {
    let win: WebviewWindow = app
        .get_webview_window("onboarding")
        .ok_or_else(|| "Onboarding window not found".to_string())?;
    win.show().map_err(|e| format!("Failed to show onboarding: {}", e))?;
    win.set_focus().map_err(|e| format!("Failed to focus onboarding: {}", e))?;
    Ok(())
}

#[tauri::command]
fn hide_onboarding_window(app: tauri::AppHandle) -> Result<(), String> {
    let win: WebviewWindow = app
        .get_webview_window("onboarding")
        .ok_or_else(|| "Onboarding window not found".to_string())?;
    win.hide().map_err(|e| format!("Failed to hide onboarding: {}", e))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    tauri::Builder::default()
        // --- Plugins ---
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        // updater plugin requires signing keys — enable when release infrastructure is ready
        // .plugin(tauri_plugin_updater::Builder::new().build())
        // --- Managed state ---
        .manage(HotkeyState::new())
        .manage(AudioState::new())
        .manage(SttState::new())
        .manage(CleanupState::new())
        .manage(InjectionState::new())
        // --- Commands ---
        .invoke_handler(tauri::generate_handler![
            hotkey::get_activation_mode,
            hotkey::set_activation_mode,
            hotkey::get_hotkey,
            hotkey::set_hotkey,
            hotkey::get_is_paused,
            hotkey::set_is_paused,
            settings::store::get_autostart,
            settings::store::set_autostart,
            audio::list_audio_devices,
            audio::set_audio_device,
            audio::get_audio_device,
            stt::list_whisper_models,
            stt::download_whisper_model,
            stt::delete_whisper_model,
            stt::load_whisper_model,
            stt::unload_whisper_model,
            stt::get_current_whisper_model,
            stt::transcribe,
            stt::get_gpu_backends,
            stt::set_gpu_backend,
            stt::get_gpu_backend,
            cleanup::cleanup_text,
            cleanup::get_cleanup_tier,
            cleanup::set_cleanup_tier,
            cleanup::get_cloud_provider,
            cleanup::set_cloud_provider,
            cleanup::save_api_key,
            cleanup::get_api_key_exists,
            cleanup::delete_api_key,
            cleanup::test_api_key,
            cleanup::list_llm_models,
            cleanup::download_llm_model,
            cleanup::delete_llm_model,
            cleanup::load_llm_model,
            cleanup::unload_llm_model,
            cleanup::get_current_llm_model,
            injection::inject_text,
            injection::undo_last_injection,
            injection::get_last_injection_exists,
            show_pill_window,
            hide_pill_window,
            show_onboarding_window,
            hide_onboarding_window,
            settings::store::get_onboarding_completed,
            settings::store::set_onboarding_completed,
        ])
        // --- App setup ---
        .setup(|app| {
            let handle = app.handle().clone();

            // 1. Register system tray
            let _tray = tray::setup(&handle)?;
            log::info!("System tray registered");

            // 2. Register global hotkey
            let hotkey_str = hotkey::DEFAULT_HOTKEY;
            eprintln!(">>> Attempting to register hotkey: {}", hotkey_str);
            match hotkey::register(&handle, hotkey_str) {
                Ok(()) => eprintln!(">>> Hotkey registered OK: {}", hotkey_str),
                Err(e) => eprintln!(">>> Hotkey registration FAILED: {}", e),
            }

            // 3. Configure app data directories
            if let Ok(data_dir) = app.path().app_data_dir() {
                // VAD model path
                let vad_path = data_dir.join("models").join("vad").join("silero_vad.onnx");
                if vad_path.exists() {
                    let audio_state: tauri::State<'_, AudioState> = handle.state();
                    audio_state.set_vad_model_path(vad_path.to_string_lossy().to_string());
                    log::info!("VAD model found: {}", vad_path.display());
                } else {
                    log::info!("VAD model not found, will capture without VAD");
                }

                // STT data directory
                let stt_state: tauri::State<'_, SttState> = handle.state();
                *stt_state.app_data_dir.lock().unwrap() = Some(data_dir.clone());

                // Cleanup data directory
                let cleanup_state: tauri::State<'_, CleanupState> = handle.state();
                *cleanup_state.app_data_dir.lock().unwrap() = Some(data_dir.clone());

                // 4. Preload Whisper model if available (default: base.en)
                stt::preload_model(&handle, &stt_state, "base.en");
            }

            // 5. Show onboarding if not completed
            if !settings::store::get_onboarding_completed(handle.clone()) {
                if let Some(win) = handle.get_webview_window("onboarding") {
                    let _ = win.show();
                    let _ = win.set_focus();
                    log::info!("Showing onboarding window");
                }
            }

            // 6. Initialize pipeline event listeners
            pipeline::init(&handle);
            log::info!("Pipeline orchestrator initialized");

            // 7. Auto-update check — disabled until signing keys are configured
            // See tauri-plugin-updater docs for setup instructions.

            // 8. Emit ready event
            let _ = handle.emit("app://ready", ());
            log::info!("App initialization complete");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

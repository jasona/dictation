use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_autostart::ManagerExt;

#[tauri::command]
pub fn get_autostart(app: AppHandle) -> Result<bool, String> {
    app.autolaunch()
        .is_enabled()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_autostart(app: AppHandle, enabled: bool) -> Result<(), String> {
    let manager = app.autolaunch();
    if enabled {
        manager.enable().map_err(|e| e.to_string())
    } else {
        manager.disable().map_err(|e| e.to_string())
    }
}

// ---- Onboarding completion flag ----

fn onboarding_flag_path(app: &AppHandle) -> Result<PathBuf, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    Ok(data_dir.join("onboarding_completed"))
}

#[tauri::command]
pub fn get_onboarding_completed(app: AppHandle) -> bool {
    onboarding_flag_path(&app)
        .map(|p| p.exists())
        .unwrap_or(false)
}

#[tauri::command]
pub fn set_onboarding_completed(app: AppHandle) -> Result<(), String> {
    let path = onboarding_flag_path(&app)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create dir: {}", e))?;
    }
    std::fs::write(&path, "1")
        .map_err(|e| format!("Failed to write onboarding flag: {}", e))
}

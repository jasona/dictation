use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Instant;
use tauri::{AppHandle, Emitter, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

/// Activation mode for recording.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ActivationMode {
    Toggle,
    Hold,
}

/// Shared hotkey state managed via Tauri's state system.
pub struct HotkeyState {
    /// Whether recording is currently active.
    pub is_recording: AtomicBool,
    /// Whether recording is paused (tray pause/resume).
    pub is_paused: AtomicBool,
    /// Current activation mode.
    pub mode: Mutex<ActivationMode>,
    /// Current hotkey string (e.g. "ctrl+shift+space").
    pub hotkey: Mutex<String>,
    /// Timestamp of last key-down, used for hold-mode threshold.
    press_start: Mutex<Option<Instant>>,
}

/// Duration threshold to distinguish a tap from a hold (ms).
const HOLD_THRESHOLD_MS: u128 = 300;

/// Default hotkey combination.
pub const DEFAULT_HOTKEY: &str = "F9";

impl HotkeyState {
    pub fn new() -> Self {
        Self {
            is_recording: AtomicBool::new(false),
            is_paused: AtomicBool::new(false),
            mode: Mutex::new(ActivationMode::Toggle),
            hotkey: Mutex::new(DEFAULT_HOTKEY.to_string()),
            press_start: Mutex::new(None),
        }
    }
}

/// Register the global hotkey with the given shortcut string.
/// Returns Ok(()) on success, or emits a conflict error event on failure.
pub fn register<R: Runtime>(app: &AppHandle<R>, shortcut: &str) -> Result<(), String> {
    let app_for_handler = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            handle_shortcut_event(&app_for_handler, event.state);
        })
        .map_err(|e| {
            let msg = format!("Failed to register hotkey '{}': {}", shortcut, e);
            log::error!("{}", msg);
            let _ = app.emit("hotkey://conflict", &msg);
            msg
        })
}

/// Unregister the current hotkey.
pub fn unregister<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let state: tauri::State<'_, HotkeyState> = app.state();
    let hotkey = state.hotkey.lock().unwrap().clone();
    app.global_shortcut()
        .unregister(hotkey.as_str())
        .map_err(|e| e.to_string())
}

/// Change the hotkey binding at runtime.
pub fn rebind<R: Runtime>(app: &AppHandle<R>, new_shortcut: &str) -> Result<(), String> {
    // Unregister old
    let _ = unregister(app);
    // Register new
    register(app, new_shortcut)?;
    // Update stored shortcut
    let state: tauri::State<'_, HotkeyState> = app.state();
    *state.hotkey.lock().unwrap() = new_shortcut.to_string();
    Ok(())
}

fn handle_shortcut_event<R: Runtime>(app: &AppHandle<R>, key_state: ShortcutState) {
    eprintln!(">>> HOTKEY PRESSED: {:?}", key_state);
    log::info!("Hotkey event: {:?}", key_state);
    let state: tauri::State<'_, HotkeyState> = app.state();

    // Ignore if paused
    if state.is_paused.load(Ordering::Relaxed) {
        log::info!("Hotkey ignored: app is paused");
        return;
    }

    let mode = *state.mode.lock().unwrap();

    match mode {
        ActivationMode::Toggle => handle_toggle(app, &state, key_state),
        ActivationMode::Hold => handle_hold(app, &state, key_state),
    }
}

/// Toggle mode: press once to start, press again to stop.
fn handle_toggle<R: Runtime>(
    app: &AppHandle<R>,
    state: &HotkeyState,
    key_state: ShortcutState,
) {
    // Only act on key-down
    if key_state != ShortcutState::Pressed {
        return;
    }

    let was_recording = state.is_recording.load(Ordering::Relaxed);

    if was_recording {
        // Stop recording → trigger processing
        log::info!("Hotkey: stopping recording, emitting vozr://stop");
        state.is_recording.store(false, Ordering::Relaxed);
        let _ = app.emit("vozr://stop", ());
        crate::tray::set_state(app, crate::tray::TrayState::Processing);
    } else {
        // Start recording
        log::info!("Hotkey: starting recording, emitting vozr://start");
        state.is_recording.store(true, Ordering::Relaxed);
        let _ = app.emit("vozr://start", ());
        crate::tray::set_state(app, crate::tray::TrayState::Listening);
    }
}

/// Hold mode: key-down starts recording, key-up stops.
/// A tap shorter than HOLD_THRESHOLD_MS is treated as a quick toggle off.
fn handle_hold<R: Runtime>(
    app: &AppHandle<R>,
    state: &HotkeyState,
    key_state: ShortcutState,
) {
    match key_state {
        ShortcutState::Pressed => {
            *state.press_start.lock().unwrap() = Some(Instant::now());

            if !state.is_recording.load(Ordering::Relaxed) {
                state.is_recording.store(true, Ordering::Relaxed);
                let _ = app.emit("vozr://start", ());
                crate::tray::set_state(app, crate::tray::TrayState::Listening);
            }
        }
        ShortcutState::Released => {
            let press_start = state.press_start.lock().unwrap().take();
            let held_long_enough = press_start
                .map(|t| t.elapsed().as_millis() >= HOLD_THRESHOLD_MS)
                .unwrap_or(true);

            if state.is_recording.load(Ordering::Relaxed) && held_long_enough {
                state.is_recording.store(false, Ordering::Relaxed);
                let _ = app.emit("vozr://stop", ());
                crate::tray::set_state(app, crate::tray::TrayState::Processing);
            }
        }
    }
}

// --------------- Tauri commands ---------------

#[tauri::command]
pub fn get_activation_mode(state: tauri::State<'_, HotkeyState>) -> String {
    let mode = *state.mode.lock().unwrap();
    match mode {
        ActivationMode::Toggle => "toggle".into(),
        ActivationMode::Hold => "hold".into(),
    }
}

#[tauri::command]
pub fn set_activation_mode(
    mode: String,
    state: tauri::State<'_, HotkeyState>,
) -> Result<(), String> {
    let new_mode = match mode.as_str() {
        "toggle" => ActivationMode::Toggle,
        "hold" => ActivationMode::Hold,
        other => return Err(format!("Unknown mode: {}", other)),
    };
    *state.mode.lock().unwrap() = new_mode;
    Ok(())
}

#[tauri::command]
pub fn get_hotkey(state: tauri::State<'_, HotkeyState>) -> String {
    state.hotkey.lock().unwrap().clone()
}

#[tauri::command]
pub fn set_hotkey(app: AppHandle, shortcut: String) -> Result<(), String> {
    rebind(&app, &shortcut)
}

#[tauri::command]
pub fn get_is_paused(state: tauri::State<'_, HotkeyState>) -> bool {
    state.is_paused.load(Ordering::Relaxed)
}

#[tauri::command]
pub fn set_is_paused(paused: bool, state: tauri::State<'_, HotkeyState>) {
    state.is_paused.store(paused, Ordering::Relaxed);
}

#[cfg(test)]
mod tests;

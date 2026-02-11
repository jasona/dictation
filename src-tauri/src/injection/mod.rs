pub mod clipboard;
pub mod keyboard;

use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::Instant;

// ---- Enums ----

/// Injection method used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InjectionMethod {
    Clipboard,
    Keyboard,
}

// ---- Result ----

/// Result of an injection operation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InjectionResult {
    pub success: bool,
    pub method_used: InjectionMethod,
    pub duration_ms: u64,
}

// ---- Last injection metadata ----

/// Metadata about the last successful injection, for undo support.
struct LastInjection {
    #[allow(dead_code)] // Used by pipeline orchestrator for redo
    text: String,
    char_count: usize,
    timestamp: Instant,
}

// ---- State ----

/// Tauri-managed state for the injection subsystem.
pub struct InjectionState {
    last_injection: Mutex<Option<LastInjection>>,
}

impl InjectionState {
    pub fn new() -> Self {
        Self {
            last_injection: Mutex::new(None),
        }
    }
}

// ---- Orchestrator ----

/// Maximum character count for undo (Ctrl+Z) operations.
const MAX_UNDO_CHARS: usize = 5000;

/// Time window (seconds) within which undo is allowed.
const UNDO_WINDOW_SECS: u64 = 10;

/// Inject text into the active application.
///
/// Tries clipboard injection first, falls back to keyboard simulation.
pub fn inject_text_impl(text: &str, state: &InjectionState) -> InjectionResult {
    let start = Instant::now();

    // Empty text: immediate success
    if text.is_empty() {
        return InjectionResult {
            success: true,
            method_used: InjectionMethod::Clipboard,
            duration_ms: 0,
        };
    }

    // Try clipboard injection first
    match clipboard::inject_via_clipboard(text) {
        Ok(()) => {
            // Record last injection
            let mut last = state.last_injection.lock().unwrap();
            *last = Some(LastInjection {
                text: text.to_string(),
                char_count: text.chars().count(),
                timestamp: Instant::now(),
            });

            return InjectionResult {
                success: true,
                method_used: InjectionMethod::Clipboard,
                duration_ms: start.elapsed().as_millis() as u64,
            };
        }
        Err(e) => {
            log::warn!("Clipboard injection failed, trying keyboard fallback: {}", e);
        }
    }

    // Fallback: keyboard simulation
    match keyboard::inject_via_keyboard(text, keyboard::DEFAULT_DELAY_MS) {
        Ok(()) => {
            // Record last injection
            let mut last = state.last_injection.lock().unwrap();
            *last = Some(LastInjection {
                text: text.to_string(),
                char_count: text.chars().count(),
                timestamp: Instant::now(),
            });

            InjectionResult {
                success: true,
                method_used: InjectionMethod::Keyboard,
                duration_ms: start.elapsed().as_millis() as u64,
            }
        }
        Err(e) => {
            log::error!("Both clipboard and keyboard injection failed: {}", e);
            InjectionResult {
                success: false,
                method_used: InjectionMethod::Keyboard,
                duration_ms: start.elapsed().as_millis() as u64,
            }
        }
    }
}

/// Undo the last injection by simulating undo shortcut repeated for each character.
fn undo_last_injection_impl(state: &InjectionState) -> Result<(), String> {
    use clipboard::MODIFIER_KEY;

    let mut last = state.last_injection.lock().unwrap();

    let injection = last
        .as_ref()
        .ok_or_else(|| "No recent injection to undo".to_string())?;

    // Check time window
    if injection.timestamp.elapsed().as_secs() > UNDO_WINDOW_SECS {
        *last = None;
        return Err("Last injection is too old to undo (>10s)".to_string());
    }

    let undo_count = injection.char_count.min(MAX_UNDO_CHARS);

    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to create enigo instance: {}", e))?;

    for _ in 0..undo_count {
        enigo
            .key(MODIFIER_KEY, Direction::Press)
            .map_err(|e| format!("Failed to press modifier: {}", e))?;
        enigo
            .key(Key::Unicode('z'), Direction::Click)
            .map_err(|e| format!("Failed to click Z: {}", e))?;
        enigo
            .key(MODIFIER_KEY, Direction::Release)
            .map_err(|e| format!("Failed to release modifier: {}", e))?;
    }

    *last = None;
    log::info!("Undid last injection ({} undo operations)", undo_count);
    Ok(())
}

// ---- Tauri commands ----

#[tauri::command]
pub fn inject_text(
    text: String,
    state: tauri::State<'_, InjectionState>,
) -> InjectionResult {
    log::info!("Injecting {} chars", text.chars().count());
    inject_text_impl(&text, &state)
}

#[tauri::command]
pub fn undo_last_injection(
    state: tauri::State<'_, InjectionState>,
) -> Result<(), String> {
    undo_last_injection_impl(&state)
}

#[tauri::command]
pub fn get_last_injection_exists(
    state: tauri::State<'_, InjectionState>,
) -> bool {
    let last = state.last_injection.lock().unwrap();
    match last.as_ref() {
        Some(inj) => inj.timestamp.elapsed().as_secs() <= UNDO_WINDOW_SECS,
        None => false,
    }
}

#[cfg(test)]
mod tests;

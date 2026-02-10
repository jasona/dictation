// Keyboard simulation fallback via enigo

use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::thread;
use std::time::Duration;

/// Default inter-keystroke delay in milliseconds.
pub const DEFAULT_DELAY_MS: u64 = 5;

/// Inject text via keyboard simulation.
///
/// First tries `enigo.text()` for the full string (handles unicode).
/// Falls back to character-by-character `key(Unicode(c), Click)` with delay.
pub fn inject_via_keyboard(text: &str, delay_ms: u64) -> Result<(), String> {
    if text.is_empty() {
        return Ok(());
    }

    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to create enigo instance: {}", e))?;

    // Try text() first — handles unicode and is faster
    match enigo.text(text) {
        Ok(()) => return Ok(()),
        Err(e) => {
            log::warn!("enigo.text() failed, falling back to char-by-char: {}", e);
        }
    }

    // Fallback: character by character
    let delay = Duration::from_millis(delay_ms);
    for ch in text.chars() {
        if ch == '\n' {
            enigo
                .key(Key::Return, Direction::Click)
                .map_err(|e| format!("Failed to type Return: {}", e))?;
        } else if ch == '\t' {
            enigo
                .key(Key::Tab, Direction::Click)
                .map_err(|e| format!("Failed to type Tab: {}", e))?;
        } else {
            enigo
                .key(Key::Unicode(ch), Direction::Click)
                .map_err(|e| format!("Failed to type '{}': {}", ch, e))?;
        }
        if delay_ms > 0 {
            thread::sleep(delay);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inject_empty_string_succeeds() {
        let result = inject_via_keyboard("", DEFAULT_DELAY_MS);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore] // Requires real window focus
    fn inject_keyboard_basic() {
        let result = inject_via_keyboard("hello", DEFAULT_DELAY_MS);
        assert!(result.is_ok());
    }
}

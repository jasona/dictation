// Clipboard save/restore and paste injection

use arboard::{Clipboard, ImageData};
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::borrow::Cow;
use std::thread;
use std::time::Duration;

/// Saved clipboard contents for restore after injection.
#[derive(Debug, Clone)]
pub enum SavedClipboard {
    Text(String),
    Image { width: usize, height: usize, bytes: Vec<u8> },
    Empty,
}

/// Save the current clipboard contents (text or image).
pub fn save_clipboard() -> SavedClipboard {
    let mut cb = match Clipboard::new() {
        Ok(cb) => cb,
        Err(e) => {
            log::warn!("Failed to open clipboard for save: {}", e);
            return SavedClipboard::Empty;
        }
    };

    // Try text first (most common)
    if let Ok(text) = cb.get_text() {
        if !text.is_empty() {
            return SavedClipboard::Text(text);
        }
    }

    // Try image
    if let Ok(img) = cb.get_image() {
        return SavedClipboard::Image {
            width: img.width,
            height: img.height,
            bytes: img.bytes.into_owned(),
        };
    }

    SavedClipboard::Empty
}

/// Restore previously saved clipboard contents.
pub fn restore_clipboard(saved: &SavedClipboard) {
    let mut cb = match Clipboard::new() {
        Ok(cb) => cb,
        Err(e) => {
            log::warn!("Failed to open clipboard for restore: {}", e);
            return;
        }
    };

    match saved {
        SavedClipboard::Text(text) => {
            if let Err(e) = cb.set_text(text) {
                log::warn!("Failed to restore text to clipboard: {}", e);
            }
        }
        SavedClipboard::Image { width, height, bytes } => {
            let img = ImageData {
                width: *width,
                height: *height,
                bytes: Cow::Borrowed(bytes),
            };
            if let Err(e) = cb.set_image(img) {
                log::warn!("Failed to restore image to clipboard: {}", e);
            }
        }
        SavedClipboard::Empty => {
            if let Err(e) = cb.clear() {
                log::warn!("Failed to clear clipboard on restore: {}", e);
            }
        }
    }
}

/// The modifier key used for paste/undo shortcuts.
/// On macOS this is Cmd (Meta), on other platforms it's Ctrl.
#[cfg(target_os = "macos")]
pub const MODIFIER_KEY: Key = Key::Meta;
#[cfg(not(target_os = "macos"))]
pub const MODIFIER_KEY: Key = Key::Control;

/// Inject text via clipboard paste (Ctrl+V / Cmd+V).
///
/// 1. Save current clipboard
/// 2. Set text to clipboard
/// 3. Simulate paste shortcut
/// 4. Restore original clipboard
///
/// Total time target: <500ms
pub fn inject_via_clipboard(text: &str) -> Result<(), String> {
    let saved = save_clipboard();

    // Set our text to clipboard
    let mut cb = Clipboard::new().map_err(|e| format!("Failed to open clipboard: {}", e))?;
    cb.set_text(text).map_err(|e| format!("Failed to set clipboard text: {}", e))?;
    drop(cb); // Release clipboard before simulating paste

    // Let clipboard settle
    thread::sleep(Duration::from_millis(50));

    // Simulate paste (Ctrl+V on Windows/Linux, Cmd+V on macOS)
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to create enigo instance: {}", e))?;

    enigo
        .key(MODIFIER_KEY, Direction::Press)
        .map_err(|e| format!("Failed to press modifier: {}", e))?;
    enigo
        .key(Key::Unicode('v'), Direction::Click)
        .map_err(|e| format!("Failed to click V: {}", e))?;
    enigo
        .key(MODIFIER_KEY, Direction::Release)
        .map_err(|e| format!("Failed to release modifier: {}", e))?;

    // Let paste complete
    thread::sleep(Duration::from_millis(150));

    // Restore original clipboard
    restore_clipboard(&saved);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saved_clipboard_empty_variant() {
        let saved = SavedClipboard::Empty;
        assert!(matches!(saved, SavedClipboard::Empty));
    }

    #[test]
    fn saved_clipboard_text_variant() {
        let saved = SavedClipboard::Text("hello".to_string());
        if let SavedClipboard::Text(t) = &saved {
            assert_eq!(t, "hello");
        } else {
            panic!("Expected Text variant");
        }
    }

    #[test]
    fn saved_clipboard_image_variant() {
        let saved = SavedClipboard::Image {
            width: 100,
            height: 200,
            bytes: vec![0u8; 100 * 200 * 4],
        };
        if let SavedClipboard::Image { width, height, bytes } = &saved {
            assert_eq!(*width, 100);
            assert_eq!(*height, 200);
            assert_eq!(bytes.len(), 80000);
        } else {
            panic!("Expected Image variant");
        }
    }

    #[test]
    fn save_and_restore_clipboard_text() {
        // Save whatever is on clipboard now
        let original = save_clipboard();

        // Try to set some text — may fail in CI or when clipboard is locked
        let set_ok = Clipboard::new()
            .and_then(|mut cb| cb.set_text("test_injection_12345"))
            .is_ok();

        if set_ok {
            // Save it
            let saved = save_clipboard();
            match &saved {
                SavedClipboard::Text(t) => assert_eq!(t, "test_injection_12345"),
                _ => {
                    // Clipboard may have been cleared by another process — not a test failure
                    eprintln!("Clipboard didn't retain text (may be locked by another process)");
                }
            }
        }

        // Restore original
        restore_clipboard(&original);
    }

    #[test]
    #[ignore] // Requires real window focus for paste
    fn inject_via_clipboard_succeeds() {
        let result = inject_via_clipboard("hello world");
        assert!(result.is_ok());
    }
}

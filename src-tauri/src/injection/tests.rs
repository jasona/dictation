use super::*;

// ---- InjectionMethod serialization ----

#[test]
fn injection_method_clipboard_serializes() {
    let json = serde_json::to_string(&InjectionMethod::Clipboard).unwrap();
    assert_eq!(json, r#""clipboard""#);
}

#[test]
fn injection_method_keyboard_serializes() {
    let json = serde_json::to_string(&InjectionMethod::Keyboard).unwrap();
    assert_eq!(json, r#""keyboard""#);
}

#[test]
fn injection_method_deserializes() {
    let method: InjectionMethod = serde_json::from_str(r#""clipboard""#).unwrap();
    assert_eq!(method, InjectionMethod::Clipboard);

    let method: InjectionMethod = serde_json::from_str(r#""keyboard""#).unwrap();
    assert_eq!(method, InjectionMethod::Keyboard);
}

// ---- InjectionResult serialization ----

#[test]
fn injection_result_serializes() {
    let result = InjectionResult {
        success: true,
        method_used: InjectionMethod::Clipboard,
        duration_ms: 42,
    };
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains(r#""success":true"#));
    assert!(json.contains(r#""methodUsed":"clipboard""#));
    assert!(json.contains(r#""durationMs":42"#));
}

// ---- InjectionState ----

#[test]
fn injection_state_starts_empty() {
    let state = InjectionState::new();
    let last = state.last_injection.lock().unwrap();
    assert!(last.is_none());
}

// ---- inject_text_impl with empty string ----

#[test]
fn inject_empty_text_succeeds() {
    let state = InjectionState::new();
    let result = inject_text_impl("", &state);
    assert!(result.success);
    assert_eq!(result.method_used, InjectionMethod::Clipboard);
    assert_eq!(result.duration_ms, 0);
}

// ---- undo with no injection ----

#[test]
fn undo_fails_with_no_injection() {
    let state = InjectionState::new();
    let result = undo_last_injection_impl(&state);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No recent injection"));
}

// ---- undo with expired injection ----

#[test]
fn undo_fails_when_too_old() {
    let state = InjectionState::new();

    // Manually insert a "stale" last injection
    {
        let mut last = state.last_injection.lock().unwrap();
        *last = Some(LastInjection {
            text: "old text".to_string(),
            char_count: 8,
            // Use an instant far in the past by subtracting from now
            timestamp: Instant::now() - std::time::Duration::from_secs(15),
        });
    }

    let result = undo_last_injection_impl(&state);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("too old"));

    // Should have cleared last injection
    let last = state.last_injection.lock().unwrap();
    assert!(last.is_none());
}

// ---- last injection tracking ----

#[test]
fn last_injection_recorded_in_state() {
    // Directly test that the state tracks metadata correctly
    // (without actually calling inject_text_impl which needs real clipboard/enigo)
    let state = InjectionState::new();

    {
        let mut last = state.last_injection.lock().unwrap();
        *last = Some(LastInjection {
            text: "hello".to_string(),
            char_count: 5,
            timestamp: Instant::now(),
        });
    }

    let last = state.last_injection.lock().unwrap();
    let inj = last.as_ref().expect("Should have last injection");
    assert_eq!(inj.text, "hello");
    assert_eq!(inj.char_count, 5);
    assert!(inj.timestamp.elapsed().as_secs() < 1);
}

#[test]
#[ignore] // Requires real window focus for clipboard/keyboard injection
fn last_injection_tracked_after_inject() {
    let state = InjectionState::new();
    let result = inject_text_impl("hello", &state);

    if result.success {
        let last = state.last_injection.lock().unwrap();
        let inj = last.as_ref().expect("Should have last injection");
        assert_eq!(inj.text, "hello");
        assert_eq!(inj.char_count, 5);
    }
}

// ---- get_last_injection_exists logic ----

#[test]
fn last_injection_exists_false_when_empty() {
    let state = InjectionState::new();
    let last = state.last_injection.lock().unwrap();
    // Replicate logic from get_last_injection_exists
    let exists = match last.as_ref() {
        Some(inj) => inj.timestamp.elapsed().as_secs() <= UNDO_WINDOW_SECS,
        None => false,
    };
    assert!(!exists);
}

#[test]
fn last_injection_exists_true_when_recent() {
    let state = InjectionState::new();
    {
        let mut last = state.last_injection.lock().unwrap();
        *last = Some(LastInjection {
            text: "recent".to_string(),
            char_count: 6,
            timestamp: Instant::now(),
        });
    }

    let last = state.last_injection.lock().unwrap();
    let exists = match last.as_ref() {
        Some(inj) => inj.timestamp.elapsed().as_secs() <= UNDO_WINDOW_SECS,
        None => false,
    };
    assert!(exists);
}

#[test]
fn last_injection_exists_false_when_expired() {
    let state = InjectionState::new();
    {
        let mut last = state.last_injection.lock().unwrap();
        *last = Some(LastInjection {
            text: "expired".to_string(),
            char_count: 7,
            timestamp: Instant::now() - std::time::Duration::from_secs(15),
        });
    }

    let last = state.last_injection.lock().unwrap();
    let exists = match last.as_ref() {
        Some(inj) => inj.timestamp.elapsed().as_secs() <= UNDO_WINDOW_SECS,
        None => false,
    };
    assert!(!exists);
}

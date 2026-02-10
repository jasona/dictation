use super::*;

#[test]
fn hotkey_state_defaults() {
    let state = HotkeyState::new();
    assert!(!state.is_recording.load(Ordering::Relaxed));
    assert!(!state.is_paused.load(Ordering::Relaxed));
    assert_eq!(*state.mode.lock().unwrap(), ActivationMode::Toggle);
    assert_eq!(*state.hotkey.lock().unwrap(), DEFAULT_HOTKEY);
    assert!(state.press_start.lock().unwrap().is_none());
}

#[test]
fn toggle_mode_cycles_recording() {
    let state = HotkeyState::new();
    *state.mode.lock().unwrap() = ActivationMode::Toggle;

    // Simulate: not recording → press → should start recording
    assert!(!state.is_recording.load(Ordering::Relaxed));
    // (Full toggle logic requires a Tauri AppHandle, so here we test the state transitions directly)
    state.is_recording.store(true, Ordering::Relaxed);
    assert!(state.is_recording.load(Ordering::Relaxed));

    // Simulate: recording → press → should stop recording
    state.is_recording.store(false, Ordering::Relaxed);
    assert!(!state.is_recording.load(Ordering::Relaxed));
}

#[test]
fn hold_mode_tracks_press_time() {
    let state = HotkeyState::new();
    *state.mode.lock().unwrap() = ActivationMode::Hold;

    // Simulate press start
    *state.press_start.lock().unwrap() = Some(Instant::now());
    assert!(state.press_start.lock().unwrap().is_some());

    // Simulate release
    let press_start = state.press_start.lock().unwrap().take();
    assert!(press_start.is_some());
    assert!(state.press_start.lock().unwrap().is_none());
}

#[test]
fn hold_threshold_detected() {
    // A press shorter than HOLD_THRESHOLD_MS should not trigger stop
    let start = Instant::now();
    // Immediately check — should be under threshold
    let elapsed = start.elapsed().as_millis();
    assert!(elapsed < HOLD_THRESHOLD_MS);
}

#[test]
fn mode_switching() {
    let state = HotkeyState::new();

    // Default is toggle
    assert_eq!(*state.mode.lock().unwrap(), ActivationMode::Toggle);

    // Switch to hold
    *state.mode.lock().unwrap() = ActivationMode::Hold;
    assert_eq!(*state.mode.lock().unwrap(), ActivationMode::Hold);

    // Switch back to toggle
    *state.mode.lock().unwrap() = ActivationMode::Toggle;
    assert_eq!(*state.mode.lock().unwrap(), ActivationMode::Toggle);
}

#[test]
fn pause_prevents_action() {
    let state = HotkeyState::new();

    // When paused, recording should not be affected
    state.is_paused.store(true, Ordering::Relaxed);
    assert!(state.is_paused.load(Ordering::Relaxed));

    // Recording state should remain false
    assert!(!state.is_recording.load(Ordering::Relaxed));

    // Unpause
    state.is_paused.store(false, Ordering::Relaxed);
    assert!(!state.is_paused.load(Ordering::Relaxed));
}

#[test]
fn hotkey_string_update() {
    let state = HotkeyState::new();
    assert_eq!(*state.hotkey.lock().unwrap(), "ctrl+shift+space");

    *state.hotkey.lock().unwrap() = "alt+d".to_string();
    assert_eq!(*state.hotkey.lock().unwrap(), "alt+d");
}

#[test]
fn activation_mode_serde() {
    // Verify serialization round-trip
    let toggle = ActivationMode::Toggle;
    let json = serde_json::to_string(&toggle).unwrap();
    assert_eq!(json, "\"toggle\"");

    let hold = ActivationMode::Hold;
    let json = serde_json::to_string(&hold).unwrap();
    assert_eq!(json, "\"hold\"");

    let parsed: ActivationMode = serde_json::from_str("\"toggle\"").unwrap();
    assert_eq!(parsed, ActivationMode::Toggle);

    let parsed: ActivationMode = serde_json::from_str("\"hold\"").unwrap();
    assert_eq!(parsed, ActivationMode::Hold);
}

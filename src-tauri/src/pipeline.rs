// End-to-end Vozr pipeline orchestrator.
//
// Listens for vozr://start and vozr://stop events emitted by the hotkey module,
// then runs the pipeline: audio capture → STT → cleanup → text injection.
// Emits pill events (pill://success, pill://error) and updates the tray state.

use crate::audio::{self, AudioState};
use crate::cleanup::{self, CleanupState};
use crate::injection::{self, InjectionState};
use crate::stt::SttState;
use crate::tray;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Listener, Manager, Runtime};

/// Set up the pipeline event listeners. Call this once during app setup.
pub fn init<R: Runtime>(app: &AppHandle<R>) {
    let handle = app.clone();

    // Track whether we're currently in a recording session to avoid double-starts
    let recording = Arc::new(AtomicBool::new(false));
    let recording_for_start = recording.clone();
    let recording_for_stop = recording.clone();

    // Listen for vozr://start — begin audio capture
    let start_handle = handle.clone();
    app.listen("vozr://start", move |_event| {
        if recording_for_start.swap(true, Ordering::SeqCst) {
            // Already recording, ignore duplicate start
            return;
        }
        on_vozr_start(&start_handle);
    });

    // Listen for vozr://stop — process the captured audio
    app.listen("vozr://stop", move |_event| {
        if !recording_for_stop.swap(false, Ordering::SeqCst) {
            // Wasn't recording, ignore stale stop
            return;
        }
        on_vozr_stop(&handle);
    });
}

/// Called when recording starts: show pill, begin audio capture.
fn on_vozr_start<R: Runtime>(app: &AppHandle<R>) {
    // Show the pill window in recording state (don't steal focus from the target app)
    if let Some(win) = app.get_webview_window("pill") {
        let _ = win.show();
    }

    // Start audio capture
    let audio_state: tauri::State<'_, AudioState> = app.state();
    if let Err(e) = audio::start_recording(app, &audio_state) {
        log::error!("Failed to start recording: {}", e);
        let _ = app.emit("pill://error", format!("Microphone error: {}", e));
        tray::set_state(app, tray::TrayState::Error);
    }
}

/// Called when recording stops: stop audio, run STT → cleanup → inject.
fn on_vozr_stop<R: Runtime>(app: &AppHandle<R>) {
    let audio_state: tauri::State<'_, AudioState> = app.state();

    // Stop audio capture and take the buffer
    audio::stop_recording(&audio_state);
    let audio_buffer = audio::take_speech_buffer(&audio_state);

    let duration_secs = audio_buffer.len() as f32 / 16000.0;
    log::info!("Audio buffer: {} samples ({:.1}s)", audio_buffer.len(), duration_secs);

    // Check for empty audio
    if audio_buffer.is_empty() {
        log::info!("No audio captured, nothing to transcribe");
        let _ = app.emit("audio://no-speech", ());
        tray::set_state(app, tray::TrayState::Idle);
        return;
    }

    // Update tray to processing
    tray::set_state(app, tray::TrayState::Processing);

    // Run the rest of the pipeline on a background thread to avoid blocking the event loop
    let app_handle = app.clone();

    std::thread::spawn(move || {
        run_processing_pipeline(app_handle, audio_buffer);
    });
}

/// The processing pipeline: STT → cleanup → inject → emit result.
/// Runs on a background thread.
fn run_processing_pipeline<R: Runtime>(app: AppHandle<R>, audio_buffer: Vec<f32>) {
    // --- Step 1: Speech-to-text ---
    let stt_state: tauri::State<'_, SttState> = app.state();

    let transcription = match stt_state.engine.transcribe(&audio_buffer, Some("en")) {
        Ok(result) => {
            if result.text.trim().is_empty() {
                log::info!("Transcription returned empty text");
                let _ = app.emit("audio://no-speech", ());
                tray::set_state(&app, tray::TrayState::Idle);
                return;
            }
            log::info!(
                "Transcription complete: {} chars in {}ms",
                result.text.len(),
                result.duration_ms
            );
            result.text
        }
        Err(e) => {
            log::error!("STT failed: {}", e);
            let _ = app.emit("pill://error", format!("Transcription failed: {}", e));
            tray::set_state(&app, tray::TrayState::Error);
            return;
        }
    };

    // --- Step 2: Text cleanup ---
    let cleanup_state: tauri::State<'_, CleanupState> = app.state();
    let selected_tier = *cleanup_state.tier.lock().unwrap();
    let cleanup_result = cleanup::run_cleanup(&cleanup_state, &transcription);

    let cleaned_text = cleanup_result.text;
    log::info!(
        "Cleanup complete: tier={:?}, {}ms",
        cleanup_result.tier_used,
        cleanup_result.duration_ms
    );

    // Notify if cleanup fell back to a lower tier
    if cleanup_result.tier_used != selected_tier {
        let msg = format!(
            "Cloud cleanup unavailable, used {:?} instead",
            cleanup_result.tier_used,
        );
        log::warn!("{}", msg);
        let _ = app.emit("cleanup://fallback", msg);
    }

    if cleaned_text.trim().is_empty() {
        log::info!("Cleaned text is empty, nothing to inject");
        let _ = app.emit("audio://no-speech", ());
        tray::set_state(&app, tray::TrayState::Idle);
        return;
    }

    // --- Step 3: Text injection ---
    let injection_state: tauri::State<'_, InjectionState> = app.state();
    let inject_result = injection::inject_text_impl(&cleaned_text, &injection_state);

    if inject_result.success {
        log::info!(
            "Injection complete: method={:?}, {}ms",
            inject_result.method_used,
            inject_result.duration_ms
        );
        let _ = app.emit("pill://success", ());
    } else {
        log::error!("Text injection failed");
        let _ = app.emit("pill://error", "Failed to inject text".to_string());
        tray::set_state(&app, tray::TrayState::Error);
        return;
    }

    // --- Done: set tray back to idle ---
    tray::set_state(&app, tray::TrayState::Idle);
}

#[cfg(test)]
#[path = "pipeline_test.rs"]
mod tests;

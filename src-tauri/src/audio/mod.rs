pub mod capture;
pub mod vad;

use capture::{AudioDeviceInfo, compute_rms};
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Runtime};
use vad::{SileroVad, SpeechDetector, SpeechEvent};

/// Tauri-managed state for audio capture.
pub struct AudioState {
    /// Selected input device ID (None = system default).
    pub selected_device: Mutex<Option<String>>,
    /// Accumulated 16kHz mono speech audio from the last recording session.
    pub speech_buffer: Arc<Mutex<Vec<f32>>>,
    /// Whether we're currently recording.
    pub is_capturing: AtomicBool,
    /// Signal to stop the recording thread.
    stop_signal: Mutex<Option<Arc<AtomicBool>>>,
    /// Handle for the recording thread.
    thread_handle: Mutex<Option<thread::JoinHandle<()>>>,
    /// Path to the Silero VAD ONNX model.
    vad_model_path: Mutex<Option<String>>,
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            selected_device: Mutex::new(None),
            speech_buffer: Arc::new(Mutex::new(Vec::new())),
            is_capturing: AtomicBool::new(false),
            stop_signal: Mutex::new(None),
            thread_handle: Mutex::new(None),
            vad_model_path: Mutex::new(None),
        }
    }

    /// Set the path to the Silero VAD model. Call during app setup.
    pub fn set_vad_model_path(&self, path: String) {
        *self.vad_model_path.lock().unwrap() = Some(path);
    }
}

/// Audio level event payload.
#[derive(Clone, Serialize)]
struct AudioLevelEvent {
    rms: f32,
}

/// Error event payload.
#[derive(Clone, Serialize)]
struct AudioErrorEvent {
    message: String,
}

/// Device disconnect event payload.
#[derive(Clone, Serialize)]
struct DeviceDisconnectEvent {
    fallback_device: String,
}

/// Start recording audio. Spawns a thread that captures audio, runs VAD,
/// and emits level/speech events to the frontend.
pub fn start_recording<R: Runtime>(app: &AppHandle<R>, state: &AudioState) -> Result<(), String> {
    if state.is_capturing.load(Ordering::Relaxed) {
        return Err("Already recording".to_string());
    }

    let device_id = state.selected_device.lock().unwrap().clone();
    let speech_buffer = state.speech_buffer.clone();
    let vad_model_path = state.vad_model_path.lock().unwrap().clone();
    let app_handle = app.clone();

    // Clear previous speech buffer
    speech_buffer.lock().unwrap().clear();

    // Create stop signal
    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = stop.clone();

    state.is_capturing.store(true, Ordering::Relaxed);
    *state.stop_signal.lock().unwrap() = Some(stop.clone());

    let handle = thread::spawn(move || {
        if let Err(e) = recording_thread(
            &app_handle,
            device_id.as_deref(),
            vad_model_path.as_deref(),
            speech_buffer,
            stop_clone,
        ) {
            log::error!("Recording thread error: {}", e);
            let _ = app_handle.emit("audio://error", AudioErrorEvent { message: e });
        }
    });

    *state.thread_handle.lock().unwrap() = Some(handle);
    Ok(())
}

/// Stop recording. The accumulated speech audio is available in `state.speech_buffer`.
pub fn stop_recording(state: &AudioState) {
    if let Some(stop) = state.stop_signal.lock().unwrap().take() {
        stop.store(true, Ordering::Relaxed);
    }

    if let Some(handle) = state.thread_handle.lock().unwrap().take() {
        let _ = handle.join();
    }

    state.is_capturing.store(false, Ordering::Relaxed);
}

/// Take the speech buffer contents (empties it).
pub fn take_speech_buffer(state: &AudioState) -> Vec<f32> {
    std::mem::take(&mut *state.speech_buffer.lock().unwrap())
}

/// The recording thread: captures audio, runs VAD, emits events.
fn recording_thread<R: Runtime>(
    app: &AppHandle<R>,
    device_id: Option<&str>,
    vad_model_path: Option<&str>,
    speech_buffer: Arc<Mutex<Vec<f32>>>,
    stop: Arc<AtomicBool>,
) -> Result<(), String> {
    let (tx, rx) = mpsc::channel::<Vec<f32>>();

    // Start capture
    let (_stream, _config) = capture::start_capture(device_id, tx)?;

    // Load VAD if model is available
    let mut vad = match vad_model_path {
        Some(path) => match SileroVad::new(path) {
            Ok(v) => {
                log::info!("VAD loaded successfully");
                Some(v)
            }
            Err(e) => {
                log::warn!("VAD not available, capturing all audio: {}", e);
                None
            }
        },
        None => {
            log::info!("No VAD model configured, capturing all audio");
            None
        }
    };

    let mut detector = SpeechDetector::new();
    detector.start();

    // Buffer for accumulating samples into VAD-sized frames
    let mut frame_buffer: Vec<f32> = Vec::new();

    // RMS emission throttle (~30fps = ~33ms)
    let mut last_rms_emit = Instant::now();
    let rms_interval = Duration::from_millis(33);

    // Accumulator for RMS computation between emissions
    let mut rms_samples: Vec<f32> = Vec::new();

    while !stop.load(Ordering::Relaxed) {
        match rx.recv_timeout(Duration::from_millis(50)) {
            Ok(chunk) => {
                rms_samples.extend_from_slice(&chunk);

                // Emit RMS at ~30fps
                if last_rms_emit.elapsed() >= rms_interval {
                    let rms = compute_rms(&rms_samples);
                    let _ = app.emit("audio://level", AudioLevelEvent { rms });
                    rms_samples.clear();
                    last_rms_emit = Instant::now();
                }

                if let Some(ref mut vad) = vad {
                    // Accumulate into frame buffer and process VAD frames
                    frame_buffer.extend_from_slice(&chunk);

                    while frame_buffer.len() >= vad::FRAME_SIZE {
                        let frame: Vec<f32> =
                            frame_buffer.drain(..vad::FRAME_SIZE).collect();

                        match vad.process_frame(&frame) {
                            Ok(prob) => {
                                let is_speech = SileroVad::is_speech(prob);
                                let events = detector.update(is_speech);

                                // If speech is active, accumulate audio
                                if detector.state() == vad::SpeechState::Speech
                                    || detector.state() == vad::SpeechState::TrailingSilence
                                {
                                    speech_buffer.lock().unwrap().extend_from_slice(&frame);
                                }

                                // Handle events
                                for event in events {
                                    match event {
                                        SpeechEvent::SpeechStart => {
                                            log::debug!("Speech started");
                                        }
                                        SpeechEvent::SpeechEnd => {
                                            log::debug!("Speech ended");
                                        }
                                        SpeechEvent::NoSpeech => {
                                            log::info!("No speech detected (5s)");
                                            let _ = app.emit("audio://no-speech", ());
                                        }
                                        SpeechEvent::Timeout => {
                                            log::info!("Speech timeout (10s), auto-stopping");
                                            let _ = app.emit("audio://timeout", ());
                                            return Ok(());
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("VAD processing error: {}", e);
                            }
                        }
                    }
                } else {
                    // No VAD — accumulate all audio as speech
                    speech_buffer.lock().unwrap().extend_from_slice(&chunk);
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // Check for silence timeouts even when no audio arrives
                if vad.is_some() {
                    let events = detector.update(false);
                    for event in events {
                        match event {
                            SpeechEvent::NoSpeech => {
                                let _ = app.emit("audio://no-speech", ());
                            }
                            SpeechEvent::Timeout => {
                                let _ = app.emit("audio://timeout", ());
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                // Stream ended (device disconnected?)
                log::warn!("Audio stream disconnected");

                // Try to fall back to default device
                match capture::list_devices() {
                    Ok(devices) => {
                        if let Some(default) = devices.iter().find(|d| d.is_default) {
                            let _ = app.emit(
                                "audio://device-disconnected",
                                DeviceDisconnectEvent {
                                    fallback_device: default.name.clone(),
                                },
                            );
                        }
                    }
                    Err(_) => {}
                }

                let _ = app.emit(
                    "audio://error",
                    AudioErrorEvent {
                        message: "Microphone disconnected".to_string(),
                    },
                );
                return Err("Audio stream disconnected".to_string());
            }
        }
    }

    Ok(())
}

// --------------- Tauri commands ---------------

#[tauri::command]
pub fn list_audio_devices() -> Result<Vec<AudioDeviceInfo>, String> {
    capture::list_devices()
}

#[tauri::command]
pub fn set_audio_device(
    device_id: Option<String>,
    state: tauri::State<'_, AudioState>,
) -> Result<(), String> {
    // Verify device exists if an ID was provided
    if let Some(ref id) = device_id {
        let devices = capture::list_devices()?;
        if !devices.iter().any(|d| d.id == *id) {
            return Err(format!("Device not found: {}", id));
        }
    }
    *state.selected_device.lock().unwrap() = device_id;
    Ok(())
}

#[tauri::command]
pub fn get_audio_device(state: tauri::State<'_, AudioState>) -> Option<String> {
    state.selected_device.lock().unwrap().clone()
}

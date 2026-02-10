use ort::session::Session;
use ort::value::Tensor;
use std::time::Instant;

/// Silero VAD processes 512 samples at 16kHz (32ms per frame).
pub const FRAME_SIZE: usize = 512;

/// Speech probability threshold.
const SPEECH_THRESHOLD: f32 = 0.5;

/// Trailing silence buffer before declaring speech end (ms).
const TRAILING_SILENCE_MS: u64 = 500;

/// Emit "noSpeech" event after this many ms with no speech detected.
const NO_SPEECH_TIMEOUT_MS: u64 = 5_000;

/// Auto-stop recording after this many ms with no speech detected.
const AUTO_STOP_TIMEOUT_MS: u64 = 10_000;

/// Number of elements in each LSTM state tensor [2, 1, 64].
const STATE_SIZE: usize = 2 * 1 * 64;

/// Silero VAD wrapper using ONNX Runtime.
pub struct SileroVad {
    session: Session,
    /// LSTM hidden state — flat [2, 1, 64].
    h_state: Vec<f32>,
    /// LSTM cell state — flat [2, 1, 64].
    c_state: Vec<f32>,
}

impl SileroVad {
    /// Load the Silero VAD ONNX model from the given path.
    pub fn new(model_path: &str) -> Result<Self, String> {
        let session = Session::builder()
            .map_err(|e| format!("Failed to create ORT session builder: {}", e))?
            .with_intra_threads(1)
            .map_err(|e| format!("Failed to set thread count: {}", e))?
            .commit_from_file(model_path)
            .map_err(|e| format!("Failed to load VAD model '{}': {}", model_path, e))?;

        log::info!("Silero VAD loaded from {}", model_path);

        Ok(Self {
            session,
            h_state: vec![0.0f32; STATE_SIZE],
            c_state: vec![0.0f32; STATE_SIZE],
        })
    }

    /// Process a single 512-sample audio frame. Returns speech probability [0.0, 1.0].
    pub fn process_frame(&mut self, audio: &[f32]) -> Result<f32, String> {
        if audio.len() != FRAME_SIZE {
            return Err(format!(
                "VAD frame must be {} samples, got {}",
                FRAME_SIZE,
                audio.len()
            ));
        }

        // Create input tensors using (shape, data) tuples
        let audio_tensor = Tensor::from_array((
            [1usize, FRAME_SIZE],
            audio.to_vec().into_boxed_slice(),
        ))
        .map_err(|e| format!("Failed to create audio tensor: {}", e))?;

        let sr_tensor =
            Tensor::from_array(([1usize], vec![16000_i64].into_boxed_slice()))
                .map_err(|e| format!("Failed to create sr tensor: {}", e))?;

        let h_tensor = Tensor::from_array((
            [2usize, 1, 64],
            self.h_state.clone().into_boxed_slice(),
        ))
        .map_err(|e| format!("Failed to create h tensor: {}", e))?;

        let c_tensor = Tensor::from_array((
            [2usize, 1, 64],
            self.c_state.clone().into_boxed_slice(),
        ))
        .map_err(|e| format!("Failed to create c tensor: {}", e))?;

        let outputs = self
            .session
            .run(ort::inputs![
                "input" => audio_tensor,
                "sr" => sr_tensor,
                "h" => h_tensor,
                "c" => c_tensor,
            ])
            .map_err(|e| format!("VAD inference failed: {}", e))?;

        // Extract probability
        let (_shape, prob_data) = outputs["output"]
            .try_extract_tensor::<f32>()
            .map_err(|e| format!("Failed to extract VAD output: {}", e))?;

        let probability = prob_data[0];

        // Update LSTM states
        let (_h_shape, h_data) = outputs["hn"]
            .try_extract_tensor::<f32>()
            .map_err(|e| format!("Failed to extract h state: {}", e))?;
        self.h_state = h_data.to_vec();

        let (_c_shape, c_data) = outputs["cn"]
            .try_extract_tensor::<f32>()
            .map_err(|e| format!("Failed to extract c state: {}", e))?;
        self.c_state = c_data.to_vec();

        Ok(probability)
    }

    /// Returns true if the probability exceeds the speech threshold.
    pub fn is_speech(prob: f32) -> bool {
        prob >= SPEECH_THRESHOLD
    }

    /// Reset LSTM states (call between recording sessions).
    pub fn reset(&mut self) {
        self.h_state = vec![0.0f32; STATE_SIZE];
        self.c_state = vec![0.0f32; STATE_SIZE];
    }
}

// --------------- Speech boundary detection ---------------

/// Speech detector state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeechState {
    /// No speech detected yet.
    Silence,
    /// User is currently speaking.
    Speech,
    /// Speech paused — waiting to see if it resumes within the trailing buffer.
    TrailingSilence,
}

/// Events emitted by the speech detector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeechEvent {
    /// User started speaking.
    SpeechStart,
    /// User stopped speaking (after trailing silence buffer).
    SpeechEnd,
    /// 5 seconds elapsed with no speech.
    NoSpeech,
    /// 10 seconds elapsed with no speech — auto-stop.
    Timeout,
}

/// Tracks speech boundaries and silence timeouts.
pub struct SpeechDetector {
    state: SpeechState,
    trailing_silence_start: Option<Instant>,
    recording_start: Option<Instant>,
    first_speech_detected: bool,
    no_speech_emitted: bool,
}

impl SpeechDetector {
    pub fn new() -> Self {
        Self {
            state: SpeechState::Silence,
            trailing_silence_start: None,
            recording_start: None,
            first_speech_detected: false,
            no_speech_emitted: false,
        }
    }

    /// Call when a recording session begins. Resets all internal state.
    pub fn start(&mut self) {
        self.state = SpeechState::Silence;
        self.trailing_silence_start = None;
        self.recording_start = Some(Instant::now());
        self.first_speech_detected = false;
        self.no_speech_emitted = false;
    }

    /// Feed a VAD decision and get back any events to emit.
    pub fn update(&mut self, is_speech: bool) -> Vec<SpeechEvent> {
        let mut events = Vec::new();

        match self.state {
            SpeechState::Silence => {
                if is_speech {
                    self.state = SpeechState::Speech;
                    self.first_speech_detected = true;
                    events.push(SpeechEvent::SpeechStart);
                } else if let Some(start) = self.recording_start {
                    let elapsed_ms = start.elapsed().as_millis() as u64;

                    if elapsed_ms >= AUTO_STOP_TIMEOUT_MS && !self.first_speech_detected {
                        events.push(SpeechEvent::Timeout);
                    } else if elapsed_ms >= NO_SPEECH_TIMEOUT_MS
                        && !self.first_speech_detected
                        && !self.no_speech_emitted
                    {
                        self.no_speech_emitted = true;
                        events.push(SpeechEvent::NoSpeech);
                    }
                }
            }

            SpeechState::Speech => {
                if !is_speech {
                    self.state = SpeechState::TrailingSilence;
                    self.trailing_silence_start = Some(Instant::now());
                }
            }

            SpeechState::TrailingSilence => {
                if is_speech {
                    // Speech resumed within the buffer — go back to Speech
                    self.state = SpeechState::Speech;
                    self.trailing_silence_start = None;
                } else if let Some(start) = self.trailing_silence_start {
                    if start.elapsed().as_millis() as u64 >= TRAILING_SILENCE_MS {
                        self.state = SpeechState::Silence;
                        self.trailing_silence_start = None;
                        events.push(SpeechEvent::SpeechEnd);
                    }
                }
            }
        }

        events
    }

    pub fn state(&self) -> SpeechState {
        self.state
    }
}

#[cfg(test)]
mod tests;

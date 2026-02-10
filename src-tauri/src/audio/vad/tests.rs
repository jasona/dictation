use super::*;

// --------------- SpeechDetector tests ---------------

#[test]
fn detector_starts_in_silence() {
    let detector = SpeechDetector::new();
    assert_eq!(detector.state(), SpeechState::Silence);
}

#[test]
fn detector_start_resets_state() {
    let mut detector = SpeechDetector::new();
    // Move to speech
    detector.start();
    detector.update(true);
    assert_eq!(detector.state(), SpeechState::Speech);

    // Restart should reset to silence
    detector.start();
    assert_eq!(detector.state(), SpeechState::Silence);
}

#[test]
fn silence_to_speech_transition() {
    let mut detector = SpeechDetector::new();
    detector.start();

    let events = detector.update(true);
    assert_eq!(events, vec![SpeechEvent::SpeechStart]);
    assert_eq!(detector.state(), SpeechState::Speech);
}

#[test]
fn speech_to_trailing_silence() {
    let mut detector = SpeechDetector::new();
    detector.start();

    // Enter speech
    detector.update(true);
    assert_eq!(detector.state(), SpeechState::Speech);

    // Silence → trailing
    let events = detector.update(false);
    assert!(events.is_empty()); // No event yet — still in trailing buffer
    assert_eq!(detector.state(), SpeechState::TrailingSilence);
}

#[test]
fn trailing_silence_resumes_to_speech() {
    let mut detector = SpeechDetector::new();
    detector.start();

    // Enter speech → trailing silence → back to speech
    detector.update(true);
    detector.update(false);
    assert_eq!(detector.state(), SpeechState::TrailingSilence);

    let events = detector.update(true);
    assert!(events.is_empty()); // No event for resuming
    assert_eq!(detector.state(), SpeechState::Speech);
}

#[test]
fn trailing_silence_expires_to_speech_end() {
    let mut detector = SpeechDetector::new();
    detector.start();

    // Enter speech
    detector.update(true);

    // Enter trailing silence
    detector.update(false);
    assert_eq!(detector.state(), SpeechState::TrailingSilence);

    // Wait for trailing silence to expire (500ms+)
    std::thread::sleep(std::time::Duration::from_millis(550));

    let events = detector.update(false);
    assert_eq!(events, vec![SpeechEvent::SpeechEnd]);
    assert_eq!(detector.state(), SpeechState::Silence);
}

#[test]
fn continuous_silence_stays_silent() {
    let mut detector = SpeechDetector::new();
    detector.start();

    for _ in 0..10 {
        let events = detector.update(false);
        // No events in first 5s
        assert!(events.is_empty() || events.contains(&SpeechEvent::NoSpeech));
    }
    assert_eq!(detector.state(), SpeechState::Silence);
}

#[test]
fn continuous_speech_stays_in_speech() {
    let mut detector = SpeechDetector::new();
    detector.start();

    // First update: SpeechStart
    let events = detector.update(true);
    assert_eq!(events, vec![SpeechEvent::SpeechStart]);

    // Subsequent updates: no events
    for _ in 0..10 {
        let events = detector.update(true);
        assert!(events.is_empty());
    }
    assert_eq!(detector.state(), SpeechState::Speech);
}

#[test]
fn no_events_without_start() {
    let mut detector = SpeechDetector::new();
    // Without calling start(), recording_start is None
    let events = detector.update(false);
    assert!(events.is_empty());
}

#[test]
fn speech_after_detection_prevents_timeout() {
    let mut detector = SpeechDetector::new();
    detector.start();

    // Detect speech
    detector.update(true);

    // Even after long silence, no timeout because speech was detected
    std::thread::sleep(std::time::Duration::from_millis(550));
    detector.update(false); // trailing silence expires
    let events = detector.update(false);

    // Should get SpeechEnd but NOT NoSpeech or Timeout
    for event in &events {
        assert_ne!(*event, SpeechEvent::NoSpeech);
        assert_ne!(*event, SpeechEvent::Timeout);
    }
}

// --------------- SileroVad static method tests ---------------

#[test]
fn is_speech_threshold() {
    assert!(!SileroVad::is_speech(0.0));
    assert!(!SileroVad::is_speech(0.49));
    assert!(SileroVad::is_speech(0.5));
    assert!(SileroVad::is_speech(0.99));
    assert!(SileroVad::is_speech(1.0));
}

// Integration tests for the Vozr pipeline.
//
// These tests verify the pipeline orchestrator logic. Tests that require
// real hardware (microphone, keyboard/clipboard injection) are marked #[ignore].

#[cfg(test)]
mod tests {
    use crate::cleanup::{CleanupState, CleanupTier};
    use crate::injection::InjectionState;
    use crate::stt::SttState;

    /// Verify that the cleanup module correctly falls back from cloud to rules
    /// when cloud is unavailable (no API key configured).
    #[test]
    fn cleanup_tier_fallback_cloud_to_rules() {
        let state = CleanupState::new();
        *state.tier.lock().unwrap() = CleanupTier::CloudLlm;

        // No API key is configured, so cloud should fail and fall back to rules
        let result = crate::cleanup::run_cleanup(&state, "hello world um you know");
        assert!(
            result.tier_used == CleanupTier::Rules,
            "Expected fallback to Rules, got {:?}",
            result.tier_used
        );
        // Rules should at least return something
        assert!(!result.text.is_empty());
    }

    /// Verify that rule-based cleanup produces cleaned text.
    #[test]
    fn rules_cleanup_removes_fillers() {
        let state = CleanupState::new();
        *state.tier.lock().unwrap() = CleanupTier::Rules;

        let result = crate::cleanup::run_cleanup(&state, "um hello uh world you know");
        assert_eq!(result.tier_used, CleanupTier::Rules);
        // Fillers should be removed
        assert!(!result.text.contains("um "));
        assert!(!result.text.contains("uh "));
    }

    /// Verify that injection with empty text is a no-op success.
    #[test]
    fn inject_empty_text_succeeds() {
        let state = InjectionState::new();
        let result = crate::injection::inject_text_impl("", &state);
        assert!(result.success);
        assert_eq!(result.duration_ms, 0);
    }

    /// Verify that the STT state starts without a loaded model.
    #[test]
    fn stt_state_starts_without_model() {
        let state = SttState::new();
        assert!(state.engine.current_model_id().is_none());
    }

    /// Verify that transcription fails gracefully when no model is loaded.
    #[test]
    fn transcribe_without_model_returns_error() {
        let state = SttState::new();
        let dummy_audio = vec![0.0f32; 16000]; // 1 second of silence
        let result = state.engine.transcribe(&dummy_audio, Some("en"));
        assert!(result.is_err(), "Expected error when no model is loaded");
    }

    /// Integration test: full Vozr flow end-to-end.
    /// Requires microphone, Whisper model, and a foreground text field.
    #[test]
    #[ignore]
    fn full_vozr_flow() {
        // This test requires:
        // 1. A working microphone
        // 2. A downloaded Whisper model (base.en)
        // 3. A foreground application with a text input field
        // It would be run manually during QA.
    }

    /// Integration test: quick redo flow.
    /// Requires real injection + undo capabilities.
    #[test]
    #[ignore]
    fn quick_redo_flow() {
        // This test requires a foreground text field and real injection.
        // Run manually during QA.
    }

    /// Integration test: clipboard injection into a real application.
    #[test]
    #[ignore]
    fn clipboard_injection_real_app() {
        // This test requires a real foreground application.
        // Run manually during QA.
    }
}

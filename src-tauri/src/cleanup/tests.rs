use super::*;

fn make_state() -> CleanupState {
    CleanupState::new()
}

#[test]
fn rules_tier_always_succeeds() {
    let state = make_state();
    *state.tier.lock().unwrap() = CleanupTier::Rules;

    let result = run_cleanup(&state, "um hello world");
    assert_eq!(result.tier_used, CleanupTier::Rules);
    assert!(!result.text.contains("um"));
}

#[test]
fn cloud_falls_back_to_rules_without_api_key() {
    let state = make_state();
    *state.tier.lock().unwrap() = CleanupTier::CloudLlm;

    // No API key configured, so cloud should fail and fall back to rules
    let result = run_cleanup(&state, "um hello world");
    assert_eq!(result.tier_used, CleanupTier::Rules);
    assert!(!result.text.contains("um"));
}

#[test]
fn local_llm_falls_back_to_rules_when_not_loaded() {
    let state = make_state();
    *state.tier.lock().unwrap() = CleanupTier::LocalLlm;

    // No model loaded, so local LLM should fail and fall back to rules
    let result = run_cleanup(&state, "um hello world");
    assert_eq!(result.tier_used, CleanupTier::Rules);
    assert!(!result.text.contains("um"));
}

#[test]
fn result_includes_duration() {
    let state = make_state();
    let result = run_cleanup(&state, "hello world");
    // Duration should be non-negative (could be 0ms for fast operations)
    assert!(result.duration_ms < 1000, "Cleanup should take <1s");
}

#[test]
fn empty_text_handled() {
    let state = make_state();
    let result = run_cleanup(&state, "");
    assert_eq!(result.text, "");
    assert_eq!(result.tier_used, CleanupTier::Rules);
}

#[test]
fn cleanup_tier_serialization() {
    assert_eq!(
        serde_json::to_string(&CleanupTier::Rules).unwrap(),
        "\"rules\""
    );
    assert_eq!(
        serde_json::to_string(&CleanupTier::LocalLlm).unwrap(),
        "\"localLlm\""
    );
    assert_eq!(
        serde_json::to_string(&CleanupTier::CloudLlm).unwrap(),
        "\"cloudLlm\""
    );
}

#[test]
fn cloud_provider_serialization() {
    assert_eq!(
        serde_json::to_string(&CloudProvider::OpenAi).unwrap(),
        "\"openAi\""
    );
    assert_eq!(
        serde_json::to_string(&CloudProvider::Anthropic).unwrap(),
        "\"anthropic\""
    );
}

#[test]
fn cleanup_tier_deserialization() {
    let tier: CleanupTier = serde_json::from_str("\"rules\"").unwrap();
    assert_eq!(tier, CleanupTier::Rules);

    let tier: CleanupTier = serde_json::from_str("\"localLlm\"").unwrap();
    assert_eq!(tier, CleanupTier::LocalLlm);

    let tier: CleanupTier = serde_json::from_str("\"cloudLlm\"").unwrap();
    assert_eq!(tier, CleanupTier::CloudLlm);
}

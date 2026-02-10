use super::*;

#[test]
fn resample_same_rate_is_identity() {
    let input = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    let output = resample(&input, 16000, 16000);
    assert_eq!(input, output);
}

#[test]
fn resample_empty_returns_empty() {
    let output = resample(&[], 48000, 16000);
    assert!(output.is_empty());
}

#[test]
fn resample_48k_to_16k_reduces_length() {
    // 48kHz -> 16kHz is a 3:1 ratio
    let input: Vec<f32> = (0..960).map(|i| (i as f32 / 960.0).sin()).collect();
    let output = resample(&input, 48000, 16000);
    // Should produce ~320 samples (960 / 3)
    assert!(output.len() >= 319 && output.len() <= 321);
}

#[test]
fn resample_preserves_dc_signal() {
    // A constant signal should remain constant after resampling
    let input = vec![0.5f32; 480];
    let output = resample(&input, 48000, 16000);
    for sample in &output {
        assert!((sample - 0.5).abs() < 1e-6, "DC signal not preserved: {}", sample);
    }
}

#[test]
fn resample_16k_to_48k_increases_length() {
    let input: Vec<f32> = (0..160).map(|i| (i as f32 / 160.0).sin()).collect();
    let output = resample(&input, 16000, 48000);
    // Should produce ~480 samples (160 * 3)
    assert!(output.len() >= 479 && output.len() <= 481);
}

#[test]
fn to_mono_single_channel_is_identity() {
    let input = vec![0.1, 0.2, 0.3];
    let output = to_mono(&input, 1);
    assert_eq!(input, output);
}

#[test]
fn to_mono_stereo_averages() {
    // Stereo: L=1.0, R=0.0 -> mono=0.5
    let input = vec![1.0f32, 0.0, 0.4, 0.6, -0.2, 0.8];
    let output = to_mono(&input, 2);
    assert_eq!(output.len(), 3);
    assert!((output[0] - 0.5).abs() < 1e-6);
    assert!((output[1] - 0.5).abs() < 1e-6);
    assert!((output[2] - 0.3).abs() < 1e-6);
}

#[test]
fn to_mono_4_channels() {
    let input = vec![0.4, 0.4, 0.4, 0.4]; // 4 channels, 1 frame
    let output = to_mono(&input, 4);
    assert_eq!(output.len(), 1);
    assert!((output[0] - 0.4).abs() < 1e-6);
}

#[test]
fn compute_rms_empty_is_zero() {
    assert_eq!(compute_rms(&[]), 0.0);
}

#[test]
fn compute_rms_silence_is_zero() {
    assert_eq!(compute_rms(&[0.0, 0.0, 0.0]), 0.0);
}

#[test]
fn compute_rms_constant_signal() {
    // RMS of constant 0.5 = 0.5
    let rms = compute_rms(&[0.5, 0.5, 0.5, 0.5]);
    assert!((rms - 0.5).abs() < 1e-6);
}

#[test]
fn compute_rms_known_value() {
    // RMS of [1, -1, 1, -1] = 1.0
    let rms = compute_rms(&[1.0, -1.0, 1.0, -1.0]);
    assert!((rms - 1.0).abs() < 1e-6);
}

#[test]
fn compute_rms_typical_audio() {
    // Sine wave with amplitude 0.7 has RMS ≈ 0.7 / sqrt(2) ≈ 0.495
    let samples: Vec<f32> = (0..1000)
        .map(|i| 0.7 * (2.0 * std::f32::consts::PI * i as f32 / 100.0).sin())
        .collect();
    let rms = compute_rms(&samples);
    assert!(rms > 0.4 && rms < 0.6, "RMS of sine wave should be ~0.495, got {}", rms);
}

#[test]
fn list_devices_returns_ok() {
    // This test requires audio hardware but should not panic
    let result = list_devices();
    assert!(result.is_ok());
}

#[test]
fn list_devices_has_default() {
    let devices = list_devices().unwrap();
    if !devices.is_empty() {
        // At least one device should be marked as default
        assert!(
            devices.iter().any(|d| d.is_default),
            "No default device found among {} devices",
            devices.len()
        );
    }
}

#[test]
fn get_device_none_returns_default() {
    // Requesting None should return the default device (if one exists)
    let result = get_device(None);
    // Don't assert Ok — CI might not have audio devices
    if result.is_ok() {
        assert!(result.unwrap().name().is_ok());
    }
}

#[test]
fn get_device_nonexistent_returns_error() {
    let result = get_device(Some("__nonexistent_device_xyz__"));
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(err.contains("not found"), "Expected 'not found' in error: {}", err);
}

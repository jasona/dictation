use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use serde::Serialize;
use std::sync::mpsc;

/// Target sample rate for Whisper and Silero VAD.
pub const TARGET_SAMPLE_RATE: u32 = 16_000;

#[derive(Debug, Clone, Serialize)]
pub struct AudioDeviceInfo {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

/// List all available audio input devices.
pub fn list_devices() -> Result<Vec<AudioDeviceInfo>, String> {
    let host = cpal::default_host();
    let default_device = host.default_input_device();
    let default_name = default_device.as_ref().and_then(|d| d.name().ok());

    let devices = host
        .input_devices()
        .map_err(|e| format!("Failed to enumerate input devices: {}", e))?;

    let mut result = Vec::new();
    for device in devices {
        let name = device.name().unwrap_or_else(|_| "Unknown".to_string());
        let is_default = default_name.as_deref() == Some(&name);
        result.push(AudioDeviceInfo {
            id: name.clone(),
            name,
            is_default,
        });
    }
    Ok(result)
}

/// Get an input device by name, or the system default if `device_id` is None.
pub fn get_device(device_id: Option<&str>) -> Result<cpal::Device, String> {
    let host = cpal::default_host();

    match device_id {
        Some(id) => {
            let devices = host
                .input_devices()
                .map_err(|e| format!("Failed to enumerate devices: {}", e))?;
            for device in devices {
                if device.name().ok().as_deref() == Some(id) {
                    return Ok(device);
                }
            }
            Err(format!("Audio device not found: {}", id))
        }
        None => host
            .default_input_device()
            .ok_or_else(|| "No default input device available. Check your microphone connection and Windows sound settings.".to_string()),
    }
}

/// Start capturing audio from the specified device.
/// Sends resampled 16kHz mono f32 chunks through the sender.
/// Returns the cpal Stream (must be kept alive) and the device's native config.
pub fn start_capture(
    device_id: Option<&str>,
    sender: mpsc::Sender<Vec<f32>>,
) -> Result<(cpal::Stream, cpal::SupportedStreamConfig), String> {
    let device = get_device(device_id)?;
    let device_name = device.name().unwrap_or_else(|_| "Unknown".to_string());

    let config = device
        .default_input_config()
        .map_err(|e| format!("Failed to get input config: {}", e))?;

    let sample_rate = config.sample_rate().0;
    let channels = config.channels();
    let sample_format = config.sample_format();

    log::info!(
        "Audio capture: device='{}', {}Hz, {} ch, {:?}",
        device_name,
        sample_rate,
        channels,
        sample_format
    );

    let stream = match sample_format {
        cpal::SampleFormat::F32 => {
            build_stream::<f32>(&device, &config.clone().into(), sample_rate, channels, sender)
        }
        cpal::SampleFormat::I16 => {
            build_stream::<i16>(&device, &config.clone().into(), sample_rate, channels, sender)
        }
        cpal::SampleFormat::I32 => {
            build_stream::<i32>(&device, &config.clone().into(), sample_rate, channels, sender)
        }
        other => Err(format!("Unsupported sample format: {:?}", other)),
    }?;

    stream
        .play()
        .map_err(|e| format!("Failed to start audio stream: {}", e))?;

    Ok((stream, config))
}

/// Build an input stream for a given sample type.
fn build_stream<T: cpal::SizedSample + Send + 'static>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    sample_rate: u32,
    channels: u16,
    sender: mpsc::Sender<Vec<f32>>,
) -> Result<cpal::Stream, String>
where
    f32: cpal::FromSample<T>,
{
    let stream = device
        .build_input_stream(
            config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                // Convert to f32
                let f32_data: Vec<f32> = data
                    .iter()
                    .map(|&s| cpal::Sample::from_sample(s))
                    .collect();

                // Convert to mono
                let mono = to_mono(&f32_data, channels);

                // Resample to 16kHz
                let resampled = resample(&mono, sample_rate, TARGET_SAMPLE_RATE);

                let _ = sender.send(resampled);
            },
            move |err| {
                log::error!("Audio stream error: {}", err);
            },
            None,
        )
        .map_err(|e| format!("Failed to build input stream: {}", e))?;

    Ok(stream)
}

/// Resample audio from `src_rate` to `dst_rate` using linear interpolation.
pub fn resample(samples: &[f32], src_rate: u32, dst_rate: u32) -> Vec<f32> {
    if src_rate == dst_rate || samples.is_empty() {
        return samples.to_vec();
    }

    let ratio = src_rate as f64 / dst_rate as f64;
    let out_len = (samples.len() as f64 / ratio).ceil() as usize;
    let mut output = Vec::with_capacity(out_len);

    for i in 0..out_len {
        let src_idx = i as f64 * ratio;
        let idx = src_idx as usize;
        let frac = (src_idx - idx as f64) as f32;

        let sample = if idx + 1 < samples.len() {
            samples[idx] * (1.0 - frac) + samples[idx + 1] * frac
        } else {
            samples[idx.min(samples.len() - 1)]
        };
        output.push(sample);
    }
    output
}

/// Convert multi-channel audio to mono by averaging channels.
pub fn to_mono(samples: &[f32], channels: u16) -> Vec<f32> {
    if channels == 1 {
        return samples.to_vec();
    }
    let ch = channels as usize;
    samples
        .chunks_exact(ch)
        .map(|frame| frame.iter().sum::<f32>() / ch as f32)
        .collect()
}

/// Compute RMS (root mean square) audio level.
pub fn compute_rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let sum_sq: f32 = samples.iter().map(|s| s * s).sum();
    (sum_sq / samples.len() as f32).sqrt()
}

#[cfg(test)]
mod tests;

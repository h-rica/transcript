use anyhow::{Context, Result, anyhow};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::{get_codecs, get_probe};

pub const TARGET_SAMPLE_RATE: u32 = 24_000;

#[allow(dead_code)]
pub struct DecodedAudio {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub duration_s: f32,
    pub channels: u16,
}

pub fn decode_audio_file(path: &str) -> Result<DecodedAudio> {
    let file =
        std::fs::File::open(path).with_context(|| format!("Cannot open audio file: {path}"))?;
    let media_source_stream = MediaSourceStream::new(Box::new(file), Default::default());
    let hint = build_hint(path);

    let probed = get_probe()
        .format(
            &hint,
            media_source_stream,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .context("Unsupported audio format")?;

    let mut format = probed.format;
    let track = format
        .default_track()
        .ok_or_else(|| anyhow!("No audio track found"))?;
    let sample_rate = track.codec_params.sample_rate.unwrap_or(TARGET_SAMPLE_RATE);
    let track_id = track.id;

    let mut decoder = get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .context("Unsupported codec parameters")?;

    let mut all_samples = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::IoError(_)) | Err(SymphoniaError::ResetRequired) => break,
            Err(error) => return Err(error.into()),
        };

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = match decoder.decode(&packet) {
            Ok(decoded) => decoded,
            Err(_) => continue,
        };

        let spec = *decoded.spec();
        let n_frames = decoded.frames();
        let mut buffer = SampleBuffer::<f32>::new(n_frames as u64, spec);
        buffer.copy_interleaved_ref(decoded);

        let channels = spec.channels.count();
        let samples = buffer.samples();

        if channels == 1 {
            all_samples.extend_from_slice(samples);
        } else {
            all_samples.extend(
                samples
                    .chunks(channels)
                    .map(|channel_samples| channel_samples.iter().sum::<f32>() / channels as f32),
            );
        }
    }

    let resampled_samples = resample_mono(&all_samples, sample_rate, TARGET_SAMPLE_RATE);
    let duration_s = resampled_samples.len() as f32 / TARGET_SAMPLE_RATE as f32;

    Ok(DecodedAudio {
        samples: resampled_samples,
        sample_rate: TARGET_SAMPLE_RATE,
        duration_s,
        channels: 1,
    })
}

pub fn get_audio_metadata(path: &str) -> Result<(f32, u64, String)> {
    let file = std::fs::File::open(path).with_context(|| format!("Cannot open: {path}"))?;
    let size = file.metadata()?.len();
    let media_source_stream = MediaSourceStream::new(Box::new(file), Default::default());
    let hint = build_hint(path);

    let probed = get_probe()
        .format(
            &hint,
            media_source_stream,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .context("Unsupported format")?;

    let format = probed.format;
    let track = format.default_track().ok_or_else(|| anyhow!("No track"))?;
    let sample_rate = track.codec_params.sample_rate.unwrap_or(0);
    let frames = track.codec_params.n_frames.unwrap_or(0);
    let duration_s = if sample_rate > 0 {
        frames as f32 / sample_rate as f32
    } else {
        0.0
    };
    let format_name = std::path::Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("unknown")
        .to_uppercase();

    Ok((duration_s, size, format_name))
}

pub fn resample_mono(samples: &[f32], from_hz: u32, to_hz: u32) -> Vec<f32> {
    if samples.is_empty() || from_hz == 0 || to_hz == 0 || from_hz == to_hz {
        return samples.to_vec();
    }

    let ratio = from_hz as f64 / to_hz as f64;
    let out_len = ((samples.len() as f64 / ratio).round() as usize).max(1);
    let mut out = Vec::with_capacity(out_len);

    for i in 0..out_len {
        let src = i as f64 * ratio;
        let lo = src.floor() as usize;
        let hi = lo.saturating_add(1).min(samples.len() - 1);
        let frac = (src - lo as f64) as f32;
        out.push(samples[lo] * (1.0 - frac) + samples[hi] * frac);
    }

    out
}

fn build_hint(path: &str) -> Hint {
    let mut hint = Hint::new();

    if let Some(extension) = std::path::Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
    {
        hint.with_extension(extension);
    }

    hint
}

#[cfg(test)]
mod tests {
    use super::{TARGET_SAMPLE_RATE, decode_audio_file};
    use anyhow::Result;
    use std::{
        f32::consts::PI,
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn decode_audio_file_should_resample_stereo_wav_to_target_rate() -> Result<()> {
        let path = write_test_wav(48_000, 2, 1.0)?;
        let decoded = decode_audio_file(path.to_string_lossy().as_ref())?;
        fs::remove_file(&path).ok();

        assert_eq!(decoded.sample_rate, TARGET_SAMPLE_RATE);
        assert_eq!(decoded.channels, 1);
        assert!((decoded.duration_s - 1.0).abs() < 0.02);
        assert!((decoded.samples.len() as i64 - TARGET_SAMPLE_RATE as i64).abs() <= 8);

        Ok(())
    }

    #[test]
    fn decode_audio_file_should_keep_target_rate_for_mono_wav() -> Result<()> {
        let path = write_test_wav(TARGET_SAMPLE_RATE, 1, 0.25)?;
        let decoded = decode_audio_file(path.to_string_lossy().as_ref())?;
        fs::remove_file(&path).ok();

        assert_eq!(decoded.sample_rate, TARGET_SAMPLE_RATE);
        assert_eq!(decoded.channels, 1);
        assert!((decoded.duration_s - 0.25).abs() < 0.02);
        assert!((decoded.samples.len() as i64 - 6_000).abs() <= 4);

        Ok(())
    }

    fn write_test_wav(sample_rate: u32, channels: u16, duration_s: f32) -> Result<PathBuf> {
        let frames = (sample_rate as f32 * duration_s) as usize;
        let wav = build_pcm_wav(sample_rate, channels, frames);
        let path = std::env::temp_dir().join(format!(
            "transcript-audio-test-{}-{}.wav",
            std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
        ));

        fs::write(&path, wav)?;
        Ok(path)
    }

    fn build_pcm_wav(sample_rate: u32, channels: u16, frames: usize) -> Vec<u8> {
        let bits_per_sample = 16u16;
        let bytes_per_sample = (bits_per_sample / 8) as usize;
        let block_align = channels as usize * bytes_per_sample;
        let data_size = frames * block_align;
        let chunk_size = 36 + data_size as u32;
        let byte_rate = sample_rate * block_align as u32;
        let mut wav = Vec::with_capacity(44 + data_size);

        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&chunk_size.to_le_bytes());
        wav.extend_from_slice(b"WAVE");
        wav.extend_from_slice(b"fmt ");
        wav.extend_from_slice(&16u32.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&channels.to_le_bytes());
        wav.extend_from_slice(&sample_rate.to_le_bytes());
        wav.extend_from_slice(&byte_rate.to_le_bytes());
        wav.extend_from_slice(&(block_align as u16).to_le_bytes());
        wav.extend_from_slice(&bits_per_sample.to_le_bytes());
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&(data_size as u32).to_le_bytes());

        for frame in 0..frames {
            let phase = 2.0 * PI * 440.0 * frame as f32 / sample_rate as f32;
            let sample = (phase.sin() * i16::MAX as f32 * 0.25) as i16;

            for _ in 0..channels {
                wav.extend_from_slice(&sample.to_le_bytes());
            }
        }

        wav
    }
}

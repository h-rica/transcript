use anyhow::{Context, Result};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use crate::audio::decoder::decode_audio_file;

pub struct WhisperSegment {
    pub speaker: String,
    pub text: String,
    pub start_s: f32,
    pub end_s: f32,
    pub language: String,
}

pub struct WhisperModel {
    ctx: WhisperContext,
}

impl WhisperModel {
    pub fn load(model_path: &str) -> Result<Self> {
        let ctx = WhisperContext::new_with_params(
            model_path,
            WhisperContextParameters::default(),
        )
            .with_context(|| format!("Failed to load Whisper model: {model_path}"))?;

        Ok(Self { ctx })
    }

    pub fn transcribe(&self, audio_path: &str, language: &str) -> Result<Vec<WhisperSegment>> {
        let audio = decode_audio_file(audio_path)?;

        let samples = if audio.sample_rate != 16_000 {
            resample(&audio.samples, audio.sample_rate, 16_000)
        } else {
            audio.samples
        };

        let mut state = self.ctx.create_state()
            .context("Failed to create Whisper state")?;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some(language));
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_n_threads(4);

        state.full(params, &samples)
            .context("Whisper inference failed")?;

        // as_iter() is the idiomatic API in whisper-rs 0.16
        let segments = state
            .as_iter()
            .map(|seg| WhisperSegment {
                speaker: "Speaker A".to_string(),
                text: seg.to_string().trim().to_string(),
                start_s: seg.start_timestamp() as f32 / 100.0,
                end_s: seg.end_timestamp() as f32 / 100.0,
                language: language.to_string(),
            })
            .collect();

        Ok(segments)
    }
}

fn resample(samples: &[f32], from_hz: u32, to_hz: u32) -> Vec<f32> {
    if from_hz == to_hz {
        return samples.to_vec();
    }
    let ratio = from_hz as f64 / to_hz as f64;
    let out_len = (samples.len() as f64 / ratio) as usize;
    let mut out = Vec::with_capacity(out_len);

    for i in 0..out_len {
        let src = i as f64 * ratio;
        let lo = src.floor() as usize;
        let hi = (lo + 1).min(samples.len() - 1);
        let frac = (src - lo as f64) as f32;
        out.push(samples[lo] * (1.0 - frac) + samples[hi] * frac);
    }

    out
}
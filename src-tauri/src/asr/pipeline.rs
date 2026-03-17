use anyhow::{Context, Result, ensure};
use ndarray::Array2;

use crate::asr::acoustic::AcousticTokenizer;
use crate::asr::decoder::QwenDecoderStub;
use crate::asr::semantic::SemanticTokenizer;

pub struct TranscriptSegment {
    pub speaker: String,
    pub text: String,
    pub start_s: f32,
    pub end_s: f32,
    pub language: String,
}

type EncodedLatents = (Vec<Vec<f32>>, Vec<Vec<f32>>);

pub struct VibeVoicePipeline {
    acoustic: AcousticTokenizer,
    semantic: SemanticTokenizer,
    decoder: QwenDecoderStub,
}

impl VibeVoicePipeline {
    pub fn load(acoustic_path: &str, semantic_path: &str) -> Result<Self> {
        let acoustic =
            AcousticTokenizer::load(acoustic_path).context("Failed to load acoustic tokenizer")?;
        let semantic =
            SemanticTokenizer::load(semantic_path).context("Failed to load semantic tokenizer")?;

        Ok(Self {
            acoustic,
            semantic,
            decoder: QwenDecoderStub::new(),
        })
    }

    pub fn encode(&mut self, samples: &[f32]) -> Result<EncodedLatents> {
        let audio = Array2::from_shape_vec((1, samples.len()), samples.to_vec())
            .context("Failed to build audio tensor")?;

        let acoustic_latents = self
            .acoustic
            .encode(&audio)
            .context("Acoustic encoding failed")?;

        let semantic_latents = self
            .semantic
            .encode(&audio)
            .context("Semantic encoding failed")?;

        Ok((acoustic_latents, semantic_latents))
    }

    pub fn transcribe(
        &mut self,
        samples: &[f32],
        language: &str,
        sample_rate: u32,
    ) -> Result<Vec<TranscriptSegment>> {
        ensure!(sample_rate > 0, "sample_rate must be greater than zero");
        ensure!(!samples.is_empty(), "Audio samples cannot be empty");

        let (acoustic_latents, semantic_latents) = self.encode(samples)?;
        let duration_s = samples.len() as f32 / sample_rate as f32;
        let placeholder_text = self.decoder.decode_placeholder(
            &acoustic_latents,
            &semantic_latents,
            duration_s,
            language,
        )?;

        Ok(vec![TranscriptSegment {
            speaker: "Speaker A".to_string(),
            text: placeholder_text,
            start_s: 0.0,
            end_s: duration_s,
            language: language.to_string(),
        }])
    }
}

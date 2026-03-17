use anyhow::{ensure, Context, Result};
use ndarray::Array2;

use crate::asr::acoustic::AcousticTokenizer;
use crate::asr::semantic::SemanticTokenizer;

#[allow(dead_code)]
pub struct TranscriptSegment {
    pub speaker: String,
    pub text: String,
    pub start_s: f32,
    pub end_s: f32,
    pub language: String,
}

#[allow(dead_code)]
pub struct VibeVoicePipeline {
    acoustic: AcousticTokenizer,
    semantic: SemanticTokenizer,
}

#[allow(dead_code)]
impl VibeVoicePipeline {
    pub fn load(acoustic_path: &str, semantic_path: &str) -> Result<Self> {
        let acoustic =
            AcousticTokenizer::load(acoustic_path).context("Failed to load acoustic tokenizer")?;
        let semantic =
            SemanticTokenizer::load(semantic_path).context("Failed to load semantic tokenizer")?;

        Ok(Self { acoustic, semantic })
    }

    /// Run both tokenizers on the audio samples.
    /// Audio is mono f32 PCM at 24kHz.
    pub fn encode(&mut self, samples: &[f32]) -> Result<(Vec<Vec<f32>>, Vec<Vec<f32>>)> {
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

    /// Full pipeline: encode audio -> decode to segments.
    /// Qwen2.5 decoder is Phase 2 and still returns placeholder segments.
    pub fn transcribe(
        &mut self,
        samples: &[f32],
        language: &str,
        sample_rate: u32,
    ) -> Result<Vec<TranscriptSegment>> {
        ensure!(sample_rate > 0, "sample_rate must be greater than zero");

        let (acoustic_latents, semantic_latents) = self.encode(samples)?;

        let acoustic_frames = acoustic_latents.len();
        let semantic_frames = semantic_latents.len();
        let duration_s = samples.len() as f32 / sample_rate as f32;

        // TODO: Phase 2 - replace with the Qwen2.5 candle-transformers decoder.
        Ok(vec![TranscriptSegment {
            speaker: "Speaker A".to_string(),
            text: format!(
                "[VibeVoice ONNX] acoustic={} semantic={} frames encoded - {:.1}s audio - decoder in Phase 2",
                acoustic_frames, semantic_frames, duration_s
            ),
            start_s: 0.0,
            end_s: duration_s,
            language: language.to_string(),
        }])
    }
}

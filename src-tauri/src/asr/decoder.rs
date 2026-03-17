use anyhow::{Result, ensure};

pub struct QwenDecoderStub;

impl QwenDecoderStub {
    pub fn new() -> Self {
        Self
    }

    pub fn decode_placeholder(
        &self,
        acoustic_latents: &[Vec<f32>],
        semantic_latents: &[Vec<f32>],
        duration_s: f32,
        language: &str,
    ) -> Result<String> {
        ensure!(
            !acoustic_latents.is_empty() || !semantic_latents.is_empty(),
            "Tokenizer output is empty"
        );

        Ok(format!(
            "[VibeVoice ONNX] acoustic={} semantic={} frames encoded - {:.1}s audio - {} decoder placeholder",
            acoustic_latents.len(),
            semantic_latents.len(),
            duration_s,
            if language.trim().is_empty() {
                "auto"
            } else {
                language
            }
        ))
    }
}

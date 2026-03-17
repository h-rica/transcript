use anyhow::{Context, Result, anyhow, bail};
use ndarray::{Array2, Axis, Ix3};
use ort::{
    session::{Session, builder::GraphOptimizationLevel},
    value::TensorRef,
};

#[allow(dead_code)]
pub struct SemanticTokenizer {
    session: Session,
}

#[allow(dead_code)]
impl SemanticTokenizer {
    pub fn load(model_path: &str) -> Result<Self> {
        let session = Session::builder()
            .map_err(|e| anyhow!("Failed to create ORT session builder: {e}"))?
            .with_optimization_level(GraphOptimizationLevel::Level1)
            .map_err(|e| anyhow!("Failed to set optimization level: {e}"))?
            .with_intra_threads(4)
            .map_err(|e| anyhow!("Failed to set intra threads: {e}"))?
            .commit_from_file(model_path)
            .map_err(|e| anyhow!("Failed to load semantic model {model_path}: {e}"))?;

        Ok(Self { session })
    }

    /// Audio is mono f32 PCM at 24kHz with shape [1, samples].
    /// Returns latents with shape [frames, 128].
    pub fn encode(&mut self, audio: &Array2<f32>) -> Result<Vec<Vec<f32>>> {
        let input = TensorRef::from_array_view(audio).context("Failed to create input tensor")?;

        let outputs = self
            .session
            .run(ort::inputs!["audio" => input])
            .context("Semantic tokenizer inference failed")?;

        let latents = outputs["semantic_latents"]
            .try_extract_array::<f32>()
            .map_err(|e| anyhow!("Failed to extract semantic latents: {e}"))?
            .into_dimensionality::<Ix3>()
            .context("Semantic latents must have shape [batch, frames, dims]")?;

        if latents.shape()[0] != 1 {
            bail!(
                "Expected semantic latents batch size of 1, got {}",
                latents.shape()[0]
            );
        }

        Ok(latents
            .index_axis(Axis(0), 0)
            .outer_iter()
            .map(|frame| frame.to_vec())
            .collect())
    }
}

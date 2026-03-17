use anyhow::{Context, Result};
use whisper_rs::{
    FullParams, SamplingStrategy, SegmentCallbackData, WhisperContext, WhisperContextParameters,
};

use crate::audio::decoder::{decode_audio_file, resample_mono};

#[derive(Clone, Debug)]
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
        let ctx = WhisperContext::new_with_params(model_path, WhisperContextParameters::default())
            .with_context(|| format!("Failed to load Whisper model: {model_path}"))?;

        Ok(Self { ctx })
    }

    pub fn transcribe_with_callbacks<FP, FS, FA>(
        &self,
        audio_path: &str,
        language: &str,
        mut on_progress: FP,
        mut on_segment: FS,
        should_abort: FA,
    ) -> Result<Vec<WhisperSegment>>
    where
        FP: FnMut(i32) + 'static,
        FS: FnMut(WhisperSegment) + 'static,
        FA: FnMut() -> bool + 'static,
    {
        let audio = decode_audio_file(audio_path)?;
        let samples = resample_mono(&audio.samples, audio.sample_rate, 16_000);

        let mut state = self
            .ctx
            .create_state()
            .context("Failed to create Whisper state")?;

        let normalized_language =
            if language.trim().is_empty() || language.eq_ignore_ascii_case("auto") {
                None
            } else {
                Some(language)
            };
        let output_language = normalized_language.unwrap_or("auto").to_string();

        let progress_callback: Box<dyn FnMut(i32)> = Box::new(move |progress: i32| {
            on_progress(progress);
        });
        let segment_callback: Box<dyn FnMut(SegmentCallbackData)> =
            Box::new(move |segment: SegmentCallbackData| {
                on_segment(WhisperSegment {
                    speaker: "Speaker A".to_string(),
                    text: segment.text.trim().to_string(),
                    start_s: segment.start_timestamp as f32 / 100.0,
                    end_s: segment.end_timestamp as f32 / 100.0,
                    language: output_language.clone(),
                });
            });
        let abort_callback: Box<dyn FnMut() -> bool> = Box::new(should_abort);

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(normalized_language);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_n_threads(4);
        params.set_progress_callback_safe::<Option<Box<dyn FnMut(i32)>>, Box<dyn FnMut(i32)>>(
            Some(progress_callback),
        );
        params.set_segment_callback_safe_lossy::<
            Option<Box<dyn FnMut(SegmentCallbackData)>>,
            Box<dyn FnMut(SegmentCallbackData)>,
        >(Some(segment_callback));
        params
            .set_abort_callback_safe::<Option<Box<dyn FnMut() -> bool>>, Box<dyn FnMut() -> bool>>(
                Some(abort_callback),
            );

        state
            .full(params, &samples)
            .context("Whisper inference failed")?;

        let segments = state
            .as_iter()
            .map(|seg| WhisperSegment {
                speaker: "Speaker A".to_string(),
                text: seg.to_string().trim().to_string(),
                start_s: seg.start_timestamp() as f32 / 100.0,
                end_s: seg.end_timestamp() as f32 / 100.0,
                language: normalized_language.unwrap_or("auto").to_string(),
            })
            .collect();

        Ok(segments)
    }
}

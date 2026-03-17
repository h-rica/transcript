use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Instant,
};

use anyhow::{Context, Result, anyhow};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State, path::BaseDirectory};

use crate::{
    asr::{
        pipeline::{TranscriptSegment, VibeVoicePipeline},
        whisper::{WhisperModel, WhisperSegment},
    },
    audio::decoder::decode_audio_file,
};

#[derive(Serialize, Clone)]
pub struct ProgressPayload {
    pub percent: f32,
    pub elapsed_s: u32,
}

#[derive(Serialize, Clone)]
pub struct SegmentPayload {
    pub speaker: String,
    pub text: String,
    pub start_s: f32,
    pub end_s: f32,
    pub language: String,
}

#[derive(Serialize, Clone)]
pub struct CompletePayload {
    pub segments: u32,
    pub speakers: u32,
    pub words: u32,
    pub language: String,
    pub elapsed_s: u32,
}

#[derive(Default)]
pub struct TranscriptionControl {
    active_cancel: Mutex<Option<Arc<AtomicBool>>>,
}

impl TranscriptionControl {
    fn start(&self) -> Result<ActiveTranscription<'_>, String> {
        let mut active = self
            .active_cancel
            .lock()
            .map_err(|_| "Transcription state is poisoned".to_string())?;

        if active.is_some() {
            return Err("A transcription is already running".to_string());
        }

        let cancel_flag = Arc::new(AtomicBool::new(false));
        *active = Some(cancel_flag.clone());

        Ok(ActiveTranscription {
            control: self,
            cancel_flag,
        })
    }

    fn cancel(&self) {
        if let Ok(active) = self.active_cancel.lock()
            && let Some(cancel_flag) = active.as_ref()
        {
            cancel_flag.store(true, Ordering::SeqCst);
        }
    }

    fn finish(&self, cancel_flag: &Arc<AtomicBool>) {
        if let Ok(mut active) = self.active_cancel.lock()
            && active
                .as_ref()
                .map(|current| Arc::ptr_eq(current, cancel_flag))
                .unwrap_or(false)
        {
            *active = None;
        }
    }
}

struct ActiveTranscription<'a> {
    control: &'a TranscriptionControl,
    cancel_flag: Arc<AtomicBool>,
}

impl ActiveTranscription<'_> {
    fn cancel_flag(&self) -> Arc<AtomicBool> {
        self.cancel_flag.clone()
    }
}

impl Drop for ActiveTranscription<'_> {
    fn drop(&mut self) {
        self.control.finish(&self.cancel_flag);
    }
}

#[tauri::command]
pub async fn transcribe_file(
    app: AppHandle,
    control: State<'_, TranscriptionControl>,
    path: String,
    model_id: String,
    language: String,
) -> Result<(), String> {
    let active_run = control.start()?;
    let cancel_flag = active_run.cancel_flag();

    emit_progress(&app, 0.0, 0).map_err(|error| error.to_string())?;

    let job_app = app.clone();
    let job_cancel = cancel_flag.clone();
    let job = tokio::task::spawn_blocking(move || {
        run_transcription_job(&job_app, &job_cancel, &path, &model_id, &language)
    });

    let summary = match job.await {
        Ok(Ok(summary)) => summary,
        Ok(Err(error)) => {
            if cancel_flag.load(Ordering::SeqCst) {
                return Err("Transcription cancelled".to_string());
            }

            return Err(error.to_string());
        }
        Err(error) => return Err(error.to_string()),
    };

    if cancel_flag.load(Ordering::SeqCst) {
        return Err("Transcription cancelled".to_string());
    }

    emit_progress(&app, 1.0, summary.elapsed_s).map_err(|error| error.to_string())?;
    app.emit("transcription_complete", summary)
        .map_err(|error| error.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn cancel_transcription(control: State<'_, TranscriptionControl>) -> Result<(), String> {
    control.cancel();
    Ok(())
}

fn run_transcription_job(
    app: &AppHandle,
    cancel_flag: &Arc<AtomicBool>,
    path: &str,
    model_id: &str,
    language: &str,
) -> Result<CompletePayload> {
    let start = Instant::now();

    match model_id {
        "vibevoice-int8" => run_vibevoice_transcription(app, cancel_flag, path, language, start),
        _ => run_whisper_transcription(app, cancel_flag, path, model_id, language, start),
    }
}

fn run_whisper_transcription(
    app: &AppHandle,
    cancel_flag: &Arc<AtomicBool>,
    path: &str,
    model_id: &str,
    language: &str,
    start: Instant,
) -> Result<CompletePayload> {
    let model_path = resolve_whisper_model_path(app, model_id)?;
    let model = WhisperModel::load(model_path.to_string_lossy().as_ref())?;
    emit_progress(app, 0.05, elapsed_s(start))?;

    let callback_error = Arc::new(Mutex::new(None::<String>));
    let progress_app = app.clone();
    let progress_cancel = cancel_flag.clone();
    let progress_error = callback_error.clone();
    let progress_started_at = start;

    let segment_app = app.clone();
    let segment_cancel = cancel_flag.clone();
    let segment_error = callback_error.clone();

    let abort_cancel = cancel_flag.clone();
    let abort_error = callback_error.clone();

    let segments = model.transcribe_with_callbacks(
        path,
        language,
        move |progress| {
            let percent = 0.05 + (progress as f32 / 100.0) * 0.90;
            if let Err(error) = emit_progress(
                &progress_app,
                percent.min(0.95),
                elapsed_s(progress_started_at),
            ) {
                record_callback_error(&progress_error, &progress_cancel, error);
            }
        },
        move |segment| {
            if segment.text.is_empty() {
                return;
            }

            if let Err(error) = segment_app.emit(
                "transcription_segment",
                segment_payload(
                    segment.speaker.clone(),
                    segment.text.clone(),
                    segment.start_s,
                    segment.end_s,
                    segment.language.clone(),
                ),
            ) {
                record_callback_error(&segment_error, &segment_cancel, error);
            }
        },
        move || abort_cancel.load(Ordering::SeqCst) || callback_failed(&abort_error),
    )?;

    if let Some(error) = take_callback_error(&callback_error) {
        return Err(anyhow!(error));
    }

    if cancel_flag.load(Ordering::SeqCst) {
        return Err(anyhow!("Transcription cancelled"));
    }

    emit_progress(app, 0.98, elapsed_s(start))?;
    Ok(build_whisper_summary(&segments, language, elapsed_s(start)))
}

fn run_vibevoice_transcription(
    app: &AppHandle,
    cancel_flag: &Arc<AtomicBool>,
    path: &str,
    language: &str,
    start: Instant,
) -> Result<CompletePayload> {
    let assets = resolve_vibevoice_assets(app)?;
    ensure_not_cancelled(cancel_flag)?;

    let audio = decode_audio_file(path)?;
    emit_progress(app, 0.20, elapsed_s(start))?;
    ensure_not_cancelled(cancel_flag)?;

    let mut pipeline = VibeVoicePipeline::load(
        assets.acoustic.to_string_lossy().as_ref(),
        assets.semantic.to_string_lossy().as_ref(),
    )
    .context("Failed to initialize VibeVoice pipeline")?;
    emit_progress(app, 0.45, elapsed_s(start))?;

    let segments = pipeline
        .transcribe(
            &audio.samples,
            &requested_language(language),
            audio.sample_rate,
        )
        .context("VibeVoice transcription failed")?;
    emit_progress(app, 0.90, elapsed_s(start))?;
    ensure_not_cancelled(cancel_flag)?;

    for segment in &segments {
        app.emit(
            "transcription_segment",
            segment_payload(
                segment.speaker.clone(),
                segment.text.clone(),
                segment.start_s,
                segment.end_s,
                segment.language.clone(),
            ),
        )?;
    }

    Ok(build_transcript_summary(
        &segments,
        language,
        elapsed_s(start),
    ))
}

fn emit_progress(app: &AppHandle, percent: f32, elapsed_s: u32) -> tauri::Result<()> {
    app.emit(
        "transcription_progress",
        ProgressPayload { percent, elapsed_s },
    )
}

fn segment_payload(
    speaker: String,
    text: String,
    start_s: f32,
    end_s: f32,
    language: String,
) -> SegmentPayload {
    SegmentPayload {
        speaker,
        text,
        start_s,
        end_s,
        language,
    }
}

fn build_whisper_summary(
    segments: &[WhisperSegment],
    requested: &str,
    elapsed_s: u32,
) -> CompletePayload {
    let speakers = segments
        .iter()
        .map(|segment| segment.speaker.as_str())
        .collect::<HashSet<_>>()
        .len() as u32;
    let words = segments
        .iter()
        .map(|segment| segment.text.split_whitespace().count() as u32)
        .sum();

    CompletePayload {
        segments: segments.len() as u32,
        speakers,
        words,
        language: resolve_output_language(
            requested,
            segments.first().map(|segment| segment.language.as_str()),
        ),
        elapsed_s,
    }
}

fn build_transcript_summary(
    segments: &[TranscriptSegment],
    requested: &str,
    elapsed_s: u32,
) -> CompletePayload {
    let speakers = segments
        .iter()
        .map(|segment| segment.speaker.as_str())
        .collect::<HashSet<_>>()
        .len() as u32;
    let words = segments
        .iter()
        .map(|segment| segment.text.split_whitespace().count() as u32)
        .sum();

    CompletePayload {
        segments: segments.len() as u32,
        speakers,
        words,
        language: resolve_output_language(
            requested,
            segments.first().map(|segment| segment.language.as_str()),
        ),
        elapsed_s,
    }
}

fn resolve_output_language(requested: &str, detected: Option<&str>) -> String {
    if requested.trim().is_empty() || requested.eq_ignore_ascii_case("auto") {
        detected.unwrap_or("auto").to_string()
    } else {
        requested.to_string()
    }
}

fn requested_language(language: &str) -> String {
    if language.trim().is_empty() {
        "auto".to_string()
    } else {
        language.to_string()
    }
}

fn ensure_not_cancelled(cancel_flag: &Arc<AtomicBool>) -> Result<()> {
    if cancel_flag.load(Ordering::SeqCst) {
        return Err(anyhow!("Transcription cancelled"));
    }

    Ok(())
}

fn callback_failed(callback_error: &Arc<Mutex<Option<String>>>) -> bool {
    callback_error
        .lock()
        .map(|error| error.is_some())
        .unwrap_or(true)
}

fn record_callback_error<E>(
    callback_error: &Arc<Mutex<Option<String>>>,
    cancel_flag: &Arc<AtomicBool>,
    error: E,
) where
    E: ToString,
{
    if let Ok(mut slot) = callback_error.lock()
        && slot.is_none()
    {
        *slot = Some(error.to_string());
    }

    cancel_flag.store(true, Ordering::SeqCst);
}

fn take_callback_error(callback_error: &Arc<Mutex<Option<String>>>) -> Option<String> {
    callback_error.lock().ok().and_then(|mut slot| slot.take())
}

fn elapsed_s(start: Instant) -> u32 {
    start.elapsed().as_secs() as u32
}

fn resolve_whisper_model_path(app: &AppHandle, model_id: &str) -> Result<PathBuf> {
    let relative_path = match model_id {
        "whisper-tiny" => "models/ggml-tiny.bin",
        "whisper-medium" => "models/ggml-medium.bin",
        "whisper-large-v3" => "models/ggml-large-v3.bin",
        other => return Err(anyhow!("Unknown model: {other}")),
    };

    resolve_model_asset(app, relative_path)
}

struct VibeVoiceAssets {
    acoustic: PathBuf,
    semantic: PathBuf,
    _acoustic_data: PathBuf,
    _semantic_data: PathBuf,
}

fn resolve_vibevoice_assets(app: &AppHandle) -> Result<VibeVoiceAssets> {
    Ok(VibeVoiceAssets {
        acoustic: resolve_model_asset(app, "models/onnx/vibevoice_acoustic.onnx")?,
        _acoustic_data: resolve_model_asset(app, "models/onnx/vibevoice_acoustic.onnx.data")?,
        semantic: resolve_model_asset(app, "models/onnx/vibevoice_semantic.onnx")?,
        _semantic_data: resolve_model_asset(app, "models/onnx/vibevoice_semantic.onnx.data")?,
    })
}

fn resolve_model_asset(app: &AppHandle, relative_path: &str) -> Result<PathBuf> {
    let app_data_path = app
        .path()
        .resolve(relative_path, BaseDirectory::AppData)
        .ok();
    if let Some(path) = app_data_path.filter(|path| path.exists()) {
        return Ok(path);
    }

    let resource_path = app
        .path()
        .resolve(relative_path, BaseDirectory::Resource)
        .ok();
    if let Some(path) = resource_path.filter(|path| path.exists()) {
        return Ok(path);
    }

    let dev_path = dev_resource_path(relative_path)?;
    if dev_path.exists() {
        return Ok(dev_path);
    }

    Err(anyhow!("Cannot locate asset: {relative_path}"))
}

fn dev_resource_path(relative_path: &str) -> Result<PathBuf> {
    let cwd = std::env::current_dir().context("Cannot resolve current working directory")?;
    let repo_root = if cwd
        .file_name()
        .map(|name| name == "src-tauri")
        .unwrap_or(false)
    {
        cwd.parent()
            .map(Path::to_path_buf)
            .ok_or_else(|| anyhow!("Cannot resolve repository root"))?
    } else {
        cwd
    };
    let resource_relative = relative_path
        .strip_prefix("models/")
        .unwrap_or(relative_path);

    Ok(repo_root
        .join("src-tauri")
        .join("resources")
        .join(Path::new(resource_relative)))
}

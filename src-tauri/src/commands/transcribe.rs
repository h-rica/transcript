use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::asr::whisper::WhisperModel;

// Tauri event payloads

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

#[tauri::command]
pub async fn transcribe_file(
    app: AppHandle,
    path: String,
    model_id: String,
    language: String,
) -> Result<(), String> {
    let start = std::time::Instant::now();

    // Emit: starting
    app.emit("transcription_progress", ProgressPayload {
        percent: 0.0,
        elapsed_s: 0,
    }).map_err(|e| e.to_string())?;

    // Resolve model path
    let model_path = resolve_model_path(&model_id)
        .map_err(|e| e.to_string())?;

    // Load model
    let model = WhisperModel::load(&model_path)
        .map_err(|e| e.to_string())?;

    app.emit("transcription_progress", ProgressPayload {
        percent: 0.05,
        elapsed_s: start.elapsed().as_secs() as u32,
    }).map_err(|e| e.to_string())?;

    // Run transcription
    let segments = model
        .transcribe(&path, &language)
        .map_err(|e| e.to_string())?;

    let total = segments.len() as f32;
    let mut word_count = 0u32;
    let mut speaker_set = std::collections::HashSet::new();

    // Emit each segment
    for (i, seg) in segments.iter().enumerate() {
        word_count += seg.text.split_whitespace().count() as u32;
        speaker_set.insert(seg.speaker.clone());

        app.emit("transcription_segment", SegmentPayload {
            speaker: seg.speaker.clone(),
            text: seg.text.clone(),
            start_s: seg.start_s,
            end_s: seg.end_s,
            language: seg.language.clone(),
        }).map_err(|e| e.to_string())?;

        app.emit("transcription_progress", ProgressPayload {
            percent: 0.05 + (i as f32 + 1.0) / total * 0.95,
            elapsed_s: start.elapsed().as_secs() as u32,
        }).map_err(|e| e.to_string())?;
    }

    // Emit: complete
    app.emit("transcription_complete", CompletePayload {
        segments: segments.len() as u32,
        speakers: speaker_set.len() as u32,
        words: word_count,
        language: language.clone(),
        elapsed_s: start.elapsed().as_secs() as u32,
    }).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn cancel_transcription() -> Result<(), String> {
    // TODO Phase 1 Week 4 — cancellation token
    Ok(())
}

fn resolve_model_path(model_id: &str) -> anyhow::Result<String> {
    let base = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot find app data directory"))?
        .join("transcript")
        .join("models");

    let filename = match model_id {
        "whisper-tiny" => "ggml-tiny.bin",
        "whisper-medium" => "ggml-medium.bin",
        "whisper-large-v3" => "ggml-large-v3.bin",
        other => return Err(anyhow::anyhow!("Unknown model: {other}")),
    };

    Ok(base.join(filename).to_string_lossy().to_string())
}
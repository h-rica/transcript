use leptos::prelude::*;
use serde::Serialize;

use crate::state::app_state::{
    AudioInfo, TranscriptSegment, TranscriptionProgress, TranscriptionRequest,
    TranscriptionSessionState, TranscriptionStatus, TranscriptionSummary, UiError,
};

#[derive(Serialize)]
struct AudioInfoArgs {
    path: String,
}

pub async fn load_audio_preview(path: String) -> Result<AudioInfo, UiError> {
    if tauri_sys::core::is_tauri() {
        tauri_sys::core::invoke_result::<AudioInfo, String>(
            "get_audio_info",
            &AudioInfoArgs { path },
        )
        .await
        .map_err(UiError::from)
    } else {
        Ok(AudioInfo {
            duration_s: 1478.0,
            size_bytes: 28_400_000,
            format: "mp3".into(),
            bitrate_kbps: Some(154),
        })
    }
}

pub async fn start_transcription(request: TranscriptionRequest) -> Result<(), UiError> {
    if tauri_sys::core::is_tauri() {
        tauri_sys::core::invoke_result::<(), String>("transcribe_file", &request)
            .await
            .map_err(UiError::from)
    } else {
        Ok(())
    }
}

pub fn seed_browser_transcript(session: &TranscriptionSessionState) {
    session.progress.set(TranscriptionProgress {
        percent: 1.0,
        elapsed_s: 52,
    });
    session.segments.set(vec![
        TranscriptSegment {
            speaker: "Speaker A".into(),
            text: "This browser fallback keeps the end-to-end UI reviewable without the Tauri runtime.".into(),
            start_s: 0.0,
            end_s: 8.0,
            language: "en".into(),
        },
        TranscriptSegment {
            speaker: "Speaker B".into(),
            text: "The desktop build replaces this mock content with real Rust events and local decoding.".into(),
            start_s: 8.0,
            end_s: 16.0,
            language: "en".into(),
        },
    ]);
    session.summary.set(Some(TranscriptionSummary {
        segments: 2,
        speakers: 2,
        words: 25,
        language: "en".into(),
        elapsed_s: 52,
    }));
    session.status.set(TranscriptionStatus::Complete);
}

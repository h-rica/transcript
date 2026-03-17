use futures::StreamExt;
use leptos::{prelude::*, task::spawn_local};

use crate::state::app_state::{
    TranscriptSegment, TranscriptionProgress, TranscriptionSessionState, TranscriptionStatus,
    TranscriptionSummary, UiError,
};

pub fn subscribe_transcription_events(session: TranscriptionSessionState) {
    if !tauri_sys::core::is_tauri() || session.listeners_ready.get_untracked() {
        return;
    }

    session.listeners_ready.set(true);

    let progress_state = session.clone();
    spawn_local(async move {
        let Ok(mut listener) =
            tauri_sys::event::listen::<TranscriptionProgress>("transcription_progress").await
        else {
            return;
        };

        while let Some(event) = listener.next().await {
            let payload = event.payload;
            progress_state.progress.set(payload.clone());
            if payload.percent <= 0.05 {
                progress_state.status.set(TranscriptionStatus::LoadingModel);
            } else if payload.percent < 1.0 {
                progress_state.status.set(TranscriptionStatus::Running);
            }
        }
    });

    let segment_state = session.clone();
    spawn_local(async move {
        let Ok(mut listener) =
            tauri_sys::event::listen::<TranscriptSegment>("transcription_segment").await
        else {
            return;
        };

        while let Some(event) = listener.next().await {
            segment_state.status.set(TranscriptionStatus::Running);
            segment_state
                .segments
                .update(|segments| segments.push(event.payload));
        }
    });

    let completion_state = session;
    spawn_local(async move {
        let Ok(mut listener) =
            tauri_sys::event::listen::<TranscriptionSummary>("transcription_complete").await
        else {
            return;
        };

        while let Some(event) = listener.next().await {
            completion_state.summary.set(Some(event.payload));
            completion_state.status.set(TranscriptionStatus::Complete);
            completion_state
                .progress
                .update(|progress| progress.percent = 1.0);
        }
    });
}

pub async fn cancel_transcription() -> Result<(), UiError> {
    tauri_sys::core::invoke_result::<(), String>("cancel_transcription", &())
        .await
        .map_err(UiError::from)
}

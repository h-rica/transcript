use futures::StreamExt;
use leptos::{prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;

use crate::{
    components::{live_segment_list::LiveSegmentList, progress_bar::ProgressBar, sidebar::Sidebar},
    state::app_state::{
        TranscriptSegment, TranscriptionProgress, TranscriptionStatus, TranscriptionSummary,
        use_app_state,
    },
};

#[component]
pub fn TranscriptionPage() -> impl IntoView {
    let state = use_app_state();
    let navigate = use_navigate();

    if tauri_sys::core::is_tauri() {
        let progress_state = state.clone();
        spawn_local(async move {
            let Ok(mut listener) =
                tauri_sys::event::listen::<TranscriptionProgress>("transcription_progress").await
            else {
                return;
            };

            while let Some(event) = listener.next().await {
                let payload = event.payload;
                progress_state.transcription_progress.set(payload.clone());
                if payload.percent <= 0.05 {
                    progress_state
                        .transcription_status
                        .set(TranscriptionStatus::LoadingModel);
                } else if payload.percent < 1.0 {
                    progress_state
                        .transcription_status
                        .set(TranscriptionStatus::Running);
                }
            }
        });

        let segment_state = state.clone();
        spawn_local(async move {
            let Ok(mut listener) =
                tauri_sys::event::listen::<TranscriptSegment>("transcription_segment").await
            else {
                return;
            };

            while let Some(event) = listener.next().await {
                let payload = event.payload;
                segment_state
                    .transcription_status
                    .set(TranscriptionStatus::Running);
                segment_state
                    .transcript_segments
                    .update(|segments| segments.push(payload));
            }
        });

        let complete_state = state.clone();
        spawn_local(async move {
            let Ok(mut listener) =
                tauri_sys::event::listen::<TranscriptionSummary>("transcription_complete").await
            else {
                return;
            };

            while let Some(event) = listener.next().await {
                complete_state.transcription_summary.set(Some(event.payload));
                complete_state
                    .transcription_status
                    .set(TranscriptionStatus::Complete);
                complete_state
                    .transcription_progress
                    .update(|progress| progress.percent = 1.0);
            }
        });
    }

    let cancel_state = state.clone();
    let cancel = Callback::new(move |_| {
        let state = cancel_state.clone();
        spawn_local(async move {
            let _ = tauri_sys::core::invoke_result::<(), String>("cancel_transcription", &()).await;
            state
                .transcription_status
                .set(TranscriptionStatus::Failed("Transcription canceled from the UI.".into()));
        });
    });

    let open_transcript = Callback::new(move |_| navigate("/transcript/current", Default::default()));

    let elapsed_state = state.clone();
    let speed_state = state.clone();
    let segment_count_state = state.clone();
    let language_state = state.clone();
    let percent_state = state.clone();
    let word_state = state.clone();
    let speaker_state = state.clone();
    let status_state = state.clone();
    let pending_state = state.clone();

    view! {
        <div class="flex h-screen w-full">
            <Sidebar/>

            <div class="min-w-0 flex-1 overflow-auto px-8 py-6">
                <div class="mx-auto flex max-w-6xl flex-col gap-6">
                    <div class="flex items-center justify-between gap-4">
                        <div>
                            <p class="text-xs font-semibold uppercase tracking-[0.24em] text-slate-400 dark:text-slate-500">
                                "Transcription"
                            </p>
                            <h1 class="mt-2 text-3xl font-semibold tracking-tight">"Live progress and streaming segments"</h1>
                        </div>
                        <div class="rounded-full bg-slate-100 px-4 py-2 text-sm text-slate-500 dark:bg-slate-900 dark:text-slate-400">
                            {move || state.active_model.get()}
                        </div>
                    </div>

                    <section class="rounded-[32px] border border-slate-200 bg-white p-6 dark:border-slate-800 dark:bg-slate-900">
                        <div class="flex flex-wrap items-center justify-between gap-4">
                            <div>
                                <div class="flex items-center gap-3">
                                    <span class="inline-flex h-3 w-3 rounded-full bg-emerald-500"></span>
                                    <span class="text-sm font-semibold text-slate-900 dark:text-slate-100">
                                        {move || status_label(state.transcription_status.get())}
                                    </span>
                                </div>
                                <p class="mt-2 text-sm text-slate-500 dark:text-slate-400">
                                    {move || {
                                        state
                                            .selected_file
                                            .get()
                                            .map(|file| file.name)
                                            .unwrap_or_else(|| "Waiting for file selection".into())
                                    }}
                                </p>
                            </div>

                            <div class="grid grid-cols-2 gap-3 text-sm sm:grid-cols-4">
                                <StatPill label="Elapsed" value=Signal::derive(move || format_eta(elapsed_state.transcription_progress.get().elapsed_s))/>
                                <StatPill label="Speed" value=Signal::derive(move || speed_label(&speed_state))/>
                                <StatPill label="Segments" value=Signal::derive(move || segment_count_state.transcript_segments.get().len().to_string())/>
                                <StatPill label="Language" value=Signal::derive(move || language_state.selected_language.get().to_uppercase())/>
                            </div>
                        </div>

                        <div class="mt-6">
                            <ProgressBar progress=state.transcription_progress/>
                        </div>
                    </section>

                    <section class="grid gap-6 lg:grid-cols-[1.2fr_0.8fr]">
                        <div class="rounded-[32px] border border-slate-200 bg-white p-6 dark:border-slate-800 dark:bg-slate-900">
                            <div class="mb-4 flex items-center justify-between">
                                <div>
                                    <h2 class="text-lg font-semibold">"Live segments"</h2>
                                    <p class="text-sm text-slate-500 dark:text-slate-400">
                                        "Auto-appends as Rust emits `transcription_segment`."
                                    </p>
                                </div>
                                <div class="rounded-full bg-slate-100 px-3 py-1 text-xs font-medium text-slate-500 dark:bg-slate-800 dark:text-slate-400">
                                    {move || format!("{} items", state.transcript_segments.get().len())}
                                </div>
                            </div>

                            <LiveSegmentList
                                segments=state.transcript_segments
                                pending=Signal::derive(move || {
                                    matches!(
                                        pending_state.transcription_status.get(),
                                        TranscriptionStatus::LoadingModel | TranscriptionStatus::Running
                                    )
                                })
                            />
                        </div>

                        <div class="flex flex-col gap-6">
                            <section class="rounded-[32px] border border-slate-200 bg-white p-6 dark:border-slate-800 dark:bg-slate-900">
                                <h2 class="text-lg font-semibold">"Run summary"</h2>
                                <p class="mt-2 text-sm text-slate-500 dark:text-slate-400">
                                    "Late-stage progress, throughput, and completion state stay visible here."
                                </p>

                                <div class="mt-5 grid gap-3 sm:grid-cols-2">
                                    <Metric label="Percent" value=Signal::derive(move || format!("{:.0}%", percent_state.transcription_progress.get().percent * 100.0))/>
                                    <Metric label="Words" value=Signal::derive(move || {
                                        word_state
                                            .transcription_summary
                                            .get()
                                            .map(|summary| summary.words.to_string())
                                            .unwrap_or_else(|| "--".into())
                                    })/>
                                    <Metric label="Speakers" value=Signal::derive(move || {
                                        speaker_state
                                            .transcription_summary
                                            .get()
                                            .map(|summary| summary.speakers.to_string())
                                            .unwrap_or_else(|| "--".into())
                                    })/>
                                    <Metric label="Status" value=Signal::derive(move || status_label(status_state.transcription_status.get()))/>
                                </div>
                            </section>

                            {move || {
                                let open_transcript = open_transcript.clone();
                                let cancel = cancel.clone();
                                if let Some(summary) = state.transcription_summary.get() {
                                    view! {
                                        <section class="rounded-[32px] border border-emerald-200 bg-emerald-50 p-6 dark:border-emerald-900 dark:bg-emerald-950/40">
                                            <h3 class="text-lg font-semibold text-emerald-900 dark:text-emerald-100">
                                                "Transcription complete"
                                            </h3>
                                            <p class="mt-2 text-sm text-emerald-800 dark:text-emerald-200">
                                                {format!(
                                                    "{} segments, {} speakers, {} words in {}",
                                                    summary.segments,
                                                    summary.speakers,
                                                    summary.words,
                                                    format_eta(summary.elapsed_s)
                                                )}
                                            </p>
                                            <button
                                                class="mt-4 inline-flex rounded-full bg-emerald-900 px-4 py-2 text-sm font-semibold text-white transition hover:bg-emerald-800 dark:bg-emerald-200 dark:text-emerald-950 dark:hover:bg-emerald-300"
                                                on:click=move |_| open_transcript.run(())
                                                type="button"
                                            >
                                                "Open transcript view"
                                            </button>
                                        </section>
                                    }
                                    .into_any()
                                } else {
                                    view! {
                                        <button
                                            class="inline-flex items-center justify-center rounded-[24px] border border-slate-200 px-4 py-3 text-sm font-semibold text-slate-500 transition hover:border-slate-300 hover:text-slate-900 dark:border-slate-800 dark:text-slate-400 dark:hover:border-slate-700 dark:hover:text-slate-100"
                                            on:click=move |_| cancel.run(())
                                            type="button"
                                        >
                                            "Cancel transcription"
                                        </button>
                                    }
                                    .into_any()
                                }
                            }}
                        </div>
                    </section>
                </div>
            </div>
        </div>
    }
}

#[component]
fn StatPill(label: &'static str, #[prop(into)] value: Signal<String>) -> impl IntoView {
    view! {
        <div class="rounded-[24px] bg-slate-100 px-4 py-3 dark:bg-slate-950">
            <p class="text-[11px] font-semibold uppercase tracking-[0.24em] text-slate-400 dark:text-slate-500">
                {label}
            </p>
            <p class="mt-2 text-sm font-semibold text-slate-900 dark:text-slate-100">{move || value.get()}</p>
        </div>
    }
}

#[component]
fn Metric(label: &'static str, #[prop(into)] value: Signal<String>) -> impl IntoView {
    view! {
        <div class="rounded-[24px] bg-slate-100 px-4 py-3 dark:bg-slate-950">
            <div class="text-xs font-semibold uppercase tracking-[0.24em] text-slate-400 dark:text-slate-500">{label}</div>
            <div class="mt-2 text-lg font-semibold text-slate-900 dark:text-slate-100">{move || value.get()}</div>
        </div>
    }
}

fn speed_label(state: &crate::state::app_state::AppState) -> String {
    let progress = state.transcription_progress.get();
    let duration = state.audio_info.get().map(|info| info.duration_s).unwrap_or_default();
    if progress.elapsed_s == 0 || progress.percent <= 0.0 || duration <= 0.0 {
        return "Speed --".to_string();
    }

    let processed_seconds = duration * progress.percent;
    let rtfx = processed_seconds / progress.elapsed_s as f32;
    let eta_seconds = ((duration - processed_seconds).max(0.0) / rtfx.max(0.1)).round() as u32;
    format!("Speed {:.2}x | ETA {}", rtfx, format_eta(eta_seconds))
}

fn format_eta(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    if hours > 0 {
        format!("{hours}:{minutes:02}:{secs:02}")
    } else {
        format!("{minutes:02}:{secs:02}")
    }
}

fn status_label(status: TranscriptionStatus) -> String {
    match status {
        TranscriptionStatus::Idle => "Waiting to start".into(),
        TranscriptionStatus::LoadingModel => "Loading model".into(),
        TranscriptionStatus::Running => "Transcribing".into(),
        TranscriptionStatus::Complete => "Complete".into(),
        TranscriptionStatus::Failed(message) => format!("Failed: {message}"),
    }
}

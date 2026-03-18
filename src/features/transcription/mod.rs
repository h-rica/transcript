mod service;

use leptos::{prelude::*, task::spawn_local};
use leptos_router::{components::A, hooks::use_navigate};

use crate::{
    components::{
        live_segment_list::LiveSegmentList,
        workspace::{WorkspaceRoute, WorkspaceShell},
    },
    features::shared::{format_elapsed, speaker_palette, status_label},
    state::app_state::{
        TranscriptSegment, TranscriptionSessionState, TranscriptionStatus, use_app_shell_state,
        use_transcription_session_state,
    },
};

use service::{cancel_transcription, subscribe_transcription_events};

#[component]
pub fn TranscriptionScreen() -> impl IntoView {
    let shell = use_app_shell_state();
    let session = use_transcription_session_state();
    let navigate = use_navigate();

    subscribe_transcription_events(session.clone());

    let cancel_session = session.clone();
    let cancel = Callback::new(move |_| {
        let session = cancel_session.clone();
        spawn_local(async move {
            let result = cancel_transcription().await;
            session.status.set(match result {
                Ok(_) => TranscriptionStatus::Failed("Transcription canceled from the UI.".into()),
                Err(error) => TranscriptionStatus::Failed(error.to_string()),
            });
        });
    });

    let navigate_to_transcript = navigate.clone();
    let open_transcript =
        Callback::new(move |_| navigate_to_transcript("/transcript/current", Default::default()));
    let navigate_to_preview = navigate.clone();
    let return_to_preview =
        Callback::new(move |_| navigate_to_preview("/preview", Default::default()));

    view! {
        <WorkspaceShell route=WorkspaceRoute::Transcription>
            {move || {
                if shell.selected_file.get().is_none() {
                    return view! {
                        <section class="rounded-[1.35rem] border border-zinc-200 bg-white px-6 py-16 text-center shadow-sm dark:border-white/5 dark:bg-[#30312d]">
                            <p class="text-base font-medium text-zinc-950 dark:text-zinc-100">"No active run"</p>
                            <p class="mt-2 text-sm text-zinc-600 dark:text-zinc-400">
                                "Choose a file and model in preview before opening the transcription workspace."
                            </p>
                            <A
                                attr:class="mt-6 inline-flex h-10 items-center rounded-[0.95rem] border border-zinc-200 px-4 text-sm font-medium text-zinc-700 transition hover:bg-zinc-100 hover:text-zinc-950 dark:border-white/10 dark:text-zinc-300 dark:hover:bg-[#34362f] dark:hover:text-zinc-100"
                                href="/preview"
                            >
                                "Open preview"
                            </A>
                        </section>
                    }
                    .into_any();
                }

                let status = session.status.get();
                let status_text = status_label(&status);
                let progress = session.progress.get();
                let percent = (progress.percent.clamp(0.0, 1.0) * 100.0).round();
                let speed_label = speed_eta_label(&session);
                let failed = matches!(status, TranscriptionStatus::Failed(_));
                let complete = matches!(status, TranscriptionStatus::Complete);
                let segments = session.segments.get();
                let speakers = unique_speakers(&segments);
                let file_name = shell
                    .selected_file
                    .get()
                    .map(|file| file.name)
                    .unwrap_or_else(|| "Waiting for file selection".into());

                view! {
                    <section class="overflow-hidden rounded-[1.35rem] border border-zinc-200 bg-white shadow-sm dark:border-white/5 dark:bg-[#30312d]">
                        <div class="flex flex-wrap items-center gap-3 border-b border-zinc-200 px-5 py-4 dark:border-white/5">
                            <div class="flex items-center gap-2 text-sm font-medium text-zinc-950 dark:text-zinc-100">
                                <span class=status_indicator_class(&status)></span>
                                {status_text.clone()}
                            </div>
                            <span class="text-sm text-zinc-500 dark:text-zinc-500">{file_name}</span>
                        </div>

                        <div class="px-5 py-5">
                            <div class="flex flex-wrap items-end justify-between gap-3">
                                <p class="text-sm font-semibold text-zinc-950 dark:text-zinc-100">
                                    {progress_headline(&status)}
                                </p>
                                <p class="text-sm font-semibold text-zinc-700 dark:text-zinc-200">{format!("{percent:.0}%")}</p>
                            </div>

                            <div class="mt-3 h-2 overflow-hidden rounded-full bg-zinc-200 dark:bg-[#232520]">
                                <div
                                    class="h-full rounded-full bg-zinc-950 transition-[width] dark:bg-zinc-100"
                                    style=format!("width: {:.2}%;", percent)
                                ></div>
                            </div>

                            <div class="mt-3 flex flex-wrap items-center justify-between gap-3 text-sm text-zinc-500 dark:text-zinc-500">
                                <span>{format!("Elapsed {}", format_elapsed(progress.elapsed_s))}</span>
                                <span>{speed_label}</span>
                            </div>

                            <Show when=move || failed>
                                <div class="mt-4 rounded-[1rem] border border-rose-300 bg-rose-50 px-4 py-3 text-sm text-rose-700 dark:border-rose-900/60 dark:bg-rose-950/30 dark:text-rose-200">
                                    {move || session.error.get().unwrap_or_else(|| status_label(&session.status.get()))}
                                </div>
                            </Show>

                            <div class="mt-6 flex flex-wrap items-center justify-between gap-3 border-t border-zinc-200 pt-5 dark:border-white/5">
                                <div>
                                    <p class="text-sm font-medium text-zinc-950 dark:text-zinc-100">"Live segments"</p>
                                    <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-400">
                                        {if complete {
                                            "The run is complete. Review the final transcript or return to preview for another file."
                                        } else {
                                            "Segments stream here while local inference progresses."
                                        }}
                                    </p>
                                </div>

                                <div class="flex flex-wrap items-center gap-4 text-sm text-zinc-500 dark:text-zinc-400">
                                    {if speakers.is_empty() {
                                        view! {
                                            <span class="text-sm text-zinc-500 dark:text-zinc-500">"Speaker legend pending"</span>
                                        }
                                            .into_any()
                                    } else {
                                        speakers
                                            .into_iter()
                                            .map(|speaker| {
                                                let (_, foreground) = speaker_palette(&speaker);
                                                view! {
                                                    <span class="inline-flex items-center gap-2">
                                                        <span class="h-2.5 w-2.5 rounded-full" style=format!("background:{};", foreground)></span>
                                                        {speaker}
                                                    </span>
                                                }
                                            })
                                            .collect_view()
                                            .into_any()
                                    }}
                                </div>
                            </div>

                            <div class="mt-4 min-h-[22rem]">
                                <LiveSegmentList
                                    segments=session.segments
                                    pending=Signal::derive(move || {
                                        matches!(
                                            session.status.get(),
                                            TranscriptionStatus::LoadingModel | TranscriptionStatus::Running
                                        )
                                    })
                                />
                            </div>

                            <div class="mt-5 border-t border-zinc-200 pt-5 dark:border-white/5">
                                {if complete {
                                    view! {
                                        <button
                                            class="inline-flex h-11 w-full items-center justify-center rounded-[0.95rem] border border-zinc-200 bg-zinc-100 px-4 text-sm font-medium text-zinc-950 transition hover:bg-zinc-200 dark:border-white/10 dark:bg-zinc-100 dark:text-zinc-950 dark:hover:bg-zinc-200"
                                            on:click=move |_| open_transcript.run(())
                                            type="button"
                                        >
                                            "Open transcript"
                                        </button>
                                    }
                                        .into_any()
                                } else if failed {
                                    view! {
                                        <button
                                            class="inline-flex h-11 w-full items-center justify-center rounded-[0.95rem] border border-zinc-200 px-4 text-sm font-medium text-zinc-700 transition hover:bg-zinc-100 hover:text-zinc-950 dark:border-white/10 dark:text-zinc-300 dark:hover:bg-[#34362f] dark:hover:text-zinc-100"
                                            on:click=move |_| return_to_preview.run(())
                                            type="button"
                                        >
                                            "Return to preview"
                                        </button>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <button
                                            class="inline-flex h-11 w-full items-center justify-center rounded-[0.95rem] border border-zinc-200 px-4 text-sm font-medium text-zinc-700 transition hover:bg-zinc-100 hover:text-zinc-950 dark:border-white/10 dark:text-zinc-300 dark:hover:bg-[#34362f] dark:hover:text-zinc-100"
                                            on:click=move |_| cancel.run(())
                                            type="button"
                                        >
                                            "Cancel transcription"
                                        </button>
                                    }
                                        .into_any()
                                }}
                            </div>
                        </div>
                    </section>
                }
                .into_any()
            }}
        </WorkspaceShell>
    }
}

fn progress_headline(status: &TranscriptionStatus) -> &'static str {
    match status {
        TranscriptionStatus::LoadingModel => "Loading model...",
        TranscriptionStatus::Complete => "Transcript ready",
        TranscriptionStatus::Failed(_) => "Transcription interrupted",
        _ => "Transcribing...",
    }
}

fn speed_eta_label(session: &TranscriptionSessionState) -> String {
    let progress = session.progress.get();
    let duration = session
        .audio_info
        .get()
        .map(|info| info.duration_s)
        .unwrap_or_default();
    if progress.elapsed_s == 0 || progress.percent <= 0.0 || duration <= 0.0 {
        return "Speed -- / ETA --".to_string();
    }

    let processed_seconds = duration * progress.percent;
    let rtfx = processed_seconds / progress.elapsed_s as f32;
    let remaining = ((duration - processed_seconds).max(0.0) / rtfx.max(0.1)).round() as u32;
    format!("Speed {:.1}x / ETA {}", rtfx, format_elapsed(remaining))
}

fn status_indicator_class(status: &TranscriptionStatus) -> &'static str {
    match status {
        TranscriptionStatus::Complete => "h-2.5 w-2.5 rounded-full bg-emerald-500",
        TranscriptionStatus::Failed(_) => "h-2.5 w-2.5 rounded-full bg-rose-500",
        _ => "h-2.5 w-2.5 rounded-full bg-zinc-500 dark:bg-zinc-300",
    }
}

fn unique_speakers(segments: &[TranscriptSegment]) -> Vec<String> {
    let mut speakers = segments
        .iter()
        .map(|segment| segment.speaker.clone())
        .collect::<Vec<_>>();
    speakers.sort();
    speakers.dedup();
    speakers
}

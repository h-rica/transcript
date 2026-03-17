mod service;

use leptos::{prelude::*, task::spawn_local};
use leptos_router::{components::A, hooks::use_navigate};

use crate::{
    components::{
        live_segment_list::LiveSegmentList,
        workspace::{WorkspaceHeader, WorkspaceRoute, WorkspaceShell},
    },
    features::shared::{format_elapsed, status_label},
    state::app_state::{
        TranscriptionSessionState, TranscriptionStatus, use_app_shell_state,
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

    let open_transcript =
        Callback::new(move |_| navigate("/transcript/current", Default::default()));

    view! {
        <WorkspaceShell route=WorkspaceRoute::Transcription>
            <WorkspaceHeader
                title="Transcribing"
                subtitle="Keep the run visible while local inference warms up, streams segments, and finishes export-ready output."
            >
                <span class="inline-flex items-center rounded-full border border-zinc-200 px-2.5 py-1 text-[11px] font-medium text-zinc-700 dark:border-zinc-800 dark:text-zinc-300">
                    {move || shell.active_model.get()}
                </span>
            </WorkspaceHeader>

            {move || {
                if shell.selected_file.get().is_none() {
                    return view! {
                        <div class="rounded-[1.5rem] border border-dashed border-zinc-300 bg-zinc-100/80 px-6 py-12 text-center dark:border-zinc-800 dark:bg-[#121316]">
                            <p class="text-base font-medium text-zinc-950 dark:text-zinc-100">"No active run"</p>
                            <p class="mt-2 text-sm text-zinc-600 dark:text-zinc-500">
                                "Choose a file and model in the preview flow before opening the transcription screen."
                            </p>
                            <A
                                attr:class="mt-5 inline-flex h-9 items-center rounded-lg border border-zinc-200 px-3 text-sm font-medium text-zinc-700 transition hover:bg-zinc-100 hover:text-zinc-950 dark:border-zinc-800 dark:text-zinc-300 dark:hover:bg-[#17181b] dark:hover:text-zinc-100"
                                href="/preview"
                            >
                                "Open preview"
                            </A>
                        </div>
                    }
                    .into_any();
                }

                let summary = session.summary.get();
                let status = session.status.get();
                let status_text = status_label(&status);
                let progress = session.progress.get();
                let percent = (progress.percent.clamp(0.0, 1.0) * 100.0).round();
                let speed = speed_label(&session);
                let failed = matches!(status, TranscriptionStatus::Failed(_));
                let complete = matches!(status, TranscriptionStatus::Complete);
                let loading = matches!(status, TranscriptionStatus::LoadingModel);

                view! {
                    <>
                        <section class="rounded-[1.5rem] border border-zinc-200 bg-white px-5 py-5 dark:border-zinc-900 dark:bg-[#141519]">
                            <div class="flex flex-wrap items-center justify-between gap-3">
                                <div>
                                    <div class="flex items-center gap-2">
                                        <span class=move || {
                                            if complete {
                                                "h-2.5 w-2.5 rounded-full bg-emerald-500"
                                            } else if failed {
                                                "h-2.5 w-2.5 rounded-full bg-rose-500"
                                            } else {
                                                "h-2.5 w-2.5 rounded-full bg-sky-500"
                                            }
                                        }></span>
                                        <p class="text-sm font-medium text-zinc-950 dark:text-zinc-100">{status_text.clone()}</p>
                                    </div>
                                    <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-500">
                                        {move || {
                                            shell
                                                .selected_file
                                                .get()
                                                .map(|file| file.name)
                                                .unwrap_or_else(|| "Waiting for file selection".into())
                                        }}
                                    </p>
                                </div>
                                <div class="text-sm text-zinc-500 dark:text-zinc-500">{format!("{percent:.0}%")}</div>
                            </div>

                            <div class="mt-4 h-1.5 overflow-hidden rounded-full bg-zinc-200 dark:bg-zinc-800">
                                <div
                                    class="h-full rounded-full bg-zinc-900 transition-[width] dark:bg-zinc-100"
                                    style=format!("width: {:.2}%;", percent)
                                ></div>
                            </div>

                            <div class="mt-4 grid gap-3 sm:grid-cols-2 xl:grid-cols-4">
                                <MetricTile label="Elapsed" value=format_elapsed(progress.elapsed_s)/>
                                <MetricTile label="Speed" value=speed/>
                                <MetricTile label="Segments" value=session.segments.get().len().to_string()/>
                                <MetricTile label="Language" value=shell.selected_language.get().to_uppercase()/>
                            </div>

                            <Show when=move || failed>
                                <div class="mt-4 rounded-xl border border-rose-300 bg-rose-50 px-4 py-3 text-sm text-rose-700 dark:border-rose-900/60 dark:bg-rose-950/30 dark:text-rose-200">
                                    {move || session.error.get().unwrap_or_else(|| status_label(&session.status.get()))}
                                </div>
                            </Show>
                        </section>

                        <section class="grid gap-6 xl:grid-cols-[minmax(0,1fr)_320px]">
                            <div class="rounded-[1.5rem] border border-zinc-200 bg-white px-5 py-5 dark:border-zinc-900 dark:bg-[#141519]">
                                <div class="mb-4 flex items-center justify-between gap-3">
                                    <div>
                                        <p class="text-sm font-medium text-zinc-950 dark:text-zinc-100">"Live segments"</p>
                                        <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-500">
                                            {if loading {
                                                "The model is warming up before the first segment arrives."
                                            } else {
                                                "Segments append here as local inference progresses."
                                            }}
                                        </p>
                                    </div>
                                </div>

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

                            <div class="space-y-4">
                                <section class="rounded-[1.5rem] border border-zinc-200 bg-white px-5 py-5 dark:border-zinc-900 dark:bg-[#141519]">
                                    <p class="text-sm font-medium text-zinc-950 dark:text-zinc-100">"Run summary"</p>
                                    <div class="mt-4 grid gap-3 sm:grid-cols-2 xl:grid-cols-1">
                                        <MetricTile label="Progress" value=format!("{percent:.0}%")/>
                                        <MetricTile
                                            label="Words"
                                            value=summary
                                                .as_ref()
                                                .map(|item| item.words.to_string())
                                                .unwrap_or_else(|| "--".into())
                                        />
                                        <MetricTile
                                            label="Speakers"
                                            value=summary
                                                .as_ref()
                                                .map(|item| item.speakers.to_string())
                                                .unwrap_or_else(|| "--".into())
                                        />
                                        <MetricTile label="Status" value=status_text.clone()/>
                                    </div>
                                </section>

                                {if let Some(summary) = summary {
                                    view! {
                                        <section class="rounded-[1.5rem] border border-emerald-300 bg-emerald-50 px-5 py-5 dark:border-emerald-900/60 dark:bg-emerald-950/20">
                                            <p class="text-sm font-medium text-emerald-900 dark:text-emerald-100">"Transcription complete"</p>
                                            <p class="mt-2 text-sm text-emerald-800 dark:text-emerald-200">
                                                {format!(
                                                    "{} segments / {} speakers / {} words in {}.",
                                                    summary.segments,
                                                    summary.speakers,
                                                    summary.words,
                                                    format_elapsed(summary.elapsed_s)
                                                )}
                                            </p>
                                            <button
                                                class="mt-4 inline-flex h-10 w-full items-center justify-center rounded-xl bg-zinc-950 px-4 text-sm font-medium text-white transition hover:bg-zinc-800 dark:bg-zinc-100 dark:text-zinc-950 dark:hover:bg-zinc-200"
                                                on:click=move |_| open_transcript.run(())
                                                type="button"
                                            >
                                                "Open transcript"
                                            </button>
                                        </section>
                                    }
                                    .into_any()
                                } else {
                                    view! {
                                        <button
                                            class="inline-flex h-10 w-full items-center justify-center rounded-xl border border-zinc-200 bg-white px-4 text-sm font-medium text-zinc-700 transition hover:bg-zinc-100 hover:text-zinc-950 dark:border-zinc-800 dark:bg-[#141519] dark:text-zinc-300 dark:hover:bg-[#17181b] dark:hover:text-zinc-100"
                                            on:click=move |_| cancel.run(())
                                            type="button"
                                        >
                                            "Cancel transcription"
                                        </button>
                                    }
                                    .into_any()
                                }}
                            </div>
                        </section>
                    </>
                }
                .into_any()
            }}
        </WorkspaceShell>
    }
}

fn speed_label(session: &TranscriptionSessionState) -> String {
    let progress = session.progress.get();
    let duration = session
        .audio_info
        .get()
        .map(|info| info.duration_s)
        .unwrap_or_default();
    if progress.elapsed_s == 0 || progress.percent <= 0.0 || duration <= 0.0 {
        return "--".to_string();
    }

    let processed_seconds = duration * progress.percent;
    let rtfx = processed_seconds / progress.elapsed_s as f32;
    let remaining = ((duration - processed_seconds).max(0.0) / rtfx.max(0.1)).round() as u32;
    format!("{rtfx:.2}x / ETA {}", format_elapsed(remaining))
}

#[component]
fn MetricTile(label: &'static str, value: String) -> impl IntoView {
    view! {
        <div class="rounded-xl border border-zinc-200 bg-zinc-100/80 px-4 py-4 dark:border-zinc-800 dark:bg-[#101114]">
            <p class="text-[11px] font-medium uppercase tracking-[0.18em] text-zinc-500">{label}</p>
            <p class="mt-2 text-sm font-semibold text-zinc-950 dark:text-zinc-100">{value}</p>
        </div>
    }
}

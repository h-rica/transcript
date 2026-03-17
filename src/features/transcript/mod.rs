mod service;

use leptos::{prelude::*, task::spawn_local};
use wasm_bindgen_futures::JsFuture;

use crate::{
    components::workspace::{WorkspaceHeader, WorkspaceRoute, WorkspaceShell},
    features::shared::{format_mm_ss, speaker_palette},
    state::app_state::{
        ExportRequest, TranscriptSegment, use_app_shell_state, use_transcript_view_state,
        use_transcription_session_state,
    },
};

use service::export_transcript;

#[component]
pub fn TranscriptScreen() -> impl IntoView {
    let shell = use_app_shell_state();
    let session = use_transcription_session_state();
    let view_state = use_transcript_view_state();

    let speaker_names = move || {
        let mut speakers = session
            .segments
            .get()
            .into_iter()
            .map(|segment| segment.speaker)
            .collect::<Vec<_>>();
        speakers.sort();
        speakers.dedup();
        speakers
    };

    let filtered_segments = move || {
        let active_speaker = view_state.speaker_filter.get();
        session
            .segments
            .get()
            .into_iter()
            .filter(|segment| active_speaker.is_empty() || segment.speaker == active_speaker)
            .collect::<Vec<_>>()
    };

    let copy_segments = {
        let session = session.clone();
        let view_state = view_state.clone();
        Callback::new(move |_| {
            let text = session
                .segments
                .get_untracked()
                .into_iter()
                .map(|segment| {
                    format!(
                        "[{} -> {}] {}: {}",
                        format_mm_ss(segment.start_s),
                        format_mm_ss(segment.end_s),
                        segment.speaker,
                        segment.text
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");

            view_state
                .export_message
                .set(Some("Copying transcript...".into()));
            let view_state = view_state.clone();
            spawn_local(async move {
                let Some(window) = web_sys::window() else {
                    view_state
                        .export_message
                        .set(Some("Clipboard unavailable in this context.".into()));
                    return;
                };
                let clipboard = window.navigator().clipboard();
                let result = JsFuture::from(clipboard.write_text(&text)).await;
                let message = if result.is_ok() {
                    "Copied transcript to clipboard.".into()
                } else {
                    "Clipboard permission was denied by the browser.".into()
                };
                view_state.export_message.set(Some(message));
            });
        })
    };

    let export_state = view_state.clone();
    let run_export = Callback::new(move |_| {
        let format = export_state.export_format.get();
        export_state
            .export_message
            .set(Some("Preparing export...".into()));

        let export_state = export_state.clone();
        spawn_local(async move {
            let request = ExportRequest {
                id: "current".into(),
                format: format.clone(),
                path: String::new(),
            };
            let message = match export_transcript(request).await {
                Ok(_) if tauri_sys::core::is_tauri() => {
                    format!("Export command sent for {}.", format.to_uppercase())
                }
                Ok(_) => format!("Mock export ready for {}.", format.to_uppercase()),
                Err(error) => error.to_string(),
            };
            export_state.export_open.set(false);
            export_state.export_message.set(Some(message));
        });
    });

    view! {
        <WorkspaceShell route=WorkspaceRoute::Transcript>
            <WorkspaceHeader
                title="Transcript"
                subtitle=Signal::derive(move || {
                    shell
                        .selected_file
                        .get()
                        .map(|file| file.name)
                        .unwrap_or_else(|| "Current transcript".into())
                })
            >
                <div class="flex items-center gap-2">
                    <button
                        class="inline-flex h-8 items-center rounded-lg border border-zinc-200 px-3 text-sm font-medium text-zinc-700 transition hover:bg-zinc-100 hover:text-zinc-950 dark:border-zinc-800 dark:text-zinc-300 dark:hover:bg-[#17181b] dark:hover:text-zinc-100"
                        on:click=move |_| copy_segments.run(())
                        type="button"
                    >
                        "Copy all"
                    </button>
                    <div class="relative">
                        <button
                            class="inline-flex h-8 items-center rounded-lg bg-zinc-950 px-3 text-sm font-medium text-white transition hover:bg-zinc-800 dark:bg-zinc-100 dark:text-zinc-950 dark:hover:bg-zinc-200"
                            on:click=move |_| view_state.export_open.update(|open| *open = !*open)
                            type="button"
                        >
                            "Export"
                        </button>
                        <Show when=move || view_state.export_open.get()>
                            <div class="absolute right-0 top-full z-20 mt-2 w-[260px] rounded-[1rem] border border-zinc-200 bg-white p-3 shadow-lg dark:border-zinc-900 dark:bg-[#141519]">
                                <p class="text-[11px] font-medium uppercase tracking-[0.18em] text-zinc-500">"Export format"</p>
                                <div class="mt-3 flex flex-wrap gap-2">
                                    {["txt", "srt"]
                                        .into_iter()
                                        .map(|value| {
                                            let label = value.to_uppercase();
                                            view! {
                                                <button
                                                    class=move || {
                                                        if view_state.export_format.get() == value {
                                                            "rounded-lg border border-zinc-300 bg-zinc-950 px-3 py-2 text-sm font-medium text-white dark:border-zinc-700 dark:bg-zinc-100 dark:text-zinc-950"
                                                        } else {
                                                            "rounded-lg border border-zinc-200 bg-zinc-100 px-3 py-2 text-sm font-medium text-zinc-700 transition hover:bg-zinc-200 dark:border-zinc-800 dark:bg-[#101114] dark:text-zinc-300 dark:hover:bg-[#17181b]"
                                                        }
                                                    }
                                                    on:click=move |_| view_state.export_format.set(value.into())
                                                    type="button"
                                                >
                                                    {label}
                                                </button>
                                            }
                                        })
                                        .collect_view()}
                                    <span class="inline-flex items-center rounded-lg border border-zinc-200 px-3 py-2 text-[11px] font-medium text-zinc-500 dark:border-zinc-800 dark:text-zinc-500">
                                        "DOCX deferred"
                                    </span>
                                </div>
                                <button
                                    class="mt-3 inline-flex h-9 w-full items-center justify-center rounded-lg bg-zinc-950 px-3 text-sm font-medium text-white transition hover:bg-zinc-800 dark:bg-zinc-100 dark:text-zinc-950 dark:hover:bg-zinc-200"
                                    on:click=move |_| run_export.run(())
                                    type="button"
                                >
                                    {move || format!("Export {}", view_state.export_format.get().to_uppercase())}
                                </button>
                            </div>
                        </Show>
                    </div>
                </div>
            </WorkspaceHeader>

            {move || {
                if session.segments.get().is_empty() {
                    return view! {
                        <div class="rounded-[1.15rem] border border-dashed border-zinc-300 bg-zinc-100/80 px-6 py-12 text-center dark:border-zinc-800 dark:bg-[#121316]">
                            <p class="text-base font-medium text-zinc-950 dark:text-zinc-100">"No transcript available"</p>
                            <p class="mt-2 text-sm text-zinc-600 dark:text-zinc-500">
                                "Complete a transcription run before opening transcript review."
                            </p>
                        </div>
                    }
                    .into_any();
                }

                let speaker_segments = filtered_segments();
                let timeline_segments = speaker_segments.clone();
                let raw_segments = speaker_segments.clone();

                view! {
                    <div class="space-y-4">
                        <Show when=move || view_state.export_message.get().is_some()>
                            <p class="text-sm text-zinc-600 dark:text-zinc-400">
                                {move || view_state.export_message.get().unwrap_or_default()}
                            </p>
                        </Show>

                        <section class="rounded-[1.15rem] border border-zinc-200 bg-white px-4 py-4 dark:border-zinc-900 dark:bg-[#141519]">
                            <div class="flex flex-wrap items-center gap-2">
                                <button
                                    class=move || if view_state.speaker_filter.get().is_empty() {
                                        "rounded-full border border-zinc-300 bg-zinc-950 px-3 py-1.5 text-sm font-medium text-white dark:border-zinc-700 dark:bg-zinc-100 dark:text-zinc-950"
                                    } else {
                                        "rounded-full border border-zinc-200 bg-zinc-100 px-3 py-1.5 text-sm font-medium text-zinc-700 transition hover:bg-zinc-200 dark:border-zinc-800 dark:bg-[#101114] dark:text-zinc-300 dark:hover:bg-[#17181b]"
                                    }
                                    on:click=move |_| view_state.speaker_filter.set(String::new())
                                    type="button"
                                >
                                    "All speakers"
                                </button>
                                {speaker_names().into_iter().map(|speaker| {
                                    let tone = speaker_palette(&speaker);
                                    let button_speaker = speaker.clone();
                                    let active_speaker = speaker.clone();
                                    view! {
                                        <button
                                            class=move || {
                                                let active = view_state.speaker_filter.get() == active_speaker;
                                                if active {
                                                    "rounded-full border px-3 py-1.5 text-sm font-medium text-zinc-950 dark:text-zinc-950".to_string()
                                                } else {
                                                    "rounded-full border border-zinc-200 bg-zinc-100 px-3 py-1.5 text-sm font-medium text-zinc-700 transition hover:bg-zinc-200 dark:border-zinc-800 dark:bg-[#101114] dark:text-zinc-300 dark:hover:bg-[#17181b]".into()
                                                }
                                            }
                                            on:click={
                                                let button_speaker = button_speaker.clone();
                                                move |_| view_state.speaker_filter.set(button_speaker.clone())
                                            }
                                            style=move || {
                                                if view_state.speaker_filter.get() == speaker {
                                                    format!("background:{}; border-color:{}; color:{};", tone.0, tone.1, tone.1)
                                                } else {
                                                    String::new()
                                                }
                                            }
                                            type="button"
                                        >
                                            {speaker.clone()}
                                        </button>
                                    }
                                }).collect_view()}
                            </div>

                            <div class="mt-5 border-b border-zinc-200 dark:border-zinc-900">
                                <div class="flex gap-6">
                                    {[('s', "Speakers"), ('t', "Timeline"), ('r', "Raw")]
                                        .into_iter()
                                        .map(|(value, label)| {
                                            let tab_value = match value {
                                                't' => "timeline",
                                                'r' => "raw",
                                                _ => "speakers",
                                            };
                                            view! {
                                                <button
                                                    class=move || {
                                                        if view_state.active_tab.get() == tab_value {
                                                            "border-b-2 border-zinc-950 pb-3 text-sm font-medium text-zinc-950 dark:border-zinc-100 dark:text-zinc-100"
                                                        } else {
                                                            "border-b-2 border-transparent pb-3 text-sm text-zinc-500 transition hover:text-zinc-900 dark:text-zinc-500 dark:hover:text-zinc-200"
                                                        }
                                                    }
                                                    on:click=move |_| view_state.active_tab.set(tab_value.into())
                                                    type="button"
                                                >
                                                    {label}
                                                </button>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            </div>

                            <div class="mt-5">
                                {move || match view_state.active_tab.get().as_str() {
                                    "timeline" => view! {
                                        <div class="space-y-3">
                                            {timeline_segments.clone().into_iter().map(render_timeline_segment).collect_view()}
                                        </div>
                                    }.into_any(),
                                    "raw" => view! {
                                        <pre class="overflow-auto rounded-xl border border-zinc-200 bg-zinc-100/80 p-4 text-sm leading-7 text-zinc-900 dark:border-zinc-800 dark:bg-[#101114] dark:text-zinc-100">
                                            {raw_segments.clone()
                                                .into_iter()
                                                .map(|segment| {
                                                    format!(
                                                        "[{} -> {}] {}: {}\n",
                                                        format_mm_ss(segment.start_s),
                                                        format_mm_ss(segment.end_s),
                                                        segment.speaker,
                                                        segment.text
                                                    )
                                                })
                                                .collect::<String>()}
                                        </pre>
                                    }.into_any(),
                                    _ => view! {
                                        <div class="space-y-3">
                                            {speaker_segments.clone().into_iter().map(render_speaker_segment).collect_view()}
                                        </div>
                                    }.into_any(),
                                }}
                            </div>
                        </section>
                    </div>
                }
                .into_any()
            }}
        </WorkspaceShell>
    }
}

fn render_speaker_segment(segment: TranscriptSegment) -> impl IntoView {
    let (background, foreground) = speaker_palette(&segment.speaker);
    view! {
        <div class="rounded-xl border border-zinc-200 bg-zinc-100/70 px-4 py-4 dark:border-zinc-800 dark:bg-[#101114]">
            <div class="flex gap-4">
                <div
                    class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full text-xs font-semibold"
                    style=format!("background:{}; color:{};", background, foreground)
                >
                    {segment.speaker.chars().next().unwrap_or('S').to_string()}
                </div>
                <div class="min-w-0 flex-1">
                    <div class="flex flex-wrap items-center gap-3 text-[11px] text-zinc-500 dark:text-zinc-500">
                        <span class="font-medium" style=format!("color:{};", foreground)>{segment.speaker.clone()}</span>
                        <span>{format!("{} -> {}", format_mm_ss(segment.start_s), format_mm_ss(segment.end_s))}</span>
                    </div>
                    <p class="mt-2 text-sm leading-7 text-zinc-900 dark:text-zinc-100">{segment.text}</p>
                </div>
            </div>
        </div>
    }
}

fn render_timeline_segment(segment: TranscriptSegment) -> impl IntoView {
    let (_, foreground) = speaker_palette(&segment.speaker);
    view! {
        <div class="grid gap-3 rounded-xl border border-zinc-200 bg-zinc-100/70 px-4 py-4 md:grid-cols-[7rem_1fr] dark:border-zinc-800 dark:bg-[#101114]">
            <div class="text-sm font-medium text-zinc-600 dark:text-zinc-400">{format_mm_ss(segment.start_s)}</div>
            <div>
                <p class="text-[11px] font-medium uppercase tracking-[0.18em]" style=format!("color:{};", foreground)>{segment.speaker.clone()}</p>
                <p class="mt-2 text-sm leading-7 text-zinc-900 dark:text-zinc-100">{segment.text}</p>
            </div>
        </div>
    }
}

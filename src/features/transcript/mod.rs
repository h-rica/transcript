mod service;

use leptos::{prelude::*, task::spawn_local};
use leptos_router::components::A;
use wasm_bindgen_futures::JsFuture;

use crate::{
    components::{
        icons::{AppIcon, UiIcon},
        workspace::{WorkspaceRoute, WorkspaceShell},
    },
    features::shared::{format_mm_ss, speaker_palette},
    state::app_state::{
        ExportRequest, TranscriptSegment, TranscriptViewState, use_app_shell_state,
        use_transcript_view_state, use_transcription_session_state,
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
            {move || {
                if session.segments.get().is_empty() {
                    return view! {
                        <section class="rounded-[1.35rem] border border-zinc-200 bg-white px-6 py-16 text-center shadow-sm dark:border-white/5 dark:bg-[#30312d]">
                            <p class="text-base font-medium text-zinc-950 dark:text-zinc-100">"No transcript available"</p>
                            <p class="mt-2 text-sm text-zinc-600 dark:text-zinc-400">
                                "Complete a transcription run before opening transcript review."
                            </p>
                        </section>
                    }
                    .into_any();
                }

                let speaker_segments = filtered_segments();
                let timeline_segments = speaker_segments.clone();
                let raw_segments = speaker_segments.clone();
                let speaker_list = speaker_names();
                let file_name = shell
                    .selected_file
                    .get()
                    .map(|file| file.name)
                    .unwrap_or_else(|| "Current transcript".into());
                view! {
                    <section class="overflow-hidden rounded-[1.35rem] border border-zinc-200 bg-white shadow-sm dark:border-white/5 dark:bg-[#30312d]">
                        <div class="flex flex-wrap items-center justify-between gap-3 border-b border-zinc-200 px-5 py-4 dark:border-white/5">
                            <div class="flex min-w-0 items-center gap-3 text-sm">
                                <A
                                    attr:class="inline-flex items-center gap-2 text-zinc-500 transition hover:text-zinc-950 dark:text-zinc-500 dark:hover:text-zinc-100"
                                    href="/"
                                >
                                    <UiIcon class="h-4 w-4" icon_name=AppIcon::ChevronLeft/>
                                    "Home"
                                </A>
                                <span class="truncate font-medium text-zinc-950 dark:text-zinc-100">{file_name}</span>
                            </div>

                            <div class="flex items-center gap-2">
                                <button
                                    class="inline-flex h-10 items-center gap-2 rounded-[0.95rem] border border-zinc-200 px-4 text-sm font-medium text-zinc-700 transition hover:bg-zinc-100 hover:text-zinc-950 dark:border-white/10 dark:text-zinc-300 dark:hover:bg-[#34362f] dark:hover:text-zinc-100"
                                    on:click=move |_| copy_segments.run(())
                                    type="button"
                                >
                                    <UiIcon class="h-4 w-4" icon_name=AppIcon::Copy/>
                                    "Copy all"
                                </button>
                                <ExportMenu run_export=run_export view_state=view_state.clone()/>
                            </div>
                        </div>

                        <div class="border-b border-zinc-200 px-5 dark:border-white/5">
                            <div class="flex gap-6 overflow-x-auto">
                                <TranscriptTabButton id="speakers" label="Speakers" view_state=view_state.clone()/>
                                <TranscriptTabButton id="timeline" label="Timeline" view_state=view_state.clone()/>
                                <TranscriptTabButton id="raw" label="Raw" view_state=view_state.clone()/>
                            </div>
                        </div>

                        <div class="flex flex-wrap items-center gap-2 border-b border-zinc-200 px-5 py-4 dark:border-white/5">
                            <button
                                class=move || {
                                    if view_state.speaker_filter.get().is_empty() {
                                        "inline-flex items-center rounded-full border border-zinc-300 bg-zinc-950 px-4 py-1.5 text-sm font-medium text-white dark:border-white/10 dark:bg-[#34362f] dark:text-zinc-50"
                                    } else {
                                        "inline-flex items-center rounded-full border border-zinc-200 px-4 py-1.5 text-sm font-medium text-zinc-700 transition hover:bg-zinc-100 hover:text-zinc-950 dark:border-white/10 dark:text-zinc-300 dark:hover:bg-[#34362f] dark:hover:text-zinc-100"
                                    }
                                }
                                on:click=move |_| view_state.speaker_filter.set(String::new())
                                type="button"
                            >
                                "All speakers"
                            </button>
                            {speaker_list
                                .into_iter()
                                .map(|speaker| {
                                    view! {
                                        <SpeakerFilterButton speaker=speaker view_state=view_state.clone()/>
                                    }
                                })
                                .collect_view()}
                        </div>

                        <Show when=move || view_state.export_message.get().is_some()>
                            <p class="px-5 pt-4 text-sm text-zinc-600 dark:text-zinc-400">
                                {move || view_state.export_message.get().unwrap_or_default()}
                            </p>
                        </Show>

                        <div class="px-5 py-5">
                            {move || match view_state.active_tab.get().as_str() {
                                "timeline" => view! {
                                    <div class="space-y-5">
                                        {timeline_segments.clone().into_iter().map(render_timeline_segment).collect_view()}
                                    </div>
                                }
                                .into_any(),
                                "raw" => view! {
                                    <pre class="overflow-auto rounded-[1rem] bg-zinc-100/80 p-4 text-sm leading-7 text-zinc-900 dark:bg-[#242621] dark:text-zinc-100">
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
                                }
                                .into_any(),
                                _ => view! {
                                    <div class="space-y-6">
                                        {speaker_segments.clone().into_iter().map(render_speaker_segment).collect_view()}
                                    </div>
                                }
                                .into_any(),
                            }}
                        </div>
                    </section>
                }
                .into_any()
            }}
        </WorkspaceShell>
    }
}

#[component]
fn ExportMenu(view_state: TranscriptViewState, run_export: Callback<()>) -> impl IntoView {
    view! {
        <div class="relative">
            <button
                class="inline-flex h-10 items-center gap-2 rounded-[0.95rem] bg-zinc-950 px-4 text-sm font-medium text-white transition hover:bg-zinc-800 dark:bg-zinc-100 dark:text-zinc-950 dark:hover:bg-zinc-200"
                on:click=move |_| view_state.export_open.update(|open| *open = !*open)
                type="button"
            >
                "Export"
                <UiIcon class="h-4 w-4" icon_name=AppIcon::ChevronDown/>
            </button>

            <Show when=move || view_state.export_open.get()>
                <div class="absolute right-0 top-full z-20 mt-2 w-[260px] rounded-[1rem] border border-zinc-200 bg-white p-3 shadow-lg dark:border-white/10 dark:bg-[#242621]">
                    <p class="text-[11px] font-medium uppercase tracking-[0.18em] text-zinc-500">"Export format"</p>
                    <div class="mt-3 flex flex-wrap gap-2">
                        {[("txt", "TXT"), ("srt", "SRT")]
                            .into_iter()
                            .map(|(value, label)| {
                                let button_view_state = view_state.clone();
                                view! {
                                    <button
                                        class=move || {
                                            if button_view_state.export_format.get() == value {
                                                "rounded-full border border-zinc-300 bg-zinc-950 px-3 py-1.5 text-sm font-medium text-white dark:border-white/10 dark:bg-zinc-100 dark:text-zinc-950"
                                            } else {
                                                "rounded-full border border-zinc-200 px-3 py-1.5 text-sm font-medium text-zinc-700 transition hover:bg-zinc-100 hover:text-zinc-950 dark:border-white/10 dark:text-zinc-300 dark:hover:bg-[#34362f] dark:hover:text-zinc-100"
                                            }
                                        }
                                        on:click=move |_| button_view_state.export_format.set(value.into())
                                        type="button"
                                    >
                                        {label}
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>
                    <button
                        class="mt-3 inline-flex h-10 w-full items-center justify-center rounded-[0.95rem] bg-zinc-950 px-4 text-sm font-medium text-white transition hover:bg-zinc-800 dark:bg-zinc-100 dark:text-zinc-950 dark:hover:bg-zinc-200"
                        on:click=move |_| run_export.run(())
                        type="button"
                    >
                        {move || format!("Export {}", view_state.export_format.get().to_uppercase())}
                    </button>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn TranscriptTabButton(
    view_state: TranscriptViewState,
    id: &'static str,
    label: &'static str,
) -> impl IntoView {
    view! {
        <button
            class=move || {
                if view_state.active_tab.get() == id {
                    "border-b-2 border-zinc-950 pb-3 pt-4 text-sm font-medium text-zinc-950 dark:border-zinc-100 dark:text-zinc-100"
                } else {
                    "border-b-2 border-transparent pb-3 pt-4 text-sm text-zinc-500 transition hover:text-zinc-900 dark:text-zinc-500 dark:hover:text-zinc-200"
                }
            }
            on:click=move |_| view_state.active_tab.set(id.into())
            type="button"
        >
            {label}
        </button>
    }
}

#[component]
fn SpeakerFilterButton(view_state: TranscriptViewState, speaker: String) -> impl IntoView {
    let (_, foreground) = speaker_palette(&speaker);
    let active_speaker = speaker.clone();
    let next_speaker = speaker.clone();

    view! {
        <button
            class=move || {
                if view_state.speaker_filter.get() == active_speaker {
                    "inline-flex items-center gap-2 rounded-full border border-zinc-300 bg-white px-4 py-1.5 text-sm font-medium text-zinc-950 dark:border-white/10 dark:bg-[#34362f] dark:text-zinc-50"
                } else {
                    "inline-flex items-center gap-2 rounded-full border border-zinc-200 px-4 py-1.5 text-sm font-medium text-zinc-700 transition hover:bg-zinc-100 hover:text-zinc-950 dark:border-white/10 dark:text-zinc-300 dark:hover:bg-[#34362f] dark:hover:text-zinc-100"
                }
            }
            on:click=move |_| view_state.speaker_filter.set(next_speaker.clone())
            type="button"
        >
            <span class="h-2.5 w-2.5 rounded-full" style=format!("background:{};", foreground)></span>
            {speaker}
        </button>
    }
}

fn render_speaker_segment(segment: TranscriptSegment) -> impl IntoView {
    let (background, foreground) = speaker_palette(&segment.speaker);
    view! {
        <div class="grid gap-3 md:grid-cols-[36px_minmax(0,1fr)]">
            <div
                class="flex h-8 w-8 items-center justify-center rounded-full text-[11px] font-semibold"
                style=format!("background:{}; color:{};", background, foreground)
            >
                {segment.speaker.chars().next().unwrap_or('S').to_string()}
            </div>
            <div class="min-w-0">
                <div class="flex flex-wrap items-center gap-3 text-[11px] text-zinc-500 dark:text-zinc-500">
                    <span class="font-medium" style=format!("color:{};", foreground)>{segment.speaker.clone()}</span>
                    <span>{format!("{} -> {}", format_mm_ss(segment.start_s), format_mm_ss(segment.end_s))}</span>
                </div>
                <p class="mt-1 text-[1.03rem] leading-8 text-zinc-900 dark:text-zinc-100">{segment.text}</p>
            </div>
        </div>
    }
}

fn render_timeline_segment(segment: TranscriptSegment) -> impl IntoView {
    let (_, foreground) = speaker_palette(&segment.speaker);
    view! {
        <div class="grid gap-3 md:grid-cols-[7rem_minmax(0,1fr)]">
            <div class="pt-1 text-sm text-zinc-500 dark:text-zinc-500">
                {format!("{} -> {}", format_mm_ss(segment.start_s), format_mm_ss(segment.end_s))}
            </div>
            <div>
                <p class="text-[11px] font-medium uppercase tracking-[0.18em]" style=format!("color:{};", foreground)>{segment.speaker.clone()}</p>
                <p class="mt-2 text-base leading-8 text-zinc-900 dark:text-zinc-100">{segment.text}</p>
            </div>
        </div>
    }
}

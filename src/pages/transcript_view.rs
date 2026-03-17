use leptos::{ev::MouseEvent, prelude::*, task::spawn_local};

use crate::{
    components::sidebar::Sidebar,
    state::app_state::{TranscriptSegment, use_app_state},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum TranscriptTab {
    Speakers,
    Timeline,
    Raw,
}

#[derive(serde::Serialize)]
struct ExportArgs {
    id: String,
    format: String,
    path: String,
}

#[component]
pub fn TranscriptViewPage() -> impl IntoView {
    let state = use_app_state();
    let tab = RwSignal::new(TranscriptTab::Speakers);
    let export_open = RwSignal::new(false);
    let export_format = RwSignal::new(state.settings.get().default_export_format);
    let speaker_filter = RwSignal::new(None::<String>);
    let export_message = RwSignal::new(None::<String>);

    let segments = move || {
        let items = state.transcript_segments.get();
        match speaker_filter.get() {
            Some(active_speaker) => items
                .into_iter()
                .filter(|segment| segment.speaker == active_speaker)
                .collect::<Vec<_>>(),
            None => items,
        }
    };

    let speaker_names = move || {
        let mut speakers = state
            .transcript_segments
            .get()
            .into_iter()
            .map(|segment| segment.speaker)
            .collect::<Vec<_>>();
        speakers.sort();
        speakers.dedup();
        speakers
    };

    let export = move |_| {
        let format = export_format.get();
        export_message.set(Some("Preparing export...".into()));

        spawn_local(async move {
            let message = if tauri_sys::core::is_tauri() {
                match tauri_sys::core::invoke_result::<(), String>(
                    "export_transcript",
                    &ExportArgs {
                        id: "current".into(),
                        format: format.clone(),
                        path: String::new(),
                    },
                )
                .await
                {
                    Ok(_) => format!("Export command sent for {}.", format.to_uppercase()),
                    Err(error) => error,
                }
            } else {
                format!("Mock export ready for {}.", format.to_uppercase())
            };
            export_message.set(Some(message));
        });
    };

    view! {
        <div class="flex h-screen w-full">
            <Sidebar/>

            <div class="min-w-0 flex-1 overflow-auto px-8 py-6">
                <div class="mx-auto flex max-w-6xl flex-col gap-6">
                    <div class="flex flex-wrap items-start justify-between gap-4">
                        <div>
                            <p class="text-xs font-semibold uppercase tracking-[0.24em] text-slate-400 dark:text-slate-500">
                                "Transcript view"
                            </p>
                            <h1 class="mt-2 text-3xl font-semibold tracking-tight">"Review, filter, and export"</h1>
                            <p class="mt-2 text-sm text-slate-500 dark:text-slate-400">
                                {move || {
                                    state
                                        .selected_file
                                        .get()
                                        .map(|file| file.name)
                                        .unwrap_or_else(|| "Current transcript".into())
                                }}
                            </p>
                        </div>

                        <div class="flex items-center gap-3">
                            <button
                                class="rounded-full border border-slate-200 px-4 py-2 text-sm text-slate-500 transition hover:border-slate-300 hover:text-slate-900 dark:border-slate-800 dark:text-slate-400 dark:hover:border-slate-700 dark:hover:text-slate-100"
                                on:click=move |_| export_open.update(|open| *open = !*open)
                                type="button"
                            >
                                {move || if export_open.get() { "Close export" } else { "Open export" }}
                            </button>
                        </div>
                    </div>

                    <section class="rounded-[32px] border border-slate-200 bg-white p-6 dark:border-slate-800 dark:bg-slate-900">
                        <div class="flex flex-wrap items-center gap-3">
                            <TabButton
                                label="Speakers"
                                active=Signal::derive(move || tab.get() == TranscriptTab::Speakers)
                                on_click=Callback::new(move |_| tab.set(TranscriptTab::Speakers))
                            />
                            <TabButton
                                label="Timeline"
                                active=Signal::derive(move || tab.get() == TranscriptTab::Timeline)
                                on_click=Callback::new(move |_| tab.set(TranscriptTab::Timeline))
                            />
                            <TabButton
                                label="Raw"
                                active=Signal::derive(move || tab.get() == TranscriptTab::Raw)
                                on_click=Callback::new(move |_| tab.set(TranscriptTab::Raw))
                            />
                        </div>

                        <div class="mt-6 flex flex-wrap gap-2">
                            <button
                                class=move || filter_class(speaker_filter.get().is_none())
                                on:click=move |_| speaker_filter.set(None)
                                type="button"
                            >
                                "All speakers"
                            </button>
                            {move || {
                                speaker_names()
                                    .into_iter()
                                    .map(|speaker| {
                                        let speaker_for_button = speaker.clone();
                                        let speaker_for_class = speaker.clone();
                                        view! {
                                            <button
                                                class=move || filter_class(speaker_filter.get() == Some(speaker_for_class.clone()))
                                                on:click=move |_| speaker_filter.set(Some(speaker_for_button.clone()))
                                                type="button"
                                            >
                                                {speaker}
                                            </button>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </div>

                        <div class="mt-6">
                            {move || match tab.get() {
                                TranscriptTab::Speakers => view! {
                                    <div class="space-y-4">
                                        {segments().into_iter().map(render_speaker_segment).collect_view()}
                                    </div>
                                }.into_any(),
                                TranscriptTab::Timeline => view! {
                                    <div class="space-y-3">
                                        {segments().into_iter().map(render_timeline_segment).collect_view()}
                                    </div>
                                }.into_any(),
                                TranscriptTab::Raw => view! {
                                    <pre class="overflow-auto rounded-[24px] bg-slate-100 p-5 text-sm leading-7 text-slate-700 dark:bg-slate-950 dark:text-slate-300">
                                        {segments().into_iter().map(|segment| {
                                            format!(
                                                "[{} -> {}] {}: {}\n",
                                                format_duration(segment.start_s),
                                                format_duration(segment.end_s),
                                                segment.speaker,
                                                segment.text
                                            )
                                        }).collect::<String>()}
                                    </pre>
                                }.into_any(),
                            }}
                        </div>
                    </section>

                    {move || {
                        if export_open.get() {
                            view! {
                                <section class="rounded-[32px] border border-slate-200 bg-white p-6 dark:border-slate-800 dark:bg-slate-900">
                                    <div class="flex flex-wrap items-center justify-between gap-4">
                                        <div>
                                            <h2 class="text-lg font-semibold">"Export panel"</h2>
                                            <p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
                                                "TXT and SRT call the backend stub. DOCX stays visible as deferred Phase 2 work."
                                            </p>
                                        </div>

                                        <div class="flex flex-wrap gap-2">
                                            <button class=move || export_button_class(export_format.get() == "txt") on:click=move |_| export_format.set("txt".into()) type="button">"TXT"</button>
                                            <button class=move || export_button_class(export_format.get() == "srt") on:click=move |_| export_format.set("srt".into()) type="button">"SRT"</button>
                                            <button class="cursor-not-allowed rounded-full border border-dashed border-slate-300 px-4 py-2 text-sm text-slate-400 dark:border-slate-700 dark:text-slate-500" disabled=true type="button">"DOCX soon"</button>
                                        </div>
                                    </div>

                                    <button
                                        class="mt-5 inline-flex rounded-full bg-slate-900 px-4 py-2 text-sm font-semibold text-white transition hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-950 dark:hover:bg-slate-200"
                                        on:click=export
                                        type="button"
                                    >
                                        {move || format!("Export {}", export_format.get().to_uppercase())}
                                    </button>

                                    {move || {
                                        if let Some(message) = export_message.get() {
                                            view! { <p class="mt-4 text-sm text-slate-500 dark:text-slate-400">{message}</p> }.into_any()
                                        } else {
                                            view! { <></> }.into_any()
                                        }
                                    }}
                                </section>
                            }
                            .into_any()
                        } else {
                            view! { <></> }.into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
fn TabButton(
    label: &'static str,
    #[prop(into)] active: Signal<bool>,
    on_click: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <button
            class=move || {
                if active.get() {
                    "rounded-full bg-slate-900 px-4 py-2 text-sm font-semibold text-white dark:bg-slate-100 dark:text-slate-950".to_string()
                } else {
                    "rounded-full border border-slate-200 px-4 py-2 text-sm text-slate-500 transition hover:border-slate-300 hover:text-slate-900 dark:border-slate-800 dark:text-slate-400 dark:hover:border-slate-700 dark:hover:text-slate-100".to_string()
                }
            }
            on:click=move |event| on_click.run(event)
            type="button"
        >
            {label}
        </button>
    }
}

fn render_speaker_segment(segment: TranscriptSegment) -> impl IntoView {
    view! {
        <article class="rounded-[24px] bg-slate-100 p-4 dark:bg-slate-950">
            <div class="flex items-center justify-between gap-4">
                <span class="text-sm font-semibold text-slate-900 dark:text-slate-100">{segment.speaker.clone()}</span>
                <span class="text-xs text-slate-400 dark:text-slate-500">
                    {format!("{} -> {}", format_duration(segment.start_s), format_duration(segment.end_s))}
                </span>
            </div>
            <p class="mt-3 text-sm leading-7 text-slate-700 dark:text-slate-300">{segment.text}</p>
        </article>
    }
}

fn render_timeline_segment(segment: TranscriptSegment) -> impl IntoView {
    view! {
        <article class="flex gap-4 rounded-[24px] border border-slate-200 p-4 dark:border-slate-800">
            <div class="w-28 flex-shrink-0 text-sm font-semibold text-slate-900 dark:text-slate-100">
                {format_duration(segment.start_s)}
            </div>
            <div class="min-w-0 flex-1">
                <p class="text-sm font-semibold text-slate-900 dark:text-slate-100">{segment.speaker.clone()}</p>
                <p class="mt-1 text-sm leading-7 text-slate-700 dark:text-slate-300">{segment.text}</p>
            </div>
        </article>
    }
}

fn filter_class(active: bool) -> String {
    if active {
        "rounded-full bg-slate-900 px-3 py-1.5 text-sm font-semibold text-white dark:bg-slate-100 dark:text-slate-950".into()
    } else {
        "rounded-full border border-slate-200 px-3 py-1.5 text-sm text-slate-500 transition hover:border-slate-300 hover:text-slate-900 dark:border-slate-800 dark:text-slate-400 dark:hover:border-slate-700 dark:hover:text-slate-100".into()
    }
}

fn export_button_class(active: bool) -> String {
    if active {
        "rounded-full bg-slate-900 px-4 py-2 text-sm font-semibold text-white dark:bg-slate-100 dark:text-slate-950".into()
    } else {
        "rounded-full border border-slate-200 px-4 py-2 text-sm text-slate-500 transition hover:border-slate-300 hover:text-slate-900 dark:border-slate-800 dark:text-slate-400 dark:hover:border-slate-700 dark:hover:text-slate-100".into()
    }
}

fn format_duration(seconds: f32) -> String {
    let total = seconds.max(0.0).round() as u32;
    let minutes = total / 60;
    let secs = total % 60;
    format!("{minutes:02}:{secs:02}")
}

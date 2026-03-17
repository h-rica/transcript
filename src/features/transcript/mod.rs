mod service;

use leptos::{prelude::*, task::spawn_local};
use singlestage::{
    Badge, Card, CardContent, CardDescription, CardHeader, CardTitle, Empty, EmptyContent,
    EmptyDescription, EmptyHeader, EmptyTitle, Tabs, TabsContent, TabsList, TabsTrigger,
};
use wasm_bindgen_futures::JsFuture;

use crate::{
    components::{
        app_ui::{ActionButton, SpeakerPill},
        workspace::{WorkspaceHeader, WorkspaceRoute, WorkspaceShell},
    },
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
                <ActionButton on_click=copy_segments variant="outline">"Copy all"</ActionButton>
                <ActionButton
                    on_click=Callback::new(move |_| {
                        view_state.export_open.update(|open| *open = !*open);
                    })
                >
                    {move || if view_state.export_open.get() { "Close export" } else { "Export" }}
                </ActionButton>
            </WorkspaceHeader>

            {move || {
                if session.segments.get().is_empty() {
                    return view! {
                        <Empty>
                            <EmptyHeader>
                                <EmptyTitle>"No transcript available"</EmptyTitle>
                                <EmptyDescription>
                                    "Complete a transcription run before opening the transcript review screen."
                                </EmptyDescription>
                            </EmptyHeader>
                            <EmptyContent>
                                <p class="text-sm text-zinc-400">"The review workspace will open here once segments are available."</p>
                            </EmptyContent>
                        </Empty>
                    }
                    .into_any();
                }

                let speaker_segments = filtered_segments();
                let timeline_segments = speaker_segments.clone();
                let raw_segments = speaker_segments.clone();

                view! {
                    <>
                        <Card class="border-zinc-800 bg-[#191919] text-zinc-50">
                            <CardHeader class="space-y-4">
                                <div class="flex flex-wrap items-center gap-2">
                                    <button
                                        class=move || if view_state.speaker_filter.get().is_empty() {
                                            "rounded-full border border-zinc-500 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-50"
                                        } else {
                                            "rounded-full border border-zinc-700 px-3 py-1.5 text-sm text-zinc-300 transition hover:border-zinc-600 hover:text-zinc-50"
                                        }
                                        on:click=move |_| view_state.speaker_filter.set(String::new())
                                        type="button"
                                    >
                                        "All speakers"
                                    </button>
                                    {speaker_names().into_iter().map(|speaker| {
                                        let speaker_label = speaker.clone();
                                        let speaker_filter_value = speaker.clone();
                                        let speaker_click_value = speaker.clone();
                                        let tone = speaker_palette(&speaker);
                                        view! {
                                            <button
                                                class=move || {
                                                    let active = view_state.speaker_filter.get() == speaker_filter_value;
                                                    if active {
                                                        "rounded-full border bg-zinc-900 px-3 py-1.5 text-sm text-zinc-50".to_string()
                                                    } else {
                                                        "rounded-full border border-zinc-700 px-3 py-1.5 text-sm text-zinc-300 transition hover:border-zinc-600 hover:text-zinc-50".into()
                                                    }
                                                }
                                                on:click={
                                                    let speaker_click_value = speaker_click_value.clone();
                                                    move |_| view_state.speaker_filter.set(speaker_click_value.clone())
                                                }
                                                style=format!("border-color:{};", tone.1)
                                                type="button"
                                            >
                                                {speaker_label}
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>
                                <CardDescription>
                                    "Switch between speaker-grouped reading, chronological review, and raw copy-ready text."
                                </CardDescription>
                            </CardHeader>
                            <CardContent class="space-y-6">
                                <Tabs value=view_state.active_tab>
                                    <TabsList class="flex flex-wrap gap-2">
                                        <TabsTrigger value="speakers">"Speakers"</TabsTrigger>
                                        <TabsTrigger value="timeline">"Timeline"</TabsTrigger>
                                        <TabsTrigger value="raw">"Raw"</TabsTrigger>
                                    </TabsList>

                                    <TabsContent value="speakers" class="pt-6">
                                        <div class="space-y-4">
                                            {speaker_segments.into_iter().map(render_speaker_segment).collect_view()}
                                        </div>
                                    </TabsContent>

                                    <TabsContent value="timeline" class="pt-6">
                                        <div class="space-y-4">
                                            {timeline_segments.into_iter().map(render_timeline_segment).collect_view()}
                                        </div>
                                    </TabsContent>

                                    <TabsContent value="raw" class="pt-6">
                                        <pre class="overflow-auto rounded-2xl border border-zinc-800 bg-zinc-950 p-5 text-sm leading-7 text-zinc-200">
                                            {raw_segments
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
                                    </TabsContent>
                                </Tabs>
                            </CardContent>
                        </Card>

                        <Show when=move || view_state.export_open.get()>
                            <Card class="border-zinc-800 bg-[#191919] text-zinc-50">
                                <CardHeader>
                                    <CardTitle>"Export panel"</CardTitle>
                                    <CardDescription>
                                        "Choose the output format and keep the review screen open while export is prepared."
                                    </CardDescription>
                                </CardHeader>
                                <CardContent class="flex flex-wrap items-center gap-3">
                                    <button
                                        class=move || if view_state.export_format.get() == "txt" {
                                            "rounded-xl border border-zinc-500 bg-zinc-900 px-3 py-2 text-sm text-zinc-50"
                                        } else {
                                            "rounded-xl border border-zinc-700 px-3 py-2 text-sm text-zinc-300 transition hover:border-zinc-600 hover:text-zinc-50"
                                        }
                                        on:click=move |_| view_state.export_format.set("txt".into())
                                        type="button"
                                    >
                                        "TXT"
                                    </button>
                                    <button
                                        class=move || if view_state.export_format.get() == "srt" {
                                            "rounded-xl border border-zinc-500 bg-zinc-900 px-3 py-2 text-sm text-zinc-50"
                                        } else {
                                            "rounded-xl border border-zinc-700 px-3 py-2 text-sm text-zinc-300 transition hover:border-zinc-600 hover:text-zinc-50"
                                        }
                                        on:click=move |_| view_state.export_format.set("srt".into())
                                        type="button"
                                    >
                                        "SRT"
                                    </button>
                                    <Badge variant="outline">"DOCX deferred"</Badge>
                                    <ActionButton on_click=run_export>
                                        {move || format!("Export {}", view_state.export_format.get().to_uppercase())}
                                    </ActionButton>
                                    <Show when=move || view_state.export_message.get().is_some()>
                                        <p class="text-sm text-zinc-400">
                                            {move || view_state.export_message.get().unwrap_or_default()}
                                        </p>
                                    </Show>
                                </CardContent>
                            </Card>
                        </Show>
                    </>
                }
                .into_any()
            }}
        </WorkspaceShell>
    }
}

fn render_speaker_segment(segment: TranscriptSegment) -> impl IntoView {
    let (background, foreground) = speaker_palette(&segment.speaker);
    view! {
        <div class="space-y-2 rounded-2xl border border-zinc-800 bg-[#141414] p-4">
            <div class="flex flex-wrap items-center gap-3 text-xs text-zinc-500">
                <div
                    class="flex h-9 w-9 items-center justify-center rounded-full text-xs font-semibold"
                    style=format!("background:{}; color:{};", background, foreground)
                >
                    {segment.speaker.chars().next().unwrap_or('S').to_string()}
                </div>
                <SpeakerPill name=segment.speaker.clone()/>
                <span>{format!("{} -> {}", format_mm_ss(segment.start_s), format_mm_ss(segment.end_s))}</span>
            </div>
            <p class="text-base leading-8 text-zinc-100">{segment.text}</p>
        </div>
    }
}

fn render_timeline_segment(segment: TranscriptSegment) -> impl IntoView {
    view! {
        <div class="grid gap-2 rounded-2xl border border-zinc-800 bg-[#141414] p-4 md:grid-cols-[7rem_1fr]">
            <div class="text-sm font-semibold text-zinc-300">{format_mm_ss(segment.start_s)}</div>
            <div class="space-y-2">
                <SpeakerPill name=segment.speaker.clone()/>
                <p class="text-sm leading-7 text-zinc-100">{segment.text}</p>
            </div>
        </div>
    }
}

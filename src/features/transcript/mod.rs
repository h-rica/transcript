mod service;

use leptos::{prelude::*, task::spawn_local};
use singlestage::{
    Badge, Card, CardContent, CardDescription, CardHeader, CardTitle, Empty, EmptyContent,
    EmptyDescription, EmptyHeader, EmptyTitle, Field, Label, Select, SelectContent, SelectItem,
    Tabs, TabsContent, TabsList, TabsTrigger,
};

use crate::{
    components::{
        app_ui::{ActionBar, ActionButton, AppPageHeader, MetricCard, SpeakerPill, StatusBadge},
        sidebar::Sidebar,
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
        <div class="flex h-screen w-full bg-slate-50 dark:bg-slate-950">
            <Sidebar/>

            <main class="min-w-0 flex-1 overflow-auto px-6 py-6 lg:px-8">
                <div class="mx-auto flex max-w-6xl flex-col gap-6">
                    <AppPageHeader
                        eyebrow="Transcript"
                        title="Review, filter, and export the run"
                        description=Signal::derive(move || {
                            shell
                                .selected_file
                                .get()
                                .map(|file| file.name)
                                .unwrap_or_else(|| "Current transcript".into())
                        })
                    >
                        <ActionButton
                            on_click=Callback::new(move |_| {
                                view_state.export_open.update(|open| *open = !*open);
                            })
                            variant="outline"
                        >
                            {move || if view_state.export_open.get() { "Close export" } else { "Open export" }}
                        </ActionButton>
                    </AppPageHeader>

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
                                        <StatusBadge value=Signal::derive(|| "Waiting for content".to_string()) variant="outline"/>
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
                                <Card>
                                    <CardHeader>
                                        <ActionBar>
                                            <StatusBadge value=Signal::derive(move || shell.selected_language.get().to_uppercase()) variant="secondary"/>
                                            <Badge variant="outline">{move || format!("{} segments", session.segments.get().len())}</Badge>
                                        </ActionBar>
                                        <CardTitle>"Transcript reader"</CardTitle>
                                        <CardDescription>
                                            "Tabs, filtering, and export state now live in the transcript feature store instead of page-local enums and button classes."
                                        </CardDescription>
                                    </CardHeader>
                                    <CardContent class="space-y-6">
                                        <div class="grid gap-4 lg:grid-cols-[1fr_auto]">
                                            <Field>
                                                <Label>"Speaker filter"</Label>
                                                <Select value=view_state.speaker_filter>
                                                    <SelectContent>
                                                        <SelectItem value="">"All speakers"</SelectItem>
                                                        {speaker_names().into_iter().map(|speaker| {
                                                            view! { <SelectItem value=speaker.clone()>{speaker}</SelectItem> }
                                                        }).collect_view()}
                                                    </SelectContent>
                                                </Select>
                                            </Field>

                                            <div class="grid gap-4 sm:grid-cols-3 lg:w-[24rem]">
                                                <MetricCard label="Visible" value=Signal::derive(move || filtered_segments().len().to_string())/>
                                                <MetricCard label="Words" value=Signal::derive(move || {
                                                    filtered_segments()
                                                        .into_iter()
                                                        .map(|segment| segment.text.split_whitespace().count())
                                                        .sum::<usize>()
                                                        .to_string()
                                                })/>
                                                <MetricCard label="Speakers" value=Signal::derive(move || speaker_names().len().to_string())/>
                                            </div>
                                        </div>

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
                                                <pre class="overflow-auto rounded-xl bg-slate-100 p-5 text-sm leading-7 text-slate-700 dark:bg-slate-900 dark:text-slate-200">
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
                                    <Card>
                                        <CardHeader>
                                            <CardTitle>"Export panel"</CardTitle>
                                            <CardDescription>
                                                "TXT and SRT still call the backend stub, but the export workflow now lives in a dedicated transcript service."
                                            </CardDescription>
                                        </CardHeader>
                                        <CardContent class="space-y-4">
                                            <Field>
                                                <Label>"Format"</Label>
                                                <Select value=view_state.export_format>
                                                    <SelectContent>
                                                        <SelectItem value="txt">"TXT"</SelectItem>
                                                        <SelectItem value="srt">"SRT"</SelectItem>
                                                    </SelectContent>
                                                </Select>
                                            </Field>

                                            <div class="flex flex-wrap items-center gap-2">
                                                <Badge variant="outline">"DOCX deferred"</Badge>
                                                <StatusBadge value=Signal::derive(move || view_state.export_format.get().to_uppercase()) variant="secondary"/>
                                            </div>

                                            <ActionButton on_click=run_export>
                                                {move || format!("Export {}", view_state.export_format.get().to_uppercase())}
                                            </ActionButton>

                                            <Show when=move || view_state.export_message.get().is_some()>
                                                <p class="text-sm text-slate-600 dark:text-slate-300">
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
                </div>
            </main>
        </div>
    }
}

fn render_speaker_segment(segment: TranscriptSegment) -> impl IntoView {
    let (background, foreground) = speaker_palette(&segment.speaker);
    view! {
        <Card>
            <CardContent class="space-y-3 p-4">
                <div class="flex flex-wrap items-center justify-between gap-3">
                    <div class="flex items-center gap-3">
                        <div
                            class="flex h-9 w-9 items-center justify-center rounded-full text-xs font-semibold"
                            style=format!("background:{}; color:{};", background, foreground)
                        >
                            {segment
                                .speaker
                                .chars()
                                .next()
                                .unwrap_or('S')
                                .to_string()}
                        </div>
                        <SpeakerPill name=segment.speaker.clone()/>
                    </div>
                    <span class="text-xs text-slate-500 dark:text-slate-400">
                        {format!("{} -> {}", format_mm_ss(segment.start_s), format_mm_ss(segment.end_s))}
                    </span>
                </div>
                <p class="text-sm leading-7 text-slate-700 dark:text-slate-200">{segment.text}</p>
            </CardContent>
        </Card>
    }
}

fn render_timeline_segment(segment: TranscriptSegment) -> impl IntoView {
    view! {
        <Card>
            <CardContent class="flex gap-4 p-4">
                <div class="w-24 flex-shrink-0 text-sm font-semibold text-slate-950 dark:text-slate-50">
                    {format_mm_ss(segment.start_s)}
                </div>
                <div class="min-w-0 flex-1 space-y-2">
                    <SpeakerPill name=segment.speaker.clone()/>
                    <p class="text-sm leading-7 text-slate-700 dark:text-slate-200">{segment.text}</p>
                </div>
            </CardContent>
        </Card>
    }
}

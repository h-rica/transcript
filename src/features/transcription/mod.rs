mod service;

use leptos::{prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;
use singlestage::{
    Alert, AlertDescription, AlertTitle, Badge, Card, CardContent, CardDescription, CardHeader,
    CardTitle, Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyTitle, Progress,
};

use crate::{
    components::{
        app_ui::{ActionBar, ActionButton, AppPageHeader, LinkButton, MetricCard, StatusBadge},
        live_segment_list::LiveSegmentList,
        sidebar::Sidebar,
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
    let progress_value = Signal::derive(move || {
        (session.progress.get().percent.clamp(0.0, 1.0) * 100.0).round() as usize
    });
    let speed_signal = {
        let speed_state = session.clone();
        Signal::derive(move || speed_label(&speed_state))
    };

    view! {
        <div class="flex h-screen w-full bg-slate-50 dark:bg-slate-950">
            <Sidebar/>

            <main class="min-w-0 flex-1 overflow-auto px-6 py-6 lg:px-8">
                <div class="mx-auto flex max-w-6xl flex-col gap-6">
                    <AppPageHeader
                        eyebrow="Transcription"
                        title="Watch progress and live segments"
                        description="The event listeners are now registered once per session in the transcription feature instead of being recreated in the route body."
                    >
                        <StatusBadge value=Signal::derive(move || shell.active_model.get()) variant="outline"/>
                    </AppPageHeader>

                    {move || {
                        if shell.selected_file.get().is_none() {
                            return view! {
                                <Empty>
                                    <EmptyHeader>
                                        <EmptyTitle>"No active run"</EmptyTitle>
                                        <EmptyDescription>
                                            "A file and model must be selected in the preview flow before the transcription screen can stream events."
                                        </EmptyDescription>
                                    </EmptyHeader>
                                    <EmptyContent>
                                        <LinkButton href="/preview">"Open preview"</LinkButton>
                                    </EmptyContent>
                                </Empty>
                            }
                            .into_any();
                        }

                        let summary = session.summary.get();
                        let failed = matches!(session.status.get(), TranscriptionStatus::Failed(_));

                        view! {
                            <>
                                <Card>
                                    <CardHeader>
                                        <ActionBar>
                                            <StatusBadge value=Signal::derive(move || status_label(&session.status.get())) variant="secondary"/>
                                            <Badge variant="outline">
                                                {move || {
                                                    shell
                                                        .selected_file
                                                        .get()
                                                        .map(|file| file.name)
                                                        .unwrap_or_else(|| "Waiting for file selection".into())
                                                }}
                                            </Badge>
                                        </ActionBar>
                                        <CardTitle>"Current run"</CardTitle>
                                        <CardDescription>
                                            "Progress is bound to the shared transcription session state, so the screen remains a composition shell over feature data."
                                        </CardDescription>
                                    </CardHeader>
                                    <CardContent class="space-y-6">
                                        <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
                                            <MetricCard label="Elapsed" value=Signal::derive(move || format_elapsed(session.progress.get().elapsed_s))/>
                                            <MetricCard label="Speed" value=speed_signal/>
                                            <MetricCard label="Segments" value=Signal::derive(move || session.segments.get().len().to_string())/>
                                            <MetricCard label="Language" value=Signal::derive(move || shell.selected_language.get().to_uppercase())/>
                                        </div>

                                        <div class="space-y-2">
                                            <div class="flex items-center justify-between text-sm text-slate-600 dark:text-slate-300">
                                                <span>"Progress"</span>
                                                <span>{move || format!("{:.0}%", session.progress.get().percent * 100.0)}</span>
                                            </div>
                                            <Progress class="w-full" max=100usize value=progress_value/>
                                        </div>

                                        <Show when=move || failed>
                                            <Alert variant="destructive">
                                                <AlertTitle>"Transcription interrupted"</AlertTitle>
                                                <AlertDescription>
                                                    {move || session.error.get().unwrap_or_else(|| status_label(&session.status.get()))}
                                                </AlertDescription>
                                            </Alert>
                                        </Show>
                                    </CardContent>
                                </Card>

                                <section class="grid gap-6 xl:grid-cols-[1.2fr_0.8fr]">
                                    <Card>
                                        <CardHeader>
                                            <Badge variant="secondary">"Live segments"</Badge>
                                            <CardTitle>"Streaming transcript feed"</CardTitle>
                                            <CardDescription>
                                                "Segments append here as `transcription_segment` events arrive from the backend."
                                            </CardDescription>
                                        </CardHeader>
                                        <CardContent>
                                            <LiveSegmentList
                                                segments=session.segments
                                                pending=Signal::derive(move || {
                                                    matches!(
                                                        session.status.get(),
                                                        TranscriptionStatus::LoadingModel | TranscriptionStatus::Running
                                                    )
                                                })
                                            />
                                        </CardContent>
                                    </Card>

                                    <div class="flex flex-col gap-6">
                                        <Card>
                                            <CardHeader>
                                                <CardTitle>"Run summary"</CardTitle>
                                                <CardDescription>
                                                    "Completion metrics remain visible even after the route changes to the transcript review flow."
                                                </CardDescription>
                                            </CardHeader>
                                            <CardContent class="grid gap-4 sm:grid-cols-2">
                                                <MetricCard label="Percent" value=Signal::derive(move || format!("{:.0}%", session.progress.get().percent * 100.0))/>
                                                <MetricCard label="Words" value=Signal::derive(move || session.summary.get().map(|item| item.words.to_string()).unwrap_or_else(|| "--".into()))/>
                                                <MetricCard label="Speakers" value=Signal::derive(move || session.summary.get().map(|item| item.speakers.to_string()).unwrap_or_else(|| "--".into()))/>
                                                <MetricCard label="Status" value=Signal::derive(move || status_label(&session.status.get()))/>
                                            </CardContent>
                                        </Card>

                                        {if let Some(summary) = summary {
                                            view! {
                                                <Alert>
                                                    <AlertTitle>"Transcription complete"</AlertTitle>
                                                    <AlertDescription>
                                                        {format!(
                                                            "{} segments, {} speakers, {} words in {}.",
                                                            summary.segments,
                                                            summary.speakers,
                                                            summary.words,
                                                            format_elapsed(summary.elapsed_s)
                                                        )}
                                                    </AlertDescription>
                                                    <div class="mt-4">
                                                        <ActionButton on_click=open_transcript>"Open transcript"</ActionButton>
                                                    </div>
                                                </Alert>
                                            }
                                            .into_any()
                                        } else {
                                            view! {
                                                <ActionButton on_click=cancel variant="outline">"Cancel transcription"</ActionButton>
                                            }
                                            .into_any()
                                        }}
                                    </div>
                                </section>
                            </>
                        }
                        .into_any()
                    }}
                </div>
            </main>
        </div>
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
        return "Speed --".to_string();
    }

    let processed_seconds = duration * progress.percent;
    let rtfx = processed_seconds / progress.elapsed_s as f32;
    let remaining = ((duration - processed_seconds).max(0.0) / rtfx.max(0.1)).round() as u32;
    format!("{:.2}x realtime | ETA {}", rtfx, format_elapsed(remaining))
}

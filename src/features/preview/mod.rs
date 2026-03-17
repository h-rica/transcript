mod service;

use leptos::{prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;
use singlestage::{
    Alert, AlertDescription, AlertTitle, Badge, Card, CardContent, CardDescription, CardFooter,
    CardHeader, CardTitle, Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyTitle, Field,
    Label, Select, SelectContent, SelectItem, Separator,
};

use crate::{
    components::{
        app_ui::{ActionBar, ActionButton, AppPageHeader, LinkButton, MetricCard, StatusBadge},
        sidebar::Sidebar,
    },
    features::shared::{
        AVAILABLE_MODELS, format_bytes, format_hms, hardware_warning, selected_model,
    },
    state::app_state::{
        TranscriptionRequest, TranscriptionStatus, reset_transcript_view,
        reset_transcription_session, use_app_shell_state, use_transcript_view_state,
        use_transcription_session_state,
    },
};

use service::{load_audio_preview, seed_browser_transcript, start_transcription};

#[component]
pub fn PreviewScreen() -> impl IntoView {
    let shell = use_app_shell_state();
    let session = use_transcription_session_state();
    let transcript_view = use_transcript_view_state();
    let navigate = use_navigate();
    let is_loading = RwSignal::new(false);
    let is_starting = RwSignal::new(false);

    let preview_session = session.clone();
    Effect::new(move |_| {
        let Some(file) = shell.selected_file.get() else {
            preview_session.audio_info.set(None);
            preview_session.error.set(None);
            return;
        };

        is_loading.set(true);
        preview_session.audio_info.set(None);
        preview_session.error.set(None);

        let session = preview_session.clone();
        spawn_local(async move {
            match load_audio_preview(file.path).await {
                Ok(audio) => session.audio_info.set(Some(audio)),
                Err(error) => session.error.set(Some(error.to_string())),
            }
            is_loading.set(false);
        });
    });

    let current_model = move || selected_model(&shell.selected_model.get());

    let start_session = session.clone();
    let start_flow = Callback::new(move |_| {
        let Some(file) = shell.selected_file.get() else {
            return;
        };

        let request = TranscriptionRequest {
            path: file.path,
            model_id: shell.selected_model.get(),
            language: shell.selected_language.get(),
        };
        let export_format = shell.settings.get().default_export_format.clone();

        reset_transcription_session(&start_session);
        reset_transcript_view(&transcript_view, export_format);
        shell.active_model.set(request.model_id.clone());
        start_session.status.set(TranscriptionStatus::LoadingModel);
        is_starting.set(true);
        navigate("/transcription", Default::default());

        let session = start_session.clone();
        spawn_local(async move {
            let result = start_transcription(request).await;
            match result {
                Ok(_) if tauri_sys::core::is_tauri() => {}
                Ok(_) => seed_browser_transcript(&session),
                Err(error) => {
                    session
                        .status
                        .set(TranscriptionStatus::Failed(error.to_string()));
                    session.error.set(Some(error.to_string()));
                }
            }
            is_starting.set(false);
        });
    });

    view! {
        <div class="flex h-screen w-full bg-slate-50 dark:bg-slate-950">
            <Sidebar/>

            <main class="min-w-0 flex-1 overflow-auto px-6 py-6 lg:px-8">
                <div class="mx-auto flex max-w-6xl flex-col gap-6">
                    <AppPageHeader
                        eyebrow="Preview"
                        title="Confirm the file, language, and model"
                        description="This route now delegates metadata loading and transcription start to feature services instead of handling Tauri wiring inline."
                    >
                        <LinkButton href="/">"Back"</LinkButton>
                    </AppPageHeader>

                    {move || {
                        let Some(file) = shell.selected_file.get() else {
                            return view! {
                                <Empty>
                                    <EmptyHeader>
                                        <EmptyTitle>"No file selected"</EmptyTitle>
                                        <EmptyDescription>
                                            "Return to the intake screen and drop an audio file before opening the preview flow."
                                        </EmptyDescription>
                                    </EmptyHeader>
                                    <EmptyContent>
                                        <LinkButton href="/">"Return home"</LinkButton>
                                    </EmptyContent>
                                </Empty>
                            }
                            .into_any();
                        };

                        let model = current_model();
                        let warning_text = hardware_warning(shell.hardware_info.get(), model.tier).unwrap_or_default();
                        let runtime_estimate = session
                            .audio_info
                            .get()
                            .map(|info| format_hms(info.duration_s / model.rtfx.max(0.1)))
                            .unwrap_or_else(|| "--".into());
                        let ready_variant = Signal::derive(move || {
                            if current_model().ready {
                                "secondary".to_string()
                            } else {
                                "outline".to_string()
                            }
                        });

                        view! {
                            <div class="grid gap-6 xl:grid-cols-[1.15fr_0.85fr]">
                                <Card>
                                    <CardHeader>
                                        <ActionBar>
                                            <StatusBadge value=Signal::derive(move || shell.selected_language.get().to_uppercase()) variant="secondary"/>
                                            <StatusBadge value=Signal::derive(move || shell.active_model.get()) variant="outline"/>
                                        </ActionBar>
                                        <CardTitle>{file.name.clone()}</CardTitle>
                                        <CardDescription>{file.path.clone()}</CardDescription>
                                    </CardHeader>
                                    <CardContent class="space-y-6">
                                        <div class="grid gap-4 sm:grid-cols-2">
                                            <MetricCard
                                                label="Duration"
                                                value=Signal::derive(move || {
                                                    session
                                                        .audio_info
                                                        .get()
                                                        .map(|info| format_hms(info.duration_s))
                                                        .unwrap_or_else(|| if is_loading.get() { "Loading...".into() } else { "--".into() })
                                                })
                                            />
                                            <MetricCard
                                                label="Size"
                                                value=Signal::derive(move || {
                                                    session
                                                        .audio_info
                                                        .get()
                                                        .map(|info| format_bytes(info.size_bytes))
                                                        .unwrap_or_else(|| "--".into())
                                                })
                                            />
                                            <MetricCard
                                                label="Format"
                                                value=Signal::derive(move || {
                                                    session
                                                        .audio_info
                                                        .get()
                                                        .map(|info| info.format.to_uppercase())
                                                        .unwrap_or_else(|| "--".into())
                                                })
                                            />
                                            <MetricCard
                                                label="Bitrate"
                                                value=Signal::derive(move || {
                                                    session
                                                        .audio_info
                                                        .get()
                                                        .and_then(|info| info.bitrate_kbps)
                                                        .map(|value| format!("{value} kbps"))
                                                        .unwrap_or_else(|| "--".into())
                                                })
                                            />
                                        </div>

                                        <Separator/>

                                        <div class="grid gap-4 md:grid-cols-2">
                                            <Field>
                                                <Label>"Language"</Label>
                                                <Select value=shell.selected_language>
                                                    <SelectContent>
                                                        <SelectItem value="fr">"French"</SelectItem>
                                                        <SelectItem value="en">"English"</SelectItem>
                                                        <SelectItem value="auto">"Auto detect"</SelectItem>
                                                    </SelectContent>
                                                </Select>
                                            </Field>

                                            <Field>
                                                <Label>"Model"</Label>
                                                <Select value=shell.selected_model>
                                                    <SelectContent>
                                                        {AVAILABLE_MODELS.into_iter().map(|item| {
                                                            view! {
                                                                <SelectItem value=item.id>
                                                                    {format!("{} ({})", item.name, item.tier)}
                                                                </SelectItem>
                                                            }
                                                        }).collect_view()}
                                                    </SelectContent>
                                                </Select>
                                            </Field>
                                        </div>

                                        <div class="grid gap-4 md:grid-cols-3">
                                            <MetricCard label="Estimated runtime" value=Signal::derive(move || runtime_estimate.clone())/>
                                            <MetricCard label="Realtime factor" value=Signal::derive(move || format!("{:.2}x", current_model().rtfx))/>
                                            <MetricCard label="Bundle size" value=Signal::derive(move || format!("{} MB", current_model().size_mb))/>
                                        </div>
                                    </CardContent>
                                </Card>

                                <Card>
                                    <CardHeader>
                                        <Badge variant="secondary">"Selected profile"</Badge>
                                        <CardTitle>{move || current_model().name}</CardTitle>
                                        <CardDescription>{move || current_model().description}</CardDescription>
                                    </CardHeader>
                                    <CardContent class="space-y-4">
                                        <div class="flex flex-wrap gap-2">
                                            <Badge variant="outline">{move || current_model().tier}</Badge>
                                            <Badge variant=ready_variant>
                                                {move || if current_model().ready { "Ready" } else { "Not downloaded" }}
                                            </Badge>
                                            <Show when=move || current_model().diarization>
                                                <Badge variant="outline">"Speaker labels"</Badge>
                                            </Show>
                                        </div>

                                        <div class="grid gap-3">
                                            {AVAILABLE_MODELS.into_iter().map(|item| {
                                                let class_name = Signal::derive(move || {
                                                    if item.id == current_model().id {
                                                        "border-slate-950 dark:border-slate-50".to_string()
                                                    } else {
                                                        String::new()
                                                    }
                                                });
                                                view! {
                                                    <Card class=class_name>
                                                        <CardContent class="flex items-start justify-between gap-4 p-4">
                                                            <div>
                                                                <p class="text-sm font-semibold text-slate-950 dark:text-slate-50">{item.name}</p>
                                                                <p class="mt-1 text-sm text-slate-600 dark:text-slate-300">{item.description}</p>
                                                            </div>
                                                            <Badge variant="outline">{format!("{} MB", item.size_mb)}</Badge>
                                                        </CardContent>
                                                    </Card>
                                                }
                                            }).collect_view()}
                                        </div>

                                        {if warning_text.is_empty() {
                                            ().into_any()
                                        } else {
                                            view! {
                                                <Alert>
                                                    <AlertTitle>"Hardware warning"</AlertTitle>
                                                    <AlertDescription>{warning_text.clone()}</AlertDescription>
                                                </Alert>
                                            }
                                            .into_any()
                                        }}

                                        <Show when=move || session.error.get().is_some()>
                                            <Alert variant="destructive">
                                                <AlertTitle>"Audio preview failed"</AlertTitle>
                                                <AlertDescription>
                                                    {move || session.error.get().unwrap_or_default()}
                                                </AlertDescription>
                                            </Alert>
                                        </Show>
                                    </CardContent>
                                    <CardFooter class="justify-end">
                                        <ActionButton
                                            disabled=Signal::derive(move || !current_model().ready || is_starting.get())
                                            on_click=start_flow
                                        >
                                            {move || if is_starting.get() { "Starting..." } else { "Start transcription" }}
                                        </ActionButton>
                                    </CardFooter>
                                </Card>
                            </div>
                        }
                        .into_any()
                    }}
                </div>
            </main>
        </div>
    }
}

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
        app_ui::{ActionBar, ActionButton, LinkButton, MetricCard, StatusBadge},
        workspace::{WorkspaceHeader, WorkspaceRoute, WorkspaceShell},
    },
    features::{
        shared::{format_bytes, format_hms, hardware_warning},
        workspace_data::{fallback_models, model_is_ready, selected_model},
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

    let models = move || {
        let loaded = shell.available_models.get();
        if loaded.is_empty() {
            fallback_models()
        } else {
            loaded
        }
    };
    let current_model = move || selected_model(&models(), &shell.selected_model.get());

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
        <WorkspaceShell route=WorkspaceRoute::Preview>
            <WorkspaceHeader
                title="File Preview"
                subtitle="Validate the file, confirm the language, and choose the local model before starting the run."
            >
                <LinkButton href="/">"Back"</LinkButton>
            </WorkspaceHeader>

            {move || {
                let Some(file) = shell.selected_file.get() else {
                    return view! {
                        <Empty>
                            <EmptyHeader>
                                <EmptyTitle>"No file selected"</EmptyTitle>
                                <EmptyDescription>
                                    "Return to Home and drop an audio file before opening the preview flow."
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
                let warning_text = hardware_warning(shell.hardware_info.get(), &model.tier).unwrap_or_default();
                let runtime_estimate = session
                    .audio_info
                    .get()
                    .map(|info| format_hms(info.duration_s / model.rtfx.max(0.1)))
                    .unwrap_or_else(|| "--".into());
                let ready_variant = Signal::derive(move || {
                    if model_is_ready(&current_model()) {
                        "secondary".to_string()
                    } else {
                        "outline".to_string()
                    }
                });

                view! {
                    <div class="grid gap-6 xl:grid-cols-[1.15fr_0.85fr]">
                        <Card class="border-zinc-800 bg-[#191919] text-zinc-50">
                            <CardHeader>
                                <ActionBar>
                                    <StatusBadge value=Signal::derive(move || shell.selected_language.get().to_uppercase()) variant="secondary"/>
                                    <StatusBadge value=Signal::derive(move || shell.selected_model.get()) variant="outline"/>
                                </ActionBar>
                                <CardTitle>{file.name.clone()}</CardTitle>
                                <CardDescription>{file.path.clone()}</CardDescription>
                            </CardHeader>
                            <CardContent class="space-y-6">
                                <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
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
                                                {models().into_iter().map(|item| {
                                                    let status = match item.status.as_str() {
                                                        "bundled" => "Bundled",
                                                        "downloaded" => "Downloaded",
                                                        _ => "Not installed",
                                                    };
                                                    view! {
                                                        <SelectItem value=item.id.clone()>
                                                            {format!("{} ({status})", item.name)}
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

                        <Card class="border-zinc-800 bg-[#191919] text-zinc-50">
                            <CardHeader>
                                <Badge variant="secondary">"Selected profile"</Badge>
                                <CardTitle>{move || current_model().name}</CardTitle>
                                <CardDescription>{move || current_model().description}</CardDescription>
                            </CardHeader>
                            <CardContent class="space-y-4">
                                <div class="flex flex-wrap gap-2">
                                    <Badge variant="outline">{move || current_model().tier}</Badge>
                                    <Badge variant=ready_variant>
                                        {move || if model_is_ready(&current_model()) { "Ready" } else { "Install required" }}
                                    </Badge>
                                    <Show when=move || current_model().diarization>
                                        <Badge variant="outline">"Speaker labels"</Badge>
                                    </Show>
                                </div>

                                <div class="grid gap-3">
                                    {models().into_iter().map(|item| {
                                        let class_name = Signal::derive(move || {
                                            if item.id == current_model().id {
                                                "border-zinc-500 bg-zinc-900/80".to_string()
                                            } else {
                                                "border-zinc-800 bg-[#141414]".to_string()
                                            }
                                        });
                                        view! {
                                            <Card class=class_name>
                                                <CardContent class="flex items-start justify-between gap-4 p-4">
                                                    <div class="space-y-1">
                                                        <p class="text-sm font-semibold text-zinc-50">{item.name}</p>
                                                        <p class="text-sm text-zinc-400">{item.description}</p>
                                                        <p class="text-xs uppercase tracking-[0.18em] text-zinc-500">
                                                            {format!("{} | {}", item.source, item.status)}
                                                        </p>
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
                                    disabled=Signal::derive(move || !model_is_ready(&current_model()) || is_starting.get())
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
        </WorkspaceShell>
    }
}

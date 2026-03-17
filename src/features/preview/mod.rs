mod service;

use leptos::{prelude::*, task::spawn_local};
use leptos_router::{components::A, hooks::use_navigate};

use crate::{
    components::workspace::{WorkspaceHeader, WorkspaceRoute, WorkspaceShell},
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
                <A
                    attr:class="inline-flex h-9 items-center rounded-lg border border-zinc-200 px-3 text-sm font-medium text-zinc-700 transition hover:bg-zinc-100 hover:text-zinc-950 dark:border-zinc-800 dark:text-zinc-300 dark:hover:bg-[#17181b] dark:hover:text-zinc-100"
                    href="/"
                >
                    "Back"
                </A>
            </WorkspaceHeader>

            {move || {
                let Some(file) = shell.selected_file.get() else {
                    return view! {
                        <div class="rounded-[1.5rem] border border-dashed border-zinc-300 bg-zinc-100/80 px-6 py-12 text-center dark:border-zinc-800 dark:bg-[#121316]">
                            <p class="text-base font-medium text-zinc-950 dark:text-zinc-100">"No file selected"</p>
                            <p class="mt-2 text-sm text-zinc-600 dark:text-zinc-500">
                                "Return to Home and drop an audio file before opening the preview flow."
                            </p>
                            <A
                                attr:class="mt-5 inline-flex h-9 items-center rounded-lg border border-zinc-200 px-3 text-sm font-medium text-zinc-700 transition hover:bg-zinc-100 hover:text-zinc-950 dark:border-zinc-800 dark:text-zinc-300 dark:hover:bg-[#17181b] dark:hover:text-zinc-100"
                                href="/"
                            >
                                "Return home"
                            </A>
                        </div>
                    }
                    .into_any();
                };

                let model = current_model();
                let warning_text = hardware_warning(shell.hardware_info.get(), &model.tier).unwrap_or_default();
                let has_warning = !warning_text.is_empty();
                let audio_info = session.audio_info.get();
                let runtime_estimate = audio_info
                    .as_ref()
                    .map(|info| format_hms(info.duration_s / model.rtfx.max(0.1)))
                    .unwrap_or_else(|| "--".into());
                let model_ready = model_is_ready(&model);

                view! {
                    <div class="grid gap-6 xl:grid-cols-[minmax(0,1fr)_340px]">
                        <div class="space-y-5">
                            <section class="rounded-[1.5rem] border border-zinc-200 bg-white px-5 py-5 dark:border-zinc-900 dark:bg-[#141519]">
                                <div class="flex items-start gap-4">
                                    <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg border border-zinc-200 bg-zinc-100 text-xs font-semibold text-zinc-600 dark:border-zinc-800 dark:bg-[#101114] dark:text-zinc-400">
                                        "FI"
                                    </div>
                                    <div class="min-w-0">
                                        <p class="truncate text-sm font-medium text-zinc-950 dark:text-zinc-100">{file.name.clone()}</p>
                                        <p class="mt-1 truncate text-xs text-zinc-500 dark:text-zinc-500">{file.path.clone()}</p>
                                    </div>
                                </div>

                                <div class="mt-5 grid gap-3 sm:grid-cols-2 xl:grid-cols-4">
                                    <MetricTile
                                        label="Duration"
                                        value=audio_info
                                            .as_ref()
                                            .map(|info| format_hms(info.duration_s))
                                            .unwrap_or_else(|| if is_loading.get() { "Loading...".into() } else { "--".into() })
                                    />
                                    <MetricTile
                                        label="Size"
                                        value=audio_info
                                            .as_ref()
                                            .map(|info| format_bytes(info.size_bytes))
                                            .unwrap_or_else(|| "--".into())
                                    />
                                    <MetricTile
                                        label="Format"
                                        value=audio_info
                                            .as_ref()
                                            .map(|info| info.format.to_uppercase())
                                            .unwrap_or_else(|| "--".into())
                                    />
                                    <MetricTile
                                        label="Bitrate"
                                        value=audio_info
                                            .as_ref()
                                            .and_then(|info| info.bitrate_kbps)
                                            .map(|value| format!("{value} kbps"))
                                            .unwrap_or_else(|| "--".into())
                                    />
                                </div>
                            </section>

                            <section class="rounded-[1.5rem] border border-zinc-200 bg-white px-5 py-5 dark:border-zinc-900 dark:bg-[#141519]">
                                <p class="text-xs font-medium uppercase tracking-[0.18em] text-zinc-500">"Language"</p>
                                <div class="mt-3 flex flex-wrap gap-2">
                                    {[("fr", "French"), ("en", "English"), ("auto", "Auto")]
                                        .into_iter()
                                        .map(|(value, label)| {
                                            let shell_state = shell.clone();
                                            view! {
                                                <button
                                                    class=move || {
                                                        if shell_state.selected_language.get() == value {
                                                            "rounded-lg border border-zinc-300 bg-zinc-950 px-3 py-2 text-sm font-medium text-white dark:border-zinc-700 dark:bg-zinc-100 dark:text-zinc-950"
                                                        } else {
                                                            "rounded-lg border border-zinc-200 bg-zinc-100 px-3 py-2 text-sm font-medium text-zinc-700 transition hover:bg-zinc-200 dark:border-zinc-800 dark:bg-[#101114] dark:text-zinc-300 dark:hover:bg-[#17181b]"
                                                        }
                                                    }
                                                    on:click=move |_| shell.selected_language.set(value.into())
                                                    type="button"
                                                >
                                                    {label}
                                                </button>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            </section>

                            <section class="rounded-[1.5rem] border border-zinc-200 bg-white px-5 py-5 dark:border-zinc-900 dark:bg-[#141519]">
                                <div class="grid gap-4 md:grid-cols-3">
                                    <MetricTile label="Estimated time" value=runtime_estimate.clone()/>
                                    <MetricTile label="Realtime factor" value=format!("{:.2}x", model.rtfx)/>
                                    <MetricTile label="Bundle size" value=format!("{} MB", model.size_mb)/>
                                </div>
                            </section>
                        </div>

                        <aside class="space-y-4">
                            <section class="rounded-[1.5rem] border border-zinc-200 bg-white px-5 py-5 dark:border-zinc-900 dark:bg-[#141519]">
                                <div class="flex flex-wrap items-center gap-2">
                                    <StatusTag label=model.tier.clone()/>
                                    <StatusTag label=if model_ready { "Ready".into() } else { "Install required".into() }/>
                                    <Show when=move || model.diarization>
                                        <StatusTag label="Diarization".into()/>
                                    </Show>
                                </div>
                                <h2 class="mt-4 text-lg font-semibold text-zinc-950 dark:text-zinc-100">{model.name.clone()}</h2>
                                <p class="mt-2 text-sm leading-6 text-zinc-600 dark:text-zinc-400">{model.description.clone()}</p>

                                <div class="mt-5 space-y-2">
                                    {models().into_iter().map(|item| {
                                        let selected = item.id == shell.selected_model.get();
                                        let shell_state = shell.clone();
                                        view! {
                                            <button
                                                class=if selected {
                                                    "w-full rounded-xl border border-zinc-300 bg-zinc-100 px-4 py-3 text-left dark:border-zinc-700 dark:bg-[#17181b]"
                                                } else {
                                                    "w-full rounded-xl border border-zinc-200 bg-white px-4 py-3 text-left transition hover:bg-zinc-100 dark:border-zinc-800 dark:bg-[#101114] dark:hover:bg-[#17181b]"
                                                }
                                                on:click={
                                                    let id = item.id.clone();
                                                    move |_| shell_state.selected_model.set(id.clone())
                                                }
                                                type="button"
                                            >
                                                <div class="flex items-start justify-between gap-4">
                                                    <div>
                                                        <p class="text-sm font-medium text-zinc-950 dark:text-zinc-100">{item.name}</p>
                                                        <p class="mt-1 text-xs text-zinc-500 dark:text-zinc-500">{format!("{} / {} MB / {}", item.source, item.size_mb, item.status)}</p>
                                                    </div>
                                                    <span class="text-[11px] text-zinc-500 dark:text-zinc-500">{item.languages.join(" + ")}</span>
                                                </div>
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>

                                <Show when=move || has_warning>
                                    <div class="mt-4 rounded-xl border border-amber-300 bg-amber-50 px-4 py-3 text-sm text-amber-800 dark:border-amber-900/60 dark:bg-amber-950/30 dark:text-amber-200">
                                        {warning_text.clone()}
                                    </div>
                                </Show>

                                <Show when=move || session.error.get().is_some()>
                                    <div class="mt-4 rounded-xl border border-rose-300 bg-rose-50 px-4 py-3 text-sm text-rose-700 dark:border-rose-900/60 dark:bg-rose-950/30 dark:text-rose-200">
                                        {move || session.error.get().unwrap_or_default()}
                                    </div>
                                </Show>

                                <div class="mt-5 space-y-3">
                                    <Show when=move || !model_ready>
                                        <p class="text-sm text-zinc-500 dark:text-zinc-500">
                                            "This model is not installed locally yet, so transcription cannot start from this profile."
                                        </p>
                                    </Show>
                                    <button
                                        class=move || {
                                            if model_ready && !is_starting.get() {
                                                "inline-flex h-10 w-full items-center justify-center rounded-xl bg-zinc-950 px-4 text-sm font-medium text-white transition hover:bg-zinc-800 dark:bg-zinc-100 dark:text-zinc-950 dark:hover:bg-zinc-200"
                                            } else {
                                                "inline-flex h-10 w-full items-center justify-center rounded-xl bg-zinc-300 px-4 text-sm font-medium text-zinc-500 dark:bg-zinc-800 dark:text-zinc-500"
                                            }
                                        }
                                        disabled=move || !model_ready || is_starting.get()
                                        on:click=move |_| start_flow.run(())
                                        type="button"
                                    >
                                        {move || if is_starting.get() { "Starting..." } else { "Start transcription" }}
                                    </button>
                                </div>
                            </section>
                        </aside>
                    </div>
                }
                .into_any()
            }}
        </WorkspaceShell>
    }
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

#[component]
fn StatusTag(label: String) -> impl IntoView {
    view! {
        <span class="inline-flex items-center rounded-full border border-zinc-200 px-2.5 py-1 text-[11px] font-medium text-zinc-700 dark:border-zinc-800 dark:text-zinc-300">
            {label}
        </span>
    }
}

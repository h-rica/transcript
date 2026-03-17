use leptos::{prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;

use crate::{
    components::sidebar::Sidebar,
    state::app_state::{
        AudioInfo, HardwareInfo, TranscriptSegment, TranscriptionProgress, TranscriptionStatus,
        TranscriptionSummary, reset_transcription_state, use_app_state,
    },
};

#[derive(Clone, Copy)]
struct UiModel {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    tier: &'static str,
    diarization: bool,
    ready: bool,
    rtfx: f32,
    size_mb: u32,
}

#[derive(serde::Serialize)]
struct AudioInfoArgs {
    path: String,
}

#[derive(serde::Serialize)]
struct TranscribeArgs {
    path: String,
    model_id: String,
    language: String,
}

#[component]
pub fn FilePreviewPage() -> impl IntoView {
    let state = use_app_state();
    let navigate = use_navigate();
    let is_loading = RwSignal::new(false);
    let is_starting = RwSignal::new(false);

    let models = [
        UiModel {
            id: "whisper-tiny",
            name: "Whisper Tiny",
            description: "Bundled fallback for Phase 1. Fastest path to first transcript.",
            tier: "entry",
            diarization: false,
            ready: true,
            rtfx: 1.65,
            size_mb: 75,
        },
        UiModel {
            id: "whisper-medium",
            name: "Whisper Medium",
            description: "Higher accuracy but not bundled yet. Shown here as a future local option.",
            tier: "balanced",
            diarization: false,
            ready: false,
            rtfx: 0.72,
            size_mb: 1420,
        },
        UiModel {
            id: "whisper-large-v3",
            name: "Whisper Large v3",
            description: "Best quality, deferred until model management lands in Sprint 5.",
            tier: "heavy",
            diarization: true,
            ready: false,
            rtfx: 0.45,
            size_mb: 3000,
        },
    ];

    let fetch_state = state.clone();
    Effect::new(move |_| {
        let Some(file) = fetch_state.selected_file.get() else {
            return;
        };

        is_loading.set(true);
        fetch_state.audio_info.set(None);
        fetch_state.error_message.set(None);

        let state = fetch_state.clone();
        spawn_local(async move {
            let info = if tauri_sys::core::is_tauri() {
                tauri_sys::core::invoke_result::<AudioInfo, String>(
                    "get_audio_info",
                    &AudioInfoArgs { path: file.path },
                )
                .await
                .map_err(|error| error.to_string())
            } else {
                Ok(AudioInfo {
                    duration_s: 1478.0,
                    size_bytes: 28_400_000,
                    format: "mp3".into(),
                    bitrate_kbps: Some(154),
                })
            };

            match info {
                Ok(audio) => state.audio_info.set(Some(audio)),
                Err(error) => state.error_message.set(Some(error)),
            }

            is_loading.set(false);
        });
    });

    let start_state = state.clone();
    let start_transcription = Callback::new(move |_| {
        let Some(file) = start_state.selected_file.get() else {
            return;
        };

        let selected_model = start_state.selected_model.get();
        let selected_language = start_state.selected_language.get();

        reset_transcription_state(&start_state);
        start_state.active_model.set(selected_model.clone());
        start_state
            .transcription_status
            .set(TranscriptionStatus::LoadingModel);
        is_starting.set(true);

        navigate("/transcription", Default::default());

        let state = start_state.clone();
        spawn_local(async move {
            if tauri_sys::core::is_tauri() {
                let result = tauri_sys::core::invoke_result::<(), String>(
                    "transcribe_file",
                    &TranscribeArgs {
                        path: file.path,
                        model_id: selected_model,
                        language: selected_language,
                    },
                )
                .await;

                if let Err(error) = result {
                    state
                        .transcription_status
                        .set(TranscriptionStatus::Failed(error.clone()));
                    state.error_message.set(Some(error));
                }
            } else {
                state.transcription_progress.set(TranscriptionProgress {
                    percent: 1.0,
                    elapsed_s: 52,
                });
                state.transcript_segments.set(vec![
                    TranscriptSegment {
                        speaker: "Speaker A".into(),
                        text: "This is a browser fallback transcript for the Sprint 4 UI flow.".into(),
                        start_s: 0.0,
                        end_s: 8.0,
                        language: "en".into(),
                    },
                    TranscriptSegment {
                        speaker: "Speaker B".into(),
                        text: "The Tauri app will replace this path with live events from Rust.".into(),
                        start_s: 8.0,
                        end_s: 16.0,
                        language: "en".into(),
                    },
                ]);
                state.transcription_summary.set(Some(TranscriptionSummary {
                    segments: 2,
                    speakers: 2,
                    words: 23,
                    language: "en".into(),
                    elapsed_s: 52,
                }));
                state.transcription_status.set(TranscriptionStatus::Complete);
            }

            is_starting.set(false);
        });
    });

    let model_state = state.clone();
    let selected_model = move || {
        let current_id = model_state.selected_model.get();
        models
            .iter()
            .find(|model| model.id == current_id)
            .copied()
            .unwrap_or(models[0])
    };

    view! {
        <div class="flex h-screen w-full">
            <Sidebar/>

            <div class="min-w-0 flex-1 overflow-auto px-8 py-6">
                <div class="mx-auto flex max-w-6xl flex-col gap-6">
                    <div class="flex items-start justify-between gap-4">
                        <div>
                            <p class="text-xs font-semibold uppercase tracking-[0.24em] text-slate-400 dark:text-slate-500">
                                "File preview"
                            </p>
                            <h1 class="mt-2 text-3xl font-semibold tracking-tight">"Confirm before transcription"</h1>
                        </div>
                        <a
                            href="/"
                            class="rounded-full border border-slate-200 px-4 py-2 text-sm text-slate-500 transition hover:border-slate-300 hover:text-slate-900 dark:border-slate-800 dark:text-slate-400 dark:hover:border-slate-700 dark:hover:text-slate-100"
                        >
                            "Back"
                        </a>
                    </div>

                    {move || {
                        let Some(file) = state.selected_file.get() else {
                            return view! {
                                <div class="rounded-[28px] border border-dashed border-slate-300 bg-white p-8 text-sm text-slate-500 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-400">
                                    "No file selected yet. Return to Home and drop an audio file first."
                                </div>
                            }
                            .into_any();
                        };

                        let current_model = selected_model();
                        let start_transcription = start_transcription.clone();
                        let audio_info = state.audio_info.get();
                        let warning = hardware_warning(state.hardware_info.get(), current_model.tier);
                        let runtime_estimate = audio_info
                            .as_ref()
                            .map(|info| info.duration_s / current_model.rtfx.max(0.1));

                        let duration_value = audio_info
                            .as_ref()
                            .map(|info| format_hms(info.duration_s))
                            .unwrap_or_else(|| {
                                if is_loading.get() {
                                    "Loading...".into()
                                } else {
                                    "--".into()
                                }
                            });
                        let size_value = audio_info
                            .as_ref()
                            .map(|info| format_bytes(info.size_bytes))
                            .unwrap_or_else(|| "--".into());
                        let format_value = audio_info
                            .as_ref()
                            .map(|info| info.format.to_uppercase())
                            .unwrap_or_else(|| "--".into());
                        let bitrate_value = audio_info
                            .as_ref()
                            .and_then(|info| info.bitrate_kbps)
                            .map(|value| format!("{value} kbps"))
                            .unwrap_or_else(|| "--".into());

                        view! {
                            <div class="grid gap-6 lg:grid-cols-[1.15fr_0.85fr]">
                                <section class="rounded-[32px] border border-slate-200 bg-white p-6 dark:border-slate-800 dark:bg-slate-900">
                                    <div class="flex items-start justify-between gap-4">
                                        <div>
                                            <p class="text-sm font-semibold text-slate-900 dark:text-slate-100">{file.name}</p>
                                            <p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
                                                {file.path}
                                            </p>
                                        </div>
                                        <div class="rounded-full bg-slate-100 px-3 py-1 text-xs font-medium text-slate-500 dark:bg-slate-800 dark:text-slate-400">
                                            {move || state.selected_language.get().to_uppercase()}
                                        </div>
                                    </div>

                                    <div class="mt-6 grid gap-3 sm:grid-cols-2">
                                        <MetricCard label="Duration" value=duration_value/>
                                        <MetricCard label="Size" value=size_value/>
                                        <MetricCard label="Format" value=format_value/>
                                        <MetricCard label="Bitrate" value=bitrate_value/>
                                    </div>

                                    <div class="mt-6 grid gap-4 md:grid-cols-[0.9fr_1.1fr]">
                                        <div class="space-y-2">
                                            <label class="text-xs font-semibold uppercase tracking-[0.24em] text-slate-400 dark:text-slate-500">
                                                "Language"
                                            </label>
                                            <select
                                                class="w-full rounded-2xl border border-slate-200 bg-white px-4 py-3 text-sm outline-none transition focus:border-slate-400 dark:border-slate-700 dark:bg-slate-950"
                                                prop:value=move || state.selected_language.get()
                                                on:change=move |event| state.selected_language.set(event_target_value(&event))
                                            >
                                                <option value="fr">"French"</option>
                                                <option value="en">"English"</option>
                                                <option value="auto">"Auto detect"</option>
                                            </select>
                                        </div>

                                        <div class="rounded-[24px] bg-slate-100 p-4 text-sm text-slate-600 dark:bg-slate-950 dark:text-slate-300">
                                            <p class="font-semibold text-slate-900 dark:text-slate-100">"Estimated runtime"</p>
                                            <p class="mt-2 text-2xl font-semibold">
                                                {runtime_estimate.map(format_hms).unwrap_or_else(|| "--".into())}
                                            </p>
                                            <p class="mt-2 text-xs text-slate-500 dark:text-slate-400">
                                                {format!("Assumes {:.2}x realtime on {}", current_model.rtfx, current_model.name)}
                                            </p>
                                        </div>
                                    </div>
                                </section>

                                <section class="rounded-[32px] border border-slate-200 bg-white p-6 dark:border-slate-800 dark:bg-slate-900">
                                    <div class="flex items-center justify-between gap-4">
                                        <div>
                                            <p class="text-xs font-semibold uppercase tracking-[0.24em] text-slate-400 dark:text-slate-500">
                                                "Model selector"
                                            </p>
                                            <h2 class="mt-2 text-xl font-semibold">"Choose a local model"</h2>
                                        </div>
                                        <div class="rounded-full bg-slate-100 px-3 py-1 text-xs font-medium text-slate-500 dark:bg-slate-800 dark:text-slate-400">
                                            {current_model.tier}
                                        </div>
                                    </div>

                                    <div class="mt-5 space-y-3">
                                        {models.into_iter().map(|model| {
                                            let state = state.clone();
                                            let is_current = model.id == current_model.id;
                                            view! {
                                                <button
                                                    class=move || format!(
                                                        "w-full rounded-[24px] border px-4 py-4 text-left transition {}",
                                                        if is_current {
                                                            "border-slate-900 bg-slate-900 text-white dark:border-slate-100 dark:bg-slate-100 dark:text-slate-950"
                                                        } else {
                                                            "border-slate-200 bg-white hover:border-slate-300 dark:border-slate-800 dark:bg-slate-900 dark:hover:border-slate-700"
                                                        }
                                                    )
                                                    on:click=move |_| state.selected_model.set(model.id.into())
                                                    type="button"
                                                >
                                                    <div class="flex items-start justify-between gap-4">
                                                        <div>
                                                            <div class="flex items-center gap-2">
                                                                <span class="text-sm font-semibold">{model.name}</span>
                                                                {if model.diarization {
                                                                    view! { <span class="rounded-full bg-white/15 px-2 py-0.5 text-[11px] font-medium uppercase tracking-wide dark:bg-slate-900/10">"Speakers"</span> }.into_any()
                                                                } else {
                                                                    view! { <></> }.into_any()
                                                                }}
                                                            </div>
                                                            <p class="mt-2 text-sm opacity-80">{model.description}</p>
                                                        </div>
                                                        <div class="text-right text-xs opacity-80">
                                                            <div>{format!("{} MB", model.size_mb)}</div>
                                                            <div class="mt-1">{if model.ready { "Ready" } else { "Not downloaded" }}</div>
                                                        </div>
                                                    </div>
                                                </button>
                                            }
                                        }).collect_view()}
                                    </div>

                                    {if let Some(message) = warning {
                                        view! {
                                            <div class="mt-5 rounded-[24px] border border-amber-200 bg-amber-50 px-4 py-3 text-sm text-amber-900 dark:border-amber-800 dark:bg-amber-950/50 dark:text-amber-200">
                                                {message}
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <></> }.into_any()
                                    }}

                                    {move || {
                                        if let Some(message) = state.error_message.get() {
                                            view! {
                                                <div class="mt-4 rounded-[24px] border border-rose-200 bg-rose-50 px-4 py-3 text-sm text-rose-900 dark:border-rose-900 dark:bg-rose-950/50 dark:text-rose-200">
                                                    {message}
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <></> }.into_any()
                                        }
                                    }}

                                    <button
                                        class=move || format!(
                                            "mt-6 inline-flex w-full items-center justify-center rounded-[24px] px-4 py-3 text-sm font-semibold transition {}",
                                            if current_model.ready {
                                                "bg-slate-900 text-white hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-950 dark:hover:bg-slate-200"
                                            } else {
                                                "cursor-not-allowed bg-slate-200 text-slate-500 dark:bg-slate-800 dark:text-slate-500"
                                            }
                                        )
                                        disabled=move || !current_model.ready || is_starting.get()
                                        on:click=move |_| start_transcription.run(())
                                        type="button"
                                    >
                                        {move || if is_starting.get() { "Starting..." } else { "Start transcription" }}
                                    </button>
                                </section>
                            </div>
                        }
                        .into_any()
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
fn MetricCard(label: &'static str, #[prop(into)] value: String) -> impl IntoView {
    view! {
        <div class="rounded-[24px] bg-slate-100 px-4 py-4 dark:bg-slate-950">
            <p class="text-xs font-semibold uppercase tracking-[0.24em] text-slate-400 dark:text-slate-500">
                {label}
            </p>
            <p class="mt-2 text-lg font-semibold text-slate-900 dark:text-slate-100">{value}</p>
        </div>
    }
}

fn warning_hardware_label(hardware: Option<HardwareInfo>) -> String {
    hardware
        .map(|info| format!("{} GB RAM | {}", info.ram_gb, info.cpu_name))
        .unwrap_or_else(|| "Hardware detection pending".into())
}

fn hardware_warning(hardware: Option<HardwareInfo>, model_tier: &str) -> Option<String> {
    let hardware = hardware?;
    if model_tier == "heavy" && hardware.ram_gb < 16 {
        Some(format!(
            "{} detected. Large models will likely exceed comfortable memory limits in Phase 1.",
            warning_hardware_label(Some(hardware))
        ))
    } else {
        None
    }
}

fn format_hms(seconds: f32) -> String {
    let total = seconds.max(0.0).round() as u32;
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    let secs = total % 60;

    if hours > 0 {
        format!("{hours}:{minutes:02}:{secs:02}")
    } else {
        format!("{minutes:02}:{secs:02}")
    }
}

fn format_bytes(size: u64) -> String {
    let kb = 1024.0;
    let mb = kb * 1024.0;
    let gb = mb * 1024.0;
    let size = size as f64;

    if size >= gb {
        format!("{:.2} GB", size / gb)
    } else if size >= mb {
        format!("{:.1} MB", size / mb)
    } else if size >= kb {
        format!("{:.1} KB", size / kb)
    } else {
        format!("{size:.0} B")
    }
}

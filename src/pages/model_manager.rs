use leptos::prelude::*;

use crate::{
    components::workspace::{WorkspaceHeader, WorkspaceRoute, WorkspaceShell},
    features::workspace_data::{
        fallback_models, installed_storage_mb, model_is_ready, recommended_model_id,
        storage_capacity_mb,
    },
    state::app_state::use_app_shell_state,
};

#[component]
pub fn ModelManagerPage() -> impl IntoView {
    let shell = use_app_shell_state();

    let models = move || {
        let loaded = shell.available_models.get();
        if loaded.is_empty() {
            fallback_models()
        } else {
            loaded
        }
    };

    view! {
        <WorkspaceShell route=WorkspaceRoute::Models>
            <WorkspaceHeader
                title="Models"
                subtitle="Inspect local model availability, hardware fit, and storage pressure before switching transcription engines."
            >
                <span class="inline-flex items-center rounded-full border border-zinc-200 px-2.5 py-1 text-[11px] font-medium text-zinc-700 dark:border-zinc-800 dark:text-zinc-300">
                    {move || format!("{} models", models().len())}
                </span>
            </WorkspaceHeader>

            <section class="rounded-[1.5rem] border border-zinc-200 bg-white px-5 py-5 dark:border-zinc-900 dark:bg-[#141519]">
                <p class="text-sm font-medium text-zinc-950 dark:text-zinc-100">"Hardware"</p>
                <div class="mt-4 grid gap-3 md:grid-cols-4">
                    <MetricTile label="RAM" value=Signal::derive(move || shell.hardware_info.get().map(|info| format!("{} GB", info.ram_gb)).unwrap_or_else(|| "--".into()))/>
                    <MetricTile label="CPU" value=Signal::derive(move || shell.hardware_info.get().map(|info| info.cpu_name).unwrap_or_else(|| "Unknown CPU".into()))/>
                    <MetricTile label="VRAM" value=Signal::derive(move || shell.hardware_info.get().and_then(|info| info.gpu_vram_gb).map(|vram| format!("{} GB", vram)).unwrap_or_else(|| "No GPU".into()))/>
                    <MetricTile label="Tier" value=Signal::derive(move || shell.hardware_info.get().map(|info| info.tier).unwrap_or_else(|| "balanced".into()))/>
                </div>
            </section>

            <section class="rounded-[1.5rem] border border-zinc-200 bg-white px-5 py-5 dark:border-zinc-900 dark:bg-[#141519]">
                <div class="flex flex-wrap items-center justify-between gap-3">
                    <div>
                        <p class="text-sm font-medium text-zinc-950 dark:text-zinc-100">"Storage"</p>
                        <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-500">"Installed model footprint across the local catalog snapshot."</p>
                    </div>
                    <p class="text-sm text-zinc-500 dark:text-zinc-500">"Managed locally by the desktop app"</p>
                </div>
                <div class="mt-4 h-1.5 overflow-hidden rounded-full bg-zinc-200 dark:bg-zinc-800">
                    <div
                        class="h-full rounded-full bg-zinc-900 dark:bg-zinc-100"
                        style=move || {
                            let current_models = models();
                            let used = installed_storage_mb(&current_models) as f32;
                            let total = storage_capacity_mb(&current_models, shell.hardware_info.get()) as f32;
                            format!("width: {:.2}%;", (used / total.max(1.0) * 100.0).clamp(0.0, 100.0))
                        }
                    ></div>
                </div>
                <p class="mt-3 text-sm text-zinc-600 dark:text-zinc-400">
                    {move || {
                        let current_models = models();
                        let used = installed_storage_mb(&current_models);
                        let total = storage_capacity_mb(&current_models, shell.hardware_info.get());
                        format!("{} MB installed / {} MB budgeted", used, total)
                    }}
                </p>
            </section>

            <div class="space-y-4">
                {move || {
                    let current_models = models();
                    let recommended = recommended_model_id(&current_models, shell.hardware_info.get());
                    let active_model_id = shell.active_model.get();
                    let shell_state = shell.clone();
                    current_models
                        .into_iter()
                        .map(move |model| {
                            let is_active = active_model_id == model.id;
                            let is_ready = model_is_ready(&model);
                            let is_recommended = recommended.as_deref() == Some(model.id.as_str());
                            let shell_for_action = shell_state.clone();
                            view! {
                                <section class="rounded-[1.5rem] border border-zinc-200 bg-white px-5 py-5 dark:border-zinc-900 dark:bg-[#141519]">
                                    <div class="flex flex-wrap items-start justify-between gap-4">
                                        <div>
                                            <div class="flex flex-wrap items-center gap-2">
                                                <h2 class="text-base font-semibold text-zinc-950 dark:text-zinc-100">{model.name.clone()}</h2>
                                                <Show when=move || is_active>
                                                    <StatePill label="Active" tone="active"/>
                                                </Show>
                                                <Show when=move || is_recommended>
                                                    <StatePill label="Recommended" tone="recommended"/>
                                                </Show>
                                                <Show when=move || !is_ready>
                                                    <StatePill label="Not installed" tone="muted"/>
                                                </Show>
                                            </div>
                                            <p class="mt-2 text-sm leading-6 text-zinc-600 dark:text-zinc-400">{model.description.clone()}</p>
                                            <p class="mt-3 text-xs text-zinc-500 dark:text-zinc-500">
                                                {format!("{} MB / {} / {}", model.size_mb, model.source, model.languages.join(" + "))}
                                            </p>
                                        </div>

                                        <div class="flex flex-wrap items-center gap-2 text-[11px] text-zinc-500 dark:text-zinc-500">
                                            <span class="inline-flex items-center rounded-full border border-zinc-200 px-2.5 py-1 dark:border-zinc-800">{model.tier.clone()}</span>
                                            <Show when=move || model.diarization>
                                                <span class="inline-flex items-center rounded-full border border-zinc-200 px-2.5 py-1 dark:border-zinc-800">"Diarization"</span>
                                            </Show>
                                        </div>
                                    </div>

                                    <div class="mt-4 flex flex-wrap items-center justify-between gap-3 border-t border-zinc-200 pt-4 dark:border-zinc-900">
                                        <p class="text-sm text-zinc-500 dark:text-zinc-500">
                                            {if is_ready {
                                                "Ready for local transcription."
                                            } else {
                                                "Model files are not installed locally in this build yet."
                                            }}
                                        </p>
                                        <Show
                                            when=move || is_ready
                                            fallback=move || view! {
                                                <button
                                                    class="inline-flex h-9 items-center rounded-lg border border-zinc-200 px-3 text-sm font-medium text-zinc-400 dark:border-zinc-800 dark:text-zinc-500"
                                                    disabled=true
                                                    type="button"
                                                >
                                                    "Install unavailable"
                                                </button>
                                            }
                                        >
                                            <button
                                                class=if is_active {
                                                    "inline-flex h-9 items-center rounded-lg border border-zinc-300 bg-zinc-950 px-3 text-sm font-medium text-white dark:border-zinc-700 dark:bg-zinc-100 dark:text-zinc-950"
                                                } else {
                                                    "inline-flex h-9 items-center rounded-lg border border-zinc-200 px-3 text-sm font-medium text-zinc-700 transition hover:bg-zinc-100 hover:text-zinc-950 dark:border-zinc-800 dark:text-zinc-300 dark:hover:bg-[#17181b] dark:hover:text-zinc-100"
                                                }
                                                on:click={
                                                    let model_id = model.id.clone();
                                                    let shell_for_action = shell_for_action.clone();
                                                    move |_| {
                                                        shell_for_action.selected_model.set(model_id.clone());
                                                        shell_for_action.active_model.set(model_id.clone());
                                                    }
                                                }
                                                type="button"
                                            >
                                                {if is_active { "Currently active" } else { "Use model" }}
                                            </button>
                                        </Show>
                                    </div>
                                </section>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </WorkspaceShell>
    }
}

#[component]
fn MetricTile(label: &'static str, #[prop(into)] value: Signal<String>) -> impl IntoView {
    view! {
        <div class="rounded-xl border border-zinc-200 bg-zinc-100/80 px-4 py-4 dark:border-zinc-800 dark:bg-[#101114]">
            <p class="text-[11px] font-medium uppercase tracking-[0.18em] text-zinc-500">{label}</p>
            <p class="mt-2 text-sm font-semibold text-zinc-950 dark:text-zinc-100">{move || value.get()}</p>
        </div>
    }
}

#[component]
fn StatePill(label: &'static str, tone: &'static str) -> impl IntoView {
    let class = match tone {
        "active" => {
            "inline-flex items-center rounded-full border border-emerald-300 bg-emerald-50 px-2.5 py-1 text-[11px] font-medium text-emerald-700 dark:border-emerald-900/60 dark:bg-emerald-950/30 dark:text-emerald-200"
        }
        "recommended" => {
            "inline-flex items-center rounded-full border border-sky-300 bg-sky-50 px-2.5 py-1 text-[11px] font-medium text-sky-700 dark:border-sky-900/60 dark:bg-sky-950/30 dark:text-sky-200"
        }
        _ => {
            "inline-flex items-center rounded-full border border-zinc-200 px-2.5 py-1 text-[11px] font-medium text-zinc-500 dark:border-zinc-800 dark:text-zinc-500"
        }
    };

    view! { <span class=class>{label}</span> }
}

use leptos::prelude::*;
use singlestage::{Badge, Card, CardContent, CardDescription, CardHeader, CardTitle};

use crate::{
    components::{
        app_ui::ActionButton,
        workspace::{WorkspaceHeader, WorkspaceRoute, WorkspaceShell},
    },
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
                <Badge variant="outline">{move || format!("{} models", models().len())}</Badge>
            </WorkspaceHeader>

            <Card class="border-zinc-800 bg-[#191919] text-zinc-50">
                <CardContent class="grid gap-4 p-5 md:grid-cols-4">
                    <MetricTile label="RAM" value=Signal::derive(move || shell.hardware_info.get().map(|info| format!("{} GB", info.ram_gb)).unwrap_or_else(|| "--".into()))/>
                    <MetricTile label="CPU" value=Signal::derive(move || shell.hardware_info.get().map(|info| info.cpu_name).unwrap_or_else(|| "Unknown CPU".into()))/>
                    <MetricTile label="VRAM" value=Signal::derive(move || shell.hardware_info.get().and_then(|info| info.gpu_vram_gb).map(|vram| format!("{} GB", vram)).unwrap_or_else(|| "No GPU".into()))/>
                    <MetricTile label="Tier" value=Signal::derive(move || shell.hardware_info.get().map(|info| info.tier).unwrap_or_else(|| "balanced".into()))/>
                </CardContent>
            </Card>

            <Card class="border-zinc-800 bg-[#191919] text-zinc-50">
                <CardHeader>
                    <CardTitle>"Storage"</CardTitle>
                    <CardDescription>
                        "Installed model size is derived from the local catalog snapshot available to the app."
                    </CardDescription>
                </CardHeader>
                <CardContent class="space-y-4">
                    <div class="h-2 overflow-hidden rounded-full bg-zinc-800">
                        <div
                            class="h-full rounded-full bg-zinc-100"
                            style=move || {
                                let current_models = models();
                                let used = installed_storage_mb(&current_models) as f32;
                                let total = storage_capacity_mb(&current_models, shell.hardware_info.get()) as f32;
                                format!("width: {:.2}%;", (used / total.max(1.0) * 100.0).clamp(0.0, 100.0))
                            }
                        ></div>
                    </div>
                    <p class="text-sm text-zinc-400">
                        {move || {
                            let current_models = models();
                            let used = installed_storage_mb(&current_models);
                            let total = storage_capacity_mb(&current_models, shell.hardware_info.get());
                            format!("{} MB installed · {} MB budgeted for local model storage", used, total)
                        }}
                    </p>
                </CardContent>
            </Card>

            <div class="grid gap-4">
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
                                <Card class="border-zinc-800 bg-[#191919] text-zinc-50">
                                    <CardHeader class="space-y-3">
                                        <div class="flex flex-wrap items-start justify-between gap-3">
                                            <div class="space-y-1">
                                                <CardTitle>{model.name.clone()}</CardTitle>
                                                <CardDescription>{model.description.clone()}</CardDescription>
                                            </div>
                                            <div class="flex flex-wrap items-center gap-2">
                                                <Badge variant="outline">{format!("{} MB", model.size_mb)}</Badge>
                                                <Show when=move || is_recommended>
                                                    <Badge variant="secondary">"Recommended"</Badge>
                                                </Show>
                                                <Show when=move || is_active>
                                                    <Badge variant="secondary">"Active"</Badge>
                                                </Show>
                                            </div>
                                        </div>
                                    </CardHeader>
                                    <CardContent class="flex flex-wrap items-center justify-between gap-4">
                                        <div class="flex flex-wrap items-center gap-2 text-sm text-zinc-400">
                                            <Badge variant="outline">{model.tier.clone()}</Badge>
                                            <Badge variant="outline">{model.source.clone()}</Badge>
                                            <Show when=move || model.diarization>
                                                <Badge variant="outline">"Speaker labels"</Badge>
                                            </Show>
                                            <span>{model.languages.join(" + ")}</span>
                                        </div>
                                        <div class="flex flex-wrap items-center gap-3">
                                            <Show when=move || is_ready fallback=move || view! {
                                                <span class="text-sm text-zinc-500">"Install model files locally to activate this profile."</span>
                                            }>
                                                <ActionButton
                                                    on_click={
                                                        let model_id = model.id.clone();
                                                        let shell_for_action = shell_for_action.clone();
                                                        Callback::new(move |_| {
                                                            shell_for_action.selected_model.set(model_id.clone());
                                                            shell_for_action.active_model.set(model_id.clone());
                                                        })
                                                    }
                                                    variant=if is_active { "secondary" } else { "outline" }
                                                >
                                                    {if is_active { "Currently active" } else { "Use model" }}
                                                </ActionButton>
                                            </Show>
                                        </div>
                                    </CardContent>
                                </Card>
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
        <div class="rounded-2xl border border-zinc-800 bg-zinc-900/70 p-4">
            <p class="text-xs uppercase tracking-[0.2em] text-zinc-500">{label}</p>
            <p class="mt-2 text-sm font-semibold text-zinc-50">{move || value.get()}</p>
        </div>
    }
}

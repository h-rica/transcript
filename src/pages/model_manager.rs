use leptos::prelude::*;

use crate::{
    components::workspace::{WorkspaceHeader, WorkspaceRoute, WorkspaceShell},
    features::{
        shared::format_bytes,
        workspace_data::{
            fallback_models, installed_storage_mb, model_is_ready, recommended_model_id,
            storage_capacity_mb,
        },
    },
    state::app_state::{HardwareInfo, WorkspaceModel, use_app_shell_state},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModelDisplayMode {
    List,
    Grid,
    Cards,
}

#[derive(Clone)]
struct ModelSectionData {
    title: &'static str,
    summary: &'static str,
    models: Vec<WorkspaceModel>,
}

fn format_storage_mb(value_mb: u32) -> String {
    format_bytes(u64::from(value_mb) * 1024 * 1024)
}

fn installed_model_count(models: &[WorkspaceModel]) -> usize {
    models.iter().filter(|model| model_is_ready(model)).count()
}

fn model_usage_label(tier: &str) -> &'static str {
    match tier {
        "minimal" => "Fast",
        "standard" | "balanced" => "Balanced",
        "heavy" => "High accuracy",
        _ => "Custom",
    }
}

fn model_fit_label(model: &WorkspaceModel, hardware: Option<HardwareInfo>) -> &'static str {
    let ram_gb = hardware.map(|info| info.ram_gb).unwrap_or(8);

    if model.bundled || model.size_mb <= 400 {
        "Very light"
    } else if (ram_gb >= 24 && model.size_mb <= 9_000)
        || (ram_gb >= 16 && model.size_mb <= 3_500)
        || (ram_gb >= 12 && model.size_mb <= 2_000)
        || (ram_gb >= 8 && model.size_mb <= 1_000)
    {
        "Good fit"
    } else if (ram_gb >= 16 && model.size_mb <= 9_000)
        || (ram_gb >= 12 && model.size_mb <= 3_500)
        || (ram_gb >= 8 && model.size_mb <= 1_800)
    {
        "Possible"
    } else {
        "Heavy"
    }
}

fn model_source_label(source: &str) -> &'static str {
    match source {
        "bundled" => "Bundled",
        "huggingface" => "Hugging Face",
        _ => "External",
    }
}

fn format_language_label(language: &str) -> String {
    match language {
        "fr" => "FR".into(),
        "en" => "EN".into(),
        "multilingual" => "Multilingual".into(),
        other if other.len() <= 3 => other.to_ascii_uppercase(),
        other => {
            let mut chars = other.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
                None => String::new(),
            }
        }
    }
}

fn language_summary(languages: &[String]) -> String {
    let labels = languages
        .iter()
        .map(|language| format_language_label(language))
        .collect::<Vec<_>>();

    match labels.as_slice() {
        [] => "Auto".into(),
        [single] => single.clone(),
        [first, second] => format!("{first}, {second}"),
        [first, second, ..] => format!("{first}, {second} +{}", labels.len() - 2),
    }
}

fn build_model_sections(
    models: &[WorkspaceModel],
    recommended_id: Option<&str>,
) -> Vec<ModelSectionData> {
    let recommended_models = recommended_id
        .and_then(|model_id| models.iter().find(|model| model.id == model_id).cloned())
        .into_iter()
        .collect::<Vec<_>>();

    let installed_models = models
        .iter()
        .filter(|model| model_is_ready(model) && Some(model.id.as_str()) != recommended_id)
        .cloned()
        .collect::<Vec<_>>();

    let more_models = models
        .iter()
        .filter(|model| Some(model.id.as_str()) != recommended_id && !model_is_ready(model))
        .cloned()
        .collect::<Vec<_>>();

    vec![
        ModelSectionData {
            title: "Recommended for this PC",
            summary: "Start here for the cleanest speed and accuracy trade-off on this device.",
            models: recommended_models,
        },
        ModelSectionData {
            title: "Installed locally",
            summary: "Ready to switch on immediately without downloading anything else.",
            models: installed_models,
        },
        ModelSectionData {
            title: "More models",
            summary: "Extra options when you need a different accuracy or speaker workflow.",
            models: more_models,
        },
    ]
}

#[component]
pub fn ModelManagerPage() -> impl IntoView {
    let shell = use_app_shell_state();
    let available_models = shell.available_models;
    let hardware_info = shell.hardware_info;
    let selected_model_signal = shell.selected_model;
    let active_model_signal = shell.active_model;
    let display_mode = RwSignal::new(ModelDisplayMode::List);
    let models = Signal::derive(move || {
        let loaded = available_models.get();
        if loaded.is_empty() {
            fallback_models()
        } else {
            loaded
        }
    });

    view! {
        <WorkspaceShell route=WorkspaceRoute::Models>
            <WorkspaceHeader
                title="Models"
                subtitle="Inspect hardware fit, local disk pressure, and model availability before switching transcription engines."
            >
                <span class="inline-flex items-center rounded-full border border-zinc-200 px-2.5 py-1 text-[11px] font-medium text-zinc-700 dark:border-zinc-800 dark:text-zinc-300">
                    {move || format!("{} models", models.get().len())}
                </span>
            </WorkspaceHeader>

            <div class="space-y-4">
                <section class="grid gap-3 sm:grid-cols-2 xl:grid-cols-4">
                    <MetricTile
                        label="System RAM"
                        value=Signal::derive(move || {
                            hardware_info
                                .get()
                                .map(|info| format!("{} GB", info.ram_gb))
                                .unwrap_or_else(|| "--".into())
                        })
                    />
                    <MetricTile
                        label="CPU"
                        value=Signal::derive(move || {
                            hardware_info
                                .get()
                                .map(|info| info.cpu_name)
                                .unwrap_or_else(|| "Unknown CPU".into())
                        })
                    />
                    <MetricTile
                        label="GPU VRAM"
                        value=Signal::derive(move || {
                            hardware_info
                                .get()
                                .and_then(|info| info.gpu_vram_gb)
                                .map(|vram| format!("{} GB", vram))
                                .unwrap_or_else(|| "No GPU".into())
                        })
                    />
                    <MetricTile
                        label="Model library"
                        value=Signal::derive(move || {
                            let current_models = models.get();
                            format!(
                                "{} installed / {} total",
                                installed_model_count(&current_models),
                                current_models.len()
                            )
                        })
                    />
                </section>

                <section class="space-y-4 rounded-[1.15rem] border border-zinc-200 bg-white px-4 py-4 dark:border-zinc-900 dark:bg-[#141519]">
                    <div class="flex flex-wrap items-start justify-between gap-4">
                        <div class="space-y-1">
                            <h2 class="text-sm font-semibold text-zinc-950 dark:text-zinc-100">
                                "Local model disk space"
                            </h2>
                            <p class="max-w-2xl text-sm leading-6 text-zinc-600 dark:text-zinc-400">
                                "Downloaded models use disk space on this device. This is separate from system RAM and GPU memory."
                            </p>
                        </div>
                        <div class="text-left sm:text-right">
                            <p class="text-lg font-semibold text-zinc-950 dark:text-zinc-100">
                                {move || {
                                    let current_models = models.get();
                                    let used = installed_storage_mb(&current_models);
                                    let total = storage_capacity_mb(&current_models, hardware_info.get());
                                    format!("{} / {}", format_storage_mb(used), format_storage_mb(total))
                                }}
                            </p>
                            <p class="text-xs font-medium uppercase tracking-[0.14em] text-zinc-500">
                                "Used on local disk"
                            </p>
                        </div>
                    </div>

                    <div class="flex flex-wrap items-center justify-between gap-3 text-sm text-zinc-600 dark:text-zinc-500">
                        <span>
                            {move || {
                                let current_models = models.get();
                                let used = installed_storage_mb(&current_models);
                                format!("{} used by downloaded and bundled models", format_storage_mb(used))
                            }}
                        </span>
                        <span>
                            {move || {
                                let current_models = models.get();
                                let total = storage_capacity_mb(&current_models, hardware_info.get());
                                format!("{} reserved for the local model library", format_storage_mb(total))
                            }}
                        </span>
                    </div>

                    <div class="h-1.5 overflow-hidden rounded-full bg-zinc-200 dark:bg-zinc-800">
                        <div
                            class="h-full rounded-full bg-zinc-950 dark:bg-zinc-100"
                            style=move || {
                                let current_models = models.get();
                                let used = installed_storage_mb(&current_models) as f32;
                                let total =
                                    storage_capacity_mb(&current_models, hardware_info.get())
                                        as f32;
                                format!(
                                    "width: {:.2}%;",
                                    (used / total.max(1.0) * 100.0).clamp(0.0, 100.0)
                                )
                            }
                        ></div>
                    </div>
                </section>

                <section class="flex flex-wrap items-center justify-between gap-3 rounded-[1.15rem] border border-zinc-200 bg-zinc-50/70 px-4 py-3 dark:border-zinc-900 dark:bg-[#15171b]">
                    <div class="space-y-1">
                        <h2 class="text-sm font-semibold text-zinc-950 dark:text-zinc-100">
                            "Model browser"
                        </h2>
                        <p class="text-sm text-zinc-600 dark:text-zinc-400">
                            "List is the tightest comparison view. Grid is quicker to scan. Cards expose the most context."
                        </p>
                    </div>

                    <div class="inline-flex items-center rounded-xl border border-zinc-200 bg-white p-1 dark:border-zinc-900 dark:bg-[#141519]">
                        <DisplayModeButton
                            current_mode=display_mode
                            label="List"
                            mode=ModelDisplayMode::List
                        />
                        <DisplayModeButton
                            current_mode=display_mode
                            label="Grid"
                            mode=ModelDisplayMode::Grid
                        />
                        <DisplayModeButton
                            current_mode=display_mode
                            label="Cards"
                            mode=ModelDisplayMode::Cards
                        />
                    </div>
                </section>

                <div class="space-y-5">
                    {move || {
                        let current_models = models.get();
                        let hardware = hardware_info.get();
                        let recommended = recommended_model_id(&current_models, hardware.clone());
                        let active_model_id = active_model_signal.get();
                        let display = display_mode.get();

                        build_model_sections(&current_models, recommended.as_deref())
                            .into_iter()
                            .filter(|section| !section.models.is_empty())
                            .map(move |section| {
                                let hardware_for_section = hardware.clone();
                                let recommended_for_section = recommended.clone();
                                let active_model_id = active_model_id.clone();

                                view! {
                                    <ModelSection
                                        count=section.models.len()
                                        display_mode=display
                                        summary=section.summary
                                        title=section.title
                                    >
                                        {section
                                            .models
                                            .into_iter()
                                            .map(move |model| {
                                                let is_active = active_model_id == model.id;
                                                let is_recommended = recommended_for_section
                                                    .as_deref()
                                                    == Some(model.id.as_str());

                                                view! {
                                                    <ModelCard
                                                        active_model=active_model_signal
                                                        display_mode=display
                                                        hardware=hardware_for_section.clone()
                                                        is_active=is_active
                                                        is_recommended=is_recommended
                                                        model=model
                                                        selected_model=selected_model_signal
                                                    />
                                                }
                                            })
                                            .collect_view()}
                                    </ModelSection>
                                }
                            })
                            .collect_view()
                    }}
                </div>
            </div>
        </WorkspaceShell>
    }
}

#[component]
fn DisplayModeButton(
    current_mode: RwSignal<ModelDisplayMode>,
    label: &'static str,
    mode: ModelDisplayMode,
) -> impl IntoView {
    view! {
        <button
            class=move || {
                if current_mode.get() == mode {
                    "inline-flex h-8 items-center rounded-lg bg-zinc-950 px-3 text-sm font-medium text-white dark:bg-zinc-100 dark:text-zinc-950"
                } else {
                    "inline-flex h-8 items-center rounded-lg px-3 text-sm font-medium text-zinc-600 transition hover:bg-zinc-100 hover:text-zinc-950 dark:text-zinc-400 dark:hover:bg-[#1a1c20] dark:hover:text-zinc-100"
                }
            }
            on:click=move |_| current_mode.set(mode)
            type="button"
        >
            {label}
        </button>
    }
}

#[component]
fn ModelSection(
    title: &'static str,
    summary: &'static str,
    count: usize,
    display_mode: ModelDisplayMode,
    children: Children,
) -> impl IntoView {
    let layout_class = match display_mode {
        ModelDisplayMode::List => "space-y-2.5",
        ModelDisplayMode::Grid => "grid gap-3 md:grid-cols-2 xl:grid-cols-3",
        ModelDisplayMode::Cards => "grid gap-4 lg:grid-cols-2",
    };

    view! {
        <section class="space-y-3">
            <div class="flex flex-wrap items-center justify-between gap-3">
                <div class="space-y-1">
                    <h2 class="text-sm font-semibold text-zinc-950 dark:text-zinc-100">{title}</h2>
                    <p class="text-sm text-zinc-600 dark:text-zinc-400">{summary}</p>
                </div>
                <span class="inline-flex items-center rounded-full border border-zinc-200 px-2.5 py-1 text-[11px] font-medium text-zinc-600 dark:border-zinc-800 dark:text-zinc-400">
                    {format!(
                        "{} {}",
                        count,
                        if count == 1 { "model" } else { "models" }
                    )}
                </span>
            </div>

            <div class=layout_class>{children()}</div>
        </section>
    }
}

#[component]
fn ModelCard(
    model: WorkspaceModel,
    display_mode: ModelDisplayMode,
    hardware: Option<HardwareInfo>,
    is_active: bool,
    is_recommended: bool,
    selected_model: RwSignal<String>,
    active_model: RwSignal<String>,
) -> impl IntoView {
    let is_ready = model_is_ready(&model);
    let has_diarization = model.diarization;
    let model_id = model.id.clone();
    let model_name = model.name.clone();
    let description = model.description.clone();
    let size_label = format_storage_mb(model.size_mb);
    let usage_label = model_usage_label(&model.tier).to_string();
    let fit_label = model_fit_label(&model, hardware.clone()).to_string();
    let languages_label = language_summary(&model.languages);
    let source_label = model_source_label(&model.source).to_string();
    let realtime_label = format!("Realtime x{:.2}", model.rtfx);

    match display_mode {
        ModelDisplayMode::List => view! {
            <article class="rounded-[1rem] border border-zinc-200 bg-white px-4 py-3 dark:border-zinc-900 dark:bg-[#141519]">
                <div class="flex flex-col gap-3 xl:flex-row xl:items-center xl:justify-between">
                    <div class="min-w-0 flex-1 space-y-2">
                        <div class="flex flex-wrap items-center gap-2">
                            <h3 class="text-sm font-semibold text-zinc-950 dark:text-zinc-100">
                                {model_name}
                            </h3>
                            <Show when=move || is_active>
                                <StatePill label="Active" tone="active" />
                            </Show>
                            <Show when=move || is_recommended>
                                <StatePill label="Recommended" tone="recommended" />
                            </Show>
                            <Show when=move || !is_active && is_ready>
                                <StatePill label="Installed" tone="ready" />
                            </Show>
                            <Show when=move || !is_ready>
                                <StatePill label="Not installed" tone="muted" />
                            </Show>
                        </div>

                        <p class="text-sm leading-5 text-zinc-600 dark:text-zinc-400">
                            {description}
                        </p>

                        <div class="flex flex-wrap items-center gap-2">
                            <MetaPill text=format!("Disk {size_label}") />
                            <MetaPill text=format!("Usage {usage_label}") />
                            <MetaPill text=format!("Languages {languages_label}") />
                            <MetaPill text=format!("Fit {fit_label}") />
                            <Show when=move || has_diarization>
                                <MetaPill text="Speaker labels".to_string() />
                            </Show>
                        </div>
                    </div>

                    <div class="flex shrink-0 items-center gap-2 xl:justify-end">
                        <ModelActionButton
                            active_model=active_model
                            is_active=is_active
                            is_ready=is_ready
                            model_id=model_id
                            selected_model=selected_model
                        />
                    </div>
                </div>
            </article>
        }
        .into_any(),
        ModelDisplayMode::Grid => view! {
            <article class="rounded-[1rem] border border-zinc-200 bg-white px-4 py-4 dark:border-zinc-900 dark:bg-[#141519]">
                <div class="flex items-start justify-between gap-3">
                    <div class="min-w-0 flex-1 space-y-2">
                        <div class="flex flex-wrap items-center gap-2">
                            <h3 class="text-sm font-semibold text-zinc-950 dark:text-zinc-100">
                                {model_name}
                            </h3>
                            <Show when=move || is_active>
                                <StatePill label="Active" tone="active" />
                            </Show>
                            <Show when=move || is_recommended>
                                <StatePill label="Recommended" tone="recommended" />
                            </Show>
                            <Show when=move || !is_active && is_ready>
                                <StatePill label="Installed" tone="ready" />
                            </Show>
                            <Show when=move || !is_ready>
                                <StatePill label="Not installed" tone="muted" />
                            </Show>
                        </div>

                        <p class="text-sm leading-5 text-zinc-600 dark:text-zinc-400">
                            {description}
                        </p>
                    </div>

                    <div class="shrink-0">
                        <ModelActionButton
                            active_model=active_model
                            is_active=is_active
                            is_ready=is_ready
                            model_id=model_id
                            selected_model=selected_model
                        />
                    </div>
                </div>

                <div class="mt-3 flex flex-wrap items-center gap-2">
                    <MetaPill text=format!("Disk {size_label}") />
                    <MetaPill text=format!("Usage {usage_label}") />
                    <MetaPill text=format!("Languages {languages_label}") />
                    <MetaPill text=format!("Fit {fit_label}") />
                    <Show when=move || has_diarization>
                        <MetaPill text="Speaker labels".to_string() />
                    </Show>
                </div>
            </article>
        }
        .into_any(),
        ModelDisplayMode::Cards => view! {
            <article class="rounded-[1rem] border border-zinc-200 bg-white px-5 py-4 dark:border-zinc-900 dark:bg-[#141519]">
                <div class="flex flex-wrap items-start justify-between gap-3">
                    <div class="min-w-0 flex-1 space-y-2">
                        <div class="flex flex-wrap items-center gap-2">
                            <h3 class="text-base font-semibold text-zinc-950 dark:text-zinc-100">
                                {model_name}
                            </h3>
                            <Show when=move || is_active>
                                <StatePill label="Active" tone="active" />
                            </Show>
                            <Show when=move || is_recommended>
                                <StatePill label="Recommended" tone="recommended" />
                            </Show>
                            <Show when=move || !is_active && is_ready>
                                <StatePill label="Installed" tone="ready" />
                            </Show>
                            <Show when=move || !is_ready>
                                <StatePill label="Not installed" tone="muted" />
                            </Show>
                        </div>

                        <p class="text-sm leading-6 text-zinc-600 dark:text-zinc-400">
                            {description}
                        </p>
                    </div>

                    <div class="shrink-0">
                        <ModelActionButton
                            active_model=active_model
                            is_active=is_active
                            is_ready=is_ready
                            model_id=model_id
                            selected_model=selected_model
                        />
                    </div>
                </div>

                <div class="mt-4 grid gap-2 sm:grid-cols-2">
                    <MetaPill text=format!("Disk footprint {size_label}") />
                    <MetaPill text=format!("Best use {usage_label}") />
                    <MetaPill text=format!("Languages {languages_label}") />
                    <MetaPill text=format!("Machine fit {fit_label}") />
                    <MetaPill text=format!("Source {source_label}") />
                    <MetaPill text=realtime_label />
                    <Show when=move || has_diarization>
                        <MetaPill text="Includes speaker separation".to_string() />
                    </Show>
                </div>
            </article>
        }
        .into_any(),
    }
}

#[component]
fn ModelActionButton(
    model_id: String,
    is_ready: bool,
    is_active: bool,
    selected_model: RwSignal<String>,
    active_model: RwSignal<String>,
) -> impl IntoView {
    if is_ready {
        view! {
            <button
                class=if is_active {
                    "inline-flex h-8 items-center rounded-lg border border-zinc-300 bg-zinc-950 px-3 text-sm font-medium text-white dark:border-zinc-700 dark:bg-zinc-100 dark:text-zinc-950"
                } else {
                    "inline-flex h-8 items-center rounded-lg border border-zinc-200 px-3 text-sm font-medium text-zinc-700 transition hover:bg-zinc-100 hover:text-zinc-950 dark:border-zinc-800 dark:text-zinc-300 dark:hover:bg-[#17181b] dark:hover:text-zinc-100"
                }
                on:click=move |_| {
                    selected_model.set(model_id.clone());
                    active_model.set(model_id.clone());
                }
                type="button"
            >
                {if is_active { "Currently active" } else { "Use model" }}
            </button>
        }
        .into_any()
    } else {
        view! {
            <button
                class="inline-flex h-8 items-center rounded-lg border border-zinc-200 px-3 text-sm font-medium text-zinc-400 dark:border-zinc-800 dark:text-zinc-500"
                disabled=true
                type="button"
            >
                "Install unavailable"
            </button>
        }
        .into_any()
    }
}

#[component]
fn MetricTile(label: &'static str, #[prop(into)] value: Signal<String>) -> impl IntoView {
    view! {
        <div class="rounded-xl border border-zinc-200 bg-white px-4 py-3 dark:border-zinc-900 dark:bg-[#101114]">
            <p class="text-[10px] font-medium uppercase tracking-[0.18em] text-zinc-500">{label}</p>
            <p class="mt-2 text-sm font-semibold leading-6 text-zinc-950 dark:text-zinc-100">
                {move || value.get()}
            </p>
        </div>
    }
}

#[component]
fn MetaPill(#[prop(into)] text: String) -> impl IntoView {
    view! {
        <span class="inline-flex items-center rounded-full border border-zinc-200 px-2.5 py-1 text-[11px] font-medium text-zinc-600 dark:border-zinc-800 dark:text-zinc-400">
            {text}
        </span>
    }
}

#[component]
fn StatePill(#[prop(into)] label: String, tone: &'static str) -> impl IntoView {
    let class = match tone {
        "active" => {
            "inline-flex items-center rounded-full border border-emerald-300 bg-emerald-50 px-2.5 py-1 text-[11px] font-medium text-emerald-700 dark:border-emerald-900/60 dark:bg-emerald-950/30 dark:text-emerald-200"
        }
        "recommended" => {
            "inline-flex items-center rounded-full border border-sky-300 bg-sky-50 px-2.5 py-1 text-[11px] font-medium text-sky-700 dark:border-sky-900/60 dark:bg-sky-950/30 dark:text-sky-200"
        }
        "ready" => {
            "inline-flex items-center rounded-full border border-zinc-300 bg-zinc-50 px-2.5 py-1 text-[11px] font-medium text-zinc-700 dark:border-zinc-700 dark:bg-zinc-900/60 dark:text-zinc-200"
        }
        _ => {
            "inline-flex items-center rounded-full border border-zinc-200 px-2.5 py-1 text-[11px] font-medium text-zinc-500 dark:border-zinc-800 dark:text-zinc-500"
        }
    };

    view! { <span class=class>{label}</span> }
}

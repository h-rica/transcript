use leptos::{prelude::*, task::spawn_local};

use crate::{
    components::{
        icons::{AppIcon, UiIcon},
        workspace::{WorkspaceRoute, WorkspaceShell},
    },
    features::workspace_data::fallback_models,
    state::app_state::{Settings, ThemePreference, use_app_shell_state, use_transcript_view_state},
};

fn persist_settings_snapshot(settings: Settings) {
    spawn_local(async move {
        let _ = crate::features::workspace_data::save_settings(settings).await;
    });
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    let shell = use_app_shell_state();
    let transcript_view = use_transcript_view_state();
    let active_section = RwSignal::new("transcription".to_string());
    let models = Signal::derive(move || {
        let loaded = shell.available_models.get();
        if loaded.is_empty() {
            fallback_models()
        } else {
            loaded
        }
    });

    view! {
        <WorkspaceShell route=WorkspaceRoute::Settings>
            <section class="flex min-h-full flex-1 overflow-hidden rounded-[1.35rem] border border-zinc-200 bg-white shadow-sm dark:border-white/5 dark:bg-[#30312d]">
                <div class="grid min-h-full flex-1 lg:grid-cols-[190px_minmax(0,1fr)]">
                    <nav class="border-b border-zinc-200 bg-zinc-50/90 px-3 py-4 dark:border-white/5 dark:bg-[#2a2b27] lg:border-b-0 lg:border-r">
                        <div class="space-y-1">
                            <SettingsNavButton
                                active_section=active_section
                                icon=AppIcon::Transcription
                                id="transcription"
                                label="Transcription"
                            />
                            <SettingsNavButton
                                active_section=active_section
                                icon=AppIcon::Export
                                id="export"
                                label="Export"
                            />
                            <SettingsNavButton
                                active_section=active_section
                                icon=AppIcon::Privacy
                                id="privacy"
                                label="Privacy"
                            />
                            <SettingsNavButton
                                active_section=active_section
                                icon=AppIcon::About
                                id="about"
                                label="About"
                            />
                        </div>
                    </nav>

                    <div class="flex min-h-full min-w-0 flex-col px-5 py-6 lg:px-8 lg:py-7">
                        {move || match active_section.get().as_str() {
                            "transcription" => {
                                let language_state = shell.clone();
                                let model_state = shell.clone();
                                let cpu_state = shell.clone();
                                let keep_memory_state = shell.clone();
                                let model_options = models
                                    .get()
                                    .into_iter()
                                    .map(|model| (model.id, model.name))
                                    .collect::<Vec<_>>();

                                view! {
                                    <div class="flex min-h-full flex-col gap-8">
                                        <SectionIntro
                                            title="Transcription"
                                            subtitle="Defaults applied to each new file. Override them from preview whenever a run needs different language or model settings."
                                        />

                                        <SettingsGroup heading="Language">
                                            <SettingsSelectRow
                                                description="Override per file in the preview screen."
                                                label="Default language"
                                                options=vec![
                                                    ("fr".to_string(), "French (FR)".to_string()),
                                                    ("en".to_string(), "English (EN)".to_string()),
                                                    ("auto".to_string(), "Auto detect".to_string()),
                                                ]
                                                value=Signal::derive(move || language_state.settings.get().default_language)
                                                on_select=Callback::new(move |value: String| {
                                                    language_state
                                                        .settings
                                                        .update(|settings| settings.default_language = value.clone());
                                                    language_state.selected_language.set(value);
                                                    persist_settings_snapshot(
                                                        language_state.settings.get_untracked(),
                                                    );
                                                })
                                            />
                                        </SettingsGroup>

                                        <SettingsGroup heading="Model">
                                            <SettingsSelectRow
                                                description="Set to Auto to keep the current hardware-driven default."
                                                label="Default model"
                                                options={
                                                    let mut options = vec![(
                                                        "auto".to_string(),
                                                        "Auto".to_string(),
                                                    )];
                                                    options.extend(model_options.clone());
                                                    options
                                                }
                                                value=Signal::derive(move || {
                                                    model_state
                                                        .settings
                                                        .get()
                                                        .default_model
                                                        .unwrap_or_else(|| "auto".into())
                                                })
                                                on_select=Callback::new(move |value: String| {
                                                    if value == "auto" {
                                                        model_state.settings.update(|settings| {
                                                            settings.default_model = None;
                                                        });
                                                    } else {
                                                        model_state.settings.update(|settings| {
                                                            settings.default_model = Some(value.clone());
                                                        });
                                                        model_state.selected_model.set(value.clone());
                                                        model_state.active_model.set(value);
                                                    }
                                                    persist_settings_snapshot(
                                                        model_state.settings.get_untracked(),
                                                    );
                                                })
                                            />
                                        </SettingsGroup>

                                        <SettingsGroup heading="Performance">
                                            <SettingsSelectRow
                                                description="Threads allocated to local inference."
                                                label="CPU threads"
                                                options=vec![
                                                    ("2".to_string(), "2 threads".to_string()),
                                                    ("4".to_string(), "4 threads".to_string()),
                                                    ("8".to_string(), "8 threads".to_string()),
                                                ]
                                                value=Signal::derive(move || cpu_state.settings.get().cpu_threads.to_string())
                                                on_select=Callback::new(move |value: String| {
                                                    if let Ok(threads) = value.parse::<u32>() {
                                                        cpu_state.settings.update(|settings| {
                                                            settings.cpu_threads = threads;
                                                        });
                                                        persist_settings_snapshot(
                                                            cpu_state.settings.get_untracked(),
                                                        );
                                                    }
                                                })
                                            />

                                            <SettingsToggleRow
                                                description="Faster consecutive transcriptions, with higher RAM usage."
                                                label="Keep model in memory"
                                                value=Signal::derive(move || {
                                                    keep_memory_state.settings.get().keep_model_in_memory
                                                })
                                                on_toggle=Callback::new(move |_| {
                                                    keep_memory_state.settings.update(|settings| {
                                                        settings.keep_model_in_memory =
                                                            !settings.keep_model_in_memory;
                                                    });
                                                    persist_settings_snapshot(
                                                        keep_memory_state.settings.get_untracked(),
                                                    );
                                                })
                                            />
                                        </SettingsGroup>
                                    </div>
                                }
                                .into_any()
                            }
                            "export" => {
                                let export_path_state = shell.clone();
                                let format_state = shell.clone();
                                let export_view_state = transcript_view.clone();
                                let timestamps_state = shell.clone();
                                let speaker_labels_state = shell.clone();

                                view! {
                                    <div class="flex min-h-full flex-col gap-8">
                                        <SectionIntro
                                            title="Export"
                                            subtitle="Choose the transcript format defaults the review screen should open with after each local transcription run."
                                        />

                                        <SettingsGroup heading="Destination">
                                            <SettingsInfoRow
                                                description="Current destination for local TXT and SRT files."
                                                label="Export folder"
                                                value=Signal::derive(move || {
                                                    let path = export_path_state.settings.get().export_path;
                                                    if path.is_empty() {
                                                        "No custom export path configured yet.".into()
                                                    } else {
                                                        path
                                                    }
                                                })
                                            />
                                        </SettingsGroup>

                                        <SettingsGroup heading="Format">
                                            <SettingsSelectRow
                                                description="The export menu opens with this format selected."
                                                label="Default format"
                                                options=vec![
                                                    ("txt".to_string(), "TXT".to_string()),
                                                    ("srt".to_string(), "SRT".to_string()),
                                                ]
                                                value=Signal::derive(move || {
                                                    format_state.settings.get().default_export_format
                                                })
                                                on_select=Callback::new(move |value: String| {
                                                    format_state.settings.update(|settings| {
                                                        settings.default_export_format = value.clone();
                                                    });
                                                    export_view_state.export_format.set(value);
                                                    persist_settings_snapshot(
                                                        format_state.settings.get_untracked(),
                                                    );
                                                })
                                            />

                                            <SettingsToggleRow
                                                description="Add timestamps automatically when the export format supports them."
                                                label="Include timestamps"
                                                value=Signal::derive(move || {
                                                    timestamps_state.settings.get().include_timestamps
                                                })
                                                on_toggle=Callback::new(move |_| {
                                                    timestamps_state.settings.update(|settings| {
                                                        settings.include_timestamps =
                                                            !settings.include_timestamps;
                                                    });
                                                    persist_settings_snapshot(
                                                        timestamps_state.settings.get_untracked(),
                                                    );
                                                })
                                            />

                                            <SettingsToggleRow
                                                description="Prefix speakers when diarization data exists in the transcript."
                                                label="Include speaker labels"
                                                value=Signal::derive(move || {
                                                    speaker_labels_state
                                                        .settings
                                                        .get()
                                                        .include_speaker_labels
                                                })
                                                on_toggle=Callback::new(move |_| {
                                                    speaker_labels_state.settings.update(|settings| {
                                                        settings.include_speaker_labels =
                                                            !settings.include_speaker_labels;
                                                    });
                                                    persist_settings_snapshot(
                                                        speaker_labels_state.settings.get_untracked(),
                                                    );
                                                })
                                            />
                                        </SettingsGroup>
                                    </div>
                                }
                                .into_any()
                            }
                            "privacy" => {
                                let telemetry_state = shell.clone();
                                let updates_state = shell.clone();

                                view! {
                                    <div class="flex min-h-full flex-col gap-8">
                                        <SectionIntro
                                            title="Privacy"
                                            subtitle="Transcript keeps inference local. These controls only affect optional diagnostics and desktop update checks."
                                        />

                                        <SettingsGroup heading="Preferences">
                                            <SettingsToggleRow
                                                description="Anonymous crash reporting for desktop debugging only."
                                                label="Telemetry"
                                                value=Signal::derive(move || {
                                                    telemetry_state.settings.get().telemetry
                                                })
                                                on_toggle=Callback::new(move |_| {
                                                    telemetry_state.settings.update(|settings| {
                                                        settings.telemetry = !settings.telemetry;
                                                    });
                                                    persist_settings_snapshot(
                                                        telemetry_state.settings.get_untracked(),
                                                    );
                                                })
                                            />

                                            <SettingsToggleRow
                                                description="Surface available releases in supported desktop builds."
                                                label="Check for updates"
                                                value=Signal::derive(move || {
                                                    updates_state.settings.get().check_for_updates
                                                })
                                                on_toggle=Callback::new(move |_| {
                                                    updates_state.settings.update(|settings| {
                                                        settings.check_for_updates =
                                                            !settings.check_for_updates;
                                                    });
                                                    persist_settings_snapshot(
                                                        updates_state.settings.get_untracked(),
                                                    );
                                                })
                                            />
                                        </SettingsGroup>
                                    </div>
                                }
                                .into_any()
                            }
                            _ => {
                                let theme_state = shell.clone();
                                let hardware_state = shell.clone();
                                let app_state = shell.clone();
                                let model_count = models;

                                view! {
                                    <div class="flex min-h-full flex-col gap-8">
                                        <SectionIntro
                                            title="About"
                                            subtitle="Operational metadata for support, debugging, and environment clarity."
                                        />

                                        <SettingsGroup heading="Appearance">
                                            <SettingsSelectRow
                                                description="Preferred app appearance across launches."
                                                label="Theme"
                                                options=vec![
                                                    ("auto".to_string(), "System".to_string()),
                                                    ("light".to_string(), "Light".to_string()),
                                                    ("dark".to_string(), "Dark".to_string()),
                                                ]
                                                value=Signal::derive(move || match theme_state.theme_preference.get() {
                                                    ThemePreference::Auto => "auto".to_string(),
                                                    ThemePreference::Light => "light".to_string(),
                                                    ThemePreference::Dark => "dark".to_string(),
                                                })
                                                on_select=Callback::new(move |value: String| {
                                                    let preference = match value.as_str() {
                                                        "light" => ThemePreference::Light,
                                                        "dark" => ThemePreference::Dark,
                                                        _ => ThemePreference::Auto,
                                                    };
                                                    theme_state.theme_preference.set(preference);
                                                    theme_state.settings.update(|settings| {
                                                        settings.theme_preference = preference;
                                                    });
                                                    persist_settings_snapshot(
                                                        theme_state.settings.get_untracked(),
                                                    );
                                                })
                                            />
                                        </SettingsGroup>

                                        <SettingsGroup heading="Environment">
                                            <SettingsInfoRow
                                                description="Current desktop package metadata."
                                                label="App"
                                                value=Signal::derive(move || {
                                                    let export_format =
                                                        app_state.settings.get().default_export_format;
                                                    format!(
                                                        "Transcript / offline-first desktop app / default {} export",
                                                        export_format.to_uppercase()
                                                    )
                                                })
                                            />

                                            <SettingsInfoRow
                                                description="Snapshot loaded when the app boots."
                                                label="Hardware"
                                                value=Signal::derive(move || {
                                                    hardware_state
                                                        .hardware_info
                                                        .get()
                                                        .map(|info| {
                                                            format!(
                                                                "{} GB RAM / {}",
                                                                info.ram_gb, info.cpu_name
                                                            )
                                                        })
                                                        .unwrap_or_else(|| {
                                                            "Hardware snapshot unavailable".into()
                                                        })
                                                })
                                            />

                                            <SettingsInfoRow
                                                description="Catalog entries currently visible to the UI."
                                                label="Models"
                                                value=Signal::derive(move || {
                                                    format!("{} entries loaded", model_count.get().len())
                                                })
                                            />
                                        </SettingsGroup>
                                    </div>
                                }
                                .into_any()
                            }
                        }}
                    </div>
                </div>
            </section>
        </WorkspaceShell>
    }
}

#[component]
fn SettingsNavButton(
    active_section: RwSignal<String>,
    id: &'static str,
    label: &'static str,
    icon: AppIcon,
) -> impl IntoView {
    view! {
        <button
            class=move || {
                if active_section.get() == id {
                    "flex w-full items-center gap-3 rounded-[1rem] bg-white px-3 py-2.5 text-left text-sm font-medium text-zinc-950 shadow-sm dark:bg-[#34362f] dark:text-zinc-50"
                } else {
                    "flex w-full items-center gap-3 rounded-[1rem] px-3 py-2.5 text-left text-sm text-zinc-600 transition hover:bg-white hover:text-zinc-950 dark:text-zinc-400 dark:hover:bg-[#34362f] dark:hover:text-zinc-100"
                }
            }
            on:click=move |_| active_section.set(id.into())
            type="button"
        >
            <span class="flex h-8 w-8 items-center justify-center rounded-[0.9rem] border border-zinc-200 bg-zinc-50 text-zinc-500 dark:border-white/5 dark:bg-[#242621] dark:text-zinc-300">
                <UiIcon class="h-4 w-4" icon_name=icon/>
            </span>
            <span>{label}</span>
        </button>
    }
}

#[component]
fn SectionIntro(title: &'static str, subtitle: &'static str) -> impl IntoView {
    view! {
        <div>
            <h1 class="text-[1.5rem] font-semibold tracking-tight text-zinc-950 dark:text-zinc-50">{title}</h1>
            <p class="mt-1 max-w-2xl text-sm leading-6 text-zinc-600 dark:text-zinc-400">{subtitle}</p>
        </div>
    }
}

#[component]
fn SettingsGroup(heading: &'static str, children: Children) -> impl IntoView {
    view! {
        <section>
            <p class="text-[11px] font-medium uppercase tracking-[0.18em] text-zinc-500 dark:text-zinc-500">{heading}</p>
            <div class="mt-4 px-1">
                {children()}
            </div>
        </section>
    }
}

#[component]
fn SettingsRow(
    label: &'static str,
    description: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-3 border-b border-zinc-200 py-5 last:border-b-0 dark:border-white/5 md:flex-row md:items-center md:justify-between md:gap-6">
            <div class="min-w-0 flex-1">
                <p class="text-sm font-medium text-zinc-950 dark:text-zinc-100">{label}</p>
                <p class="mt-1 text-sm leading-6 text-zinc-600 dark:text-zinc-400">{description}</p>
            </div>
            <div class="w-full md:w-auto md:min-w-[180px] md:max-w-[220px] md:flex-shrink-0 md:text-right">
                {children()}
            </div>
        </div>
    }
}

#[component]
fn SettingsSelectRow(
    label: &'static str,
    description: &'static str,
    options: Vec<(String, String)>,
    #[prop(into)] value: Signal<String>,
    on_select: Callback<String>,
) -> impl IntoView {
    view! {
        <SettingsRow label=label description=description>
            <select
                class="h-11 w-full rounded-[0.95rem] border border-zinc-200 bg-white px-4 text-sm text-zinc-950 outline-none transition focus:border-zinc-400 dark:border-white/10 dark:bg-[#242621] dark:text-zinc-100 dark:focus:border-zinc-500"
                on:change=move |event| on_select.run(event_target_value(&event))
                prop:value=move || value.get()
            >
                {options
                    .into_iter()
                    .map(|(option_value, option_label)| {
                        view! { <option value=option_value>{option_label}</option> }
                    })
                    .collect_view()}
            </select>
        </SettingsRow>
    }
}

#[component]
fn SettingsToggleRow(
    label: &'static str,
    description: &'static str,
    #[prop(into)] value: Signal<bool>,
    on_toggle: Callback<()>,
) -> impl IntoView {
    view! {
        <SettingsRow label=label description=description>
            <div class="flex w-full justify-start md:justify-end">
                <button
                    aria-pressed=move || value.get().to_string()
                    class=move || {
                        if value.get() {
                            "relative h-7 w-12 rounded-full bg-zinc-950 transition dark:bg-zinc-100"
                        } else {
                            "relative h-7 w-12 rounded-full bg-zinc-300 transition dark:bg-[#1f211d]"
                        }
                    }
                    on:click=move |_| on_toggle.run(())
                    type="button"
                >
                    <span
                        class=move || {
                            if value.get() {
                                "absolute left-[26px] top-1 block h-5 w-5 rounded-full bg-white dark:bg-[#242621]"
                            } else {
                                "absolute left-1 top-1 block h-5 w-5 rounded-full bg-white dark:bg-zinc-400"
                            }
                        }
                    ></span>
                </button>
            </div>
        </SettingsRow>
    }
}

#[component]
fn SettingsInfoRow(
    label: &'static str,
    description: &'static str,
    #[prop(into)] value: Signal<String>,
) -> impl IntoView {
    view! {
        <SettingsRow label=label description=description>
            <p class="text-sm leading-6 text-zinc-700 dark:text-zinc-300">{move || value.get()}</p>
        </SettingsRow>
    }
}

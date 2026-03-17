use leptos::{prelude::*, task::spawn_local};

use crate::{
    components::workspace::{WorkspaceHeader, WorkspaceRoute, WorkspaceShell},
    features::workspace_data::{fallback_models, save_settings},
    state::app_state::{Settings, ThemePreference, use_app_shell_state, use_transcript_view_state},
};

fn persist_settings_snapshot(settings: Settings) {
    spawn_local(async move {
        let _ = save_settings(settings).await;
    });
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    let shell = use_app_shell_state();
    let transcript_view = use_transcript_view_state();
    let active_section = RwSignal::new("transcription".to_string());

    let models = move || {
        let loaded = shell.available_models.get();
        if loaded.is_empty() {
            fallback_models()
        } else {
            loaded
        }
    };

    view! {
        <WorkspaceShell route=WorkspaceRoute::Settings>
            <WorkspaceHeader
                title="Settings"
                subtitle="Control local defaults, export rules, privacy choices, and desktop support metadata."
            >
                <span class="inline-flex items-center rounded-full border border-zinc-200 px-2.5 py-1 text-[11px] font-medium text-zinc-700 dark:border-zinc-800 dark:text-zinc-300">
                    "Stored locally"
                </span>
            </WorkspaceHeader>

            <div class="grid gap-5 lg:grid-cols-[160px_minmax(0,1fr)]">
                <nav class="rounded-[1.2rem] border border-zinc-200 bg-white p-2 dark:border-zinc-900 dark:bg-[#141519]">
                    {[
                        ("transcription", "Transcription"),
                        ("export", "Export"),
                        ("privacy", "Privacy"),
                        ("about", "About"),
                    ]
                        .into_iter()
                        .map(|(id, label)| {
                            view! {
                                <button
                                    class=move || {
                                        if active_section.get() == id {
                                            "mb-1 flex w-full items-center gap-3 rounded-lg bg-zinc-100 px-3 py-2 text-left text-sm font-medium text-zinc-950 dark:bg-[#101114] dark:text-zinc-50"
                                        } else {
                                            "mb-1 flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left text-sm text-zinc-600 transition hover:bg-zinc-100 hover:text-zinc-950 dark:text-zinc-400 dark:hover:bg-[#101114] dark:hover:text-zinc-100"
                                        }
                                    }
                                    on:click=move |_| active_section.set(id.into())
                                    type="button"
                                >
                                    <span class="inline-flex h-6 w-6 items-center justify-center rounded-md border border-zinc-200 bg-zinc-50 text-[10px] font-semibold text-zinc-500 dark:border-zinc-800 dark:bg-[#0f1012] dark:text-zinc-400">
                                        {nav_abbrev(label)}
                                    </span>
                                    {label}
                                </button>
                            }
                        })
                        .collect_view()}
                </nav>

                <section class="rounded-[1.2rem] border border-zinc-200 bg-white px-5 py-5 dark:border-zinc-900 dark:bg-[#141519]">
                    {move || match active_section.get().as_str() {
                        "transcription" => {
                            let model_catalog = models();
                            let language_state = shell.clone();
                            let model_state = shell.clone();
                            let cpu_state = shell.clone();
                            let keep_memory_state = shell.clone();

                            view! {
                                <div class="space-y-6">
                                    <SectionIntro
                                        title="Transcription"
                                        subtitle="Defaults applied to each new file before manual overrides."
                                    />

                                    <SettingBlock label="Language" description="Default language applied when preview opens.">
                                        <SegmentedRow
                                            options=vec![
                                                ("fr".to_string(), "French".to_string()),
                                                ("en".to_string(), "English".to_string()),
                                                ("auto".to_string(), "Auto".to_string()),
                                            ]
                                            value=Signal::derive(move || language_state.settings.get().default_language)
                                            on_select=Callback::new(move |value: String| {
                                                language_state.settings.update(|settings| settings.default_language = value.clone());
                                                language_state.selected_language.set(value);
                                                persist_settings_snapshot(language_state.settings.get_untracked());
                                            })
                                        />
                                    </SettingBlock>

                                    <SettingBlock label="Model" description="Preferred local profile before choosing a different one in preview.">
                                        <SegmentedRow
                                            options={model_catalog
                                                .into_iter()
                                                .map(|model| (model.id, model.name))
                                                .collect::<Vec<_>>()}
                                            value=Signal::derive(move || {
                                                model_state
                                                    .settings
                                                    .get()
                                                    .default_model
                                                    .unwrap_or_else(|| "whisper-tiny".into())
                                            })
                                            on_select=Callback::new(move |value: String| {
                                                model_state.settings.update(|settings| settings.default_model = Some(value.clone()));
                                                model_state.selected_model.set(value.clone());
                                                persist_settings_snapshot(model_state.settings.get_untracked());
                                            })
                                        />
                                    </SettingBlock>

                                    <SettingBlock label="CPU threads" description="Threads allocated to local inference by default.">
                                        <SegmentedRow
                                            options=vec![
                                                ("2".to_string(), "2".to_string()),
                                                ("4".to_string(), "4".to_string()),
                                                ("8".to_string(), "8".to_string()),
                                            ]
                                            value=Signal::derive(move || cpu_state.settings.get().cpu_threads.to_string())
                                            on_select=Callback::new(move |value: String| {
                                                if let Ok(threads) = value.parse::<u32>() {
                                                    cpu_state.settings.update(|settings| settings.cpu_threads = threads);
                                                    persist_settings_snapshot(cpu_state.settings.get_untracked());
                                                }
                                            })
                                        />
                                    </SettingBlock>

                                    <ToggleRow
                                        label="Keep model in memory"
                                        description="Faster consecutive runs, with higher RAM usage."
                                        value=Signal::derive(move || keep_memory_state.settings.get().keep_model_in_memory)
                                        on_toggle=Callback::new(move |_| {
                                            keep_memory_state.settings.update(|settings| {
                                                settings.keep_model_in_memory = !settings.keep_model_in_memory;
                                            });
                                            persist_settings_snapshot(keep_memory_state.settings.get_untracked());
                                        })
                                    />
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
                                <div class="space-y-6">
                                    <SectionIntro
                                        title="Export"
                                        subtitle="Where transcript files go and which formatting options are preselected."
                                    />

                                    <InfoRow
                                        label="Export folder"
                                        description="Current destination for local TXT and SRT files."
                                        value=Signal::derive(move || {
                                            let path = export_path_state.settings.get().export_path;
                                            if path.is_empty() {
                                                "No custom export path configured yet.".into()
                                            } else {
                                                path
                                            }
                                        })
                                    />

                                    <SettingBlock label="Default format" description="The export overlay opens with this format selected.">
                                        <SegmentedRow
                                            options=vec![
                                                ("txt".to_string(), "TXT".to_string()),
                                                ("srt".to_string(), "SRT".to_string()),
                                            ]
                                            value=Signal::derive(move || format_state.settings.get().default_export_format)
                                            on_select=Callback::new(move |value: String| {
                                                format_state.settings.update(|settings| settings.default_export_format = value.clone());
                                                export_view_state.export_format.set(value);
                                                persist_settings_snapshot(format_state.settings.get_untracked());
                                            })
                                        />
                                    </SettingBlock>

                                    <ToggleRow
                                        label="Include timestamps"
                                        description="Add timestamps automatically to export output when supported."
                                        value=Signal::derive(move || timestamps_state.settings.get().include_timestamps)
                                        on_toggle=Callback::new(move |_| {
                                            timestamps_state.settings.update(|settings| {
                                                settings.include_timestamps = !settings.include_timestamps;
                                            });
                                            persist_settings_snapshot(timestamps_state.settings.get_untracked());
                                        })
                                    />

                                    <ToggleRow
                                        label="Include speaker labels"
                                        description="Prefix speakers when diarization data exists for the transcript."
                                        value=Signal::derive(move || speaker_labels_state.settings.get().include_speaker_labels)
                                        on_toggle=Callback::new(move |_| {
                                            speaker_labels_state.settings.update(|settings| {
                                                settings.include_speaker_labels = !settings.include_speaker_labels;
                                            });
                                            persist_settings_snapshot(speaker_labels_state.settings.get_untracked());
                                        })
                                    />
                                </div>
                            }
                            .into_any()
                        }
                        "privacy" => {
                            let telemetry_state = shell.clone();
                            let updates_state = shell.clone();

                            view! {
                                <div class="space-y-6">
                                    <SectionIntro
                                        title="Privacy"
                                        subtitle="All processing is local. No audio leaves your device during transcription."
                                    />

                                    <ToggleRow
                                        label="Telemetry"
                                        description="Anonymous crash reporting for desktop debugging only."
                                        value=Signal::derive(move || telemetry_state.settings.get().telemetry)
                                        on_toggle=Callback::new(move |_| {
                                            telemetry_state.settings.update(|settings| settings.telemetry = !settings.telemetry);
                                            persist_settings_snapshot(telemetry_state.settings.get_untracked());
                                        })
                                    />

                                    <ToggleRow
                                        label="Check for updates"
                                        description="Surface available releases in supported desktop builds."
                                        value=Signal::derive(move || updates_state.settings.get().check_for_updates)
                                        on_toggle=Callback::new(move |_| {
                                            updates_state.settings.update(|settings| settings.check_for_updates = !settings.check_for_updates);
                                            persist_settings_snapshot(updates_state.settings.get_untracked());
                                        })
                                    />
                                </div>
                            }
                            .into_any()
                        }
                        _ => {
                            let about_hardware_state = shell.clone();
                            let theme_state = shell.clone();
                            let model_count = models().len();
                            view! {
                                <div class="space-y-6">
                                    <SectionIntro
                                        title="About"
                                        subtitle="Operational metadata for support, debugging, and environment clarity."
                                    />

                                    <SettingBlock label="Theme" description="Preferred app appearance across launches.">
                                        <SegmentedRow
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
                                                theme_state.settings.update(|settings| settings.theme_preference = preference);
                                                persist_settings_snapshot(theme_state.settings.get_untracked());
                                            })
                                        />
                                    </SettingBlock>

                                    <InfoRow
                                        label="App"
                                        description="Current desktop package metadata."
                                        value=Signal::derive(|| String::from("Transcript / Leptos + Tauri / offline-first transcription"))
                                    />

                                    <InfoRow
                                        label="Hardware"
                                        description="Snapshot loaded when the app boots."
                                        value=Signal::derive(move || {
                                            about_hardware_state
                                                .hardware_info
                                                .get()
                                                .map(|info| format!("{} GB RAM / {}", info.ram_gb, info.cpu_name))
                                                .unwrap_or_else(|| "Hardware snapshot unavailable".into())
                                        })
                                    />

                                    <InfoRow
                                        label="Models"
                                        description="Catalog entries currently visible to the UI."
                                        value=Signal::derive(move || format!("{} entries loaded", model_count))
                                    />
                                </div>
                            }
                            .into_any()
                        }
                    }}
                </section>
            </div>
        </WorkspaceShell>
    }
}

#[component]
fn SectionIntro(title: &'static str, subtitle: &'static str) -> impl IntoView {
    view! {
        <div>
            <h2 class="text-lg font-semibold text-zinc-950 dark:text-zinc-100">{title}</h2>
            <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-500">{subtitle}</p>
        </div>
    }
}

#[component]
fn SettingBlock(
    label: &'static str,
    description: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="space-y-3 border-b border-zinc-200 pb-4 last:border-b-0 last:pb-0 dark:border-zinc-900">
            <div>
                <p class="text-sm font-medium text-zinc-950 dark:text-zinc-100">{label}</p>
                <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-500">{description}</p>
            </div>
            {children()}
        </div>
    }
}

#[component]
fn InfoRow(
    label: &'static str,
    description: &'static str,
    #[prop(into)] value: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="border-b border-zinc-200 pb-4 last:border-b-0 last:pb-0 dark:border-zinc-900">
            <p class="text-sm font-medium text-zinc-950 dark:text-zinc-100">{label}</p>
            <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-500">{description}</p>
            <p class="mt-3 text-sm text-zinc-700 dark:text-zinc-300">{move || value.get()}</p>
        </div>
    }
}

#[component]
fn SegmentedRow(
    options: Vec<(String, String)>,
    #[prop(into)] value: Signal<String>,
    on_select: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2">
            {options
                .into_iter()
                .map(|(option_value, option_label)| {
                    let callback = on_select;
                    let active_value = option_value.clone();
                    view! {
                        <button
                            class=move || {
                                if value.get() == active_value {
                                    "rounded-lg border border-zinc-300 bg-zinc-950 px-3 py-2 text-sm font-medium text-white dark:border-zinc-700 dark:bg-zinc-100 dark:text-zinc-950"
                                } else {
                                    "rounded-lg border border-zinc-200 bg-zinc-100 px-3 py-2 text-sm font-medium text-zinc-700 transition hover:bg-zinc-200 dark:border-zinc-800 dark:bg-[#101114] dark:text-zinc-300 dark:hover:bg-[#17181b]"
                                }
                            }
                            on:click={
                                let next = option_value.clone();
                                move |_| callback.run(next.clone())
                            }
                            type="button"
                        >
                            {option_label}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn ToggleRow(
    label: &'static str,
    description: &'static str,
    #[prop(into)] value: Signal<bool>,
    on_toggle: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between gap-4 border-b border-zinc-200 pb-4 last:border-b-0 last:pb-0 dark:border-zinc-900">
            <div>
                <p class="text-sm font-medium text-zinc-950 dark:text-zinc-100">{label}</p>
                <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-500">{description}</p>
            </div>
            <button
                class=move || {
                    if value.get() {
                        "relative h-5 w-9 rounded-full bg-zinc-950 dark:bg-zinc-100"
                    } else {
                        "relative h-5 w-9 rounded-full bg-zinc-300 dark:bg-zinc-800"
                    }
                }
                on:click=move |_| on_toggle.run(())
                type="button"
            >
                <span
                    class=move || {
                        if value.get() {
                            "absolute left-[18px] top-0.5 block h-4 w-4 rounded-full bg-white dark:bg-zinc-950"
                        } else {
                            "absolute left-0.5 top-0.5 block h-4 w-4 rounded-full bg-white dark:bg-zinc-100"
                        }
                    }
                ></span>
            </button>
        </div>
    }
}

fn nav_abbrev(label: &str) -> &'static str {
    match label {
        "Transcription" => "TR",
        "Export" => "EX",
        "Privacy" => "PR",
        _ => "AB",
    }
}

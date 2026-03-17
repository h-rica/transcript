use leptos::{prelude::*, task::spawn_local};

use crate::{
    components::workspace::{WorkspaceHeader, WorkspaceRoute, WorkspaceShell},
    features::workspace_data::{fallback_models, save_settings},
    state::app_state::{Settings, use_app_shell_state, use_transcript_view_state},
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
                subtitle="Control default transcription behavior, export preferences, privacy controls, and local app metadata."
            >
                <span class="inline-flex items-center rounded-full border border-zinc-200 px-2.5 py-1 text-[11px] font-medium text-zinc-700 dark:border-zinc-800 dark:text-zinc-300">
                    "Stored locally"
                </span>
            </WorkspaceHeader>

            <div class="grid gap-6 lg:grid-cols-[13rem_minmax(0,1fr)]">
                <div class="rounded-[1.25rem] border border-zinc-200 bg-white p-3 dark:border-zinc-900 dark:bg-[#141519]">
                    {[("transcription", "Transcription"), ("export", "Export"), ("privacy", "Privacy"), ("about", "About")]
                        .into_iter()
                        .map(|(id, label)| {
                            view! {
                                <button
                                    class=move || {
                                        if active_section.get() == id {
                                            "w-full rounded-lg border border-zinc-300 bg-zinc-950 px-3 py-2 text-left text-sm font-medium text-white dark:border-zinc-700 dark:bg-zinc-100 dark:text-zinc-950"
                                        } else {
                                            "w-full rounded-lg border border-transparent px-3 py-2 text-left text-sm text-zinc-600 transition hover:bg-zinc-100 hover:text-zinc-950 dark:text-zinc-400 dark:hover:bg-[#17181b] dark:hover:text-zinc-100"
                                        }
                                    }
                                    on:click=move |_| active_section.set(id.into())
                                    type="button"
                                >
                                    {label}
                                </button>
                            }
                        })
                        .collect_view()}
                </div>

                <div class="rounded-[1.5rem] border border-zinc-200 bg-white px-5 py-5 dark:border-zinc-900 dark:bg-[#141519]">
                    {move || match active_section.get().as_str() {
                        "transcription" => {
                            let model_catalog = models();
                            let language_state = shell.clone();
                            let model_state = shell.clone();
                            let keep_memory_state = shell.clone();

                            view! {
                                <div class="space-y-6">
                                    <div>
                                        <h2 class="text-lg font-semibold text-zinc-950 dark:text-zinc-100">"Transcription"</h2>
                                        <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-500">"Defaults applied to each newly selected file."</p>
                                    </div>

                                    <SettingBlock label="Default language" description="Used when the preview screen opens before a manual override.">
                                        <div class="flex flex-wrap gap-2">
                                            {[("fr", "French"), ("en", "English"), ("auto", "Auto")]
                                                .into_iter()
                                                .map(|(value, label)| {
                                                    let shell_for_action = language_state.clone();
                                                    view! {
                                                        <button
                                                            class=move || {
                                                                if shell_for_action.settings.get().default_language == value {
                                                                    "rounded-lg border border-zinc-300 bg-zinc-950 px-3 py-2 text-sm font-medium text-white dark:border-zinc-700 dark:bg-zinc-100 dark:text-zinc-950"
                                                                } else {
                                                                    "rounded-lg border border-zinc-200 bg-zinc-100 px-3 py-2 text-sm font-medium text-zinc-700 transition hover:bg-zinc-200 dark:border-zinc-800 dark:bg-[#101114] dark:text-zinc-300 dark:hover:bg-[#17181b]"
                                                                }
                                                            }
                                                            on:click=move |_| {
                                                                shell_for_action.settings.update(|settings| settings.default_language = value.into());
                                                                shell_for_action.selected_language.set(value.into());
                                                                persist_settings_snapshot(shell_for_action.settings.get_untracked());
                                                            }
                                                            type="button"
                                                        >
                                                            {label}
                                                        </button>
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                    </SettingBlock>

                                    <SettingBlock label="Default model" description="Suggested profile before the user makes a different choice.">
                                        <div class="flex flex-wrap gap-2">
                                            {model_catalog.into_iter().map(|model| {
                                                let shell_for_class = model_state.clone();
                                                let shell_for_action = model_state.clone();
                                                let model_id_for_class = model.id.clone();
                                                let model_id_for_action = model.id.clone();
                                                let model_name = model.name.clone();
                                                view! {
                                                    <button
                                                        class=move || {
                                                            if shell_for_class.settings.get().default_model.as_deref() == Some(model_id_for_class.as_str()) {
                                                                "rounded-lg border border-zinc-300 bg-zinc-950 px-3 py-2 text-sm font-medium text-white dark:border-zinc-700 dark:bg-zinc-100 dark:text-zinc-950"
                                                            } else {
                                                                "rounded-lg border border-zinc-200 bg-zinc-100 px-3 py-2 text-sm font-medium text-zinc-700 transition hover:bg-zinc-200 dark:border-zinc-800 dark:bg-[#101114] dark:text-zinc-300 dark:hover:bg-[#17181b]"
                                                            }
                                                        }
                                                        on:click={
                                                            let model_id = model_id_for_action.clone();
                                                            move |_| {
                                                                shell_for_action.settings.update(|settings| settings.default_model = Some(model_id.clone()));
                                                                shell_for_action.selected_model.set(model_id.clone());
                                                                persist_settings_snapshot(shell_for_action.settings.get_untracked());
                                                            }
                                                        }
                                                        type="button"
                                                    >
                                                        {model_name}
                                                    </button>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </SettingBlock>

                                    <ToggleRow
                                        label="Keep model in memory"
                                        description="Faster back-to-back runs, at the cost of higher RAM use."
                                        value=Signal::derive(move || keep_memory_state.settings.get().keep_model_in_memory)
                                        on_toggle=Callback::new(move |_| {
                                            keep_memory_state.settings.update(|settings| {
                                                settings.keep_model_in_memory = !settings.keep_model_in_memory;
                                            });
                                            persist_settings_snapshot(keep_memory_state.settings.get_untracked());
                                        })
                                    />
                                </div>
                            }.into_any()
                        }
                        "export" => {
                            let export_path_state = shell.clone();
                            let format_state = shell.clone();
                            let export_view_state = transcript_view.clone();

                            view! {
                                <div class="space-y-6">
                                    <div>
                                        <h2 class="text-lg font-semibold text-zinc-950 dark:text-zinc-100">"Export"</h2>
                                        <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-500">"Output location and default formatting rules."</p>
                                    </div>

                                    <SettingBlock label="Export location" description="Current local destination for transcript exports.">
                                        <p class="text-sm text-zinc-700 dark:text-zinc-300">
                                            {move || {
                                                let path = export_path_state.settings.get().export_path;
                                                if path.is_empty() {
                                                    "No custom export path configured yet.".into()
                                                } else {
                                                    path
                                                }
                                            }}
                                        </p>
                                    </SettingBlock>

                                    <SettingBlock label="Default format" description="The transcript screen opens with this export format selected.">
                                        <div class="flex flex-wrap gap-2">
                                            {[("txt", "TXT"), ("srt", "SRT")]
                                                .into_iter()
                                                .map(|(value, label)| {
                                                    let shell_for_action = format_state.clone();
                                                    let transcript_for_action = export_view_state.clone();
                                                    view! {
                                                        <button
                                                            class=move || {
                                                                if shell_for_action.settings.get().default_export_format == value {
                                                                    "rounded-lg border border-zinc-300 bg-zinc-950 px-3 py-2 text-sm font-medium text-white dark:border-zinc-700 dark:bg-zinc-100 dark:text-zinc-950"
                                                                } else {
                                                                    "rounded-lg border border-zinc-200 bg-zinc-100 px-3 py-2 text-sm font-medium text-zinc-700 transition hover:bg-zinc-200 dark:border-zinc-800 dark:bg-[#101114] dark:text-zinc-300 dark:hover:bg-[#17181b]"
                                                                }
                                                            }
                                                            on:click=move |_| {
                                                                shell_for_action.settings.update(|settings| settings.default_export_format = value.into());
                                                                transcript_for_action.export_format.set(value.into());
                                                                persist_settings_snapshot(shell_for_action.settings.get_untracked());
                                                            }
                                                            type="button"
                                                        >
                                                            {label}
                                                        </button>
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                    </SettingBlock>
                                </div>
                            }.into_any()
                        }
                        "privacy" => {
                            let telemetry_state = shell.clone();
                            let updates_state = shell.clone();

                            view! {
                                <div class="space-y-6">
                                    <div>
                                        <h2 class="text-lg font-semibold text-zinc-950 dark:text-zinc-100">"Privacy"</h2>
                                        <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-500">"No audio leaves your device during local transcription."</p>
                                    </div>

                                    <ToggleRow
                                        label="Telemetry"
                                        description="Anonymous crash reporting for desktop debugging."
                                        value=Signal::derive(move || telemetry_state.settings.get().telemetry)
                                        on_toggle=Callback::new(move |_| {
                                            telemetry_state.settings.update(|settings| settings.telemetry = !settings.telemetry);
                                            persist_settings_snapshot(telemetry_state.settings.get_untracked());
                                        })
                                    />

                                    <ToggleRow
                                        label="Check for updates"
                                        description="Surface available releases in future desktop builds."
                                        value=Signal::derive(move || updates_state.settings.get().check_for_updates)
                                        on_toggle=Callback::new(move |_| {
                                            updates_state.settings.update(|settings| settings.check_for_updates = !settings.check_for_updates);
                                            persist_settings_snapshot(updates_state.settings.get_untracked());
                                        })
                                    />
                                </div>
                            }.into_any()
                        }
                        _ => {
                            let model_count = models().len();
                            let about_hardware_state = shell.clone();
                            view! {
                                <div class="space-y-6">
                                    <div>
                                        <h2 class="text-lg font-semibold text-zinc-950 dark:text-zinc-100">"About"</h2>
                                        <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-500">"Operational metadata for local support and debugging."</p>
                                    </div>

                                    <SettingBlock label="App" description="Current desktop package metadata.">
                                        <p class="text-sm text-zinc-700 dark:text-zinc-300">"Transcript / Leptos + Tauri / offline-first transcription"</p>
                                    </SettingBlock>

                                    <SettingBlock label="Hardware" description="Snapshot loaded at startup.">
                                        <p class="text-sm text-zinc-700 dark:text-zinc-300">
                                            {move || about_hardware_state.hardware_info.get().map(|info| format!("{} GB RAM / {}", info.ram_gb, info.cpu_name)).unwrap_or_else(|| "Hardware snapshot unavailable".into())}
                                        </p>
                                    </SettingBlock>

                                    <SettingBlock label="Models" description="Catalog entries currently visible to the UI.">
                                        <p class="text-sm text-zinc-700 dark:text-zinc-300">{format!("{} entries loaded", model_count)}</p>
                                    </SettingBlock>
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
            </div>
        </WorkspaceShell>
    }
}

#[component]
fn SettingBlock(
    label: &'static str,
    description: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="space-y-3 border-b border-zinc-200 pb-5 last:border-b-0 last:pb-0 dark:border-zinc-900">
            <div>
                <p class="text-sm font-medium text-zinc-950 dark:text-zinc-100">{label}</p>
                <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-500">{description}</p>
            </div>
            {children()}
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
        <div class="flex items-center justify-between gap-4 border-b border-zinc-200 pb-5 last:border-b-0 last:pb-0 dark:border-zinc-900">
            <div>
                <p class="text-sm font-medium text-zinc-950 dark:text-zinc-100">{label}</p>
                <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-500">{description}</p>
            </div>
            <button
                class=move || {
                    if value.get() {
                        "relative h-6 w-11 rounded-full bg-zinc-950 dark:bg-zinc-100"
                    } else {
                        "relative h-6 w-11 rounded-full bg-zinc-300 dark:bg-zinc-800"
                    }
                }
                on:click=move |_| on_toggle.run(())
                type="button"
            >
                <span
                    class=move || {
                        if value.get() {
                            "absolute left-[24px] top-1 block h-4 w-4 rounded-full bg-white dark:bg-zinc-950"
                        } else {
                            "absolute left-1 top-1 block h-4 w-4 rounded-full bg-white dark:bg-zinc-100"
                        }
                    }
                ></span>
            </button>
        </div>
    }
}

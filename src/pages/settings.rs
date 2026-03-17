use leptos::{prelude::*, task::spawn_local};
use singlestage::Badge;

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
                subtitle="Control default transcription behavior, export preferences, and local privacy options."
            >
                <Badge variant="outline">"Stored locally"</Badge>
            </WorkspaceHeader>

            <div class="grid gap-6 lg:grid-cols-[13rem_1fr]">
                <div class="space-y-2 rounded-[1.5rem] border border-zinc-800 bg-[#191919] p-3">
                    {[("transcription", "Transcription"), ("export", "Export"), ("privacy", "Privacy"), ("about", "About")]
                        .into_iter()
                        .map(|(id, label)| {
                            view! {
                                <button
                                    class=move || {
                                        if active_section.get() == id {
                                            "w-full rounded-xl border border-zinc-600 bg-zinc-900 px-3 py-2 text-left text-sm font-medium text-zinc-50"
                                        } else {
                                            "w-full rounded-xl border border-transparent px-3 py-2 text-left text-sm text-zinc-400 transition hover:border-zinc-800 hover:bg-zinc-900/60 hover:text-zinc-50"
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

                <div class="rounded-[1.5rem] border border-zinc-800 bg-[#191919] p-5 text-zinc-50">
                    {move || match active_section.get().as_str() {
                        "transcription" => {
                            let model_catalog = models();
                            let language_class_state = shell.clone();
                            let language_click_state = shell.clone();
                            let model_class_state = shell.clone();
                            let model_click_state = shell.clone();
                            let keep_memory_value_state = shell.clone();
                            let keep_memory_toggle_state = shell.clone();

                            view! {
                                <div class="space-y-6">
                                    <div>
                                        <h2 class="text-xl font-semibold">"Transcription"</h2>
                                        <p class="mt-1 text-sm text-zinc-400">"Defaults applied to each newly selected file."</p>
                                    </div>

                                    <SettingBlock label="Language" description="Override per file in the preview screen.">
                                        <div class="flex flex-wrap gap-2">
                                            {[("fr", "French"), ("en", "English"), ("auto", "Auto")]
                                                .into_iter()
                                                .map(|(value, label)| {
                                                    let shell_for_class = language_class_state.clone();
                                                    let shell_for_click = language_click_state.clone();
                                                    view! {
                                                        <button
                                                            class=move || {
                                                                if shell_for_class.settings.get().default_language == value {
                                                                    "rounded-xl border border-zinc-500 bg-zinc-900 px-3 py-2 text-sm text-zinc-50"
                                                                } else {
                                                                    "rounded-xl border border-zinc-700 px-3 py-2 text-sm text-zinc-300 transition hover:border-zinc-600 hover:text-zinc-50"
                                                                }
                                                            }
                                                            on:click=move |_| {
                                                                shell_for_click.settings.update(|settings| settings.default_language = value.into());
                                                                shell_for_click.selected_language.set(value.into());
                                                                persist_settings_snapshot(shell_for_click.settings.get_untracked());
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

                                    <SettingBlock label="Default model" description="Use this when the preview screen opens before the user chooses a different profile.">
                                        <div class="flex flex-wrap gap-2">
                                            {model_catalog.into_iter().map(|model| {
                                                let model_class_id = model.id.clone();
                                                let model_click_id = model.id.clone();
                                                let model_label = model.name.clone();
                                                let shell_for_class = model_class_state.clone();
                                                let shell_for_click = model_click_state.clone();
                                                view! {
                                                    <button
                                                        class=move || {
                                                            if shell_for_class.settings.get().default_model.as_deref() == Some(model_class_id.as_str()) {
                                                                "rounded-xl border border-zinc-500 bg-zinc-900 px-3 py-2 text-sm text-zinc-50"
                                                            } else {
                                                                "rounded-xl border border-zinc-700 px-3 py-2 text-sm text-zinc-300 transition hover:border-zinc-600 hover:text-zinc-50"
                                                            }
                                                        }
                                                        on:click=move |_| {
                                                            shell_for_click.settings.update(|settings| settings.default_model = Some(model_click_id.clone()));
                                                            shell_for_click.selected_model.set(model_click_id.clone());
                                                            persist_settings_snapshot(shell_for_click.settings.get_untracked());
                                                        }
                                                        type="button"
                                                    >
                                                        {model_label}
                                                    </button>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </SettingBlock>

                                    <SettingToggle
                                        label="Keep model in memory"
                                        description="Faster back-to-back runs, higher RAM usage."
                                        value=Signal::derive(move || keep_memory_value_state.settings.get().keep_model_in_memory)
                                        on_toggle=Callback::new(move |_| {
                                            keep_memory_toggle_state.settings.update(|settings| {
                                                settings.keep_model_in_memory = !settings.keep_model_in_memory;
                                            });
                                            persist_settings_snapshot(keep_memory_toggle_state.settings.get_untracked());
                                        })
                                    />
                                </div>
                            }.into_any()
                        }
                        "export" => {
                            let export_path_state = shell.clone();
                            let export_format_class_state = shell.clone();
                            let export_format_click_state = shell.clone();
                            let export_format_view_state = transcript_view.clone();

                            view! {
                                <div class="space-y-6">
                                    <div>
                                        <h2 class="text-xl font-semibold">"Export"</h2>
                                        <p class="mt-1 text-sm text-zinc-400">"Where transcripts go after the review screen."</p>
                                    </div>

                                    <SettingBlock label="Export path" description="Current local destination for transcript exports.">
                                        <p class="text-sm text-zinc-300">
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

                                    <SettingBlock label="Default format" description="The review screen will open with this export format selected.">
                                        <div class="flex flex-wrap gap-2">
                                            {[("txt", "TXT"), ("srt", "SRT")]
                                                .into_iter()
                                                .map(|(value, label)| {
                                                    let shell_for_class = export_format_class_state.clone();
                                                    let shell_for_click = export_format_click_state.clone();
                                                    let transcript_for_click = export_format_view_state.clone();
                                                    view! {
                                                        <button
                                                            class=move || {
                                                                if shell_for_class.settings.get().default_export_format == value {
                                                                    "rounded-xl border border-zinc-500 bg-zinc-900 px-3 py-2 text-sm text-zinc-50"
                                                                } else {
                                                                    "rounded-xl border border-zinc-700 px-3 py-2 text-sm text-zinc-300 transition hover:border-zinc-600 hover:text-zinc-50"
                                                                }
                                                            }
                                                            on:click=move |_| {
                                                                shell_for_click.settings.update(|settings| settings.default_export_format = value.into());
                                                                transcript_for_click.export_format.set(value.into());
                                                                persist_settings_snapshot(shell_for_click.settings.get_untracked());
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
                            let telemetry_value_state = shell.clone();
                            let telemetry_toggle_state = shell.clone();
                            let updates_value_state = shell.clone();
                            let updates_toggle_state = shell.clone();

                            view! {
                                <div class="space-y-6">
                                    <div>
                                        <h2 class="text-xl font-semibold">"Privacy"</h2>
                                        <p class="mt-1 text-sm text-zinc-400">"No audio leaves your device during local transcription."</p>
                                    </div>

                                    <SettingToggle
                                        label="Telemetry"
                                        description="Anonymous crash reporting for desktop debugging."
                                        value=Signal::derive(move || telemetry_value_state.settings.get().telemetry)
                                        on_toggle=Callback::new(move |_| {
                                            telemetry_toggle_state.settings.update(|settings| settings.telemetry = !settings.telemetry);
                                            persist_settings_snapshot(telemetry_toggle_state.settings.get_untracked());
                                        })
                                    />

                                    <SettingToggle
                                        label="Check for updates"
                                        description="Allow the app to surface available releases in future builds."
                                        value=Signal::derive(move || updates_value_state.settings.get().check_for_updates)
                                        on_toggle=Callback::new(move |_| {
                                            updates_toggle_state.settings.update(|settings| settings.check_for_updates = !settings.check_for_updates);
                                            persist_settings_snapshot(updates_toggle_state.settings.get_untracked());
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
                                        <h2 class="text-xl font-semibold">"About"</h2>
                                        <p class="mt-1 text-sm text-zinc-400">"Operational metadata for local support and debugging."</p>
                                    </div>

                                    <SettingBlock label="App" description="Current desktop package metadata.">
                                        <p class="text-sm text-zinc-300">"Transcript · Leptos + Tauri · offline-first audio transcription"</p>
                                    </SettingBlock>
                                    <SettingBlock label="Hardware" description="Snapshot loaded at startup.">
                                        <p class="text-sm text-zinc-300">
                                            {move || about_hardware_state.hardware_info.get().map(|info| format!("{} GB RAM · {}", info.ram_gb, info.cpu_name)).unwrap_or_else(|| "Hardware snapshot unavailable".into())}
                                        </p>
                                    </SettingBlock>
                                    <SettingBlock label="Models" description="Catalog entries currently visible to the UI.">
                                        <p class="text-sm text-zinc-300">{format!("{} entries loaded", model_count)}</p>
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
        <div class="space-y-3 border-b border-zinc-800 pb-5 last:border-b-0 last:pb-0">
            <div>
                <p class="text-sm font-semibold text-zinc-50">{label}</p>
                <p class="mt-1 text-sm text-zinc-400">{description}</p>
            </div>
            {children()}
        </div>
    }
}

#[component]
fn SettingToggle(
    label: &'static str,
    description: &'static str,
    #[prop(into)] value: Signal<bool>,
    on_toggle: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between gap-4 border-b border-zinc-800 pb-5 last:border-b-0 last:pb-0">
            <div>
                <p class="text-sm font-semibold text-zinc-50">{label}</p>
                <p class="mt-1 text-sm text-zinc-400">{description}</p>
            </div>
            <button
                class=move || {
                    if value.get() {
                        "rounded-full border border-zinc-500 bg-zinc-900 px-3 py-2 text-sm text-zinc-50"
                    } else {
                        "rounded-full border border-zinc-700 px-3 py-2 text-sm text-zinc-300 transition hover:border-zinc-600 hover:text-zinc-50"
                    }
                }
                on:click=move |_| on_toggle.run(())
                type="button"
            >
                {move || if value.get() { "On" } else { "Off" }}
            </button>
        </div>
    }
}

use leptos::{prelude::*, task::spawn_local};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use singlestage::{Mode, ThemeProvider, ThemeProviderContext};

use crate::features::workspace_data::{load_hardware_info, load_settings, load_workspace_models};
use crate::pages::{
    file_preview::FilePreviewPage, home::HomePage, model_manager::ModelManagerPage,
    settings::SettingsPage, transcript_view::TranscriptViewPage, transcription::TranscriptionPage,
};
use crate::state::app_state::{
    ThemePreference, provide_app_state, use_app_shell_state, use_transcript_view_state,
};

#[component]
pub fn App() -> impl IntoView {
    provide_app_state();

    view! {
        <ThemeProvider mode="auto">
            <AppShell/>
        </ThemeProvider>
    }
}

#[component]
fn AppShell() -> impl IntoView {
    let shell = use_app_shell_state();
    let transcript_view = use_transcript_view_state();
    let theme = expect_context::<ThemeProviderContext>();
    let bootstrapped = RwSignal::new(false);

    Effect::new(move |_| {
        theme.mode.set(match shell.theme_preference.get() {
            ThemePreference::Auto => Mode::Auto,
            ThemePreference::Dark => Mode::Dark,
            ThemePreference::Light => Mode::Light,
        });
    });

    Effect::new(move |_| {
        if bootstrapped.get() {
            return;
        }
        bootstrapped.set(true);

        let shell_for_settings = shell.clone();
        let transcript_view = transcript_view.clone();
        spawn_local(async move {
            if let Ok(settings) = load_settings().await {
                shell_for_settings
                    .theme_preference
                    .set(settings.theme_preference);
                if let Some(default_model) = settings.default_model.clone() {
                    shell_for_settings.selected_model.set(default_model.clone());
                    shell_for_settings.active_model.set(default_model);
                }
                shell_for_settings
                    .selected_language
                    .set(settings.default_language.clone());
                transcript_view
                    .export_format
                    .set(settings.default_export_format.clone());
                shell_for_settings.settings.set(settings);
            }
        });

        let shell_for_hardware = shell.clone();
        spawn_local(async move {
            if let Ok(info) = load_hardware_info().await {
                shell_for_hardware.hardware_info.set(Some(info));
            }
        });

        let shell_for_models = shell.clone();
        spawn_local(async move {
            if let Ok(models) = load_workspace_models().await {
                let selected_id = shell_for_models.selected_model.get_untracked();
                let has_selected = models.iter().any(|model| model.id == selected_id);
                if !has_selected && let Some(first) = models.first() {
                    shell_for_models.selected_model.set(first.id.clone());
                    shell_for_models.active_model.set(first.id.clone());
                }
                shell_for_models.available_models.set(models);
            }
        });
    });

    view! {
        <Router>
            <Routes fallback=|| view! { <p class="p-8 text-zinc-400">"Page not found"</p> }>
                <Route path=path!("/")               view=HomePage/>
                <Route path=path!("/preview")        view=FilePreviewPage/>
                <Route path=path!("/transcription")  view=TranscriptionPage/>
                <Route path=path!("/transcript/:id") view=TranscriptViewPage/>
                <Route path=path!("/models")         view=ModelManagerPage/>
                <Route path=path!("/settings")       view=SettingsPage/>
            </Routes>
        </Router>
    }
}

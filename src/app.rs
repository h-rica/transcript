use leptos::{prelude::*, task::spawn_local};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use singlestage::{Mode, ThemeProvider, ThemeProviderContext};

use crate::pages::{
    file_preview::FilePreviewPage, home::HomePage, model_manager::ModelManagerPage,
    settings::SettingsPage, transcript_view::TranscriptViewPage, transcription::TranscriptionPage,
};
use crate::state::app_state::{
    HardwareInfo, Settings, ThemePreference, provide_app_state, use_app_shell_state,
    use_transcript_view_state,
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

        if !tauri_sys::core::is_tauri() {
            return;
        }

        let shell_for_settings = shell.clone();
        let transcript_view = transcript_view.clone();
        spawn_local(async move {
            if let Ok(settings) =
                tauri_sys::core::invoke_result::<Settings, String>("get_settings", &()).await
            {
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
            let fallback = HardwareInfo {
                ram_gb: 8,
                cpu_name: "Unknown CPU".into(),
                gpu_vram_gb: None,
                tier: "balanced".into(),
            };

            let info =
                tauri_sys::core::invoke_result::<HardwareInfo, String>("get_hardware_info", &())
                    .await
                    .unwrap_or(fallback);
            shell_for_hardware.hardware_info.set(Some(info));
        });
    });

    view! {
        <Router>
            <div class="min-h-screen bg-slate-50 text-slate-950 transition-colors dark:bg-slate-950 dark:text-slate-50">
                <Routes fallback=|| view! { <p class="p-8 text-slate-500">"Page not found"</p> }>
                    <Route path=path!("/")               view=HomePage/>
                    <Route path=path!("/preview")        view=FilePreviewPage/>
                    <Route path=path!("/transcription")  view=TranscriptionPage/>
                    <Route path=path!("/transcript/:id") view=TranscriptViewPage/>
                    <Route path=path!("/models")         view=ModelManagerPage/>
                    <Route path=path!("/settings")       view=SettingsPage/>
                </Routes>
            </div>
        </Router>
    }
}

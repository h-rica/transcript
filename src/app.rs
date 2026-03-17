use leptos::{prelude::*, task::spawn_local};
use leptos_darkmode::Darkmode;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::pages::{
    file_preview::FilePreviewPage, home::HomePage, model_manager::ModelManagerPage,
    settings::SettingsPage, transcript_view::TranscriptViewPage, transcription::TranscriptionPage,
};
use crate::state::app_state::{HardwareInfo, Settings, provide_app_state, use_app_state};

#[component]
pub fn App() -> impl IntoView {
    provide_app_state();
    let state = use_app_state();
    let darkmode = Darkmode::init();

    Effect::new(move |_| {
        if !tauri_sys::core::is_tauri() {
            return;
        }

        let state = state.clone();
        spawn_local(async move {
            if let Ok(settings) =
                tauri_sys::core::invoke_result::<Settings, String>("get_settings", &()).await
            {
                if let Some(default_model) = settings.default_model.clone() {
                    state.selected_model.set(default_model.clone());
                    state.active_model.set(default_model);
                }
                state.selected_language.set(settings.default_language.clone());
                state.settings.set(settings);
            }
        });

        let state = state.clone();
        spawn_local(async move {
            let fallback = HardwareInfo {
                ram_gb: 8,
                cpu_name: "Unknown CPU".into(),
                gpu_vram_gb: None,
                tier: "balanced".into(),
            };

            let info = tauri_sys::core::invoke_result::<HardwareInfo, String>(
                "get_hardware_info",
                &(),
            )
            .await
            .unwrap_or(fallback);
            state.hardware_info.set(Some(info));
        });
    });

    view! {
        <Router>
            <div
                class=move || format!(
                    "{} min-h-screen bg-slate-50 text-slate-950 transition-colors dark:bg-slate-950 dark:text-slate-100",
                    if darkmode.is_dark() { "dark" } else { "" }
                )
            >
                <div class="flex h-screen font-sans">
                    <Routes fallback=|| view! { <p class="p-8 text-slate-500 dark:text-slate-400">"Page not found"</p> }>
                        <Route path=path!("/")               view=HomePage/>
                        <Route path=path!("/preview")        view=FilePreviewPage/>
                        <Route path=path!("/transcription")  view=TranscriptionPage/>
                        <Route path=path!("/transcript/:id") view=TranscriptViewPage/>
                        <Route path=path!("/models")         view=ModelManagerPage/>
                        <Route path=path!("/settings")       view=SettingsPage/>
                    </Routes>
                </div>
            </div>
        </Router>
    }
}

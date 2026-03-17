use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::pages::{
    file_preview::FilePreviewPage, home::HomePage, model_manager::ModelManagerPage,
    settings::SettingsPage, transcript_view::TranscriptViewPage, transcription::TranscriptionPage,
};
use crate::state::app_state::provide_app_state;

#[component]
pub fn App() -> impl IntoView {
    provide_app_state();

    view! {
        <Router>
            <div class="flex h-screen bg-gray-50 font-sans">
                <Routes fallback=|| view! { <p class="p-8 text-gray-500">"Page not found"</p> }>
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

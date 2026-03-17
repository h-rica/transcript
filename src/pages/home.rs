use leptos::prelude::*;
use leptos_use::use_drop_zone;

use crate::state::app_state::use_app_state;

#[component]
pub fn HomePage() -> impl IntoView {
    let state = use_app_state();
    let navigate = leptos_router::hooks::use_navigate();

    // Drop zone
    let drop_ref = NodeRef::new();
    let UseDropZoneReturn { is_over_drop_zone, files, .. } = use_drop_zone(drop_ref);

    // When a file is dropped — navigate to preview
    Effect::new(move |_| {
        let dropped = files.get();
        if let Some(file) = dropped.first() {
            let name = file.name();
            state.selected_file.set(Some(name));
            navigate("/preview", Default::default());
        }
    });

    // Recent files (hardcoded for now — Phase 1 Week 8 will load from disk)
    let recents = vec![
        ("interview_2026_03.mp3", "24:38", "FR", "VibeVoice INT8"),
        ("meeting_recording.wav", "1:02:14", "EN", "Whisper Large"),
        ("podcast_ep42.m4a", "48:22", "FR", "VibeVoice INT8"),
    ];

    view! {
        <div class="flex h-screen w-full">
            <Sidebar active="home"/>

            <div class="flex-1 flex flex-col overflow-hidden">
                // Topbar
                <div class="h-11 border-b border-gray-200 flex items-center px-5 justify-between flex-shrink-0">
                    <span class="text-sm font-medium text-gray-900">"Transcribe"</span>
                    <input
                        type="search"
                        placeholder="Search transcripts…"
                        class="h-7 text-xs border border-gray-200 rounded-md px-3 bg-gray-50 text-gray-500 w-48 outline-none"
                    />
                </div>

                <div class="flex-1 p-5 overflow-auto">
                    // Drop zone
                    <div
                        node_ref=drop_ref
                        class=move || format!(
                            "border-2 rounded-xl h-32 flex flex-col items-center justify-center gap-2 mb-5 transition-all cursor-pointer {}",
                            if is_over_drop_zone.get() {
                                "border-gray-900 bg-gray-100 border-solid"
                            } else {
                                "border-gray-200 border-dashed hover:border-gray-400"
                            }
                        )
                    >
                        <div class="w-9 h-9 bg-gray-100 rounded-lg flex items-center justify-center">
                            <svg width="18" height="18" viewBox="0 0 18 18" fill="none">
                                <path d="M9 11V3M9 3L6 6M9 3L12 6" stroke="#6B7280" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/>
                                <path d="M2 12v2a1 1 0 001 1h12a1 1 0 001-1v-2" stroke="#6B7280" stroke-width="1.3" stroke-linecap="round"/>
                            </svg>
                        </div>
                        <span class="text-sm font-medium text-gray-700">"Drop audio file here"</span>
                        <span class="text-xs text-gray-400">"MP3 · WAV · M4A"</span>
                    </div>

                    // Recent
                    <div class="text-xs font-medium text-gray-500 mb-2">"Recent"</div>
                    <div class="flex flex-col gap-1">
                        {recents.into_iter().map(|(name, dur, lang, model)| {
                            view! {
                                <div class="flex items-center gap-3 px-3 py-2.5 rounded-lg hover:bg-gray-100 cursor-pointer transition-colors">
                                    <div class="w-8 h-8 rounded-lg bg-gray-100 flex-shrink-0"></div>
                                    <div class="flex-1 min-w-0">
                                        <div class="text-sm font-medium text-gray-900 truncate">{name}</div>
                                        <div class="text-xs text-gray-400">{format!("{dur} · {lang} · {model}")}</div>
                                    </div>
                                    <span class="text-gray-300 text-sm">"›"</span>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>
        </div>
    }
}
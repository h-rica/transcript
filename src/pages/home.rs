use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::{drop_zone::DropZone, sidebar::Sidebar};
use crate::state::app_state::{SelectedFile, use_app_state};

#[component]
pub fn HomePage() -> impl IntoView {
    let state = use_app_state();
    let navigate = use_navigate();
    let search = RwSignal::new(String::new());

    let recents = vec![
        (
            "interview_2026_03.mp3",
            "24:38",
            "FR",
            "2 speakers",
            "VibeVoice INT8",
        ),
        (
            "meeting_recording.wav",
            "1:02:14",
            "EN",
            "1 speaker",
            "Whisper Tiny",
        ),
        (
            "podcast_ep42.m4a",
            "48:22",
            "FR",
            "2 speakers",
            "VibeVoice INT8",
        ),
    ];

    let on_file = Callback::new(move |selected: SelectedFile| {
        state.selected_file.set(Some(selected));
        navigate("/preview", Default::default());
    });

    view! {
        <div class="flex h-screen w-full">
            <Sidebar/>

            <div class="flex min-w-0 flex-1 flex-col overflow-hidden">
                <div class="flex h-16 items-center justify-between border-b border-slate-200 px-8 dark:border-slate-800">
                    <div>
                        <h1 class="text-lg font-semibold">"Sprint 4 flow"</h1>
                        <p class="text-sm text-slate-500 dark:text-slate-400">
                            "Drop a file, confirm the model, then watch segments stream in."
                        </p>
                    </div>
                    <input
                        type="search"
                        placeholder="Search recents"
                        prop:value=move || search.get()
                        on:input=move |event| search.set(event_target_value(&event))
                        class="h-10 w-56 rounded-2xl border border-slate-200 bg-white px-4 text-sm outline-none transition focus:border-slate-400 dark:border-slate-700 dark:bg-slate-900"
                    />
                </div>

                <div class="flex-1 overflow-auto px-8 py-6">
                    <DropZone on_file=on_file/>

                    <div class="mt-8">
                        <div class="mb-3 flex items-center justify-between">
                            <h2 class="text-xs font-semibold uppercase tracking-[0.24em] text-slate-400 dark:text-slate-500">
                                "Recent transcripts"
                            </h2>
                            <span class="text-xs text-slate-400 dark:text-slate-500">
                                "Local index placeholder for Phase 1"
                            </span>
                        </div>

                        <div class="grid gap-3">
                            {move || {
                                let query = search.get().to_lowercase();
                                recents
                                    .iter()
                                    .filter(|(name, ..)| query.is_empty() || name.to_lowercase().contains(&query))
                                    .map(|(name, duration, language, speakers, model)| {
                                        view! {
                                            <button
                                                class="flex items-center gap-4 rounded-[24px] border border-slate-200 bg-white px-5 py-4 text-left transition hover:border-slate-300 hover:shadow-sm dark:border-slate-800 dark:bg-slate-900 dark:hover:border-slate-700"
                                                type="button"
                                            >
                                                <div class="flex h-12 w-12 items-center justify-center rounded-2xl bg-slate-100 text-xs font-semibold text-slate-700 dark:bg-slate-800 dark:text-slate-300">
                                                    "TXT"
                                                </div>
                                                <div class="min-w-0 flex-1">
                                                    <div class="truncate text-sm font-semibold text-slate-900 dark:text-slate-100">
                                                        {(*name).to_string()}
                                                    </div>
                                                    <div class="mt-1 text-xs text-slate-500 dark:text-slate-400">
                                                        {format!("{duration} | {language} | {speakers} | {model}")}
                                                    </div>
                                                </div>
                                                <div class="rounded-full bg-slate-100 px-3 py-1 text-xs font-medium text-slate-500 dark:bg-slate-800 dark:text-slate-400">
                                                    "Open soon"
                                                </div>
                                            </button>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

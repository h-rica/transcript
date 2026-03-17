use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_navigate};

use crate::{
    components::{
        drop_zone::DropZone,
        workspace::{WorkspaceHeader, WorkspaceRoute, WorkspaceShell},
    },
    state::app_state::{SelectedFile, use_app_shell_state},
};

const RECENTS: [(&str, &str, &str, &str, &str, &str); 4] = [
    (
        "interview_2026_03.mp3",
        "24:38",
        "FR",
        "2 speakers",
        "VibeVoice",
        "3h ago",
    ),
    (
        "meeting_recording.wav",
        "1:02:14",
        "EN",
        "4 speakers",
        "Whisper",
        "Yesterday",
    ),
    (
        "podcast_ep42.m4a",
        "48:22",
        "FR",
        "2 speakers",
        "VibeVoice",
        "2 days ago",
    ),
    (
        "conf_keynote.mp3",
        "58:10",
        "EN",
        "1 speaker",
        "Whisper",
        "1 week ago",
    ),
];

#[component]
pub fn IntakeScreen() -> impl IntoView {
    let shell = use_app_shell_state();
    let navigate = use_navigate();
    let search = RwSignal::new(String::new());

    let on_file = Callback::new(move |selected: SelectedFile| {
        shell.selected_file.set(Some(selected));
        navigate("/preview", Default::default());
    });

    view! {
        <WorkspaceShell route=WorkspaceRoute::Home>
            <WorkspaceHeader title="Transcribe">
                <div class="flex w-full max-w-sm items-center gap-2 rounded-lg border border-zinc-200 bg-zinc-100 px-3 py-2 text-sm text-zinc-500 dark:border-zinc-800 dark:bg-[#121316] dark:text-zinc-500">
                    <svg class="h-4 w-4 shrink-0" fill="none" viewBox="0 0 12 12">
                        <circle cx="5" cy="5" r="3.5" stroke="currentColor" stroke-width="1.2"/>
                        <path d="M8 8L10.5 10.5" stroke="currentColor" stroke-linecap="round" stroke-width="1.2"/>
                    </svg>
                    <input
                        class="w-full border-0 bg-transparent text-sm text-zinc-900 outline-none placeholder:text-zinc-400 dark:text-zinc-100 dark:placeholder:text-zinc-500"
                        on:input=move |ev| search.set(event_target_value(&ev))
                        placeholder="Search transcripts..."
                        prop:value=move || search.get()
                        type="search"
                    />
                </div>
            </WorkspaceHeader>

            <DropZone on_file=on_file/>

            <section class="overflow-hidden rounded-[1.15rem] border border-zinc-200 bg-white dark:border-zinc-900 dark:bg-[#141519]">
                <div class="flex items-center justify-between border-b border-zinc-200 px-5 py-3 dark:border-zinc-900">
                    <div>
                        <p class="text-xs font-medium uppercase tracking-[0.18em] text-zinc-500">"Recent"</p>
                        <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-400">"Open a previous transcript from local history."</p>
                    </div>
                    <A
                        attr:class="text-sm text-zinc-500 transition hover:text-zinc-950 dark:hover:text-zinc-100"
                        href="/transcript/current"
                    >
                        "View all"
                    </A>
                </div>

                <div class="divide-y divide-zinc-200 dark:divide-zinc-900">
                    {move || {
                        let query = search.get().to_lowercase();
                        RECENTS
                            .iter()
                            .filter(|(name, ..)| query.is_empty() || name.to_lowercase().contains(&query))
                            .map(|(name, duration, language, speakers, model, age)| {
                                let href = "/transcript/current";
                                let badge_class = if *model == "VibeVoice" {
                                    "inline-flex items-center rounded-md bg-sky-100 px-2 py-0.5 text-[10px] font-medium text-sky-700 dark:bg-sky-500/10 dark:text-sky-200"
                                } else {
                                    "inline-flex items-center rounded-md bg-zinc-100 px-2 py-0.5 text-[10px] font-medium text-zinc-600 dark:bg-zinc-800 dark:text-zinc-300"
                                };
                                view! {
                                    <A
                                        attr:class="group flex items-center gap-4 px-5 py-4 transition hover:bg-zinc-100 dark:hover:bg-[#181a1f]"
                                        href=href
                                    >
                                        <div class="flex h-10 w-10 items-center justify-center rounded-lg border border-zinc-200 bg-zinc-100 text-[11px] font-semibold text-zinc-500 dark:border-zinc-800 dark:bg-[#101114] dark:text-zinc-400">
                                            "TR"
                                        </div>

                                        <div class="min-w-0 flex-1">
                                            <div class="flex min-w-0 items-center gap-2">
                                                <p class="truncate text-sm font-medium text-zinc-950 dark:text-zinc-100">{(*name).to_string()}</p>
                                                <span class=badge_class>{(*model).to_string()}</span>
                                            </div>
                                            <p class="mt-1 text-xs text-zinc-500 dark:text-zinc-500">
                                                {format!("{} / {} / {}", duration, language, speakers)}
                                            </p>
                                        </div>

                                        <div class="hidden text-xs text-zinc-500 sm:block dark:text-zinc-500">{(*age).to_string()}</div>
                                        <div class="text-sm text-zinc-400 transition group-hover:text-zinc-700 dark:text-zinc-700 dark:group-hover:text-zinc-300">">"</div>
                                    </A>
                                }
                            })
                            .collect_view()
                    }}
                </div>
            </section>
        </WorkspaceShell>
    }
}

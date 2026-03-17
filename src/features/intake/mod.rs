use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use singlestage::{Badge, Card, CardContent, Field, Input, Label};

use crate::{
    components::{
        drop_zone::DropZone,
        workspace::{WorkspaceHeader, WorkspaceRoute, WorkspaceShell},
    },
    state::app_state::{SelectedFile, use_app_shell_state},
};

const RECENTS: [(&str, &str, &str, &str, &str); 4] = [
    (
        "interview_2026_03.mp3",
        "24:38",
        "French",
        "2 speakers",
        "VibeVoice",
    ),
    (
        "meeting_recording.wav",
        "1:02:14",
        "English",
        "4 speakers",
        "Whisper",
    ),
    (
        "podcast_ep42.m4a",
        "48:22",
        "French",
        "2 speakers",
        "VibeVoice",
    ),
    (
        "conf_keynote.mp3",
        "58:10",
        "English",
        "1 speaker",
        "Whisper",
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
            <WorkspaceHeader
                title="Transcribe"
                subtitle="Drop a file to start a local transcription run or reopen a recent transcript from this device."
            >
                <Field class="min-w-72">
                    <Label>"Search transcripts"</Label>
                    <Input input_type="search" placeholder="Search transcripts..." value=search/>
                </Field>
            </WorkspaceHeader>

            <DropZone on_file=on_file/>

            <Card class="border-zinc-800 bg-[#191919] text-zinc-50">
                <CardContent class="p-0">
                    <div class="flex items-center justify-between border-b border-zinc-800 px-5 py-4">
                        <div>
                            <p class="text-sm font-semibold text-zinc-50">"Recent"</p>
                            <p class="text-sm text-zinc-400">"Recent local transcripts available on this machine."</p>
                        </div>
                        <a class="text-sm text-zinc-400 transition hover:text-zinc-100" href="/transcript/current">
                            "View all"
                        </a>
                    </div>

                    <div class="divide-y divide-zinc-800">
                        {move || {
                            let query = search.get().to_lowercase();
                            RECENTS
                                .iter()
                                .filter(|(name, ..)| query.is_empty() || name.to_lowercase().contains(&query))
                                .map(|(name, duration, language, speakers, model)| {
                                    let href = "/transcript/current";
                                    view! {
                                        <a class="flex flex-wrap items-center gap-4 px-5 py-4 transition hover:bg-zinc-900/70" href=href>
                                            <div class="flex h-10 w-10 items-center justify-center rounded-xl border border-zinc-700 bg-zinc-900 text-xs font-semibold text-zinc-300">
                                                "TR"
                                            </div>
                                            <div class="min-w-0 flex-1 space-y-1">
                                                <p class="truncate text-sm font-semibold text-zinc-50">{(*name).to_string()}</p>
                                                <p class="text-sm text-zinc-400">{format!("{} · {} · {}", duration, language, speakers)}</p>
                                            </div>
                                            <Badge variant="outline">{(*model).to_string()}</Badge>
                                        </a>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                </CardContent>
            </Card>
        </WorkspaceShell>
    }
}

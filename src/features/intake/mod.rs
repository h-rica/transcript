use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use singlestage::{
    Badge, Card, CardContent, CardDescription, CardHeader, CardTitle, Field, Input, Label,
};

use crate::{
    components::{app_ui::AppPageHeader, drop_zone::DropZone, sidebar::Sidebar},
    state::app_state::{SelectedFile, use_app_shell_state},
};

const RECENTS: [(&str, &str, &str, &str, &str); 3] = [
    (
        "interview_2026_03.mp3",
        "24:38",
        "FR",
        "2 speakers",
        "Whisper Tiny",
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
        "Whisper Medium",
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
        <div class="flex h-screen w-full bg-slate-50 dark:bg-slate-950">
            <Sidebar/>

            <main class="min-w-0 flex-1 overflow-auto px-6 py-6 lg:px-8">
                <div class="mx-auto flex max-w-6xl flex-col gap-6">
                    <AppPageHeader
                        eyebrow="Intake"
                        title="Drop a file and prepare a local transcript"
                        description="The active flow now uses the Single Stage component layer for the search field, surfaces, and metadata cards."
                    >
                        <Field class="min-w-72">
                            <Label>"Search recents"</Label>
                            <Input input_type="search" placeholder="Search recent transcripts" value=search/>
                        </Field>
                    </AppPageHeader>

                    <DropZone on_file=on_file/>

                    <Card>
                        <CardHeader>
                            <Badge variant="secondary">"Recent transcripts"</Badge>
                            <CardTitle>"Local transcript history"</CardTitle>
                            <CardDescription>
                                "The library view is still placeholder data in Phase 1, but the screen now uses shared UI primitives instead of page-local button styling."
                            </CardDescription>
                        </CardHeader>
                        <CardContent class="grid gap-3">
                            {move || {
                                let query = search.get().to_lowercase();
                                RECENTS
                                    .iter()
                                    .filter(|(name, ..)| query.is_empty() || name.to_lowercase().contains(&query))
                                    .map(|(name, duration, language, speakers, model)| {
                                        view! {
                                            <Card>
                                                <CardContent class="flex flex-wrap items-center justify-between gap-4 p-4">
                                                    <div class="space-y-1">
                                                        <p class="text-sm font-semibold text-slate-950 dark:text-slate-50">
                                                            {(*name).to_string()}
                                                        </p>
                                                        <p class="text-sm text-slate-600 dark:text-slate-300">
                                                            {format!("{} | {} | {}", duration, speakers, model)}
                                                        </p>
                                                    </div>
                                                    <div class="flex flex-wrap items-center gap-2">
                                                        <Badge variant="outline">{(*language).to_string()}</Badge>
                                                        <Badge variant="secondary">"Open soon"</Badge>
                                                    </div>
                                                </CardContent>
                                            </Card>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </CardContent>
                    </Card>
                </div>
            </main>
        </div>
    }
}

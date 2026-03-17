use leptos::prelude::*;

use crate::state::app_state::{ThemePreference, use_app_shell_state};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkspaceRoute {
    Home,
    Preview,
    Transcription,
    Transcript,
    Models,
    Settings,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorkspaceSection {
    Home,
    Models,
    Settings,
}

impl WorkspaceRoute {
    fn section(self) -> WorkspaceSection {
        match self {
            Self::Home | Self::Preview | Self::Transcription | Self::Transcript => {
                WorkspaceSection::Home
            }
            Self::Models => WorkspaceSection::Models,
            Self::Settings => WorkspaceSection::Settings,
        }
    }

    fn window_title(self) -> &'static str {
        match self {
            Self::Home => "Transcribe",
            Self::Preview => "File Preview",
            Self::Transcription => "Transcribing",
            Self::Transcript => "Transcript",
            Self::Models => "Models",
            Self::Settings => "Settings",
        }
    }
}

#[component]
pub fn WorkspaceShell(route: WorkspaceRoute, children: Children) -> impl IntoView {
    let shell = use_app_shell_state();
    let active = route.section();

    let nav_button = move |section: WorkspaceSection,
                           href: &'static str,
                           label: &'static str,
                           glyph: &'static str| {
        let is_active = active == section;
        let classes = if is_active {
            "flex h-11 w-11 items-center justify-center rounded-2xl border border-zinc-600 bg-zinc-800 text-zinc-50 shadow-sm"
        } else {
            "flex h-11 w-11 items-center justify-center rounded-2xl border border-transparent text-zinc-400 transition hover:border-zinc-700 hover:bg-zinc-900 hover:text-zinc-100"
        };
        let sr_label = format!("Open {label}");
        view! {
            <a class=classes href=href title=label>
                <span class="text-sm font-semibold tracking-wide">{glyph}</span>
                <span class="sr-only">{sr_label}</span>
            </a>
        }
    };

    view! {
        <div class="min-h-screen bg-[#111111] text-zinc-50">
            <div class="flex min-h-screen">
                <aside class="hidden w-16 flex-col items-center border-r border-zinc-800 bg-[#0a0a0a] py-4 lg:flex">
                    <div class="flex h-11 w-11 items-center justify-center rounded-2xl border border-zinc-700 bg-zinc-900 text-sm font-semibold tracking-[0.24em] text-zinc-100">
                        "TR"
                    </div>

                    <nav class="mt-8 flex flex-col gap-3">
                        {nav_button(WorkspaceSection::Home, "/", "Home", "H")}
                        {nav_button(WorkspaceSection::Models, "/models", "Models", "M")}
                        {nav_button(WorkspaceSection::Settings, "/settings", "Settings", "S")}
                    </nav>

                    <button
                        class="mt-auto flex h-11 w-11 items-center justify-center rounded-2xl border border-zinc-800 bg-zinc-900 text-xs font-semibold text-zinc-300 transition hover:border-zinc-700 hover:text-zinc-50"
                        on:click=move |_| {
                            shell.theme_preference.update(|mode| *mode = mode.toggle());
                        }
                        title="Toggle theme"
                        type="button"
                    >
                        {move || match shell.theme_preference.get() {
                            ThemePreference::Dark => "DK",
                            ThemePreference::Light => "LT",
                            ThemePreference::Auto => "AU",
                        }}
                    </button>
                </aside>

                <main class="min-w-0 flex-1">
                    <div class="border-b border-zinc-800 bg-[#171717] px-5 py-3 text-xs uppercase tracking-[0.22em] text-zinc-500 lg:px-8">
                        {route.window_title()}
                    </div>
                    <div class="px-5 py-6 lg:px-8 lg:py-8">
                        <div class="mx-auto flex max-w-5xl flex-col gap-6">
                            {children()}
                        </div>
                    </div>
                </main>
            </div>
        </div>
    }
}

#[component]
pub fn WorkspaceHeader(
    title: &'static str,
    #[prop(optional, into)] subtitle: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="flex flex-wrap items-start justify-between gap-4 border-b border-zinc-800 pb-4">
            <div class="space-y-1">
                <h1 class="text-[1.7rem] font-semibold tracking-tight text-zinc-50">{title}</h1>
                <Show when=move || subtitle.get().is_some()>
                    <p class="max-w-2xl text-sm leading-6 text-zinc-400">
                        {move || subtitle.get().unwrap_or_default()}
                    </p>
                </Show>
            </div>

            <div class="flex flex-wrap items-center gap-3">
                {children()}
            </div>
        </div>
    }
}

use leptos::prelude::*;
use leptos_router::components::A;

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
}

#[component]
pub fn WorkspaceShell(route: WorkspaceRoute, children: Children) -> impl IntoView {
    let active = route.section();

    let nav_button = move |section: WorkspaceSection, href: &'static str, label: &'static str| {
        let classes = if active == section {
            "flex h-9 w-9 items-center justify-center rounded-[0.9rem] bg-zinc-950 text-white shadow-sm dark:bg-[#34362f] dark:text-zinc-50"
        } else {
            "flex h-9 w-9 items-center justify-center rounded-[0.9rem] text-zinc-500 transition hover:bg-zinc-200 hover:text-zinc-950 dark:text-zinc-500 dark:hover:bg-[#2d2f29] dark:hover:text-zinc-100"
        };

        view! {
            <A attr:class=classes href=href attr:title=label attr:aria-label=label>
                {nav_icon(section)}
            </A>
        }
    };

    view! {
        <div class="h-screen w-full overflow-hidden bg-zinc-50 text-zinc-950 dark:bg-[#1d1f1c] dark:text-zinc-50">
            <div class="flex h-full w-full overflow-hidden">
                <aside class="flex w-[52px] shrink-0 flex-col items-center border-r border-zinc-200 bg-zinc-100 px-[7px] py-3 dark:border-white/5 dark:bg-[#22231f]">
                    <div class="flex flex-col items-center gap-2">
                        {nav_button(WorkspaceSection::Home, "/", "Home")}
                        {nav_button(WorkspaceSection::Models, "/models", "Models")}
                    </div>

                    <div class="mt-auto flex flex-col items-center gap-2">
                        {nav_button(WorkspaceSection::Settings, "/settings", "Settings")}
                    </div>
                </aside>

                <main class="min-h-0 min-w-0 flex-1 overflow-hidden bg-white dark:bg-[#2c2d29]">
                    <div class="h-full overflow-y-auto overscroll-contain">
                        <div class="flex min-h-full flex-col px-4 py-4 lg:px-6 lg:py-5">
                            <div class="flex w-full flex-col gap-5">{children()}</div>
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
        <div class="flex flex-wrap items-start justify-between gap-4 border-b border-zinc-200 pb-4 dark:border-zinc-800">
            <div class="min-w-0 space-y-1">
                <h1 class="text-[1.45rem] font-semibold tracking-tight text-zinc-950 dark:text-zinc-50">{title}</h1>
                <Show when=move || subtitle.get().is_some()>
                    <p class="max-w-2xl text-sm leading-6 text-zinc-600 dark:text-zinc-400">
                        {move || subtitle.get().unwrap_or_default()}
                    </p>
                </Show>
            </div>

            <div class="flex min-w-0 flex-wrap items-center justify-end gap-3">{children()}</div>
        </div>
    }
}

fn nav_icon(section: WorkspaceSection) -> AnyView {
    match section {
        WorkspaceSection::Home => view! {
            <svg class="h-[18px] w-[18px]" fill="none" viewBox="0 0 16 16">
                <path
                    d="M2 7L8 2L14 7V14H10V10H6V14H2V7Z"
                    stroke="currentColor"
                    stroke-linejoin="round"
                    stroke-width="1.2"
                />
            </svg>
        }
        .into_any(),
        WorkspaceSection::Models => view! {
            <svg class="h-[18px] w-[18px]" fill="none" viewBox="0 0 16 16">
                <rect
                    height="12"
                    rx="2"
                    stroke="currentColor"
                    stroke-width="1.2"
                    width="12"
                    x="2"
                    y="2"
                />
                <path d="M5 8H11" stroke="currentColor" stroke-linecap="round" stroke-width="1.2"/>
                <path d="M5 5.5H11" stroke="currentColor" stroke-linecap="round" stroke-width="1.2"/>
                <path d="M5 10.5H9" stroke="currentColor" stroke-linecap="round" stroke-width="1.2"/>
            </svg>
        }
        .into_any(),
        WorkspaceSection::Settings => view! {
            <svg class="h-[18px] w-[18px]" fill="none" viewBox="0 0 16 16">
                <circle cx="8" cy="8" r="2.5" stroke="currentColor" stroke-width="1.2"/>
                <path d="M8 1V3" stroke="currentColor" stroke-linecap="round" stroke-width="1.2"/>
                <path d="M8 13V15" stroke="currentColor" stroke-linecap="round" stroke-width="1.2"/>
                <path d="M1 8H3" stroke="currentColor" stroke-linecap="round" stroke-width="1.2"/>
                <path d="M13 8H15" stroke="currentColor" stroke-linecap="round" stroke-width="1.2"/>
                <path d="M3.1 3.1L4.5 4.5" stroke="currentColor" stroke-linecap="round" stroke-width="1.2"/>
                <path d="M11.5 11.5L12.9 12.9" stroke="currentColor" stroke-linecap="round" stroke-width="1.2"/>
                <path d="M3.1 12.9L4.5 11.5" stroke="currentColor" stroke-linecap="round" stroke-width="1.2"/>
                <path d="M11.5 4.5L12.9 3.1" stroke="currentColor" stroke-linecap="round" stroke-width="1.2"/>
            </svg>
        }
        .into_any(),
    }
}

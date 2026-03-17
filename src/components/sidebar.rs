use leptos::prelude::*;
use leptos_router::hooks::use_location;
use singlestage::{Badge, Mode, ThemeProviderContext};

use crate::{
    components::app_ui::ActionButton,
    state::app_state::{ThemePreference, use_app_shell_state},
};

#[component]
pub fn Sidebar() -> impl IntoView {
    let location = use_location();
    let shell = use_app_shell_state();
    let theme = expect_context::<ThemeProviderContext>();

    Effect::new(move |_| {
        theme.mode.set(match shell.theme_preference.get() {
            ThemePreference::Auto => Mode::Auto,
            ThemePreference::Dark => Mode::Dark,
            ThemePreference::Light => Mode::Light,
        });
    });

    let nav_item = move |href: &'static str, label: &'static str, icon: &'static str| {
        view! {
            <a
                class=move || {
                    let path = location.pathname.get();
                    let is_active = (href == "/" && path == "/") || (href != "/" && path.starts_with(href));
                    let variant = if is_active {
                        "singlestage-btn-primary"
                    } else {
                        "singlestage-btn-ghost"
                    };
                    format!("{} w-full justify-start", variant)
                }
                href=href
            >
                <span class="text-sm font-semibold">{icon}</span>
                <span>{label}</span>
            </a>
        }
    };

    view! {
        <aside class="hidden w-64 flex-col border-r border-slate-200 bg-white/80 px-4 py-5 backdrop-blur dark:border-slate-800 dark:bg-slate-950/80 lg:flex">
            <div class="space-y-3">
                <Badge variant="secondary">"Single Stage UI"</Badge>
                <div>
                    <p class="text-lg font-semibold text-slate-950 dark:text-slate-50">
                        "Local speech workspace"
                    </p>
                    <p class="mt-1 text-sm text-slate-600 dark:text-slate-300">
                        "Offline transcription flow with Leptos, Tauri, and local models."
                    </p>
                </div>
            </div>

            <nav class="mt-6 flex flex-col gap-2">
                {nav_item("/", "Home", "H")}
                {nav_item("/preview", "Preview", "P")}
                {nav_item("/transcription", "Live run", "R")}
                {nav_item("/transcript/current", "Transcript", "T")}
                {nav_item("/models", "Models", "M")}
            </nav>

            <div class="mt-auto space-y-3 border-t border-slate-200 pt-4 dark:border-slate-800">
                {nav_item("/settings", "Settings", "S")}
                <ActionButton
                    class="w-full justify-between"
                    on_click=Callback::new(move |_| {
                        shell.theme_preference.update(|mode| *mode = mode.toggle());
                    })
                    variant="outline"
                >
                    <span>"Theme"</span>
                    <span>{move || shell.theme_preference.get().label()}</span>
                </ActionButton>
            </div>
        </aside>
    }
}

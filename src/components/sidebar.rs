use leptos::prelude::*;
use leptos_darkmode::Darkmode;
use leptos_router::hooks::use_location;

#[component]
pub fn Sidebar() -> impl IntoView {
    let location = use_location();
    let mut darkmode = expect_context::<Darkmode>();
    let darkmode_label = darkmode.clone();

    let nav_item = move |href: &'static str, label: &'static str, icon: &'static str| {
        view! {
            <a
                href=href
                class=move || {
                    let path = location.pathname.get();
                    let is_active = href == "/" && path == "/"
                        || href != "/" && path.starts_with(href);
                    format!(
                        "group flex w-full items-center gap-3 rounded-2xl px-3 py-2 text-sm transition {}",
                        if is_active {
                            "bg-slate-900 text-white shadow-sm dark:bg-slate-100 dark:text-slate-950"
                        } else {
                            "text-slate-500 hover:bg-white hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-100"
                        }
                    )
                }
                title=label
            >
                <span class="text-lg leading-none">{icon}</span>
                <span>{label}</span>
            </a>
        }
    };

    view! {
        <aside class="flex w-60 flex-col border-r border-slate-200 bg-slate-100/80 px-3 py-4 dark:border-slate-800 dark:bg-slate-900/80">
            <div class="mb-6 px-2">
                <p class="text-xs font-semibold uppercase tracking-[0.24em] text-slate-400 dark:text-slate-500">
                    "Transcript"
                </p>
                <p class="mt-2 text-lg font-semibold text-slate-900 dark:text-slate-100">
                    "Local speech workspace"
                </p>
            </div>

            <nav class="flex flex-col gap-2">
                {nav_item("/", "Home", "H")}
                {nav_item("/preview", "Preview", "P")}
                {nav_item("/transcription", "Live run", "R")}
                {nav_item("/transcript/current", "Transcript", "T")}
                {nav_item("/models", "Models", "M")}
            </nav>

            <div class="mt-auto flex flex-col gap-2 border-t border-slate-200 pt-4 dark:border-slate-800">
                {nav_item("/settings", "Settings", "S")}
                <button
                    class="flex w-full items-center justify-between rounded-2xl px-3 py-2 text-sm text-slate-500 transition hover:bg-white hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-100"
                    on:click=move |_| darkmode.toggle()
                    type="button"
                >
                    <span>"Theme"</span>
                    <span class="text-xs font-medium uppercase tracking-wide">
                        {move || if darkmode_label.is_dark() { "Dark" } else { "Light" }}
                    </span>
                </button>
            </div>
        </aside>
    }
}

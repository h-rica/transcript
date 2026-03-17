use leptos::prelude::*;

#[component]
pub fn Sidebar(#[prop(into)] active: String) -> impl IntoView {
    let nav_item = move |href: &'static str, label: &'static str, is_active: bool| {
        view! {
            <a
                href=href
                class=move || format!(
                    "w-8 h-8 rounded-lg flex items-center justify-center transition-colors {}",
                    if is_active { "bg-white shadow-sm" } else { "hover:bg-gray-200" }
                )
                title=label
            >
                <div class="w-4 h-4 bg-gray-400 rounded-sm"></div>
            </a>
        }
    };

    view! {
        <div class="w-12 bg-gray-100 border-r border-gray-200 flex flex-col items-center py-3 gap-1 flex-shrink-0">
            {nav_item("/", "Home", active == "home")}
            {nav_item("/models", "Models", active == "models")}
            <div class="mt-auto">
                {nav_item("/settings", "Settings", active == "settings")}
            </div>
        </div>
    }
}

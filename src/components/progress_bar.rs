use leptos::prelude::*;

use crate::state::app_state::TranscriptionProgress;

#[component]
pub fn ProgressBar(progress: RwSignal<TranscriptionProgress>) -> impl IntoView {
    view! {
        <div class="space-y-3">
            <div class="flex items-center justify-between">
                <span class="text-sm font-medium text-slate-900 dark:text-slate-100">"Progress"</span>
                <span class="text-sm font-semibold text-slate-900 dark:text-slate-100">
                    {move || format!("{:.0}%", progress.get().percent * 100.0)}
                </span>
            </div>

            <div class="h-3 overflow-hidden rounded-full bg-slate-200 dark:bg-slate-800">
                <div
                    class="h-full rounded-full bg-slate-900 transition-all duration-500 ease-out dark:bg-slate-100"
                    style=move || format!("width: {:.2}%;", progress.get().percent.clamp(0.0, 1.0) * 100.0)
                ></div>
            </div>
        </div>
    }
}

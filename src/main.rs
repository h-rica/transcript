use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (test_passed, set_test_passed) = signal(false);
    let (count, set_count) = signal(0u32);

    view! {
        <div class="min-h-screen bg-gray-50 flex flex-col items-center justify-center p-8 font-sans">

            // Header
            <div class="text-center mb-12">
                <div class="inline-flex items-center justify-center w-16 h-16 bg-gray-900 rounded-2xl mb-4">
                    <svg width="28" height="28" viewBox="0 0 28 28" fill="none">
                        <rect x="4" y="3" width="20" height="22" rx="3" stroke="white" stroke-width="1.5"/>
                        <path d="M9 10h10M9 14h7M9 18h5" stroke="white" stroke-width="1.5" stroke-linecap="round"/>
                    </svg>
                </div>
                <h1 class="text-3xl font-semibold text-gray-900 mb-2">"Transcript"</h1>
                <p class="text-gray-500 text-base max-w-sm">
                    "Offline-first audio transcription — Tauri v2 + Leptos 0.7 + Tailwind CSS v4"
                </p>
            </div>

            // Stack badges
            <div class="flex flex-wrap gap-2 justify-center mb-12">
                <Badge label="Tauri v2" color="blue"/>
                <Badge label="Leptos 0.7" color="purple"/>
                <Badge label="Tailwind v4" color="teal"/>
                <Badge label="Singlestage UI" color="gray"/>
                <Badge label="VibeVoice ONNX" color="green"/>
            </div>

            // Component overview grid
            <div class="grid grid-cols-2 gap-4 w-full max-w-xl mb-10">
                <OverviewCard icon="🎙" title="Drop zone" desc="MP3 · WAV · M4A"/>
                <OverviewCard icon="📊" title="Live segments" desc="Real-time streaming"/>
                <OverviewCard icon="👥" title="Diarization" desc="Speaker identification"/>
                <OverviewCard icon="💾" title="Export" desc="TXT · SRT"/>
            </div>

            // Test button
            <div class="flex flex-col items-center gap-4">
                <button
                    class="inline-flex items-center gap-2 px-6 h-11 bg-gray-900 text-white text-sm font-medium rounded-xl hover:bg-gray-700 active:scale-95 transition-all cursor-pointer"
                    on:click=move |_| {
                        set_count.update(|n| *n += 1);
                        set_test_passed.set(true);
                    }
                >
                    <span>"▶  Run UI test"</span>
                    {move || if count.get() > 0 {
                        view! {
                            <span class="bg-white text-gray-900 text-xs font-semibold px-2 py-0.5 rounded-full">
                                {count}
                            </span>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </button>

                {move || if test_passed.get() {
                    view! {
                        <div class="flex items-center gap-2 text-green-600 text-sm font-medium">
                            <span>"✓ Leptos signals · Tailwind v4 · UI stack validated"</span>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <p class="text-xs text-gray-400">"Click to validate the UI stack"</p>
                    }.into_any()
                }}
            </div>

            // Footer
            <div class="mt-16 text-xs text-gray-300">
                "Phase 1 — Week 1-2 scaffold · MiicaLabs 2026"
            </div>

        </div>
    }
}

#[component]
fn Badge(label: &'static str, color: &'static str) -> impl IntoView {
    let class = match color {
        "blue" => "bg-blue-100 text-blue-800",
        "purple" => "bg-purple-100 text-purple-800",
        "teal" => "bg-teal-100 text-teal-800",
        "green" => "bg-green-100 text-green-800",
        _ => "bg-gray-100 text-gray-700",
    };
    view! {
        <span class=format!("inline-flex items-center px-3 py-1 rounded-full text-xs font-medium {class}")>
            {label}
        </span>
    }
}

#[component]
fn OverviewCard(icon: &'static str, title: &'static str, desc: &'static str) -> impl IntoView {
    view! {
        <div class="bg-white border border-gray-200 rounded-xl p-5 flex items-start gap-4 hover:border-gray-300 transition-all">
            <div class="text-2xl flex-shrink-0">{icon}</div>
            <div>
                <div class="text-sm font-medium text-gray-900 mb-0.5">{title}</div>
                <div class="text-xs text-gray-500">{desc}</div>
            </div>
        </div>
    }
}
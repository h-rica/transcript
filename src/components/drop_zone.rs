use std::path::Path;

use futures::StreamExt;
use leptos::{html, prelude::*, task::spawn_local};
use leptos_use::{UseDropZoneReturn, use_drop_zone};

use crate::state::app_state::SelectedFile;

#[component]
pub fn DropZone(on_file: Callback<SelectedFile>) -> impl IntoView {
    let drop_ref = NodeRef::<html::Div>::new();
    let native_over = RwSignal::new(false);
    let UseDropZoneReturn {
        is_over_drop_zone,
        files,
        ..
    } = use_drop_zone(drop_ref);

    Effect::new(move |_| {
        let dropped = files.read();
        if let Some(file) = dropped.first() {
            let name = file.name();
            on_file.run(SelectedFile {
                path: name.clone(),
                name,
            });
        }
    });

    if tauri_sys::core::is_tauri() {
        let on_file = on_file.clone();
        spawn_local(async move {
            let window = tauri_sys::window::Window::get_current();
            let Ok(mut listener) = window.on_drag_drop_event().await else {
                return;
            };

            while let Some(event) = listener.next().await {
                match event.payload {
                    tauri_sys::window::DragDropEvent::Enter(_)
                    | tauri_sys::window::DragDropEvent::Over(_) => {
                        native_over.set(true);
                    }
                    tauri_sys::window::DragDropEvent::Leave => {
                        native_over.set(false);
                    }
                    tauri_sys::window::DragDropEvent::Drop(payload) => {
                        native_over.set(false);

                        if let Some(path) = payload.paths().first() {
                            let absolute = path.to_string_lossy().to_string();
                            let name = Path::new(&absolute)
                                .file_name()
                                .and_then(|value| value.to_str())
                                .unwrap_or("audio-file")
                                .to_string();

                            on_file.run(SelectedFile {
                                path: absolute,
                                name,
                            });
                        }
                    }
                }
            }
        });
    }

    view! {
        <div
            node_ref=drop_ref
            class=move || {
                let is_over = native_over.get() || is_over_drop_zone.get();
                format!(
                    "group relative overflow-hidden rounded-[28px] border-2 p-8 transition-all {}",
                    if is_over {
                        "border-slate-900 bg-slate-900 text-white shadow-xl shadow-slate-900/10 dark:border-slate-100 dark:bg-slate-100 dark:text-slate-950 dark:shadow-slate-100/10"
                    } else {
                        "border-dashed border-slate-300 bg-white/90 hover:border-slate-500 hover:bg-white dark:border-slate-700 dark:bg-slate-900/90 dark:hover:border-slate-500"
                    }
                )
            }
        >
            <div class="pointer-events-none absolute inset-y-0 right-0 hidden w-48 bg-gradient-to-l from-slate-100/70 to-transparent dark:from-slate-800/40 lg:block"></div>
            <div class="relative flex flex-col gap-4">
                <div class="flex h-14 w-14 items-center justify-center rounded-2xl bg-slate-100 text-lg font-semibold text-slate-900 transition group-hover:scale-105 dark:bg-slate-800 dark:text-slate-100">
                    "AI"
                </div>

                <div class="space-y-2">
                    <h2 class="text-2xl font-semibold tracking-tight">"Drop audio to start a local transcript"</h2>
                    <p class="max-w-2xl text-sm text-slate-500 dark:text-slate-400">
                        "Supports MP3, WAV, and M4A. In Tauri, dropping from the desktop carries the full file path into the transcription flow."
                    </p>
                </div>

                <div class="flex flex-wrap items-center gap-3 text-xs font-medium uppercase tracking-[0.24em] text-slate-400 dark:text-slate-500">
                    <span>"MP3"</span>
                    <span>"WAV"</span>
                    <span>"M4A"</span>
                    <span>"Whisper Tiny bundled"</span>
                </div>
            </div>
        </div>
    }
}

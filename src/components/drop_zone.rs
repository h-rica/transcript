use std::path::Path;

use futures::StreamExt;
use leptos::{html, prelude::*, task::spawn_local};
use leptos_use::{UseDropZoneReturn, use_drop_zone};
use singlestage::Badge;
use wasm_bindgen::JsCast;

use crate::state::app_state::SelectedFile;

#[component]
pub fn DropZone(on_file: Callback<SelectedFile>) -> impl IntoView {
    let drop_ref = NodeRef::<html::Div>::new();
    let input_ref = NodeRef::<html::Input>::new();
    let native_over = RwSignal::new(false);
    let UseDropZoneReturn {
        is_over_drop_zone,
        files,
        ..
    } = use_drop_zone(drop_ref);
    let is_active = Signal::derive(move || native_over.get() || is_over_drop_zone.get());

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
        spawn_local(async move {
            let window = tauri_sys::window::Window::get_current();
            let Ok(mut listener) = window.on_drag_drop_event().await else {
                return;
            };

            while let Some(event) = listener.next().await {
                match event.payload {
                    tauri_sys::window::DragDropEvent::Enter(_)
                    | tauri_sys::window::DragDropEvent::Over(_) => native_over.set(true),
                    tauri_sys::window::DragDropEvent::Leave => native_over.set(false),
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

    let open_picker = move |_| {
        if let Some(input) = input_ref.get() {
            input.click();
        }
    };

    let pick_file = move |event: leptos::ev::Event| {
        let Some(target) = event.target() else {
            return;
        };
        let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() else {
            return;
        };
        let Some(files) = input.files() else {
            return;
        };
        let Some(file) = files.get(0) else {
            return;
        };
        let name = file.name();
        on_file.run(SelectedFile {
            path: name.clone(),
            name,
        });
    };

    view! {
        <div node_ref=drop_ref>
            <input
                accept=".mp3,.wav,.m4a,audio/*"
                class="hidden"
                node_ref=input_ref
                on:change=pick_file
                type="file"
            />

            <div class=move || {
                if is_active.get() {
                    "rounded-[1.75rem] border border-zinc-500 bg-zinc-900/80 px-6 py-10 shadow-2xl shadow-black/30"
                } else {
                    "rounded-[1.75rem] border border-dashed border-zinc-700 bg-[#171717] px-6 py-10"
                }
            }>
                <div class="mx-auto flex max-w-3xl flex-col items-center text-center">
                    <div class="flex h-14 w-14 items-center justify-center rounded-2xl border border-zinc-700 bg-zinc-900 text-sm font-semibold text-zinc-200">
                        "UP"
                    </div>
                    <h2 class="mt-5 text-2xl font-semibold tracking-tight text-zinc-50">
                        "Drop audio file here"
                    </h2>
                    <p class="mt-2 text-sm text-zinc-400">
                        "MP3 · WAV · M4A · optimized for offline desktop transcription"
                    </p>
                    <button
                        class="mt-6 rounded-xl border border-zinc-700 bg-zinc-900 px-4 py-2 text-sm font-medium text-zinc-100 transition hover:border-zinc-600 hover:bg-zinc-800"
                        on:click=open_picker
                        type="button"
                    >
                        "Browse files"
                    </button>
                    <div class="mt-6 flex flex-wrap items-center justify-center gap-2">
                        <Badge variant="outline">"Local only"</Badge>
                        <Badge variant="outline">"Speaker labels when supported"</Badge>
                        <Badge variant="outline">"No cloud upload"</Badge>
                    </div>
                </div>
            </div>
        </div>
    }
}

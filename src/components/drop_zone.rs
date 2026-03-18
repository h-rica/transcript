use std::path::Path;

use futures::StreamExt;
use leptos::{html, prelude::*, task::spawn_local};
use leptos_use::{UseDropZoneReturn, use_drop_zone};
use wasm_bindgen::JsCast;

use crate::{
    components::icons::{AppIcon, UiIcon},
    state::app_state::SelectedFile,
};

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
                    "rounded-[1.15rem] border border-zinc-950 bg-zinc-100 px-6 py-6 shadow-sm dark:border-zinc-400 dark:bg-[#15171b]"
                } else {
                    "rounded-[1.15rem] border border-dashed border-zinc-300 bg-white px-6 py-6 dark:border-zinc-800 dark:bg-[#121316]"
                }
            }>
                <div class="mx-auto flex h-[170px] max-w-3xl flex-col items-center justify-center text-center">
                    <div class=move || {
                        if is_active.get() {
                            "flex h-10 w-10 items-center justify-center rounded-xl bg-zinc-950 text-white dark:bg-zinc-100 dark:text-zinc-950"
                        } else {
                            "flex h-10 w-10 items-center justify-center rounded-xl bg-zinc-100 text-zinc-600 dark:bg-[#1a1c20] dark:text-zinc-300"
                        }
                    }>
                        <UiIcon class="h-5 w-5" icon_name=AppIcon::Upload/>
                    </div>

                    <h2 class="mt-4 text-[1.2rem] font-semibold tracking-tight text-zinc-950 dark:text-zinc-50">
                        {move || if is_active.get() {
                            "Release to transcribe"
                        } else {
                            "Drop audio file here"
                        }}
                    </h2>

                    <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-500">
                        {move || if is_active.get() {
                            "MP3 / WAV / M4A detected"
                        } else {
                            "MP3 / WAV / M4A"
                        }}
                    </p>

                    <Show when=move || !is_active.get()>
                        <button
                            class="mt-5 h-8 rounded-lg border border-zinc-200 bg-zinc-100 px-4 text-sm font-medium text-zinc-700 transition hover:bg-zinc-200 dark:border-zinc-800 dark:bg-[#1a1c20] dark:text-zinc-100 dark:hover:border-zinc-700 dark:hover:bg-[#1f2126]"
                            on:click=open_picker
                            type="button"
                        >
                            "Browse files"
                        </button>
                    </Show>
                </div>
            </div>
        </div>
    }
}

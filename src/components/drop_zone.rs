use std::path::Path;

use futures::StreamExt;
use leptos::{html, prelude::*, task::spawn_local};
use leptos_use::{UseDropZoneReturn, use_drop_zone};
use singlestage::{Badge, Card, CardContent, CardHeader, CardTitle};

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
    let card_class = Signal::derive(move || {
        let is_active = native_over.get() || is_over_drop_zone.get();
        if is_active {
            "border-2 border-slate-950 shadow-lg shadow-slate-950/10 dark:border-slate-50 dark:shadow-slate-50/10".to_string()
        } else {
            "border-2 border-dashed border-slate-300 dark:border-slate-700".to_string()
        }
    });

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

    view! {
        <div node_ref=drop_ref>
            <Card class=card_class>
                <CardHeader class="space-y-4">
                    <Badge variant="secondary">"Audio intake"</Badge>
                    <div class="space-y-2">
                        <CardTitle>"Drop audio to start a local transcript"</CardTitle>
                        <p class="text-sm text-slate-600 dark:text-slate-300">
                            "Supports MP3, WAV, and M4A. In Tauri, dragging from the desktop keeps the full local file path attached to the next step."
                        </p>
                    </div>
                </CardHeader>
                <CardContent class="flex flex-wrap items-center gap-2">
                    <Badge variant="outline">"MP3"</Badge>
                    <Badge variant="outline">"WAV"</Badge>
                    <Badge variant="outline">"M4A"</Badge>
                    <Badge variant="outline">"Offline-first"</Badge>
                </CardContent>
            </Card>
        </div>
    }
}

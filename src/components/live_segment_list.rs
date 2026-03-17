use leptos::{html, prelude::*};

use crate::state::app_state::TranscriptSegment;

#[component]
pub fn LiveSegmentList(
    segments: RwSignal<Vec<TranscriptSegment>>,
    #[prop(into)] pending: Signal<bool>,
) -> impl IntoView {
    let list_ref = NodeRef::<html::Div>::new();

    Effect::new(move |_| {
        let len = segments.with(|items| items.len());
        if len == 0 {
            return;
        }

        let Some(list) = list_ref.get() else {
            return;
        };

        let distance_from_bottom = list.scroll_height() - list.scroll_top() - list.client_height();
        if distance_from_bottom < 80 {
            list.set_scroll_top(list.scroll_height());
        }
    });

    view! {
        <div
            node_ref=list_ref
            class="flex max-h-[26rem] flex-col gap-4 overflow-auto pr-2"
        >
            {move || {
                let items = segments.get();
                if items.is_empty() {
                    view! {
                        <div class="rounded-3xl border border-dashed border-slate-300 p-6 text-sm text-slate-500 dark:border-slate-700 dark:text-slate-400">
                            "Live transcript segments will appear here as the backend emits them."
                        </div>
                    }.into_any()
                } else {
                    let len = items.len();
                    items
                        .into_iter()
                        .enumerate()
                        .map(|(index, segment)| {
                            let colors = speaker_palette(&segment.speaker);
                            let show_cursor = pending.get() && index == len.saturating_sub(1);

                            view! {
                                <article class="flex gap-3">
                                    <div
                                        class="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-full text-xs font-semibold"
                                        style=format!("background:{}; color:{};", colors.0, colors.1)
                                    >
                                        {speaker_initial(&segment.speaker)}
                                    </div>

                                    <div class="flex-1">
                                        <div class="rounded-3xl bg-slate-100 px-4 py-3 text-sm leading-6 text-slate-800 dark:bg-slate-900 dark:text-slate-200">
                                            <div class="mb-1 flex items-center justify-between gap-4">
                                                <span class="font-semibold">{segment.speaker.clone()}</span>
                                                <span class="text-xs text-slate-400 dark:text-slate-500">
                                                    {format!("{} -> {}", format_duration(segment.start_s), format_duration(segment.end_s))}
                                                </span>
                                            </div>
                                            <span>{segment.text.clone()}</span>
                                            {if show_cursor {
                                                view! { <span class="ml-1 inline-block h-4 w-1 animate-pulse rounded bg-slate-900 align-middle dark:bg-slate-100"></span> }.into_any()
                                            } else {
                                                view! { <></> }.into_any()
                                            }}
                                        </div>
                                    </div>
                                </article>
                            }
                        })
                        .collect_view()
                        .into_any()
                }
            }}
        </div>
    }
}

fn speaker_initial(speaker: &str) -> String {
    speaker
        .chars()
        .find(|character| character.is_ascii_alphanumeric())
        .map(|character| character.to_ascii_uppercase().to_string())
        .unwrap_or_else(|| "S".into())
}

fn speaker_palette(speaker: &str) -> (&'static str, &'static str) {
    let hash = speaker.bytes().fold(0u8, |acc, byte| acc.wrapping_add(byte)) % 4;
    match hash {
        0 => ("#dbeafe", "#1d4ed8"),
        1 => ("#fce7f3", "#9d174d"),
        2 => ("#dcfce7", "#166534"),
        _ => ("#ede9fe", "#5b21b6"),
    }
}

fn format_duration(seconds: f32) -> String {
    let total_seconds = seconds.max(0.0).round() as u32;
    let minutes = total_seconds / 60;
    let remaining_seconds = total_seconds % 60;
    format!("{minutes:02}:{remaining_seconds:02}")
}

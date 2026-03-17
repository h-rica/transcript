use leptos::{html, prelude::*};

use crate::{
    features::shared::{format_mm_ss, speaker_initial, speaker_palette},
    state::app_state::TranscriptSegment,
};

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
        <div node_ref=list_ref class="flex max-h-[28rem] flex-col gap-3 overflow-auto pr-1">
            {move || {
                let items = segments.get();
                if items.is_empty() {
                    view! {
                        <div class="rounded-2xl border border-dashed border-zinc-300 bg-zinc-100/80 px-5 py-8 text-center dark:border-zinc-800 dark:bg-[#121316]">
                            <p class="text-sm font-medium text-zinc-950 dark:text-zinc-100">"No live segments yet"</p>
                            <p class="mt-2 text-sm text-zinc-600 dark:text-zinc-500">
                                "Segments will appear here once the model starts emitting transcript text."
                            </p>
                        </div>
                    }
                    .into_any()
                } else {
                    let len = items.len();
                    items
                        .into_iter()
                        .enumerate()
                        .map(|(index, segment)| {
                            let (background, foreground) = speaker_palette(&segment.speaker);
                            let show_cursor = pending.get() && index == len.saturating_sub(1);
                            let time_range = format!(
                                "{} -> {}",
                                format_mm_ss(segment.start_s),
                                format_mm_ss(segment.end_s)
                            );

                            view! {
                                <div class="rounded-2xl border border-zinc-200 bg-white px-4 py-4 dark:border-zinc-900 dark:bg-[#141519]">
                                    <div class="flex gap-4">
                                        <div
                                            class="flex h-9 w-9 flex-shrink-0 items-center justify-center rounded-full text-xs font-semibold"
                                            style=format!("background:{}; color:{};", background, foreground)
                                        >
                                            {speaker_initial(&segment.speaker)}
                                        </div>
                                        <div class="min-w-0 flex-1 space-y-2">
                                            <div class="flex flex-wrap items-center justify-between gap-2">
                                                <span class="inline-flex items-center rounded-full border border-zinc-200 px-2.5 py-1 text-[11px] font-medium text-zinc-700 dark:border-zinc-700 dark:text-zinc-300">
                                                    {segment.speaker.clone()}
                                                </span>
                                                <span class="text-[11px] text-zinc-500 dark:text-zinc-500">{time_range}</span>
                                            </div>
                                            <p class="text-sm leading-7 text-zinc-900 dark:text-zinc-100">
                                                {segment.text.clone()}
                                                <Show when=move || show_cursor>
                                                    <span class="ml-1 inline-block h-4 w-1 animate-pulse rounded bg-zinc-900 align-middle dark:bg-zinc-100"></span>
                                                </Show>
                                            </p>
                                        </div>
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()
                        .into_any()
                }
            }}
        </div>
    }
}

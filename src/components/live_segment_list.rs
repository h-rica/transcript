use leptos::{html, prelude::*};
use singlestage::{
    Card, CardContent, Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyTitle,
};

use crate::{
    components::app_ui::SpeakerPill,
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
        <div node_ref=list_ref class="flex max-h-[28rem] flex-col gap-4 overflow-auto pr-2">
            {move || {
                let items = segments.get();
                if items.is_empty() {
                    view! {
                        <Empty>
                            <EmptyHeader>
                                <EmptyTitle>"No live segments yet"</EmptyTitle>
                                <EmptyDescription>
                                    "Transcript segments will stream here as the Rust backend emits progress events."
                                </EmptyDescription>
                            </EmptyHeader>
                            <EmptyContent>
                                <p class="text-sm text-slate-500 dark:text-slate-400">
                                    "Start a run from the preview screen to populate this feed."
                                </p>
                            </EmptyContent>
                        </Empty>
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
                                <Card>
                                    <CardContent class="flex gap-4 p-4">
                                        <div
                                            class="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-full text-xs font-semibold"
                                            style=format!("background:{}; color:{};", background, foreground)
                                        >
                                            {speaker_initial(&segment.speaker)}
                                        </div>
                                        <div class="min-w-0 flex-1 space-y-3">
                                            <div class="flex flex-wrap items-center justify-between gap-2">
                                                <SpeakerPill name=segment.speaker.clone()/>
                                                <span class="text-xs text-slate-500 dark:text-slate-400">
                                                    {time_range}
                                                </span>
                                            </div>
                                            <p class="text-sm leading-6 text-slate-700 dark:text-slate-200">
                                                {segment.text.clone()}
                                                <Show when=move || show_cursor>
                                                    <span class="ml-1 inline-block h-4 w-1 animate-pulse rounded bg-slate-900 align-middle dark:bg-slate-100"></span>
                                                </Show>
                                            </p>
                                        </div>
                                    </CardContent>
                                </Card>
                            }
                        })
                        .collect_view()
                        .into_any()
                }
            }}
        </div>
    }
}

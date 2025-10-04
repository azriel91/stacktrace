use leptos::{
    component,
    either::Either,
    prelude::{ClassAttribute, ElementChild, For, Get, IntoAny, OnAttribute, Write},
    view, IntoView,
};
use stacktrace::sem_log::LogBlock;

use crate::components::LogLineSegmentsDiv;

#[component]
pub fn LogBlockDiv(log_block: LogBlock<'static>) -> impl IntoView {
    let expanded_initially = log_block.children.is_empty();
    let (expanded, expanded_set) = leptos::prelude::signal(expanded_initially);
    if log_block.children.is_empty() {
        Either::Left(view! {
            <div class="hover:bg-gray-600 py-1 rounded">
                <LogLineSegmentsDiv
                    expanded
                    line_segments={log_block.line_segments.clone()}
                    line_segments_collapsed={log_block.line_segments_collapsed.clone()}
                />
            </div>
        })
    } else {
        Either::Right(view! {
            <div class="py-1 rounded">
                <details
                    open={move || expanded.get()}
                    on:toggle=move |event| *expanded_set.write() = event.new_state() == "open"
                >
                    <summary class="hover:bg-gray-600 py-1 rounded">
                        <LogLineSegmentsDiv
                            expanded
                            line_segments={log_block.line_segments.clone()}
                            line_segments_collapsed={log_block.line_segments_collapsed.clone()}
                        />
                    </summary>
                    <div>
                        <For
                            each=move || log_block.children.clone()
                            key=LogBlock::hash_with_default_hasher
                            children=|log_block| view! { <LogBlockDiv log_block /> }
                        />
                    </div>
                </details>
            </div>
        })
    }
    .into_any()
}

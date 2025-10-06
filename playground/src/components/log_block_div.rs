use leptos::{
    component,
    either::Either,
    prelude::{ClassAttribute, ElementChild, For, Get, IntoAny, OnAttribute, Write},
    view, IntoView,
};
use stacktrace::sem_log::LogBlock;

use crate::components::LogLineSegmentsDiv;

const LINE_CLASSES: &str = "\
    hover:bg-gray-600 \
    py-1 \
    rounded \
";

/// Multiple `hover:open` selectors so that we only highlight the border if
/// there are no nested [`LogBlockDiv`]s that are also hovered.
const BLOCK_CLASSES: &str = "\
    rounded-lg \
    border-s \
    border-transparent \
    open:px-2 \
    open:border-blue-400 \
    hover:open:border-blue-200 \
    hover:open:has-[:hover:open]:border-blue-400 \
";

#[component]
pub fn LogBlockDiv(log_block: LogBlock<'static>) -> impl IntoView {
    let expanded_initially = log_block.children.is_empty();
    let (expanded, expanded_set) = leptos::prelude::signal(expanded_initially);
    if log_block.children.is_empty() {
        Either::Left(view! {
            <div class=LINE_CLASSES>
                <LogLineSegmentsDiv
                    expanded
                    line_segments={log_block.line_segments.clone()}
                    line_segments_collapsed={log_block.line_segments_collapsed.clone()}
                    children_collapsed_text={log_block.children_collapsed_text.clone()}
                />
            </div>
        })
    } else {
        Either::Right(view! {
            <div class="py-1 rounded">
                <details
                    open={move || expanded.get()}
                    on:toggle=move |event| *expanded_set.write() = event.new_state() == "open"
                    class=BLOCK_CLASSES
                >
                    <summary class=LINE_CLASSES>
                        <LogLineSegmentsDiv
                            expanded
                            line_segments={log_block.line_segments.clone()}
                            line_segments_collapsed={log_block.line_segments_collapsed.clone()}
                            children_collapsed_text={log_block.children_collapsed_text.clone()}
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

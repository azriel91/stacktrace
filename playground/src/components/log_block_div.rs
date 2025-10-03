use leptos::{
    component,
    either::Either,
    prelude::{ClassAttribute, ElementChild, For, IntoAny},
    view, IntoView,
};
use stacktrace::sem_log::{LogBlock, LogLineSegment};

use crate::components::LogLineSegmentSpan;

#[component]
pub fn LogBlockDiv(log_block: LogBlock<'static>) -> impl IntoView {
    if log_block.children.is_empty() {
        Either::Left(view! {
            <div class="hover:bg-gray-600 py-1 rounded">
                <For
                    each=move || log_block.line_segments.clone()
                    key=LogLineSegment::hash_with_default_hasher
                    children=|line_segment| view! { <LogLineSegmentSpan line_segment /> }
                />
            </div>
        })
    } else {
        Either::Right(view! {
            <div class="border border-blue-300 py-1 rounded">
                <details open>
                    <summary>
                        <For
                            each=move || log_block.line_segments.clone()
                            key=LogLineSegment::hash_with_default_hasher
                            children=|line_segment| view! { <LogLineSegmentSpan line_segment /> }
                        />
                    </summary>
                    <div class="pl-2">
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

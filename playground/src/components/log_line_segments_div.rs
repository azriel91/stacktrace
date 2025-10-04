use leptos::{
    component,
    prelude::{ElementChild, For, Get, ReadSignal},
    view, IntoView,
};
use stacktrace::sem_log::LogLineSegment;

use crate::components::LogLineSegmentSpan;

#[component]
pub fn LogLineSegmentsDiv(
    expanded: ReadSignal<bool>,
    line_segments: Vec<LogLineSegment<'static>>,
    line_segments_collapsed: Vec<LogLineSegment<'static>>,
) -> impl IntoView {
    view! {
        <div>
            <For
                each=move || {
                    if expanded.get() {
                        line_segments.clone()
                    } else {
                        line_segments_collapsed.clone()
                    }
                }
                key=LogLineSegment::hash_with_default_hasher
                children=|line_segment| view! { <LogLineSegmentSpan line_segment /> }
            />
        </div>
    }
}

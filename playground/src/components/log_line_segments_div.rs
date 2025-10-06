use std::borrow::Cow;

use leptos::{
    component,
    either::Either,
    prelude::{ClassAttribute, ElementChild, For, Get, ReadSignal},
    view, IntoView,
};
use stacktrace::sem_log::LogLineSegment;

use crate::components::LogLineSegmentSpan;

#[component]
pub fn LogLineSegmentsDiv(
    expanded: ReadSignal<bool>,
    line_segments: Vec<LogLineSegment<'static>>,
    line_segments_collapsed: Vec<LogLineSegment<'static>>,
    children_collapsed_text: Cow<'static, str>,
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
            { move || {
                if !expanded.get() {
                    let children_collapsed_text = children_collapsed_text.clone().to_string();
                    Either::Left(view! {
                        <div class="border border-gray-800 bg-gray-600 rounded px-1 ml-4">
                            {children_collapsed_text}
                        </div>
                    })
                } else {
                    Either::Right(())
                }
            }}
        </div>
    }
}

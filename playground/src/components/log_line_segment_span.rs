use leptos::{
    component,
    prelude::{ClassAttribute, ElementChild},
    view, IntoView,
};
use stacktrace::sem_log::{LogLineSegment, LogLineSegmentKind};

#[component]
pub fn LogLineSegmentSpan(line_segment: LogLineSegment<'static>) -> impl IntoView {
    let classes = match line_segment.kind {
        LogLineSegmentKind::Context => "",
        LogLineSegmentKind::CommonWithParent => "opacity-25",
        LogLineSegmentKind::Introduced => "",
        LogLineSegmentKind::CollapsedBlockPlaceholder => {
            "border border-yellow-800 bg-yellow-700 rounded px-1 ml-2"
        }
    };
    view! {
        <span class=classes>{line_segment.text}{line_segment.separator}</span>
    }
}

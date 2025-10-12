use std::borrow::Cow;

use leptos::{
    component,
    either::Either,
    prelude::{ClassAttribute, ElementChild, For, Get, IntoAny, OnAttribute, Write},
    view, IntoView,
};
use stacktrace::sem_log::LogBlock;

use crate::components::LogLineSegmentsDiv;

const LINE_CLASSES: &str = "\
    border \
    border-transparent \
    hover:border-blue-400 \
    px-2 \
    py-1 \
    rounded-lg \
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

const BLOCK_BG_COLOURS: [&str; 6] = [
    "bg-teal-950",
    "bg-sky-950",
    "bg-blue-950",
    "bg-indigo-950",
    "bg-violet-950",
    "bg-purple-950",
];

#[component]
pub fn LogBlockDiv(
    log_block: LogBlock<'static>,
    #[prop(optional)] block_index: usize,
) -> impl IntoView {
    let expanded_initially = log_block.children.is_empty() ||
        // expand the first two levels
        log_block.nesting_level < 2;
    let (expanded, expanded_set) = leptos::prelude::signal(expanded_initially);
    if log_block.children.is_empty() {
        let classes = match log_block.nesting_level {
            // On second level blocks, cycle through background colours.
            1 => {
                let bg_colour = BLOCK_BG_COLOURS[block_index % BLOCK_BG_COLOURS.len()];
                Cow::Owned(format!("{LINE_CLASSES} {bg_colour}"))
            }
            _ => Cow::Borrowed(LINE_CLASSES),
        };
        Either::Left(view! {
            <div class=classes>
                <LogLineSegmentsDiv
                    expanded
                    line_segments={log_block.line_segments.clone()}
                    line_segments_collapsed={log_block.line_segments_collapsed.clone()}
                    children_collapsed_text={log_block.children_collapsed_text.clone()}
                />
            </div>
        })
    } else {
        let classes = match log_block.nesting_level {
            // On second level blocks, cycle through background colours.
            1 => {
                let bg_colour = BLOCK_BG_COLOURS[block_index % BLOCK_BG_COLOURS.len()];
                Cow::Owned(format!("{BLOCK_CLASSES} {bg_colour}"))
            }
            _ => Cow::Borrowed(BLOCK_CLASSES),
        };
        Either::Right(view! {
            <div class="py-1 rounded">
                <details
                    open={move || expanded.get()}
                    on:toggle=move |event| *expanded_set.write() = event.new_state() == "open"
                    class=classes
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
                            each=move || log_block.children.clone().into_iter().enumerate()
                            key=|(_index, log_block)| log_block.hash_with_default_hasher()
                            children=|(block_index, log_block)| view! { <LogBlockDiv log_block block_index /> }
                        />
                    </div>
                </details>
            </div>
        })
    }
    .into_any()
}
